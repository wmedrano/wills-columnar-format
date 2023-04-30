// [[file:../wills-columnar-format.org::#IntroductionCargotoml-cqc696o03tj0][Dependencies:4]]
use crate::Result;
use bincode::{Decode, Encode};
use itertools::Itertools;
use std::io::Read;

#[derive(Clone, Debug, PartialEq)]
enum RleDecodeErr {
    NotEnoughValuesInReader {
        expected_total: usize,
        actual_total: usize,
    },
}

impl std::error::Error for RleDecodeErr {}

impl std::fmt::Display for RleDecodeErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RleDecodeErr::NotEnoughValuesInReader {
                expected_total,
                actual_total,
            } => write!(
                f,
                "expected at least {} values but only found {}",
                expected_total, actual_total,
            ),
        }
    }
}
// Dependencies:4 ends here

// [[file:../wills-columnar-format.org::#DataEncodingRunLengthEncoding-0vm696o03tj0][Run Length Encoding:2]]
#[derive(Encode, Decode, Copy, Clone, PartialEq, Debug)]
pub struct Values<T> {
    // The underlying element.
    pub value: T,
    // Run length is stored as a u64. We could try using a smaller datatype,
    // but Bincode uses "variable length encoding" for integers which is
    // efficient for smaller sizes.
    pub run_length: u64,
}

impl<T> Values<T> {
    pub fn single(element: T) -> Self {
        Values {
            value: element,
            run_length: 1,
        }
    }
}
// Run Length Encoding:2 ends here

// [[file:../wills-columnar-format.org::#DataEncodingRunLengthEncoding-0vm696o03tj0][Run Length Encoding:3]]
pub fn encode_iter<T: 'static + bincode::Encode + Eq>(
    data: impl Iterator<Item = T>,
) -> impl Iterator<Item = Values<T>> {
    data.peekable().batching(|iter| -> Option<Values<T>> {
        let element = iter.next()?;
        let mut run_length = 1;
        while iter.next_if_eq(&element).is_some() {
            run_length += 1;
        }
        Some(Values {
            value: element,
            run_length,
        })
    })
}
// Run Length Encoding:3 ends here

// [[file:../wills-columnar-format.org::#DataEncodingRunLengthEncoding-0vm696o03tj0][Run Length Encoding:4]]
pub fn decode_rle_data<T: 'static + bincode::Decode>(
    elements: usize,
    r: &'_ mut impl Read,
) -> impl '_ + Iterator<Item = Result<Values<T>>> {
    let mut elements_left_to_read = elements;
    std::iter::from_fn(move || {
        if elements_left_to_read == 0 {
            return None;
        }
        let rle_element: Values<T> =
            match bincode::decode_from_std_read(r, crate::BINCODE_DATA_CONFIG) {
                Ok(e) => e,
                Err(err) => return Some(Err(err.into())),
            };
        if rle_element.run_length as usize > elements_left_to_read {
            let actual_total = elements - elements_left_to_read + rle_element.run_length as usize;
            let err = RleDecodeErr::NotEnoughValuesInReader {
                expected_total: elements,
                actual_total,
            };
            return Some(Err(err.into()));
        }
        elements_left_to_read -= rle_element.run_length as usize;
        Some(Ok(rle_element))
    })
}
// Run Length Encoding:4 ends here
