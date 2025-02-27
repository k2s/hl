// std imports
use std::boxed::Box;
use std::io;
use std::num::{ParseIntError, TryFromIntError};
use std::path::PathBuf;
use std::sync::mpsc;

// third-party imports
use config::ConfigError;
use nu_ansi_term::Color;
use thiserror::Error;

/// Error is an error which may occur in the application.
#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    SizeParseError(#[from] SizeParseError),
    #[error(transparent)]
    NonZeroSizeParseError(#[from] NonZeroSizeParseError),
    #[error("failed to load configuration: {0}")]
    Config(#[from] ConfigError),
    #[error(transparent)]
    Infallible(#[from] std::convert::Infallible),
    #[error(transparent)]
    Capnp(#[from] capnp::Error),
    #[error(transparent)]
    Bincode(#[from] bincode::Error),
    #[error(transparent)]
    Boxed(#[from] Box<dyn std::error::Error + std::marker::Send>),
    #[error("file {filename:?} not found")]
    FileNotFoundError { filename: String },
    #[error(transparent)]
    InvalidLevel(#[from] InvalidLevelError),
    #[error("cannot recognize time {0:?}")]
    UnrecognizedTime(String),
    #[error("unknown theme {name:?}, use any of {known:?}")]
    UnknownTheme { name: String, known: Vec<String> },
    #[error("failed to parse utf-8 string: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("failed to construct utf-8 string from bytes: {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("failed to parse yaml: {0}")]
    YamlError(#[from] serde_yaml::Error),
    #[error("wrong field filter format: {0}")]
    WrongFieldFilter(String),
    #[error("wrong regular expression: {0}")]
    WrongRegularExpression(#[from] regex::Error),
    #[error("inconsistent index: {details}")]
    InconsistentIndex { details: String },
    #[error("failed to open file '{}' for reading: {source}", HILITE.paint(.path.to_string_lossy()))]
    FailedToOpenFileForReading {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("failed to open file '{}' for writing: {source}", HILITE.paint(.path.to_string_lossy()))]
    FailedToOpenFileForWriting {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("failed to get metadata of file '{}': {source}", HILITE.paint(.path.to_string_lossy()))]
    FailedToGetFileMetadata {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("invalid index header")]
    InvalidIndexHeader,
    #[error("requested sorting of messages in {} file '{}' that is not currently supported", HILITE.paint(.format), HILITE.paint(.path.to_string_lossy()))]
    UnsupportedFormatForIndexing { path: PathBuf, format: String },
    #[error("failed to parse json: {0}")]
    JsonParseError(#[from] serde_json::Error),
    #[error(transparent)]
    TryFromIntError(#[from] TryFromIntError),
    #[error(transparent)]
    NotifyError(#[from] notify::Error),
    #[error("failed to receive from mpsc channel: {source}")]
    RecvTimeoutError {
        #[source]
        source: mpsc::RecvTimeoutError,
    },
}

/// SizeParseError is an error which may occur when parsing size.
#[derive(Error, Debug)]
pub enum SizeParseError {
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error(transparent)]
    TryFromIntError(#[from] TryFromIntError),
    #[error(
        "invalid size {0:?}, use {:?} or {:?} format for IEC units or {:?} format for SI units",
        "64K",
        "64KiB",
        "64KB"
    )]
    InvalidSize(String),
}

/// NonZeroSizeParseError is an error which may occur when parsing non-zero size.
#[derive(Error, Debug)]
pub enum NonZeroSizeParseError {
    #[error(transparent)]
    SizeParseError(#[from] SizeParseError),
    #[error("zero size")]
    ZeroSize,
}

/// NonZeroSizeParseError is an error which may occur when parsing non-zero size.
#[derive(Error, Debug)]
#[error("invalid level {value:?}, use any of {valid_values:?}")]
pub struct InvalidLevelError {
    pub value: String,
    pub valid_values: Vec<String>,
}

/// Result is an alias for standard result with bound Error type.
pub type Result<T> = std::result::Result<T, Error>;

pub const HILITE: Color = Color::Yellow;
