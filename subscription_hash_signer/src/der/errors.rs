use std::result;

use base64::DecodeError;
use hex::FromHexError;
use pem::PemError;
use signature::Error as SignatureError;
use thiserror::Error;

use types::crypto;

/// A specialized `std::result::Result` type for cryptographic errors.
pub type Result<T> = result::Result<T, Error>;

/// Cryptographic errors.
#[derive(Debug, Error)]
pub enum Error {
    /// Error resulting from creating or using asymmetric key types.
    #[error("asymmetric key error: {0}")]
    AsymmetricKey(String),

    /// Error resulting when decoding a type from a hex-encoded representation.
    #[error("parsing from hex: {0}")]
    FromHex(#[from] FromHexError),

    /// Error resulting when decoding a type from a base64 representation.
    #[error("decoding error: {0}")]
    FromBase64(#[from] DecodeError),

    /// Pem format error.
    #[error("pem error: {0}")]
    FromPem(String),

    /// DER format error.
    #[error("der error: {0}")]
    FromDer(#[from] derp::Error),

    /// Error trying to manipulate the system key.
    #[error("invalid operation on system key: {0}")]
    System(String),

    /// Error related to the underlying signature crate.
    #[error("error in signature")]
    Signature(SignatureError),
}

impl From<PemError> for Error {
    fn from(error: PemError) -> Self {
        Error::FromPem(error.to_string())
    }
}

impl From<crypto::Error> for Error {
    fn from(error: crypto::Error) -> Self {
        match error {
            crypto::Error::AsymmetricKey(string) => Error::AsymmetricKey(string),
            crypto::Error::FromHex(error) => Error::FromHex(error),
            crypto::Error::FromBase64(error) => Error::FromBase64(error),
            crypto::Error::SignatureError(error) => Error::Signature(error),
        }
    }
}