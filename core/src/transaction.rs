use alloc::vec::Vec;

use crate::{ByteLength, Commitment, Decodeable, Encodeable, Error, Nullifier, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub inputs: Vec<Nullifier>,
    pub outputs: Vec<Commitment>,
}

impl Encodeable for Transaction {
    fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        // Encode inputs length (2 bytes)
        bytes.extend_from_slice(&(self.inputs.len() as u16).to_be_bytes());
        // Encode inputs
        for input in &self.inputs {
            bytes.extend_from_slice(&input.encode());
        }

        // Encode outputs length (2 bytes)
        bytes.extend_from_slice(&(self.outputs.len() as u16).to_be_bytes());
        // Encode outputs
        for output in &self.outputs {
            bytes.extend_from_slice(&output.encode());
        }

        bytes
    }
}

impl ByteLength for Transaction {
    fn byte_length(&self) -> usize {
        self.inputs
            .iter()
            .map(|input| input.byte_length())
            .sum::<usize>()
            + self
                .outputs
                .iter()
                .map(|output| output.byte_length())
                .sum::<usize>()
    }
}

impl Decodeable for Transaction {
    fn decode(bytes: &[u8]) -> Result<Self> {
        let mut inputs = Vec::new();
        let mut outputs = Vec::new();

        let mut cursor = 0;

        // Parse length of inputs (2 bytes)
        if bytes.len() < 2 {
            return Err(Error::FailedToDecode);
        }
        let inputs_len = u16::from_be_bytes([bytes[0], bytes[1]]) as usize;
        cursor += 2;

        log::debug!("inputs_len: {}", inputs_len);

        // Parse inputs
        for _ in 0..inputs_len {
            let input = Nullifier::decode(&bytes[cursor..])?;
            cursor += input.byte_length();
            inputs.push(input);
        }

        // Parse length of outputs (2 bytes)
        if bytes.len() < cursor + 2 {
            return Err(Error::FailedToDecode);
        }
        let outputs_len = u16::from_be_bytes([bytes[cursor], bytes[cursor + 1]]) as usize;
        cursor += 2;

        log::debug!("outputs_len: {}", outputs_len);

        // Parse outputs
        for _ in 0..outputs_len {
            let output = Commitment::decode(&bytes[cursor..])?;
            cursor += output.byte_length();
            outputs.push(output);
        }

        Ok(Transaction { inputs, outputs })
    }
}

#[cfg(test)]
mod tests {
    use crate::{OutputId, UnspentOutput};

    use super::*;
    use primitive_types::{H160, H256, U256};

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_transaction_encode_decode() {
        init();

        // Create a sample transaction
        let transaction = Transaction {
            inputs: [
                Nullifier::Private(H256::random()),
                Nullifier::Public(OutputId {
                    txhash: H256::random(),
                    index: 0,
                }),
            ]
            .to_vec(),
            outputs: [
                Commitment::Private(H256::random()),
                Commitment::Public(UnspentOutput {
                    amount: U256::from(1000u32),
                    asset: H256::random(),
                    owner: H160::random(),
                }),
            ]
            .to_vec(),
        };

        // Encode the transaction
        let encoded = transaction.encode();

        // Decode the transaction
        let decoded = Transaction::decode(&encoded).unwrap();

        // Assert that the decoded transaction matches the original
        assert_eq!(transaction, decoded);

        // Test with empty transaction
        let empty_transaction = Transaction {
            inputs: [].to_vec(),
            outputs: [].to_vec(),
        };
        let encoded_empty = empty_transaction.encode();
        let decoded_empty = Transaction::decode(&encoded_empty).unwrap();
        assert_eq!(empty_transaction, decoded_empty);

        // Test with invalid input
        let invalid_input = [0, 1, 2, 3, 4]; // Too short to be a valid transaction
        assert_eq!(
            Transaction::decode(&invalid_input),
            Err(Error::FailedToDecode)
        );
    }
}
