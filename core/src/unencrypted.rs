use alloc::vec::Vec;
use primitive_types::H256;

use crate::{ByteLength, Decodeable, Decryptor, Encodeable, Encryptor, Output, Result};

pub struct UnencryptedOutput {
    pub output: Output,
    pub salt: H256,
}

impl ByteLength for UnencryptedOutput {
    fn byte_length(&self) -> usize {
        self.output.byte_length() + 32
    }
}

impl Encodeable for UnencryptedOutput {
    fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.output.byte_length() + 32);
        bytes.extend_from_slice(&self.output.encode());
        bytes.extend_from_slice(self.salt.as_bytes());
        bytes
    }
}

impl Decodeable for UnencryptedOutput {
    fn decode(bytes: &[u8]) -> Result<Self> {
        let output = Output::decode(bytes)?;
        let salt = H256::from_slice(&bytes[output.byte_length()..]);
        Ok(UnencryptedOutput { output, salt })
    }
}

impl UnencryptedOutput {
    pub fn encrypt(&self, encryptor: &impl Encryptor) -> Result<Vec<u8>> {
        encryptor.encrypt(&self.encode())
    }

    pub fn decrypt(&self, decryptor: &impl Decryptor) -> Result<Self> {
        let bytes = decryptor.decrypt(&self.encode())?;
        UnencryptedOutput::decode(&bytes)
    }
}
