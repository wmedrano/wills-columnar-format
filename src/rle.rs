// [[file:../wills-columnar-format.org::#IntroductionCargotoml-cqc696o03tj0][Dependencies:4]]
use bincode::{Decode, Encode};
use itertools::Itertools;
// Dependencies:4 ends here

// [[file:../wills-columnar-format.org::#DataEncodingRunLengthEncoding-0vm696o03tj0][Run Length Encoding:1]]
#[derive(Encode, Decode, Copy, Clone, PartialEq, Debug)]
pub struct Element<T> {
    // The underlying element.
    pub element: T,
    // Run length is stored as a u64. We could try using a smaller datatype,
    // but Bincode uses "variable length encoding" for integers which is
    // efficient for smaller sizes.
    pub run_length: u64,
}
// Run Length Encoding:1 ends here

// [[file:../wills-columnar-format.org::#DataEncodingRunLengthEncoding-0vm696o03tj0][Run Length Encoding:4]]
pub fn encode_iter<I>(iter: I) -> impl Iterator<Item = Element<I::Item>>
where
    I: Iterator,
    I::Item: PartialEq,
{
    iter.peekable().batching(|iter| {
        let element = match iter.next() {
            Some(e) => e,
            None => return None,
        };
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
