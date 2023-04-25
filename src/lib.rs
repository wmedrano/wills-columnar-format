// [[file:../wills-columnar-format.org::#IntroductionCargotoml-cqc696o03tj0][Dependencies:3]]
pub mod rle;

#[cfg(test)]
mod test_bincode;
#[cfg(test)]
mod test_lib;
#[cfg(test)]
mod test_rle;

use bincode::{Decode, Encode};
use itertools::Either;
use std::{any::TypeId, io::Read, marker::PhantomData};
// Dependencies:3 ends here

// [[file:../wills-columnar-format.org::#APIEncoding-w0g696o03tj0][Encoding:1]]
pub fn encode_column<Iter, T>(data: Iter, use_rle: bool) -> Vec<u8>
where
    Iter: ExactSizeIterator + Iterator<Item = T>,
    T: 'static + bincode::Encode + Eq,
{
    encode_column_impl(data, use_rle)
}
// Encoding:1 ends here

// [[file:../wills-columnar-format.org::#APIDecoding-npg696o03tj0][Decoding:1]]
pub fn decode_column<T>(r: &'_ mut impl std::io::Read) -> impl '_ + Iterator<Item = rle::Element<T>>
where
    T: 'static + bincode::Decode,
{
    decode_column_impl(r)
}
// Decoding:1 ends here

// [[file:../wills-columnar-format.org::#FormatSpecificationFormatOverview-j3k696o03tj0][Format Overview:2]]
fn encode_column_impl<T>(
    data: impl ExactSizeIterator + Iterator<Item = T>,
    use_rle: bool,
) -> Vec<u8>
where
    T: 'static + bincode::Encode + Eq,
{
    let elements = data.len();
    let encoded_data = if use_rle {
        encode_data_rle_impl(data.into_iter())
    } else {
        encode_data_base_impl(data.into_iter())
    };
    let header = Header {
        data_type: DataType::from_type::<T>().unwrap(),
        use_rle,
        elements,
        data_size: encoded_data.len(),
    };
    encode_header_and_data(MAGIC_BYTES, header, encoded_data)
}
// Format Overview:2 ends here

// [[file:../wills-columnar-format.org::#FormatSpecificationFormatOverview-j3k696o03tj0][Format Overview:3]]
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
    r: &'_ mut impl std::io::Read,
) -> impl '_ + Iterator<Item = rle::Element<T>> {
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
        let rle_elements /*: impl Iterator<Item=Element<T>>*/ = decode_rle_data(header.elements, r);
        Either::Left(rle_elements)
    } else {
        let elements: DataDecoder<T, _> = DataDecoder::new(header.elements, r);
        let rle_elements = elements.map(|element| rle::Element {
            element,
            run_length: 1,
        });
        Either::Right(rle_elements)
    }
}
// Format Overview:3 ends here

// [[file:../wills-columnar-format.org::#FormatSpecificationMagicBytes-iyl7tna13tj0][Magic Bytes:1]]
const MAGIC_BYTES: &[u8; MAGIC_BYTES_LEN] = b"wmedrano0";
const MAGIC_BYTES_LEN: usize = 9;
// Magic Bytes:1 ends here

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
fn encode_data_base_impl<T: 'static + bincode::Encode>(data: impl Iterator<Item = T>) -> Vec<u8> {
    let mut encoded = Vec::new();
    for element in data {
        bincode::encode_into_std_write(element, &mut encoded, BINCODE_DATA_CONFIG).unwrap();
    }
    encoded
}
// Basic Encoding:1 ends here

// [[file:../wills-columnar-format.org::#DataEncodingRunLengthEncoding-0vm696o03tj0][Run Length Encoding:2]]
fn encode_data_rle_impl<T: 'static + bincode::Encode + Eq>(
    data: impl Iterator<Item = T>,
) -> Vec<u8> {
    let rle_data /*: impl Iterator<Item=rle::Element<T>>*/ = rle::encode_iter(data);
    encode_data_base_impl(rle_data)
}
// Run Length Encoding:2 ends here

// [[file:../wills-columnar-format.org::#DataEncodingRunLengthEncoding-0vm696o03tj0][Run Length Encoding:3]]
fn decode_rle_data<T: 'static + bincode::Decode>(
    elements: usize,
    r: &'_ mut impl Read,
) -> impl '_ + Iterator<Item = rle::Element<T>> {
    let mut elements = elements;
    std::iter::from_fn(move || {
        if elements == 0 {
            return None;
        }
        let rle_element: rle::Element<T> =
            bincode::decode_from_std_read(r, BINCODE_DATA_CONFIG).unwrap();
        assert!(
            rle_element.run_length as usize <= elements,
            "{} <= {}",
            elements,
            rle_element.run_length
        );
        elements -= rle_element.run_length as usize;
        Some(rle_element)
    })
}

struct DataDecoder<T, R> {
    reader: R,
    element_count: usize,
    element_type: PhantomData<T>,
}

impl<T, R> DataDecoder<T, R> {
    pub fn new(element_count: usize, reader: R) -> DataDecoder<T, R> {
        DataDecoder {
            reader,
            element_count,
            element_type: PhantomData,
        }
    }
}

impl<T, R> Iterator for DataDecoder<T, R>
where
    T: bincode::Decode,
    R: Read,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.element_count == 0 {
            return None;
        }
        self.element_count -= 1;
        let element: T =
            bincode::decode_from_std_read(&mut self.reader, BINCODE_DATA_CONFIG).unwrap();
        Some(element)
    }
}
// Run Length Encoding:3 ends here
