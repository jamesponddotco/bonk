use clap::Parser;
use std::path::PathBuf;

mod model;
mod output;
mod prediction;
mod process;

use model::Model;

const EXIT_SUCCESS: i32 = 0;
const EXIT_MODEL_LOADING_ERROR: i32 = 79;

#[derive(Parser)]
#[command(name = "bonk")]
#[command(about = "Use machine learning to detect nudity in images.", long_about = None)]
#[command(version)]
struct Cli {
    #[arg(help = "path to an image file or directory of images to analyze")]
    path: PathBuf,

    #[arg(
        short,
        long,
        default_value_t = 0.7,
        help = "probability threshold above which an image is considered to contain nudity (0.0 - 1.0)"
    )]
    threshold: f32,

    #[arg(short, long, default_value_t = num_cpus::get(), help = "number of images to process concurrently")]
    parallel: usize,
}

fn main() {
    let cli = Cli::parse();

    let model_data = include_bytes!("../data/model.onnx");
    let model = match Model::load(&model_data[..]) {
        Ok(model) => model,
        Err(err) => {
            eprintln!("Error: Failed to load the nudity detection model.");
            eprintln!("Make sure the program was compiled with the correct model data.");
            eprintln!("Error: {}", err);
            std::process::exit(EXIT_MODEL_LOADING_ERROR);
        }
    };

    rayon::ThreadPoolBuilder::new()
        .num_threads(cli.parallel)
        .build_global()
        .unwrap();

    let path = cli.path;

    match path.is_dir() {
        true => match process::directory(&model, &path, cli.threshold) {
            Ok(output) => match output::print(&output) {
                Ok(_) => std::process::exit(EXIT_SUCCESS),
                Err(code) => std::process::exit(code),
            },
            Err(code) => std::process::exit(code),
        },
        false => match process::image(&model, &path, cli.threshold) {
            Ok(output) => match output::print(&output) {
                Ok(_) => std::process::exit(EXIT_SUCCESS),
                Err(code) => std::process::exit(code),
            },
            Err(code) => std::process::exit(code),
        },
    }
}
