use alloc::vec::Vec;
use primitive_types::H256;

use crate::{ByteLength, Decodeable, Encodeable, Error, Result};

/// Represents the unique identifier of an output in a transaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputId {
    /// The hash of the transaction containing this output.
    pub txhash: H256,
    /// The index of this output within the transaction.
    pub index: u32,
}

/// The byte length of an OutputId when encoded.
const OUTPUT_ID_BYTE_LENGTH: usize = 36; // 32 (txhash) + 4 (index)

impl ByteLength for OutputId {
    /// Returns the byte length of the encoded OutputId.
    fn byte_length(&self) -> usize {
        OUTPUT_ID_BYTE_LENGTH
    }
}

impl Encodeable for OutputId {
    /// Encodes the OutputId into a byte vector.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the encoded OutputId.
    fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(self.txhash.as_bytes());
        bytes.extend_from_slice(&self.index.to_be_bytes());
        bytes
    }
}

impl Decodeable for OutputId {
    /// Decodes a byte slice into an OutputId.
    ///
    /// # Arguments
    ///
    /// * `bytes` - A slice of bytes to decode from.
    ///
    /// # Returns
    ///
    /// A `Result` containing either the successfully decoded OutputId or an error.
    ///
    /// # Errors
    ///
    /// Returns an error if the input byte slice is not exactly 36 bytes long.
    fn decode(bytes: &[u8]) -> Result<Self>
    where
        Self: Sized,
    {
        if bytes.len() < OUTPUT_ID_BYTE_LENGTH {
            return Err(Error::FailedToDecode);
        }

        let txhash = H256::from_slice(&bytes[..32]);
        let index = u32::from_be_bytes(bytes[32..36].try_into().unwrap());

        Ok(OutputId { txhash, index })
    }
}

/// Represents a nullifier, which can be either private or public.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Nullifier {
    /// A private nullifier represented by a 256-bit hash.
    Private(H256),
    /// A public nullifier represented by an OutputId.
    Public(OutputId),
}

impl ByteLength for Nullifier {
    /// Returns the byte length of the encoded Nullifier.
    fn byte_length(&self) -> usize {
        match self {
            Nullifier::Private(_) => 33,
            Nullifier::Public(output_id) => output_id.byte_length() + 1,
        }
    }
}

impl Encodeable for Nullifier {
    /// Encodes the Nullifier into a byte vector.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the encoded Nullifier.
    fn encode(&self) -> Vec<u8> {
        match self {
            Nullifier::Private(nullifier) => {
                let mut bytes = Vec::with_capacity(33);
                bytes.push(1u8);
                bytes.extend_from_slice(nullifier.as_bytes());
                bytes
            }
            Nullifier::Public(output_id) => {
                let mut bytes = Vec::with_capacity(37);
                bytes.push(2u8);
                bytes.extend(output_id.encode());
                bytes
            }
        }
    }
}

impl Decodeable for Nullifier {
    /// Decodes a byte slice into a Nullifier.
    ///
    /// # Arguments
    ///
    /// * `bytes` - A slice of bytes to decode from.
    ///
    /// # Returns
    ///
    /// A `Result` containing either the successfully decoded Nullifier or an error.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is empty or if the nullifier type is unsupported.
    fn decode(bytes: &[u8]) -> Result<Self>
    where
        Self: Sized,
    {
        if bytes.is_empty() {
            return Err(Error::FailedToDecode);
        }

        match bytes[0] {
            1 => {
                if bytes.len() < 33 {
                    return Err(Error::FailedToDecode);
                }

                let nullifier = H256::from_slice(&bytes[1..33]);
                Ok(Nullifier::Private(nullifier))
            }
            2 => {
                let output_id = OutputId::decode(&bytes[1..])?;
                Ok(Nullifier::Public(output_id))
            }
            _ => Err(Error::UnsupportedNullifierType),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use primitive_types::H256;

    #[test]
    fn test_nullifier_encode_decode() {
        // Test Private Nullifier
        let private_nullifier = Nullifier::Private(H256::random());
        let encoded_private = private_nullifier.encode();
        let decoded_private = Nullifier::decode(&encoded_private).unwrap();
        assert_eq!(private_nullifier, decoded_private);

        // Test Public Nullifier
        let public_output_id = OutputId {
            txhash: H256::random(),
            index: 0,
        };
        let public_nullifier = Nullifier::Public(public_output_id);
        let encoded_public = public_nullifier.encode();
        let decoded_public = Nullifier::decode(&encoded_public).unwrap();
        assert_eq!(public_nullifier, decoded_public);

        // Test invalid type
        let invalid_bytes = [3u8; 33]; // 3 is not a valid nullifier type
        assert!(matches!(
            Nullifier::decode(&invalid_bytes),
            Err(Error::UnsupportedNullifierType)
        ));

        // Test empty input
        assert!(matches!(Nullifier::decode(&[]), Err(Error::FailedToDecode)));
    }
}
