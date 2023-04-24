// [[file:../wills-columnar-format.org::*Basic Encoding][Basic Encoding:2]]
use crate::rle;
fn bincode_encoded_size<T: bincode::Encode>(element: T) -> usize {
    bincode::encode_to_vec(element, bincode::config::standard()).unwrap().len()
}
// Basic Encoding:2 ends here

// [[file:../wills-columnar-format.org::*Basic Encoding][Basic Encoding:3]]
#[test]
fn test_encoding_size() {
    // Small numbers are encoded efficiently.
    assert_eq!(bincode_encoded_size(1u8), 1);
    assert_eq!(bincode_encoded_size(-1i8), 1);
    assert_eq!(bincode_encoded_size(1u64), 1);
    assert_eq!(bincode_encoded_size(-1i64), 1);

    // Larger numbers use more bytes with varint encoding. This does not apply
    // to u8 and i8 which do not use varint.
    assert_eq!(bincode_encoded_size(255u16), 3);
    assert_eq!(bincode_encoded_size(255u8), 1);
    assert_eq!(bincode_encoded_size(127i8), 1);
    assert_eq!(bincode_encoded_size(-128i8), 1);

    // Derived types (like Structs and Tuples) take up as much space as their subcomponents.
    assert_eq!(bincode_encoded_size(1u64), 1);
    assert_eq!(bincode_encoded_size(25564), 3);
    assert_eq!(bincode_encoded_size((1u64, 255u64)), 4);
    assert_eq!(
        bincode_encoded_size(rle::Element {
            element: 1u64,
            run_length: 255
        }),
        4
    );

    // Strings take up string_length + 1.
    assert_eq!(bincode_encoded_size("string"), 7);
    assert_eq!(bincode_encoded_size(String::from("string")), 7);
}
// Basic Encoding:3 ends here
