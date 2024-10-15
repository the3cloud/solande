/// Represents the possible errors that can occur.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Indicates a failure in decoding data.
    FailedToDecode,
    /// Indicates an unsupported commitment type was encountered.
    UnsupportedCommitmentType,
    /// Indicates an unsupported nullifier type was encountered.
    UnsupportedNullifierType,
}

/// A type alias for `Result` with the error type set to our custom `Error`.
pub type Result<T> = core::result::Result<T, Error>;
