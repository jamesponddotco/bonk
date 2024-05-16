use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(tag = "category", content = "probability")]
#[serde(rename_all = "lowercase")]
pub enum Prediction {
    Drawing(f32),
    Hentai(f32),
    Neutral(f32),
    Porn(f32),
    Sexy(f32),
}
