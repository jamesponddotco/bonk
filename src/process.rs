use crate::model::Model;
use crate::prediction::Prediction;
use rayon::prelude::*;
use serde::Serialize;
use std::ffi::OsStr;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

const EXIT_DIRECTORY_NOT_FOUND: i32 = 2;
const EXIT_DIRECTORY_IO_ERROR: i32 = 5;
const EXIT_DIRECTORY_PERMISSION_DENIED: i32 = 13;
const EXIT_IMAGE_CLASSIFICATION_ERROR: i32 = EXIT_DIRECTORY_IO_ERROR;
const EXIT_FILE_NOT_FOUND: i32 = EXIT_DIRECTORY_NOT_FOUND;
const EXIT_FILE_PERMISSION_DENIED: i32 = EXIT_DIRECTORY_PERMISSION_DENIED;
const EXIT_FILE_IO_ERROR: i32 = EXIT_DIRECTORY_IO_ERROR;

#[derive(Serialize)]
pub struct Output {
    has_nudity: bool,
    path: String,
    filename: String,
    predictions: Vec<Prediction>,
}

#[derive(Serialize)]
pub struct SingleOutput {
    has_nudity: bool,
    path: String,
    filename: String,
    predictions: Vec<Prediction>,
}

#[derive(Serialize)]
pub struct MultiOutput {
    images: Vec<Output>,
}

pub fn image(model: &Model, path: &PathBuf, threshold: f32) -> Result<SingleOutput, i32> {
    let image_file = match File::open(path) {
        Ok(file) => file,
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => {
                eprintln!("Error: Image file '{}' not found.", path.display());
                eprintln!("Make sure the file path is correct and the file exists.");
                return Err(EXIT_FILE_NOT_FOUND);
            }
            std::io::ErrorKind::PermissionDenied => {
                eprintln!(
                    "Error: Permission denied when opening image file '{}'.",
                    path.display()
                );
                eprintln!("Check the file permissions and ensure you have read access.");
                return Err(EXIT_FILE_PERMISSION_DENIED);
            }
            _ => {
                eprintln!(
                    "Error: Failed to open image file '{}': {}",
                    path.display(),
                    err
                );
                return Err(EXIT_FILE_IO_ERROR);
            }
        },
    };

    let predictions = match model.classify(BufReader::new(image_file)) {
        Ok(preds) => preds,
        Err(err) => {
            eprintln!(
                "Error: Failed to classify image '{}': {}",
                path.display(),
                err
            );
            eprintln!("Make sure the image file is a valid JPEG, PNG, GIF, or WebP file.");
            return Err(EXIT_IMAGE_CLASSIFICATION_ERROR);
        }
    };

    let has_nudity = predictions.iter().any(|p| match p {
        Prediction::Porn(score) | Prediction::Hentai(score) => *score > threshold,
        _ => false,
    });

    Ok(SingleOutput {
        has_nudity,
        path: path
            .parent()
            .unwrap()
            .canonicalize()
            .unwrap()
            .to_string_lossy()
            .into_owned(),
        filename: path.file_name().unwrap().to_string_lossy().into_owned(),
        predictions,
    })
}

pub fn directory(model: &Model, path: &Path, threshold: f32) -> Result<MultiOutput, i32> {
    let images: Vec<Output> = match path.read_dir() {
        Ok(entries) => entries
            .par_bridge()
            .filter_map(|entry| {
                let entry = match entry {
                    Ok(entry) => entry,
                    Err(err) => {
                        eprintln!(
                            "Error: Failed to read directory entry in '{}': {}",
                            path.display(),
                            err
                        );
                        return None;
                    }
                };

                let file_path = entry.path();

                if !file_path.is_file() {
                    return None;
                }

                let extension = match file_path.extension().and_then(OsStr::to_str) {
                    Some(ext) => ext.to_lowercase(),
                    None => return None,
                };

                if !["jpg", "jpeg", "png", "gif", "webp"].contains(&extension.as_str()) {
                    return None;
                }

                let image_file = match File::open(&file_path) {
                    Ok(file) => file,
                    Err(err) => {
                        eprintln!(
                            "Error: Failed to open image file '{}': {}",
                            file_path.display(),
                            err
                        );
                        eprintln!("Make sure the file is a valid image format and you have read permissions.");
                        return None;
                    }
                };

                let predictions = match model.classify(BufReader::new(image_file)) {
                    Ok(preds) => preds,
                    Err(err) => {
                        eprintln!(
                            "Error: Failed to classify image '{}': {}",
                            file_path.display(),
                            err
                        );
                        eprintln!("Make sure the image file is a valid JPEG, PNG, GIF, or WebP file.");
                        return None;
                    }
                };

                let has_nudity = predictions.iter().any(|p| match p {
                    Prediction::Porn(score) | Prediction::Hentai(score) => *score > threshold,
                    _ => false,
                });

                Some(Output {
                    has_nudity,
                    path: file_path.parent().unwrap().canonicalize().unwrap().to_string_lossy().into_owned(),
                    filename: file_path
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .into_owned(),
                    predictions,
                })
            })
            .collect(),
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => {
                eprintln!("Error: Directory '{}' not found.", path.display());
                eprintln!("Make sure the directory path is correct and exists.");
                return Err(EXIT_DIRECTORY_NOT_FOUND);
            }
            std::io::ErrorKind::PermissionDenied => {
                eprintln!(
                    "Error: Permission denied when reading directory '{}'.",
                    path.display()
                );
                eprintln!("Check the directory permissions and ensure you have read access.");
                return Err(EXIT_DIRECTORY_PERMISSION_DENIED);
            }
            _ => {
                eprintln!("Error: Failed to read directory '{}': {}", path.display(), err);
                return Err(EXIT_DIRECTORY_IO_ERROR);
            }
        },
    };

    Ok(MultiOutput { images })
}
