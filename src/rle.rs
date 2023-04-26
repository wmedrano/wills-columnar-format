// [[file:../wills-columnar-format.org::#IntroductionCargotoml-cqc696o03tj0][Dependencies:4]]
use bincode::{Decode, Encode};
use itertools::Itertools;
use std::io::Read;
// Dependencies:4 ends here

// [[file:../wills-columnar-format.org::#DataEncodingRunLengthEncoding-0vm696o03tj0][Run Length Encoding:2]]
#[derive(Encode, Decode, Copy, Clone, PartialEq, Debug)]
pub struct Element<T> {
    // The underlying element.
    pub element: T,
    // Run length is stored as a u64. We could try using a smaller datatype,
    // but Bincode uses "variable length encoding" for integers which is
    // efficient for smaller sizes.
    pub run_length: u64,
}
// Run Length Encoding:2 ends here

// [[file:../wills-columnar-format.org::#DataEncodingRunLengthEncoding-0vm696o03tj0][Run Length Encoding:4]]
pub fn encode_iter<T: 'static + bincode::Encode + Eq>(data: impl Iterator<Item = T>) -> impl Iterator<Item=Element<T>> {
    data.peekable().batching(|iter| -> Option<Element<T>> {
        let element = iter.next()?;
        let mut run_length = 1;
        while iter.next_if_eq(&element).is_some() {
            run_length += 1;
        }
        Some(Element {
            element,
            run_length,
        })
    })
}
// Run Length Encoding:4 ends here

// [[file:../wills-columnar-format.org::#DataEncodingRunLengthEncoding-0vm696o03tj0][Run Length Encoding:5]]
pub fn decode_rle_data<T: 'static + bincode::Decode>(
    elements: usize,
    r: &'_ mut impl Read,
) -> impl '_ + Iterator<Item = Element<T>> {
    let mut elements = elements;
    std::iter::from_fn(move || {
        if elements == 0 {
            return None;
        }
        let rle_element: Element<T> =
            bincode::decode_from_std_read(r, crate::BINCODE_DATA_CONFIG).unwrap();
        assert!(rle_element.run_length as usize <= elements,);
        elements -= rle_element.run_length as usize;
        Some(rle_element)
    })
}
// Run Length Encoding:5 ends here
