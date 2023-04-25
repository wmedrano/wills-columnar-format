// [[file:../wills-columnar-format.org::*Introduction][Introduction:4]]
use super::*;
// Introduction:4 ends here

// [[file:../wills-columnar-format.org::*API Tests][API Tests:1]]
#[test]
fn test_header_contains_magic_bytes() {
    let data: Vec<i64> = vec![1, 2, 3, 4];
    let encoded_data: Vec<u8> = encode_column(data.clone(), false);
    assert_eq!(&encoded_data[0..MAGIC_BYTES_LEN], b"wmedrano0");
}
// API Tests:1 ends here

// [[file:../wills-columnar-format.org::*API Tests][API Tests:2]]
fn test_can_encode_and_decode_for_type<T>(elements: [T; 2])
where
    T: 'static + Clone + Encode + Decode + Eq + std::fmt::Debug,
{
    let data: Vec<T> = elements.to_vec();
    let encoded_data: Vec<u8> = encode_column(data.clone(), false);
    let mut encoded_data_cursor = std::io::Cursor::new(encoded_data);
    assert_eq!(decode_column::<T>(&mut encoded_data_cursor), elements);
}
// API Tests:2 ends here

// [[file:../wills-columnar-format.org::*API Tests][API Tests:3]]
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
// API Tests:3 ends here

// [[file:../wills-columnar-format.org::*API Tests][API Tests:4]]
#[test]
fn test_encode_decode_integer() {
    let data: Vec<i64> = vec![-1, 10, 10, 10, 11, 12, 12, 10];
    let encoded_data = encode_column(data.clone(), false);
    assert_eq!(encoded_data.len(), 22);

    let mut encoded_data_cursor = std::io::Cursor::new(encoded_data);
    assert_eq!(
        decode_column::<i64>(&mut encoded_data_cursor),
        vec![-1, 10, 10, 10, 11, 12, 12, 10]
    );
}
// API Tests:4 ends here

// [[file:../wills-columnar-format.org::*API Tests][API Tests:5]]
#[test]
fn test_encode_decode_string() {
    let data: Vec<&'static str> = vec!["foo", "foo", "foo", "bar", "baz", "foo"];
    let encoded_data = encode_column(data.clone(), false);
    assert_eq!(encoded_data.len(), 38);

    let mut encoded_data_cursor = std::io::Cursor::new(encoded_data);
    assert_eq!(
        decode_column::<String>(&mut encoded_data_cursor),
        vec!["foo", "foo", "foo", "bar", "baz", "foo"]
    );
}
// API Tests:5 ends here

// [[file:../wills-columnar-format.org::*API Tests][API Tests:6]]
#[test]
fn test_encode_decode_string_with_rle() {
    let data = ["foo", "foo", "foo", "bar", "baz", "foo"];
    let encoded_data = encode_column(data.to_vec(), true);
    assert_eq!(encoded_data.len(), 34);

    let mut encoded_data_cursor = std::io::Cursor::new(encoded_data);
    assert_eq!(
        decode_column::<String>(&mut encoded_data_cursor),
        vec!["foo", "foo", "foo", "bar", "baz", "foo"]
    );
}
// API Tests:6 ends here
