// [[file:../wills-columnar-format.org::*Introduction][Introduction:1]]
pub mod rle;

#[cfg(test)]
pub mod test_bincode;
#[cfg(test)]
mod test_lib;
#[cfg(test)]
mod test_rle;

use bincode::{Decode, Encode};
use std::any::TypeId;
// Introduction:1 ends here

// [[file:../wills-columnar-format.org::*Encoding][Encoding:1]]
pub fn encode_column<T>(data: Vec<T>, use_rle: bool) -> Vec<u8>
where
    T: 'static + bincode::Encode + Eq,
{
    encode_column_impl(data, use_rle)
}
// Encoding:1 ends here

// [[file:../wills-columnar-format.org::*Decoding][Decoding:1]]
pub fn decode_column<T>(r: &mut impl std::io::Read) -> Vec<T>
where
    T: 'static + Clone + bincode::Decode,
{
    decode_column_impl(r)
}
// Decoding:1 ends here

// [[file:../wills-columnar-format.org::*Format Overview][Format Overview:1]]
const MAGIC_BYTES_LEN: usize = 9;
const MAGIC_BYTES: &[u8; MAGIC_BYTES_LEN] = b"wmedrano0";
const BINCODE_DATA_CONFIG: bincode::config::Configuration = bincode::config::standard();

fn encode_column_impl<T: 'static + bincode::Encode + Eq>(data: Vec<T>, use_rle: bool) -> Vec<u8> {
    let elements = data.len();
    let encoded_data = if use_rle {
        let rle_data = rle::encode_data(data.into_iter()).collect::<Vec<_>>();
        bincode::encode_to_vec(rle_data, BINCODE_DATA_CONFIG).unwrap()
    } else {
        bincode::encode_to_vec(data, BINCODE_DATA_CONFIG).unwrap()
    };
    let header = Header {
        data_type: DataType::from_type::<T>().unwrap(),
        is_rle: use_rle,
        elements,
        data_size: encoded_data.len(),
    };
    Vec::from_iter(
        MAGIC_BYTES
            .iter()
            .copied()
            .chain(header.encode())
            .chain(encoded_data.iter().copied()),
    )
}

fn decode_column_impl<T: 'static + Clone + bincode::Decode>(r: &mut impl std::io::Read) -> Vec<T> {
    let mut magic_string = [0u8; MAGIC_BYTES_LEN];
    r.read_exact(&mut magic_string).unwrap();
    assert_eq!(
        &magic_string, MAGIC_BYTES,
        "Expected magic string {:?}.",
        MAGIC_BYTES
    );
    let header = Header::decode(r);
    assert!(
        header.data_type.is_supported::<T>(),
        "Format of expected type {:?} does not support {:?}.",
        header.data_type,
        std::any::type_name::<T>(),
    );
    if header.is_rle {
        let rle_elements: Vec<rle::Element<T>> =
            bincode::decode_from_std_read(r, BINCODE_DATA_CONFIG).unwrap();
        vec_from_iter_with_hint(
            rle::decode_data(rle_elements.iter()).cloned(),
            header.elements,
        )
    } else {
        bincode::decode_from_std_read(r, BINCODE_DATA_CONFIG).unwrap()
    }
}

fn vec_from_iter_with_hint<T>(iter: impl Iterator<Item = T>, len_hint: usize) -> Vec<T> {
    let mut ret = Vec::with_capacity(len_hint);
    ret.extend(iter);
    ret
}
// Format Overview:1 ends here

// [[file:../wills-columnar-format.org::*Header][Header:1]]
impl Header {
    const CONFIGURATION: bincode::config::Configuration = bincode::config::standard();
}

impl DataType {
    const ALL_DATA_TYPE: [DataType; 2] = [DataType::Integer, DataType::String];

    fn from_type<T: 'static>() -> Option<DataType> {
        DataType::ALL_DATA_TYPE
            .into_iter()
            .find(|dt| dt.is_supported::<T>())
    }

    fn is_supported<T: 'static>(&self) -> bool {
        let type_id = TypeId::of::<T>();
        match self {
            DataType::Integer => [
                TypeId::of::<i8>(),
                TypeId::of::<u8>(),
                TypeId::of::<i16>(),
                TypeId::of::<u16>(),
                TypeId::of::<i32>(),
                TypeId::of::<u32>(),
                TypeId::of::<i64>(),
                TypeId::of::<u64>(),
            ]
            .contains(&type_id),
            DataType::String => {
                [TypeId::of::<String>(), TypeId::of::<&'static str>()].contains(&type_id)
            }
        }
    }
}

impl Header {
    fn encode(&self) -> Vec<u8> {
        bincode::encode_to_vec(self, Self::CONFIGURATION).unwrap()
    }

    fn decode(r: &mut impl std::io::Read) -> Header {
        bincode::decode_from_std_read(r, Self::CONFIGURATION).unwrap()
    }
}
// Header:1 ends here

// [[file:../wills-columnar-format.org::*Header][Header:2]]
#[derive(Encode, Decode, PartialEq, Eq, Copy, Clone, Debug)]
pub struct Header {
    pub data_type: DataType,
    pub is_rle: bool,
    pub elements: usize,
    pub data_size: usize,
}

#[derive(Encode, Decode, PartialEq, Eq, Copy, Clone, Debug)]
pub enum DataType {
    Integer = 0,
    String = 1,
}
// Header:2 ends here
