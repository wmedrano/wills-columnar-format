// [[file:../wills-columnar-format.org::*Tests][Tests:2]]
use super::*;
// Tests:2 ends here

// [[file:../wills-columnar-format.org::*Tests][Tests:3]]
#[test]
fn test_header_contains_magic_bytes() {
    let data: Vec<i64> = vec![1, 2, 3, 4];
    let encoded_data = encode_column(data.clone(), false);
    assert_eq!(&encoded_data[0..MAGIC_BYTES_LEN], b"wmedrano0");
}
// Tests:3 ends here

// [[file:../wills-columnar-format.org::*Tests][Tests:4]]
fn test_encode_decode_for_type<T>()
where
    T: 'static + Clone + Default + Encode + Decode + Eq + std::fmt::Debug{
    let data: Vec<T> = vec![T::default()];
    let encoded_data = encode_column(data.clone(), false);
    let mut encoded_data_cursor = std::io::Cursor::new(encoded_data);
    assert_eq!(
        decode_column::<T>(&mut encoded_data_cursor),
        vec![T::default()]);
}
#[test]
fn test_encode_decode_several() {
    test_encode_decode_for_type::<i8>();
    test_encode_decode_for_type::<u8>();
    test_encode_decode_for_type::<i16>();
    test_encode_decode_for_type::<u16>();
    test_encode_decode_for_type::<i32>();
    test_encode_decode_for_type::<u32>();
    test_encode_decode_for_type::<i64>();
    test_encode_decode_for_type::<u64>();
    test_encode_decode_for_type::<String>();
}
// Tests:4 ends here

// [[file:../wills-columnar-format.org::*Tests][Tests:5]]
#[test]
fn test_encode_decode_integer() {
    let data: Vec<i64> = vec![-1, 10, 10, 10, 11, 12, 12, 10];
    let encoded_data = encode_column(data.clone(), false);
    assert_eq!(encoded_data.len(), 22);

    let mut encoded_data_cursor = std::io::Cursor::new(encoded_data);
    assert_eq!(
        decode_column::<i64>(&mut encoded_data_cursor),
        vec![-1, 10, 10, 10, 11, 12, 12, 10]);
}
// Tests:5 ends here

// [[file:../wills-columnar-format.org::*Tests][Tests:6]]
#[test]
fn test_encode_decode_string() {
    let data: Vec<&'static str> = Vec::from_iter([
        "foo",
        "foo",
        "foo",
        "bar",
        "baz",
        "foo",
    ].into_iter());
    let encoded_data = encode_column(data.clone(), false);
    assert_eq!(encoded_data.len(), 38);

    let mut encoded_data_cursor = std::io::Cursor::new(encoded_data);
    assert_eq!(
        decode_column::<String>(&mut encoded_data_cursor),
        vec!["foo", "foo", "foo", "bar", "baz", "foo"]);
}
// Tests:6 ends here

// [[file:../wills-columnar-format.org::*Tests][Tests:7]]
#[test]
fn test_encode_decode_string_with_rle() {
    let data = [
        "foo",
        "foo",
        "foo",
        "bar",
        "baz",
        "foo",
    ];
    let encoded_data = encode_column(data.to_vec(), true);
    assert_eq!(encoded_data.len(), 34);

    let mut encoded_data_cursor = std::io::Cursor::new(encoded_data);
    assert_eq!(
        decode_column::<String>(&mut encoded_data_cursor),
        vec!["foo", "foo", "foo", "bar", "baz", "foo"]);
}
// Tests:7 ends here
