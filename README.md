- [Introduction](#org377ccf3)
  - [Conventions](#org5b9a842)
- [API](#orga722ea8)
  - [V0 Features](#orgac58ce3)
  - [Tentative V1 Features](#orgf9f220b)
  - [Encoding](#org30f4f5f)
  - [Decoding](#org2168113)
  - [Tests](#org4c987cf)
- [Format Specification](#orgd66247f)
  - [Header](#org5f59a68)
  - [Data](#orgee08fde)
    - [Basic Encoding](#orgd984108)
    - [RLE](#org4344e29)
- [Source Code](#org73f3ec6)



<a id="org377ccf3"></a>

# Introduction

[Will's columnar format](https://wmedrano.dev/living-programs/wills-columnar-format) is a columnar format made by will.s.medrano@gmail.com. It is primarily implemented for educational purposes. If you are interested in using a well supported columnar format, consider using [Apache Parquet](https://parquet.apache.org/).


<a id="org5b9a842"></a>

## Conventions

The following conventions are used:

-   All structs are encoded using [Bincode](https://github.com/bincode-org/bincode). Bincode is a binary encoding/decoding scheme implemented in Rust.
-   Source code snippets are presented for relatively high level constructs. Lower level details may be omitted from presentation.


<a id="orga722ea8"></a>

# API


<a id="orgac58ce3"></a>

## V0 Features

V0 is roughly implemented but still requires graceful error handling, and bench-marking.

Supports:

-   Only a single column per encode/decode.
-   `i64` and `String` types.
-   Run length encoding.


<a id="orgf9f220b"></a>

## Tentative V1 Features

-   Dictionary encoding for better string compression.
-   Compression (like zstd or snappy) for data.
-   Multiple columns.
-   Push down filtering.
-   Split column data into blocks. Required to implement effective push down filtering.


<a id="org30f4f5f"></a>

## Encoding

`encode_column` encodes a `Vec<T>` into Will's Columnar Format. If `use_rle` is true, then run length encoding will be used.

TODO: `use_rle` should have more granular values like `NEVER`, `ALWAYS`, and `AUTO`.

```rust
pub fn encode_column<T>(data: Vec<T>, use_rle: bool) -> Vec<u8>
where
    T: 'static + bincode::Encode + Eq {
    encode_column_impl(data, use_rle)
}
```


<a id="org2168113"></a>

## Decoding

`decode_column` decodes data from a byte stream into a `Vec<T>`.

TODO: Decoding should return an iterator of `RleElement<T>` to support efficient reads of run-length-encoded data.

```rust
pub fn decode_column<T>(r: &mut impl std::io::Read) -> Vec<T>
where
    T: 'static + Clone + bincode::Decode {
    decode_column_impl(r)
}
```


<a id="org4c987cf"></a>

## Tests

```rust
#[test]
fn test_header_contains_magic_bytes() {
    let data: Vec<i64> = vec![1, 2, 3, 4];
    let encoded_data = encode_column(data.clone(), false);
    assert_eq!(&encoded_data[0..MAGIC_BYTES_LEN], b"wmedrano0");
}
```

```rust
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
```

```rust
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
```

```rust
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
```


<a id="orgd66247f"></a>

# Format Specification

-   `magic-bytes` - The magic bytes are "wmedrano0".
-   `header` - The header contains metadata about the column.
-   `data` - The encoded column data.


<a id="org5f59a68"></a>

## Header

The header contains an encoded struct:

```rust
#[derive(Encode, Decode, PartialEq, Eq, Copy, Clone, Debug)]
pub struct Header {
    pub data_type: DataType,
    pub is_rle: bool,
    pub elements: usize,
    pub data_size: usize,
}

#[derive(Encode, Decode, PartialEq, Eq, Copy, Clone, Debug)]
pub enum DataType {
    I64 = 0,
    String = 1,
}
```


<a id="orgee08fde"></a>

## Data


<a id="orgd984108"></a>

### Basic Encoding

The data consists of a sequence of encoded data. Encoding happens using the `bincode` package to encode/decode data of type `&[T]` and `Vec<T>`.


<a id="org4344e29"></a>

### RLE

[Run length encoding](https://en.wikipedia.org/wiki/Run-length_encoding#:~:text=Run%2Dlength%20encoding%20(RLE),than%20as%20the%20original%20run.) is a compression technique for repeated values.

For RLE, the data is encoded as a Struct with the run length and the element. With Bincode, this is the equivalent of encoding a tuple of type `(run_length, element)`.

```rust
#[derive(Encode, Decode, Copy, Clone, PartialEq, Debug)]
pub struct RleElement<T> {
    pub run_length: u64,
    pub element: T,
}

fn rle_encode_data<T: Eq>(data: impl Iterator<Item = T>) -> Vec<RleElement<T>> {
    let mut data = data;
    let mut rle = match data.next() {
        Some(e) => RleElement{run_length: 1, element: e},
        None => return Vec::new(),
    };

    let mut ret = Vec::new();
    for element in data {
        if element != rle.element || rle.run_length == u64::MAX {
            ret.push(std::mem::replace(&mut rle, RleElement{run_length: 1, element}));
        } else {
            rle.run_length += 1;
        }
    }
    if rle.run_length > 0 {
        ret.push(rle);
    }
    ret
}

fn rle_decode_data<'a, T: 'static>(
    iter: impl 'a + Iterator<Item = &'a RleElement<T>>,
) -> impl Iterator<Item = &'a T> {
    iter.flat_map(move |rle| {
        let run_length = rle.run_length as usize;
        std::iter::repeat(&rle.element).take(run_length)
    })
}
```


<a id="org73f3ec6"></a>

# Source Code

The source code is stored at <https://github.com/wmedrano/wills-columnar-format>. The main source file is `wills-columnar-format.org` which is used to generate the `src/lib.rs`.