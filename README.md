- [Introduction](#orgadbfa5e)
  - [Conventions](#orgc5d8550)
  - [Building and Testing Library](#orga5cdf4f)
- [Features](#orge778b76)
    - [V0 Features](#org7ef76dc)
    - [Tentative V1 Features](#org770772f)
- [API](#orgde83f7c)
  - [Encoding](#org13d2272)
  - [Decoding](#orge53aebb)
  - [Tests](#org52f82dd)
- [Optimization Tips](#orga855c8c)
  - [Sorting Data](#org68dae8f)
- [Format Specification](#orgcb9d487)
  - [Format Overview](#org86664c0)
  - [Header](#orga3d90f8)
- [Data Encoding](#org912c081)
  - [Basic Encoding](#org90f2ec1)
  - [Run Length Encoding](#org1bec034)
    - [Tests](#org3721ec1)
- [Source Code](#orgcd9ee37)



<a id="orgadbfa5e"></a>

# Introduction

**Will's Columnar Format V0**

[Will's Columnar Format](https://wmedrano.dev/literate-programs/wills-columnar-format) is a columnar format made by will.s.medrano@gmail.com. It is primarily implemented for educational purposes. If you are interested in using a well supported columnar format, consider using [Apache Parquet](https://parquet.apache.org/).


<a id="orgc5d8550"></a>

## Conventions

The following conventions are used:

-   All structs are encoded using [Bincode](https://github.com/bincode-org/bincode). Bincode is a binary encoding/decoding scheme implemented in Rust.
-   Source code snippets are presented for relatively high level constructs. Lower level details may be omitted from presentation.


<a id="orga5cdf4f"></a>

## Building and Testing Library

Will's Columnar Format is programmed in Org mode with Rust code blocks. Compiling requires Emacs and Cargo, the Rust package manager.

To generate the Rust source code, run `M-x org-babel-tangle` for `wills-columnar-format.org` within Emacs. To automatically tangle the current file on save, run:

```emacs-lisp
(add-hook 'after-save-hook #'org-babel-tangle 0 t)
```

Building and testing relies on Cargo.

```shell
cargo build
cargo test
cargo test $FN_TO_TEST
```


<a id="orge778b76"></a>

# Features


<a id="org7ef76dc"></a>

### V0 Features

V0 is roughly implemented but still requires graceful error handling, and bench-marking.

Supports:

-   Only a single column per encode/decode.
-   Integer (both signed and unsigned) and String types.
-   Run length encoding.


<a id="org770772f"></a>

### Tentative V1 Features

-   Dictionary encoding for better string compression.
-   Compression (like zstd or snappy) for data.
-   Multiple columns.
-   Push down filtering.
-   Split column data into blocks. Required to implement effective push down filtering.


<a id="orgde83f7c"></a>

# API


<a id="org13d2272"></a>

## Encoding

`encode_column` encodes a `Vec<T>` into Will's Columnar Format. If `use_rle` is true, then run length encoding will be used.

TODO: `use_rle` should have more granular values like `NEVER`, `ALWAYS`, and `AUTO`.

```rust
pub fn encode_column<T>(data: Vec<T>, use_rle: bool) -> Vec<u8>
where
    T: 'static + bincode::Encode + Eq,
{
    encode_column_impl(data, use_rle)
}
```


<a id="orge53aebb"></a>

## Decoding

`decode_column` decodes data from a byte stream into a `Vec<T>`.

TODO: Decoding should return an iterator of `rle::Element<T>` to support efficient reads of run-length-encoded data.

```rust
pub fn decode_column<T>(r: &mut impl std::io::Read) -> Vec<T>
where
    T: 'static + Clone + bincode::Decode,
{
    decode_column_impl(r)
}
```


<a id="org52f82dd"></a>

## Tests

```rust
#[test]
fn test_header_contains_magic_bytes() {
    let data: Vec<i64> = vec![1, 2, 3, 4];
    let encoded_data: Vec<u8> = encode_column(data.clone(), false);
    assert_eq!(&encoded_data[0..MAGIC_BYTES_LEN], b"wmedrano0");
}
```

```rust
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
```

```rust
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
```

```rust
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
```

```rust
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
```


<a id="orga855c8c"></a>

# Optimization Tips


<a id="org68dae8f"></a>

## Sorting Data

Sorting may be very beneficial if:

-   Order is not important.
-   There are lots of repeated values.

If the above are true, try sorting and enabling run length encoding. Run length encoding is efficient at storing data that is heavily repeated. By sorting, the data will have longer runs of consecutive repeated values.


<a id="orgcb9d487"></a>

# Format Specification


<a id="org86664c0"></a>

## Format Overview

-   `magic-bytes` - The magic bytes are 9 bytes long with the contents being "wmedrano0".
-   `header` - The header contains metadata about the column.
-   `data` - The encoded column data.


<a id="orga3d90f8"></a>

## Header

The header contains a Bincode V2 encoded struct:

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
    Integer = 0,
    String = 1,
}
```


<a id="org912c081"></a>

# Data Encoding


<a id="org90f2ec1"></a>

## Basic Encoding

The data consists of a sequence of encoded data. Encoding happens using the Rust [Bincode](https:github.com/bincode-org/bincode) v2 package to encode/decode data of type `&[T]` and `Vec<T>`.

Note: Bincode v2 currently in release candidate mode.

```rust
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
    assert_eq!(bincode_encoded_size((1u8, String::from("string"))), 8);

    // Fixed sized slices take up space for each of its encoded
    // elements. Variable size slices (or slice references) and vectors take
    // up an additional varint integer of overhead for encoding the length.
    assert_eq!(bincode_encoded_size::<&[u8; 3]>(&[1u8, 2, 3]), 3);
    assert_eq!(bincode_encoded_size::<[u8; 3]>([1u8, 2, 3]), 3);
    assert_eq!(bincode_encoded_size::<&[u8]>(&[1u8, 2, 3]), 4);
    assert_eq!(bincode_encoded_size(vec![1u8, 2, 3]), 4);
}
```


<a id="org1bec034"></a>

## Run Length Encoding

[Run length encoding](https://en.wikipedia.org/wiki/Run-length_encoding#:~:text=Run%2Dlength%20encoding%20(RLE),than%20as%20the%20original%20run.) is a compression technique for repeated values.

For RLE, the data is encoded as a Struct with the run length and the element. With Bincode, this is the equivalent (storage wise) of encoding a tuple of type `(run_length, element)`.

```rust
#[derive(Encode, Decode, Copy, Clone, PartialEq, Debug)]
pub struct Element<T> {
    // Run length is stored as a u64. We could try using a smaller datatype,
    // but Bincode uses "variable length encoding" for integers which is
    // efficient for smaller sizes.
    pub run_length: u64,
    pub element: T,
}

pub fn encode_data<T: Eq>(data: impl Iterator<Item = T>) -> impl Iterator<Item = Element<T>> {
    EncodeIter {
        inner: data.peekable(),
    }
}

pub fn decode_data<'a, T: 'static>(
    iter: impl 'a + Iterator<Item = &'a Element<T>>,
) -> impl Iterator<Item = &'a T> {
    iter.flat_map(|rle| {
        let run_length = rle.run_length as usize;
        std::iter::repeat(&rle.element).take(run_length)
    })
}
```


<a id="org3721ec1"></a>

### Tests

```rust
#[test]
fn test_encode_data_compacts_repeated_elements() {
    let data = [
        "repeated-3",
        "repeated-3",
        "repeated-3",
        "no-repeat",
        "repeated-2",
        "repeated-2",
        "repeated-3",
        "repeated-3",
        "repeated-3",
    ];
    assert_eq!(
        encode_data(data.into_iter()).collect::<Vec<_>>(),
        vec![
            Element {
                run_length: 3,
                element: "repeated-3"
            },
            Element {
                run_length: 1,
                element: "no-repeat"
            },
            Element {
                run_length: 2,
                element: "repeated-2"
            },
            Element {
                run_length: 3,
                element: "repeated-3"
            },
        ],
    );
}
```

```rust
#[test]
fn test_decode_repeats_elements_by_run_length() {
    let data = vec![
        Element {
            run_length: 3,
            element: "repeated-3",
        },
        Element {
            run_length: 1,
            element: "no-repeat",
        },
        Element {
            run_length: 2,
            element: "repeated-2",
        },
        Element {
            run_length: 3,
            element: "repeated-3",
        },
    ];
    let decoded_data: Vec<&str> = decode_data(data.iter()).cloned().collect();
    assert_eq!(
        decoded_data,
        [
            "repeated-3",
            "repeated-3",
            "repeated-3",
            "no-repeat",
            "repeated-2",
            "repeated-2",
            "repeated-3",
            "repeated-3",
            "repeated-3",
        ]
    );
}
```


<a id="orgcd9ee37"></a>

# Source Code

The source code is stored at <https://github.com/wmedrano/wills-columnar-format>. The main source file is `wills-columnar-format.org` which is used to generate the Rust source files like `src/lib.rs`.