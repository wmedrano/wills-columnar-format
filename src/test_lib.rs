// [[file:../wills-columnar-format.org::#Introduction-h6a696o03tj0][Introduction:4]]
use super::*;
use itertools::assert_equal;
// Introduction:4 ends here

// [[file:../wills-columnar-format.org::#APITests-vfh696o03tj0][Tests:1]]
#[test]
fn test_header_contains_magic_bytes() {
    let data: Vec<i64> = vec![1, 2, 3, 4];
    let encoded_data: Vec<u8> = encode_column(data.clone(), false);
    assert_eq!(&encoded_data[0..MAGIC_BYTES_LEN], b"wmedrano0");
}
// Tests:1 ends here

// [[file:../wills-columnar-format.org::#APITests-vfh696o03tj0][Tests:2]]
fn test_can_encode_and_decode_for_type<T>(elements: [T; 2])
where
    T: 'static + Clone + Encode + Decode + Eq + std::fmt::Debug,
{
    let data: Vec<T> = elements.to_vec();
    let encoded_data: Vec<u8> = encode_column(data.clone(), false);
    let mut encoded_data_cursor = std::io::Cursor::new(encoded_data);
    assert_equal(
        decode_column::<T>(&mut encoded_data_cursor),
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
    let encoded_data = encode_column(data.clone(), false);
    assert_eq!(encoded_data.len(), 22);

    let mut encoded_data_cursor = std::io::Cursor::new(encoded_data);
    assert_equal(
        decode_column::<i64>(&mut encoded_data_cursor),
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
    let encoded_data = encode_column(data.clone(), false);
    assert_eq!(encoded_data.len(), 38);

    let mut encoded_data_cursor = std::io::Cursor::new(encoded_data);
    assert_equal(
        decode_column::<String>(&mut encoded_data_cursor),
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
    let encoded_data = encode_column(data.to_vec(), true);
    assert_eq!(encoded_data.len(), 34);

    let mut encoded_data_cursor = std::io::Cursor::new(encoded_data);
    assert_equal(
        decode_column::<String>(&mut encoded_data_cursor),
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
