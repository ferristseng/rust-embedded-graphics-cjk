use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuildError {
    #[error("io error")]
    IoError(#[from] io::Error),

    #[error("error writing image")]
    ImageError(#[from] image::ImageError),

    #[error("reading font")]
    ReadFontError { message: &'static str },
}
