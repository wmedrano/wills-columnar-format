// [[file:../wills-columnar-format.org::#Introduction-h6a696o03tj0][Introduction:1]]
pub mod rle;

#[cfg(test)]
pub mod test_bincode;
#[cfg(test)]
mod test_lib;
#[cfg(test)]
mod test_rle;

use bincode::{Decode, Encode};
use itertools::Either;
use std::any::TypeId;
// Introduction:1 ends here

// [[file:../wills-columnar-format.org::#APIEncoding-w0g696o03tj0][Encoding:1]]
pub fn encode_column<T>(data: Vec<T>, use_rle: bool) -> Vec<u8>
where
    T: 'static + bincode::Encode + Eq,
{
    encode_column_impl(data, use_rle)
}
// Encoding:1 ends here

// [[file:../wills-columnar-format.org::#APIDecoding-npg696o03tj0][Decoding:1]]
pub fn decode_column<T>(r: &mut impl std::io::Read) -> impl Iterator<Item = rle::Element<T>>
where
    T: 'static + Clone + bincode::Decode,
{
    decode_column_impl(r)
}
// Decoding:1 ends here

// [[file:../wills-columnar-format.org::#FormatSpecificationFormatOverview-j3k696o03tj0][Format Overview:1]]
fn encode_column_impl<T>(data: Vec<T>, use_rle: bool) -> Vec<u8>
where
    T: 'static + bincode::Encode + Eq,
{
    let elements = data.len();
    let encoded_data = if use_rle {
        encode_data_rle_impl(data)
    } else {
        encode_data_base_impl(data)
    };
    let header = Header {
        data_type: DataType::from_type::<T>().unwrap(),
        use_rle,
        elements,
        data_size: encoded_data.len(),
    };
    encode_header_and_data(MAGIC_BYTES, header, encoded_data)
}
// Format Overview:1 ends here

// [[file:../wills-columnar-format.org::#FormatSpecificationFormatOverview-j3k696o03tj0][Format Overview:2]]
const MAGIC_BYTES_LEN: usize = 9;
const MAGIC_BYTES: &[u8; MAGIC_BYTES_LEN] = b"wmedrano0";
const BINCODE_DATA_CONFIG: bincode::config::Configuration = bincode::config::standard();

fn encode_header_and_data(
    magic_bytes: &'static [u8],
    header: Header,
    encoded_data: Vec<u8>,
) -> Vec<u8> {
    assert_eq!(header.data_size, encoded_data.len());
    Vec::from_iter(
        magic_bytes
            .iter()
            .copied()
            .chain(header.encode())
            .chain(encoded_data.iter().copied()),
    )
}

fn decode_column_impl<T: 'static + bincode::Decode>(
    r: &mut impl std::io::Read,
) -> impl Iterator<Item = rle::Element<T>> {
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
    if header.use_rle {
        let rle_elements: Vec<rle::Element<T>> =
            bincode::decode_from_std_read(r, BINCODE_DATA_CONFIG).unwrap();
        Either::Left(rle_elements.into_iter())
    } else {
        let elements: Vec<T> = bincode::decode_from_std_read(r, BINCODE_DATA_CONFIG).unwrap();
        Either::Right(elements.into_iter().map(|element| rle::Element {
            element,
            run_length: 1,
        }))
    }
}
// Format Overview:2 ends here

// [[file:../wills-columnar-format.org::#FormatSpecificationHeader-3tk696o03tj0][Header:1]]
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

// [[file:../wills-columnar-format.org::#FormatSpecificationHeader-3tk696o03tj0][Header:2]]
#[derive(Encode, Decode, PartialEq, Eq, Copy, Clone, Debug)]
pub struct Header {
    pub data_type: DataType,
    pub use_rle: bool,
    pub elements: usize,
    pub data_size: usize,
}

#[derive(Encode, Decode, PartialEq, Eq, Copy, Clone, Debug)]
pub enum DataType {
    Integer = 0,
    String = 1,
}
// Header:2 ends here

// [[file:../wills-columnar-format.org::#DataEncodingBasicEncoding-e4m696o03tj0][Basic Encoding:1]]
fn encode_data_base_impl<T: 'static + bincode::Encode>(data: Vec<T>) -> Vec<u8> {
    bincode::encode_to_vec(data, BINCODE_DATA_CONFIG).unwrap()
}
// Basic Encoding:1 ends here

// [[file:../wills-columnar-format.org::#DataEncodingRunLengthEncoding-0vm696o03tj0][Run Length Encoding:2]]
fn encode_data_rle_impl<T: 'static + bincode::Encode + Eq>(data: Vec<T>) -> Vec<u8> {
    let rle_data: Vec<rle::Element<T>> = rle::EncodeIter::new(data.into_iter()).collect();
    encode_data_base_impl(rle_data)
}
// Run Length Encoding:2 ends here
