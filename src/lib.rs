// [[file:../wills-columnar-format.org::#IntroductionCargotoml-cqc696o03tj0][Dependencies:3]]
mod decode;
mod encode;
pub mod rle;

#[cfg(test)]
mod test_bincode;
#[cfg(test)]
mod test_lib;
#[cfg(test)]
mod test_rle;

use bincode::{Decode, Encode};
use std::{
    any::TypeId,
    io::{Read, Seek, Write},
};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;
const BINCODE_DATA_CONFIG: bincode::config::Configuration = bincode::config::standard();
// Dependencies:3 ends here

// [[file:../wills-columnar-format.org::#APIEncoding-w0g696o03tj0][Encoding:1]]
pub fn encode_column<Iter, T, W>(data: Iter, w: &mut W, use_rle: bool) -> Result<Footer>
where
    Iter: Iterator<Item = T>,
    T: 'static + bincode::Encode + Eq,
    W: Write,
{
    encode::encode_column_impl(w, data, use_rle)
}
// Encoding:1 ends here

// [[file:../wills-columnar-format.org::#APIDecoding-npg696o03tj0][Decoding:1]]
pub fn decode_column<'a, T>(
    r: impl 'a + Read + Seek,
) -> Result<impl 'a + Iterator<Item = Result<rle::Values<T>>>>
where
    T: 'static + bincode::Decode,
{
    decode::decode_column_impl(r)
}

pub fn decode_footer(r: impl Read + Seek) -> Result<Footer> {
    let mut r = r;
    decode::decode_footer_impl(&mut r)
}
// Decoding:1 ends here

// [[file:../wills-columnar-format.org::#FormatSpecificationFileFooter-nn404df05tj0][File Footer:2]]
#[derive(Encode, Decode, PartialEq, Eq, Clone, Debug)]
pub struct Footer {
    pub data_type: DataType,
    pub use_rle: bool,
    pub pages: Vec<PageInfo>,
}

#[derive(Encode, Decode, PartialEq, Eq, Copy, Clone, Debug)]
pub enum DataType {
    UnsignedByte = 0,
    SignedByte = 1,
    UnsignedInteger = 2,
    SignedInteger = 3,
    String = 4,
}

#[derive(Encode, Decode, PartialEq, Eq, Copy, Clone, Debug)]
pub struct PageInfo {
    pub file_offset: i64,
    pub values_count: usize,
    pub encoded_values_count: usize,
}
// File Footer:2 ends here

// [[file:../wills-columnar-format.org::#FormatSpecificationFileFooter-nn404df05tj0][File Footer:3]]
impl DataType {
    const ALL_DATA_TYPE: [DataType; 5] = [
        DataType::UnsignedByte,
        DataType::SignedByte,
        DataType::UnsignedInteger,
        DataType::SignedInteger,
        DataType::String,
    ];

    fn from_type<T: 'static>() -> Option<DataType> {
        DataType::ALL_DATA_TYPE
            .into_iter()
            .find(|dt| dt.is_supported::<T>())
    }

    fn is_supported<T: 'static>(&self) -> bool {
        let type_id = TypeId::of::<T>();
        match self {
            DataType::UnsignedByte => TypeId::of::<u8>() == type_id,
            DataType::SignedByte => TypeId::of::<i8>() == type_id,
            DataType::UnsignedInteger => [
                TypeId::of::<u16>(),
                TypeId::of::<u32>(),
                TypeId::of::<u64>(),
            ]
            .contains(&type_id),
            DataType::SignedInteger => [
                TypeId::of::<i16>(),
                TypeId::of::<i32>(),
                TypeId::of::<i64>(),
            ]
            .contains(&type_id),
            DataType::String => {
                [TypeId::of::<String>(), TypeId::of::<&'static str>()].contains(&type_id)
            }
        }
    }
}
// File Footer:3 ends here
