// [[file:../wills-columnar-format.org::#Introduction-h6a696o03tj0][Introduction:2]]
use bincode::{Decode, Encode};
// Introduction:2 ends here

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

// [[file:../wills-columnar-format.org::#DataEncodingRunLengthEncoding-0vm696o03tj0][Run Length Encoding:3]]
pub struct EncodeIter<I: Iterator> {
    inner: std::iter::Peekable<I>,
}

impl<I> Iterator for EncodeIter<I>
where
    I: Iterator,
    I::Item: PartialEq,
{
    type Item = Element<I::Item>;

    fn next(&mut self) -> Option<Element<I::Item>> {
        // Start the run or exit if the underlying iterator is empty.
        let element = match self.inner.next() {
            Some(e) => e,
            None => return None,
        };
        let mut run_length = 1;

        // Continue the run as long as the next element is equal to the current running element.
        while self.inner.next_if_eq(&element).is_some() {
            run_length += 1;
        }

        Some(Element {
            element,
            run_length,
        })
    }
}
// Run Length Encoding:3 ends here

// [[file:../wills-columnar-format.org::#DataEncodingRunLengthEncoding-0vm696o03tj0][Run Length Encoding:4]]
impl<I> EncodeIter<I>
where
    I: Iterator,
{
    pub fn new(iter: I) -> EncodeIter<I> {
        EncodeIter {
            inner: iter.peekable(),
        }
    }
}
// Run Length Encoding:4 ends here
