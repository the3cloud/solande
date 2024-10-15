use alloc::vec::Vec;

use crate::Result;

/// A trait for types that can be encoded into a byte vector.
pub trait Encodeable {
    /// Encodes the implementing type into a byte vector.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the encoded representation of the implementing type.
    fn encode(&self) -> Vec<u8>;
}

/// A trait for types that can be decoded from a byte slice.
pub trait Decodeable {
    /// Attempts to decode an instance of the implementing type from a byte slice.
    ///
    /// # Arguments
    ///
    /// * `bytes` - A slice of bytes to decode from.
    ///
    /// # Returns
    ///
    /// A `Result` containing either the successfully decoded instance or an error.
    fn decode(bytes: &[u8]) -> Result<Self>
    where
        Self: Sized;
}

/// A trait for types that can compute their encoded byte length.
pub trait ByteLength {
    /// Computes the length of the encoded byte representation of the implementing type.
    ///
    /// # Returns
    ///
    /// A `usize` representing the number of bytes in the encoded form of the type.
    fn byte_length(&self) -> usize;
}
