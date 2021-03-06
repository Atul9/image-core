
use std::error::Error;
use std::{fmt, io};
use crate::colortype::ColorType;

/// An enumeration of Image errors
#[derive(Debug)]
pub enum ImageError {
    /// The Image is not formatted properly
    InvalidData(String),

    /// The Image's dimensions are either too small or too large
    DimensionError,

    /// This image format is not supported
    UnsupportedFormat(String),

    /// This color type is not supported
    UnsupportedColor(ColorType),

    /// Unsupported feature in a image format
    UnsupportedFeature(String),

    /// Not enough data was provided to the Decoder
    /// to decode the image
    NotEnoughData,

    /// An I/O Error occurred while decoding the image
    IoError(io::Error),

    /// The end of the image has been reached
    ImageEnd,

    /// There is not enough memory to complete the given operation
    InsufficientMemory,
}

impl fmt::Display for ImageError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            ImageError::InvalidData(ref e) => write!(fmt, "Invalid data: {}", e),
            ImageError::DimensionError => write!(
                fmt,
                "The Image's dimensions are either too \
                 small or too large"
            ),
            ImageError::UnsupportedFormat(ref f) => write!(
                fmt,
                "The format `{}` is not supported",
                f
            ),
            ImageError::UnsupportedColor(ref c) => write!(
                fmt,
                "The color type {:?} is not supported",
                c
            ),
            ImageError::UnsupportedFeature(ref fe) => write!(
                fmt,
                "Unsupported feature '{}'",
                fe,
            ),
            ImageError::NotEnoughData => write!(
                fmt,
                "Not enough data was provided to the \
                 Decoder to decode the image"
            ),
            ImageError::IoError(ref e) => e.fmt(fmt),
            ImageError::ImageEnd => write!(fmt, "The end of the image has been reached"),
            ImageError::InsufficientMemory => write!(fmt, "Insufficient memory"),
        }
    }
}

impl Error for ImageError {
    fn description(&self) -> &str {
        match *self {
            ImageError::InvalidData(..) => "Invalid datar",
            ImageError::DimensionError => "Dimension error",
            ImageError::UnsupportedFormat(..) => "Unsupported format",
            ImageError::UnsupportedColor(..) => "Unsupported color",
            ImageError::UnsupportedFeature(..) => "Unsupported feature",
            ImageError::NotEnoughData => "Not enough data",
            ImageError::IoError(..) => "IO error",
            ImageError::ImageEnd => "Image end",
            ImageError::InsufficientMemory => "Insufficient memory",
        }
    }

    // TODO: use `Error::source` when minimal rust version is updated
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            ImageError::IoError(ref e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for ImageError {
    fn from(err: io::Error) -> ImageError {
        ImageError::IoError(err)
    }
}

impl From<ImageError> for io::Error {
    fn from(err: ImageError) -> io::Error {
        match err {
            ImageError::IoError(err) => err,
            err => io::Error::new(
                io::ErrorKind::Other,
                err.description()
            )
        }
    }
}

/// Result of an image decoding/encoding process
pub type ImageResult<T> = Result<T, ImageError>;
