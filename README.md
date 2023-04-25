- [Introduction](#Introduction-h6a696o03tj0)
  - [Conventions](#IntroductionConventions-gbb696o03tj0)
  - [Building and Testing Library](#IntroductionBuildingandTestingLibrary-r0c696o03tj0)
  - [Cargo.toml](#IntroductionCargotoml-cqc696o03tj0)
- [Features](#Features-0ed696o03tj0)
    - [V0 Features](#FeaturesV0Features-81e696o03tj0)
    - [Tentative V1 Features](#FeaturesTentativeV1Features-ppe696o03tj0)
- [API](#API-6ef696o03tj0)
  - [Encoding](#APIEncoding-w0g696o03tj0)
  - [Decoding](#APIDecoding-npg696o03tj0)
  - [Optimization Tips](#OptimizationTips-45i696o03tj0)
    - [Sorting Data](#OptimizationTipsSortingData-rsi696o03tj0)
  - [Tests](#APITests-vfh696o03tj0)
- [Format Specification](#FormatSpecification-zfj696o03tj0)
  - [Format Overview](#FormatSpecificationFormatOverview-j3k696o03tj0)
  - [Header](#FormatSpecificationHeader-3tk696o03tj0)
- [Data Encoding](#DataEncoding-sgl696o03tj0)
  - [Basic Encoding](#DataEncodingBasicEncoding-e4m696o03tj0)
  - [Run Length Encoding](#DataEncodingRunLengthEncoding-0vm696o03tj0)
    - [Tests](#DataEncodingRunLengthEncodingTests-xhn696o03tj0)
- [Source Code](#SourceCode-45o696o03tj0)



<a id="Introduction-h6a696o03tj0"></a>

# Introduction

**Will's Columnar Format V0**

[Will's Columnar Format](https://wmedrano.dev/literate-programs/wills-columnar-format) is a columnar format made by will.s.medrano@gmail.com. It is primarily implemented for educational purposes. If you are interested in using a well supported columnar format, consider using [Apache Parquet](https://parquet.apache.org/).


<a id="IntroductionConventions-gbb696o03tj0"></a>

## Conventions

The following conventions are used:

-   All structs are encoded using [Bincode](https://github.com/bincode-org/bincode). Bincode is a binary encoding/decoding scheme implemented in Rust.
-   Source code snippets are presented for relatively high level constructs. Lower level details may be omitted from presentation.


<a id="IntroductionBuildingandTestingLibrary-r0c696o03tj0"></a>

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


<a id="IntroductionCargotoml-cqc696o03tj0"></a>

## Cargo.toml

```toml
[package]
name = "columnar-format"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
bincode = "2.0.0-rc.3"
itertools = "0.10"
```


<a id="Features-0ed696o03tj0"></a>

# Features


<a id="FeaturesV0Features-81e696o03tj0"></a>

### V0 Features

V0 is roughly implemented but still requires graceful error handling, and bench-marking.

Supports:

-   Only a single column per encode/decode.
-   Integer (both signed and unsigned) and String types.
-   Run length encoding.


<a id="FeaturesTentativeV1Features-ppe696o03tj0"></a>

### Tentative V1 Features

-   Automatically determine if RLE should be applied.
-   Dictionary encoding for better string compression.
-   Compression (like zstd or snappy) for data.
-   Multiple columns.
-   Push down filtering.
-   Split column data into blocks. Required to implement effective push down filtering.


<a id="API-6ef696o03tj0"></a>

# API


<a id="APIEncoding-w0g696o03tj0"></a>

## Encoding

`encode_column` encodes a `Vec<T>` into Will's Columnar Format. If `use_rle` is true, then run length encoding will be used.

```rust
pub fn encode_column<T>(data: Vec<T>, use_rle: bool) -> Vec<u8>
where
    T: 'static + bincode::Encode + Eq,
{
    encode_column_impl(data, use_rle)
}
```


<a id="APIDecoding-npg696o03tj0"></a>

## Decoding

`decode_column` decodes data from a byte stream into an iterator of `rle::Element<T>`. See [Run Length Encoding](#DataEncodingRunLengthEncoding-0vm696o03tj0).

```rust
pub fn decode_column<T>(r: &mut impl std::io::Read) -> impl Iterator<Item = rle::Element<T>>
where
    T: 'static + Clone + bincode::Decode,
{
    decode_column_impl(r)
}
```


<a id="OptimizationTips-45i696o03tj0"></a>

## Optimization Tips


<a id="OptimizationTipsSortingData-rsi696o03tj0"></a>

### Sorting Data

Sorting may be very beneficial if:

-   Order is not important.
-   There are lots of repeated values.

If the above are true, try sorting and enabling run length encoding. Run length encoding is efficient at storing data that is heavily repeated. By sorting, the data will have longer runs of consecutive repeated values.


<a id="APITests-vfh696o03tj0"></a>

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
```

```rust
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
```

```rust
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
```


<a id="FormatSpecification-zfj696o03tj0"></a>

# Format Specification


<a id="FormatSpecificationFormatOverview-j3k696o03tj0"></a>

## Format Overview

-   `magic-bytes` - The magic bytes are 9 bytes long with the contents being "wmedrano0".
-   `header` - The header contains metadata about the column.
-   `data` - The encoded column data.

```rust
fn encode_column_impl<T>(data: Vec<T>, use_rle: bool) -> Vec<u8>
where
    T: 'static + bincode::Encode + Eq,
{
    let elements = data.len();
    let encoded_data = if use_rle {
        encode_data_rle_impl(data)
    } else {
        encode_data_base_impl(data)
    };
    let header = Header {
        data_type: DataType::from_type::<T>().unwrap(),
        use_rle,
        elements,
        data_size: encoded_data.len(),
    };
    encode_header_and_data(MAGIC_BYTES, header, encoded_data)
}
```


<a id="FormatSpecificationHeader-3tk696o03tj0"></a>

## Header

The header contains a Bincode V2 encoded struct:

```rust
#[derive(Encode, Decode, PartialEq, Eq, Copy, Clone, Debug)]
pub struct Header {
    pub data_type: DataType,
    pub use_rle: bool,
    pub elements: usize,
    pub data_size: usize,
}

#[derive(Encode, Decode, PartialEq, Eq, Copy, Clone, Debug)]
pub enum DataType {
    Integer = 0,
    String = 1,
}
```


<a id="DataEncoding-sgl696o03tj0"></a>

# Data Encoding


<a id="DataEncodingBasicEncoding-e4m696o03tj0"></a>

## Basic Encoding

The data consists of a sequence of encoded data. Encoding happens using the Rust [Bincode](https:github.com/bincode-org/bincode) v2 package to encode/decode data of type `&[T]` and `Vec<T>`.

Note: Bincode v2 currently in release candidate.

```rust
fn encode_data_base_impl<T: 'static + bincode::Encode>(data: Vec<T>) -> Vec<u8> {
    bincode::encode_to_vec(data, BINCODE_DATA_CONFIG).unwrap()
}
```

```rust
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
```


<a id="DataEncodingRunLengthEncoding-0vm696o03tj0"></a>

## Run Length Encoding

Run length encoding [[Wikipedia](https://en.wikipedia.org/wiki/Run-length_encoding#:~:text=Run%2Dlength%20encoding%20(RLE),than%20as%20the%20original%20run.)] is a compression technique for repeated values.

```rust
#[derive(Encode, Decode, Copy, Clone, PartialEq, Debug)]
pub struct Element<T> {
    // The underlying element.
    pub element: T,
    // Run length is stored as a u64. We could try using a smaller datatype,
    // but Bincode uses "variable length encoding" for integers which is
    // efficient for smaller sizes.
    pub run_length: u64,
}
```

To encode data of type `Vec<T>` with RLE, it is first converted into a `Vec<rle::Element<T>>`. It is then used to encode the run length encoded vector into bytes.

```rust
fn encode_data_rle_impl<T: 'static + bincode::Encode + Eq>(data: Vec<T>) -> Vec<u8> {
    let rle_data: Vec<rle::Element<T>> = rle::EncodeIter::new(data.into_iter()).collect();
    encode_data_base_impl(rle_data)
}
```

```rust
pub struct EncodeIter<I: Iterator> {
    inner: std::iter::Peekable<I>,
}

impl<I> Iterator for EncodeIter<I>
where
    I: Iterator,
    I::Item: PartialEq,
{
    type Item = Element<I::Item>;

    fn next(&mut self) -> Option<Element<I::Item>> {
        // Start the run or exit if the underlying iterator is empty.
        let element = match self.inner.next() {
            Some(e) => e,
            None => return None,
        };
        let mut run_length = 1;

        // Continue the run as long as the next element is equal to the current running element.
        while self.inner.next_if_eq(&element).is_some() {
            run_length += 1;
        }

        Some(Element {
            element,
            run_length,
        })
    }
}
```


<a id="DataEncodingRunLengthEncodingTests-xhn696o03tj0"></a>

### Tests

```rust
#[test]
fn test_encode_data_without_elements_produces_no_elements() {
    let data: Vec<String> = vec![];
    assert_equal(EncodeIter::new(data.into_iter()), []);
}

#[test]
fn test_encode_data_combines_repeated_elements() {
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
    assert_equal(
        EncodeIter::new(data.into_iter()),
        [
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
        ],
    );
}
```


<a id="SourceCode-45o696o03tj0"></a>

# Source Code

The source code is stored at <https://github.com/wmedrano/wills-columnar-format>. The main source file is `wills-columnar-format.org` which is used to generate the Rust source files like `src/lib.rs`.