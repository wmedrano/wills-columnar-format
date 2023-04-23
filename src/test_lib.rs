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
#[test]
fn test_encode_decode_i64() {
    let data: Vec<i64> = vec![-1, 10, 10, 10, 11, 12, 12, 10];
    let encoded_data = encode_column(data.clone(), false);
    assert_eq!(encoded_data.len(), 22);

    let mut encoded_data_cursor = std::io::Cursor::new(encoded_data);
    assert_eq!(
        decode_column::<i64>(&mut encoded_data_cursor),
        vec![-1, 10, 10, 10, 11, 12, 12, 10]);
}
// Tests:4 ends here

// [[file:../wills-columnar-format.org::*Tests][Tests:5]]
#[test]
fn test_encode_decode_string() {
    let data: Vec<String> = Vec::from_iter([
        "foo",
        "foo",
        "foo",
        "bar",
        "baz",
        "foo",
    ].into_iter().map(String::from));
    let encoded_data = encode_column(data.clone(), false);
    assert_eq!(encoded_data.len(), 38);

    let mut encoded_data_cursor = std::io::Cursor::new(encoded_data);
    assert_eq!(
        decode_column::<String>(&mut encoded_data_cursor),
        vec!["foo", "foo", "foo", "bar", "baz", "foo"]);
}
// Tests:5 ends here

// [[file:../wills-columnar-format.org::*Tests][Tests:6]]
#[test]
fn test_encode_decode_string_with_rle() {
    let data: Vec<String> = Vec::from_iter([
        "foo",
        "foo",
        "foo",
        "bar",
        "baz",
        "foo",
    ].into_iter().map(String::from));
    let encoded_data = encode_column(data.clone(), true);
    assert_eq!(encoded_data.len(), 34);

    let mut encoded_data_cursor = std::io::Cursor::new(encoded_data);
    assert_eq!(
        decode_column::<String>(&mut encoded_data_cursor),
        vec!["foo", "foo", "foo", "bar", "baz", "foo"]);
}
// Tests:6 ends here
