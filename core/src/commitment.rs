use alloc::vec::Vec;
use digest::Digest;
use primitive_types::{H160, H256, U256};

use crate::{ByteLength, Decodeable, Encodeable, Error, Result};

/// Transparent unspent output
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Output {
    /// The amount of the output.
    pub amount: U256,
    /// The asset identifier.
    pub asset: H256,
    /// The owner's address.
    pub owner: H160,
}

impl Output {
    /// Computes the commitment of the UnspentOutput using the provided salt.
    ///
    /// Note: Digest function must return 32 bytes hash.
    pub fn commitment<D: Digest>(&self, salt: H256) -> PrivateCommitment {
        let mut hasher = D::new();
        hasher.update(self.amount.to_big_endian());
        hasher.update(self.asset.as_bytes());
        hasher.update(self.owner.as_bytes());
        hasher.update(salt.as_bytes());

        PrivateCommitment(H256::from_slice(hasher.finalize().as_slice()))
    }
}

/// The byte length of an UnspentOutput when encoded.
const UNSPENT_OUTPUT_BYTE_LENGTH: usize = 84; // 32 (amount) + 32 (asset) + 20 (owner)

impl ByteLength for Output {
    fn byte_length(&self) -> usize {
        UNSPENT_OUTPUT_BYTE_LENGTH
    }
}

impl Encodeable for Output {
    /// Encodes the UnspentOutput into a byte vector.
    fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.amount.to_big_endian());
        bytes.extend_from_slice(self.asset.as_bytes());
        bytes.extend_from_slice(self.owner.as_bytes());
        bytes
    }
}

impl Decodeable for Output {
    /// Decodes a byte slice into an UnspentOutput.
    ///
    /// # Errors
    ///
    /// Returns an error if the input byte slice is not exactly 84 bytes long.
    fn decode(bytes: &[u8]) -> Result<Self>
    where
        Self: Sized,
    {
        if bytes.len() < UNSPENT_OUTPUT_BYTE_LENGTH {
            return Err(Error::FailedToDecode);
        }

        let amount = U256::from_big_endian(&bytes[..32]);
        let asset = H256::from_slice(&bytes[32..64]);
        let owner = H160::from_slice(&bytes[64..84]);
        Ok(Output {
            amount,
            asset,
            owner,
        })
    }
}

/// A private commitment represented by a 256-bit hash.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrivateCommitment(pub H256);

impl PrivateCommitment {
    /// Computes the nullifier of the PrivateCommitment using the provided salt.
    ///
    /// Note: Digest function must return 32 bytes hash.
    pub fn nullifier<D: Digest>(&self, salt: H256) -> H256 {
        let mut hasher = D::new();
        hasher.update(self.0.as_bytes());
        hasher.update(salt.as_bytes());
        H256::from_slice(hasher.finalize().as_slice())
    }
}

/// Represents a commitment in the system, which can be either private or public.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Commitment {
    /// A private commitment represented by a 256-bit hash.
    Private(PrivateCommitment),
    /// A public commitment represented by an UnspentOutput.
    Public(Output),
}

impl ByteLength for Commitment {
    fn byte_length(&self) -> usize {
        match self {
            Commitment::Private(_) => 33,
            Commitment::Public(output) => output.byte_length() + 1,
        }
    }
}

impl Encodeable for Commitment {
    /// Encodes the Commitment into a byte vector.
    fn encode(&self) -> Vec<u8> {
        match self {
            Commitment::Private(commitment) => {
                let mut bytes = Vec::with_capacity(33);
                bytes.push(1u8);
                bytes.extend_from_slice(commitment.0.as_bytes());
                bytes
            }
            Commitment::Public(output) => {
                let mut bytes = Vec::with_capacity(85);
                bytes.push(2u8);
                bytes.extend(output.encode());
                bytes
            }
        }
    }
}

impl Decodeable for Commitment {
    /// Decodes a byte slice into a Commitment.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is empty or if the commitment type is unsupported.
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

                let commitment = H256::from_slice(&bytes[1..33]);
                Ok(Commitment::Private(PrivateCommitment(commitment)))
            }
            2 => {
                let output = Output::decode(&bytes[1..])?;
                Ok(Commitment::Public(output))
            }
            _ => Err(Error::UnsupportedCommitmentType),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use primitive_types::H256;

    #[test]
    fn test_commitment_encode_decode() {
        // Test Private Commitment
        let private_commitment = Commitment::Private(PrivateCommitment(H256::random()));
        let encoded_private = private_commitment.encode();
        let decoded_private = Commitment::decode(&encoded_private).unwrap();
        assert_eq!(private_commitment, decoded_private);

        // Test Public Commitment
        let public_output = Output {
            amount: U256::from(1000u32),
            asset: H256::random(),
            owner: H160::random(),
        };
        let public_commitment = Commitment::Public(public_output);
        let encoded_public = public_commitment.encode();
        let decoded_public = Commitment::decode(&encoded_public).unwrap();
        assert_eq!(public_commitment, decoded_public);

        // Test invalid type
        let invalid_bytes = [3u8; 33]; // 3 is not a valid commitment type
        assert!(matches!(
            Commitment::decode(&invalid_bytes),
            Err(Error::UnsupportedCommitmentType)
        ));

        // Test empty input
        assert!(matches!(
            Commitment::decode(&[]),
            Err(Error::FailedToDecode)
        ));
    }
}
