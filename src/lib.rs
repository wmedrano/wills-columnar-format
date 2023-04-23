// [[file:../wills-columnar-format.org::*Encoding][Encoding:1]]
pub fn encode_column<T>(data: Vec<T>, use_rle: bool) -> Vec<u8>
where
    T: 'static + bincode::Encode + Eq {
    encode_column_impl(data, use_rle)
}
// Encoding:1 ends here

// [[file:../wills-columnar-format.org::*Decoding][Decoding:1]]
pub fn decode_column<T>(r: &mut impl std::io::Read) -> Vec<T>
where
    T: 'static + Clone + bincode::Decode {
    decode_column_impl(r)
}
// Decoding:1 ends here

// [[file:../wills-columnar-format.org::*Tests][Tests:1]]
#[cfg(test)]
mod test_lib;
// Tests:1 ends here

// [[file:../wills-columnar-format.org::*Format Specification][Format Specification:1]]
const MAGIC_BYTES_LEN: usize = 9;
const MAGIC_BYTES: &[u8; MAGIC_BYTES_LEN] = b"wmedrano0";
const BINCODE_DATA_CONFIG: bincode::config::Configuration = bincode::config::standard();

fn encode_column_impl<T: 'static + bincode::Encode + Eq>(data: Vec<T>, use_rle: bool) -> Vec<u8> {
    let elements = data.len();
    let encoded_data = if use_rle {
        let rle_data = rle_encode_data(data.into_iter());
        bincode::encode_to_vec(rle_data, BINCODE_DATA_CONFIG).unwrap()
    } else {
        bincode::encode_to_vec(data, BINCODE_DATA_CONFIG).unwrap()
    };
    let header = Header{
        data_type: DataType::from_type::<T>().unwrap(),
        is_rle: use_rle,
        elements,
        data_size: encoded_data.len(),
    };
    Vec::from_iter(
        MAGIC_BYTES.iter().copied()
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
        let rle_elements: Vec<RleElement<T>> =
            bincode::decode_from_std_read(r, BINCODE_DATA_CONFIG).unwrap();
        vec_from_iter_with_hint(
            rle_decode_data(rle_elements.iter()).cloned(),
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
// Format Specification:1 ends here

// [[file:../wills-columnar-format.org::*Header][Header:1]]
use bincode::{Decode, Encode};
use std::any::TypeId;

impl Header {
    const CONFIGURATION: bincode::config::Configuration = bincode::config::standard();
}

impl DataType {
    const ALL_DATA_TYPE: [DataType; 2] = [
        DataType::I64,
        DataType::String,
    ];
    fn from_type<T: 'static>() -> Option<DataType> {
        DataType::ALL_DATA_TYPE.into_iter().find(|dt| dt.is_supported::<T>())
    }

    fn supported_type_id(&self) -> TypeId {
        match self {
           DataType::I64 => TypeId::of::<i64>(),
           DataType::String => TypeId::of::<String>(),
        }
    }

    fn is_supported<T: 'static>(&self) -> bool {
        TypeId::of::<T>() == self.supported_type_id()
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
    I64 = 0,
    String = 1,
}
// Header:2 ends here

// [[file:../wills-columnar-format.org::*RLE][RLE:1]]
#[derive(Encode, Decode, Copy, Clone, PartialEq, Debug)]
pub struct RleElement<T> {
    pub run_length: u64,
    pub element: T,
}

fn rle_encode_data<T: Eq>(data: impl Iterator<Item = T>) -> Vec<RleElement<T>> {
    let mut data = data;
    let mut rle = match data.next() {
        Some(e) => RleElement{run_length: 1, element: e},
        None => return Vec::new(),
    };

    let mut ret = Vec::new();
    for element in data {
        if element != rle.element || rle.run_length == u64::MAX {
            ret.push(std::mem::replace(&mut rle, RleElement{run_length: 1, element}));
        } else {
            rle.run_length += 1;
        }
    }
    if rle.run_length > 0 {
        ret.push(rle);
    }
    ret
}

fn rle_decode_data<'a, T: 'static>(
    iter: impl 'a + Iterator<Item = &'a RleElement<T>>,
) -> impl Iterator<Item = &'a T> {
    iter.flat_map(move |rle| {
        let run_length = rle.run_length as usize;
        std::iter::repeat(&rle.element).take(run_length)
    })
}
// RLE:1 ends here
