// [[file:../wills-columnar-format.org::#IntroductionCargotoml-cqc696o03tj0][Dependencies:4]]
use bincode::{Decode, Encode};
use itertools::Itertools;
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
pub fn encode_iter<'a, T: 'a + bincode::Encode + Eq>(
    data: impl 'a + Iterator<Item = T>,
) -> impl 'a + Iterator<Item = Values<T>> {
    data.peekable().batching(move |iter| -> Option<Values<T>> {
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
impl<T> Values<T> {
    pub fn repeated(&self) -> impl '_ + Iterator<Item = &'_ T> {
        std::iter::repeat(&self.value).take(self.run_length as usize)
    }
}
// Run Length Encoding:4 ends here
