use crate::prediction::Prediction;
use image::{self, imageops::FilterType};
use std::io::Read;
use tract_onnx::prelude::*;

const SIZE: usize = 224;
const SIZE_U32: u32 = 224;

pub struct Model {
    model: SimplePlan<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>>,
}

impl Model {
    pub fn load<R: Read>(mut model_reader: R) -> Result<Self, Box<dyn std::error::Error>> {
        let model = tract_onnx::onnx()
            .model_for_read(&mut model_reader.by_ref())?
            .with_input_fact(
                0,
                InferenceFact::dt_shape(f32::datum_type(), tvec!(1, SIZE, SIZE, 3)),
            )?
            .into_optimized()?
            .into_runnable()?;

        Ok(Self { model })
    }

    pub fn classify<R: Read>(
        &self,
        mut image_reader: R,
    ) -> Result<Vec<Prediction>, Box<dyn std::error::Error>> {
        let mut image_data = Vec::new();
        image_reader.read_to_end(&mut image_data)?;

        let image_format = image::guess_format(&image_data)?;
        let image = image::load_from_memory_with_format(&image_data, image_format)?.to_rgb8();
        let resized = image::imageops::resize(&image, SIZE_U32, SIZE_U32, FilterType::Triangle);
        let image: Tensor =
            tract_ndarray::Array4::from_shape_fn((1, SIZE, SIZE, 3), |(_, y, x, c)| {
                resized[(x as _, y as _)][c] as f32 / 255.0
            })
            .into();

        let result = self.model.run(tvec!(image.into()))?;
        let best = result[0].to_array_view::<f32>()?;

        Ok(best
            .iter()
            .enumerate()
            .map(|(i, &p)| match i {
                0 => Prediction::Drawing(p),
                1 => Prediction::Hentai(p),
                2 => Prediction::Neutral(p),
                3 => Prediction::Porn(p),
                4 => Prediction::Sexy(p),
                _ => unreachable!(),
            })
            .collect())
    }
}
