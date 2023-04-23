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

pub fn encode_data<T: Eq>(data: impl Iterator<Item = T>) -> Vec<Element<T>> {
    let mut data = data;
    let mut rle = match data.next() {
        Some(e) => Element{run_length: 1, element: e},
        None => return Vec::new(),
    };

    let mut ret = Vec::new();
    for element in data {
        if element != rle.element || rle.run_length == u64::MAX {
            ret.push(std::mem::replace(&mut rle, Element{run_length: 1, element}));
        } else {
            rle.run_length += 1;
        }
    }
    if rle.run_length > 0 {
        ret.push(rle);
    }
    ret
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
