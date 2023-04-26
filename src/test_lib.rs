// [[file:../wills-columnar-format.org::#IntroductionCargotoml-cqc696o03tj0][Dependencies:6]]
use super::*;
use itertools::assert_equal;
// Dependencies:6 ends here

// [[file:../wills-columnar-format.org::#APITests-vfh696o03tj0][Tests:1]]
#[test]
fn test_encoding_prefixed_by_magic_bytes() {
    let data: Vec<i64> = vec![1, 2, 3, 4];
    let encoded_data: Vec<u8> = encode_column(data.into_iter(), false).unwrap();
    assert_eq!(&encoded_data[0..MAGIC_BYTES_LEN], b"wmedrano0");
}
// Tests:1 ends here

// [[file:../wills-columnar-format.org::#APITests-vfh696o03tj0][Tests:2]]
fn test_can_encode_and_decode_for_type<T>(elements: [T; 2])
where
    T: 'static + Clone + Encode + Decode + Eq + std::fmt::Debug,
{
    let data: Vec<T> = elements.to_vec();
    let encoded_data: Vec<u8> = encode_column(data.into_iter(), false).unwrap();
    assert_eq!(&encoded_data[0..9], b"wmedrano0");
    let mut encoded_data_cursor = std::io::Cursor::new(encoded_data);
    assert_equal(
        decode_column::<T>(&mut encoded_data_cursor)
            .unwrap()
            .map(Result::unwrap),
        [
            rle::Element {
                element: elements[0].clone(),
                run_length: 1,
            },
            rle::Element {
                element: elements[1].clone(),
                run_length: 1,
            },
        ],
    );
}
// Tests:2 ends here

// [[file:../wills-columnar-format.org::#APITests-vfh696o03tj0][Tests:3]]
#[test]
fn test_encode_decode_several() {
    test_can_encode_and_decode_for_type::<i8>([-1, -1]);
    test_can_encode_and_decode_for_type::<u8>([1, 2]);
    test_can_encode_and_decode_for_type::<i16>([-1, 1]);
    test_can_encode_and_decode_for_type::<u16>([1, 2]);
    test_can_encode_and_decode_for_type::<i32>([-1, 1]);
    test_can_encode_and_decode_for_type::<u32>([1, 2]);
    test_can_encode_and_decode_for_type::<i64>([-1, 1]);
    test_can_encode_and_decode_for_type::<u64>([1, 2]);
    test_can_encode_and_decode_for_type::<String>(["a".to_string(), "b".to_string()]);
}
// Tests:3 ends here

// [[file:../wills-columnar-format.org::#APITests-vfh696o03tj0][Tests:4]]
#[test]
fn test_encode_decode_integer() {
    let data: Vec<i64> = vec![-1, 10, 10, 10, 11, 12, 12, 10];
    let encoded_data = encode_column(data.into_iter(), false).unwrap();
    assert_eq!(
        encoded_data.len(),
        [
            9, // magic_bytes
            1, // u8 header:data_type
            1, // u8 header:use_rle
            1, // varint header:element_count
            1, // varint header:data_size
            8, // data contains 8 elements of varint with size 1.
        ]
        .iter()
        .sum()
    );

    let mut encoded_data_cursor = std::io::Cursor::new(encoded_data);
    assert_equal(
        decode_column::<i64>(&mut encoded_data_cursor)
            .unwrap()
            .map(Result::unwrap),
        [
            rle::Element {
                element: -1,
                run_length: 1,
            },
            rle::Element {
                element: 10,
                run_length: 1,
            },
            rle::Element {
                element: 10,
                run_length: 1,
            },
            rle::Element {
                element: 10,
                run_length: 1,
            },
            rle::Element {
                element: 11,
                run_length: 1,
            },
            rle::Element {
                element: 12,
                run_length: 1,
            },
            rle::Element {
                element: 12,
                run_length: 1,
            },
            rle::Element {
                element: 10,
                run_length: 1,
            },
        ],
    );
}
// Tests:4 ends here

// [[file:../wills-columnar-format.org::#APITests-vfh696o03tj0][Tests:5]]
#[test]
fn test_encode_decode_string() {
    let data: Vec<&'static str> = vec!["foo", "foo", "foo", "bar", "baz", "foo"];
    let encoded_data = encode_column(data.into_iter(), false).unwrap();
    assert_eq!(
        encoded_data.len(),
        [
            9,  // magic_bytes
            1,  // u8 header:data_type
            1,  // u8 header:use_rle
            1,  // varint header:element_count
            1,  // varint header:data_size
            24, // data contains 8 elements of varint with size 1.
        ]
        .iter()
        .sum()
    );

    let mut encoded_data_cursor = std::io::Cursor::new(encoded_data);
    assert_equal(
        decode_column::<String>(&mut encoded_data_cursor)
            .unwrap()
            .map(Result::unwrap),
        [
            rle::Element {
                element: "foo".to_string(),
                run_length: 1,
            },
            rle::Element {
                element: "foo".to_string(),
                run_length: 1,
            },
            rle::Element {
                element: "foo".to_string(),
                run_length: 1,
            },
            rle::Element {
                element: "bar".to_string(),
                run_length: 1,
            },
            rle::Element {
                element: "baz".to_string(),
                run_length: 1,
            },
            rle::Element {
                element: "foo".to_string(),
                run_length: 1,
            },
        ],
    );
}
// Tests:5 ends here

// [[file:../wills-columnar-format.org::#APITests-vfh696o03tj0][Tests:6]]
#[test]
fn test_encode_decode_string_with_rle() {
    let data = ["foo", "foo", "foo", "bar", "baz", "foo"];
    let encoded_data = encode_column(data.into_iter(), true).unwrap();
    assert_eq!(
        encoded_data.len(),
        [
            9, // magic_bytes
            1, // u8 header:data_type
            1, // u8 header:use_rle
            1, // varint header:element_count
            1, // varint header:data_size
            4, // data:element_1:rle_element string "foo" of encoding size 4.
            1, // data:element_1:rle_run_length varint of size 1.
            4, // data:element_2:rle_element string "bar" of encoding size 4.
            1, // data:element_2:rle_run_length varint of size 1.
            4, // data:element_3:rle_element string "baz" of encoding size 4.
            1, // data:element_3:rle_run_length varint of size 1.
            4, // data:element_3:rle_element string "foo" of encoding size 4.
            1, // data:element_3:rle_run_length varint of size 1.
        ]
        .iter()
        .sum()
    );

    let mut encoded_data_cursor = std::io::Cursor::new(encoded_data);
    assert_equal(
        decode_column::<String>(&mut encoded_data_cursor)
            .unwrap()
            .map(Result::unwrap),
        [
            rle::Element {
                element: "foo".to_string(),
                run_length: 3,
            },
            rle::Element {
                element: "bar".to_string(),
                run_length: 1,
            },
            rle::Element {
                element: "baz".to_string(),
                run_length: 1,
            },
            rle::Element {
                element: "foo".to_string(),
                run_length: 1,
            },
        ],
    );
}
// Tests:6 ends here
