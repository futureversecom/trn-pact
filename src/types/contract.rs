// Copyright 2019 Centrality Investments Limited
// This file is part of Pact.
//
// Licensed under the Apache License v2.0;
// you may not use this file except in compliance with the License.
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// You should have received a copy of the Apache License v2.0
// along with Pact. If not, see:
//   <https://futureverse.com/licenses/apachev2.txt>

//!
//! Contract struct
//!
use crate::types::DataTable;
use alloc::vec::Vec;
use bit_reverse::ParallelReverse;

#[cfg_attr(feature = "std", derive(Debug, PartialEq))]
/// A binary format error
pub enum BinaryFormatErr {
    /// Version mismatch
    UnsupportedVersion,
    /// DataTable is invalid
    MalformedDataTable(&'static str),
    // The buffer is to short to be valid
    TooShort,
}

/// A pact contract
/// It has byte code and an accompanying data section
#[cfg_attr(feature = "std", derive(Debug, PartialEq))]
pub struct Contract {
    pub data_table: DataTable,
    pub bytecode: Vec<u8>,
}

impl Contract {
    /// Encode the contract as v0 binary format into `buf`
    pub fn encode(&self, buf: &mut Vec<u8>) {
        buf.push(0); // binary format version: `0`
        self.data_table.encode(buf);
        buf.extend(self.bytecode.clone());
    }
    /// Decode a pact contract from v0 binary format
    pub fn decode(buf: &Vec<u8>) -> Result<Self, BinaryFormatErr> {
        if buf.len() < 2 {
            return Err(BinaryFormatErr::TooShort);
        }
        if buf[0].swap_bits() != 0 {
            return Err(BinaryFormatErr::UnsupportedVersion);
        }
        let (data_table, offset) = DataTable::decode(buf[1..].to_vec())
            .map_err(|err| BinaryFormatErr::MalformedDataTable(err))?;
        let bytecode = buf[1usize + offset..].to_vec();
        Ok(Self {
            data_table,
            bytecode,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::interpreter::{Comparator, Conjunction, OpCode, OpComp, OpConj, OpLoad};
    use crate::types::{Numeric, PactType, StringLike};

    #[test]
    fn contract_binary_format_unsupported_version() {
        assert_eq!(
            Contract::decode([1, 0].to_vec().as_ref()),
            Err(BinaryFormatErr::UnsupportedVersion)
        );
    }

    #[test]
    fn contract_binary_format_too_short() {
        assert_eq!(
            Contract::decode([0].to_vec().as_ref()),
            Err(BinaryFormatErr::TooShort)
        );
    }

    #[test]
    fn contract_encode_1() {
        let contract = Contract {
            data_table: DataTable::new(vec![
                PactType::Numeric(Numeric(10)),
                PactType::Numeric(Numeric(20)),
            ]),
            bytecode: vec![OpCode::COMP(Comparator::new(OpComp::EQ)).into(), 0x00],
        };
        let mut encoded_payload = vec![];
        contract.encode(&mut encoded_payload);
        println!("{:?}", encoded_payload);
    }

    #[test]
    fn contract_encode_2() {
        let contract = Contract {
            data_table: DataTable::new(vec![
                PactType::Numeric(Numeric(10)),
                PactType::StringLike(StringLike(b"hello, world".to_vec())),
            ]),
            bytecode: vec![
                OpCode::COMP(Comparator::new(OpComp::EQ)).into(),
                0x00,
                OpCode::COMP(Comparator::new(OpComp::EQ)).into(),
                0x11,
            ],
        };
        let mut encoded_payload = vec![];
        contract.encode(&mut encoded_payload);
        println!("{:?}", encoded_payload);
    }

    #[test]
    fn contract_encode_3() {
        let contract = Contract {
            data_table: DataTable::new(vec![
                PactType::Numeric(Numeric(10)),
                PactType::StringLike(StringLike(b"hello, world".to_vec())),
            ]),
            bytecode: vec![
                OpCode::COMP(Comparator::new(OpComp::EQ).invert()).into(),
                0x00,
                OpCode::COMP(Comparator::new(OpComp::EQ).load(OpLoad::INPUT_VS_INPUT)).into(),
                0x11,
            ],
        };
        let mut encoded_payload = vec![];
        contract.encode(&mut encoded_payload);
        println!("{:?}", encoded_payload);
    }

    #[test]
    fn contract_encode_4() {
        let contract = Contract {
            data_table: DataTable::new(vec![
                PactType::Numeric(Numeric(10)),
                PactType::Numeric(Numeric(20)),
            ]),
            bytecode: vec![
                OpCode::COMP(Comparator::new(OpComp::EQ)).into(),
                0x00,
                OpCode::CONJ(Conjunction::new(OpConj::AND)).into(),
            ],
        };
        let mut encoded_payload = vec![];
        contract.encode(&mut encoded_payload);
        println!("{:?}", encoded_payload);
    }
}
