// [[file:../wills-columnar-format.org::*Run Length Encoding][Run Length Encoding:2]]
use bincode::{Decode, Encode};
// Run Length Encoding:2 ends here

// [[file:../wills-columnar-format.org::*Run Length Encoding][Run Length Encoding:3]]
#[derive(Encode, Decode, Copy, Clone, PartialEq, Debug)]
pub struct Element<T> {
    // Run length is stored as a u64. We could try using a smaller datatype,
    // but Bincode uses "variable length encoding" for integers which is
    // efficient for smaller sizes.
    pub run_length: u64,
    pub element: T,
}

pub fn encode_data<T: Eq>(data: impl Iterator<Item = T>) -> impl Iterator<Item=Element<T>> {
    EncodeIter{inner: data.peekable()}
}

pub fn decode_data<'a, T: 'static>(
    iter: impl 'a + Iterator<Item = &'a Element<T>>,
) -> impl Iterator<Item = &'a T> {
    iter.flat_map(move |rle| {
        let run_length = rle.run_length as usize;
        std::iter::repeat(&rle.element).take(run_length)
    })
}
// Run Length Encoding:3 ends here

// [[file:../wills-columnar-format.org::*Run Length Encoding][Run Length Encoding:4]]
struct EncodeIter<I: Iterator> {
    inner: std::iter::Peekable<I>,
}

impl<I> Iterator for EncodeIter<I>
where I: Iterator,
      I::Item: PartialEq {
    type Item = Element<I::Item>;

    fn next(&mut self) -> Option<Element<I::Item>> {
        let element = match self.inner.next() {
            Some(e) => e,
            None => return None,
        };
        let mut run_length = 1;
        while self.inner.next_if_eq(&element).is_some() {
            run_length += 1;
        }
        Some(Element{element, run_length})
    }
}
// Run Length Encoding:4 ends here
