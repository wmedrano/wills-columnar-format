// [[file:../wills-columnar-format.org::#IntroductionCargotoml-cqc696o03tj0][Dependencies:5]]
use crate::rle;
// Dependencies:5 ends here

// [[file:../wills-columnar-format.org::#DataEncodingBasicEncodingTests-sfz7wx714tj0][Tests:1]]
fn encoded_size<T: bincode::Encode>(element: T) -> usize {
    bincode::encode_to_vec(element, bincode::config::standard())
        .unwrap()
        .len()
}
// Tests:1 ends here

// [[file:../wills-columnar-format.org::#DataEncodingBasicEncodingTests-sfz7wx714tj0][Tests:2]]
#[test]
fn test_encoding_size() {
    // Small numbers are encoded efficiently.
    assert_eq!(encoded_size(1u8), 1);
    assert_eq!(encoded_size(-1i8), 1);
    assert_eq!(encoded_size(1u64), 1);
    assert_eq!(encoded_size(-1i64), 1);

    // Larger numbers use more bytes with varint encoding. This does not apply
    // to u8 and i8 which do not use varint.
    assert_eq!(encoded_size(255u16), 3);
    assert_eq!(encoded_size(255u8), 1);
    assert_eq!(encoded_size(127i8), 1);
    assert_eq!(encoded_size(-128i8), 1);

    // Derived types (like Structs and Tuples) take up as much space as their subcomponents.
    assert_eq!(encoded_size(1u64), 1);
    assert_eq!(encoded_size(25564), 3);
    assert_eq!(encoded_size((1u64, 255u64)), 4);
    assert_eq!(
        encoded_size(rle::Element {
            element: 1u64,
            run_length: 255
        }),
        4
    );

    // Strings take up string_length + 1.
    assert_eq!(encoded_size("string"), 7);
    assert_eq!(encoded_size(String::from("string")), 7);
    assert_eq!(encoded_size((1u8, String::from("string"))), 8);

    // Fixed sized slices take up space for each of its encoded
    // elements. Variable size slices (or slice references) and vectors take
    // up an additional varint integer of overhead for encoding the length.
    assert_eq!(encoded_size::<&[u8; 3]>(&[1u8, 2, 3]), 3);
    assert_eq!(encoded_size::<[u8; 3]>([1u8, 2, 3]), 3);
    assert_eq!(encoded_size::<&[u8]>(&[1u8, 2, 3]), 4);
    assert_eq!(encoded_size(vec![1u8, 2, 3]), 4);
}
// Tests:2 ends here
