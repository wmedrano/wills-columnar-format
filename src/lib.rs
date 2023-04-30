// [[file:../wills-columnar-format.org::#IntroductionCargotoml-cqc696o03tj0][Dependencies:3]]
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
// Dependencies:3 ends here

// [[file:../wills-columnar-format.org::#APIEncoding-w0g696o03tj0][Encoding:1]]
pub fn encode_column<Iter, T, W>(data: Iter, w: &mut W, use_rle: bool) -> Result<()>
where
    Iter: ExactSizeIterator + Iterator<Item = T>,
    T: 'static + bincode::Encode + Eq,
    W: Write,
{
    encode_column_impl(w, data, use_rle)
}
// Encoding:1 ends here

// [[file:../wills-columnar-format.org::#APIDecoding-npg696o03tj0][Decoding:1]]
pub fn decode_column<T>(
    r: &'_ mut (impl Read + Seek),
) -> Result<impl '_ + Iterator<Item = Result<rle::Values<T>>>>
where
    T: 'static + bincode::Decode,
{
    decode_column_impl(r)
}
// Decoding:1 ends here

// [[file:../wills-columnar-format.org::#FormatSpecificationFormatOverview-j3k696o03tj0][Format Overview:2]]
fn encode_column_impl<T>(
    w: &mut impl Write,
    values_iter: impl ExactSizeIterator + Iterator<Item = T>,
    use_rle: bool,
) -> Result<()>
where
    T: 'static + bincode::Encode + Eq,
{
    let values = values_iter.len();
    let mut file_offset = w.write(MAGIC_BYTES)?;
    file_offset += bincode::encode_into_std_write(
        Header {
            data_type: DataType::from_type::<T>().unwrap(),
            use_rle,
        },
        w,
        BINCODE_DATA_CONFIG,
    )?;
    // TODO: Use multiple pages instead of writing to a single page.
    let encoding = if use_rle {
        let rle_data /*: impl Iterator<Item=rle::Values<T>>*/ = rle::encode_iter(values_iter);
        encode_values_as_bincode(rle_data)?
    } else {
        encode_values_as_bincode(values_iter)?
    };
    file_offset += w.write(encoding.encoded_values.as_slice())?;
    let page_offset = file_offset;
    let footer_size = bincode::encode_into_std_write(
        Footer {
            pages: vec![PageInfo {
                file_offset: page_offset as i64,
                values_count: values,
                encoded_values_count: encoding.values_count,
            }],
        },
        w,
        BINCODE_DATA_CONFIG,
    )? as u64;
    w.write(&footer_size.to_le_bytes())?;
    Ok(())
}
// Format Overview:2 ends here

// [[file:../wills-columnar-format.org::#FormatSpecificationFormatOverview-j3k696o03tj0][Format Overview:3]]
const BINCODE_DATA_CONFIG: bincode::config::Configuration = bincode::config::standard();

fn decode_column_impl<T: 'static + bincode::Decode>(
    r: impl Read + Seek,
) -> Result<impl Iterator<Item = Result<rle::Values<T>>>> {
    let mut r = r;
    let mut magic_string = [0u8; MAGIC_BYTES_LEN];
    r.read_exact(&mut magic_string)?;
    assert_eq!(
        &magic_string, MAGIC_BYTES,
        "Expected magic string {:?}.",
        MAGIC_BYTES
    );
    let header = Header::decode(&mut r);
    let data_start = r.stream_position()?;
    assert!(
        header.data_type.is_supported::<T>(),
        "Format of expected type {:?} does not support {:?}.",
        header.data_type,
        std::any::type_name::<T>(),
    );
    r.seek(std::io::SeekFrom::End(-8))?;
    let footer_length_bytes = bincode::decode_from_std_read(&mut r, BINCODE_DATA_CONFIG)?;
    let footer_length = u64::from_le_bytes(footer_length_bytes);
    r.seek(std::io::SeekFrom::End(-8 - footer_length as i64))?;
    let footer: Footer = bincode::decode_from_std_read(&mut r, BINCODE_DATA_CONFIG)?;
    r.seek(std::io::SeekFrom::Start(data_start))?;

    let mut iter_pages = footer.pages.into_iter().peekable();
    let iter = std::iter::from_fn(move || -> Option<Result<rle::Values<T>>> {
        // TODO: Verify
        while iter_pages.next_if(|p| p.values_count == 0).is_some() {}
        let page = iter_pages.peek_mut()?;
        let rle_element_or_err = if header.use_rle {
            bincode::decode_from_std_read(&mut r, BINCODE_DATA_CONFIG)
        } else {
            bincode::decode_from_std_read(&mut r, BINCODE_DATA_CONFIG).map(rle::Values::single)
        };
        if let Ok(e) = &rle_element_or_err {
            page.values_count -= e.run_length as usize;
        }
        Some(rle_element_or_err.map_err(std::convert::Into::into))
    });
    Ok(iter)
}
// Format Overview:3 ends here

// [[file:../wills-columnar-format.org::#FormatSpecificationMagicBytes-iyl7tna13tj0][Magic Bytes:2]]
const MAGIC_BYTES: &[u8; MAGIC_BYTES_LEN] = b"wmedrano0";
const MAGIC_BYTES_LEN: usize = 9;
// Magic Bytes:2 ends here

// [[file:../wills-columnar-format.org::#FormatSpecificationHeader-3tk696o03tj0][File Header:2]]
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
    fn decode(r: &mut impl std::io::Read) -> Self {
        bincode::decode_from_std_read(r, BINCODE_DATA_CONFIG).unwrap()
    }
}
// File Header:2 ends here

// [[file:../wills-columnar-format.org::#FormatSpecificationHeader-3tk696o03tj0][File Header:3]]
#[derive(Encode, Decode, PartialEq, Eq, Copy, Clone, Debug)]
pub struct Header {
    pub data_type: DataType,
    pub use_rle: bool,
}

#[derive(Encode, Decode, PartialEq, Eq, Copy, Clone, Debug)]
pub enum DataType {
    Integer = 0,
    String = 1,
}
// File Header:3 ends here

// [[file:../wills-columnar-format.org::#FormatSpecificationFileFooter-nn404df05tj0][File Footer:2]]
#[derive(Encode, Decode, PartialEq, Eq, Clone, Debug)]
pub struct Footer {
    pub pages: Vec<PageInfo>,
}

#[derive(Encode, Decode, PartialEq, Eq, Copy, Clone, Debug)]
pub struct PageInfo {
    pub file_offset: i64,
    pub values_count: usize,
    pub encoded_values_count: usize,
}
// File Footer:2 ends here

// [[file:../wills-columnar-format.org::#DataEncodingBasicEncoding-e4m696o03tj0][Basic Encoding:2]]
struct Encoding {
    pub encoded_values: Vec<u8>,
    pub values_count: usize,
}

fn encode_values_as_bincode<T: 'static + bincode::Encode>(
    values: impl Iterator<Item = T>,
) -> Result<Encoding> {
    let mut encoded_values = Vec::new();
    let mut values_count = 0;
    for element in values {
        bincode::encode_into_std_write(element, &mut encoded_values, BINCODE_DATA_CONFIG)?;
        values_count += 1;
    }
    Ok(Encoding {
        encoded_values,
        values_count,
    })
}
// Basic Encoding:2 ends here
