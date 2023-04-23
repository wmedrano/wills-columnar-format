- [Introduction](#org9dddb4b)
  - [Conventions](#org5b870de)
- [API](#orgcdbc1f4)
  - [V0 Features](#org9ae0653)
  - [V1 Features Brainstorm](#orgdc63f79)
  - [Encoding](#orgd5b0f5c)
  - [Decoding](#org547a809)
- [Format Specification](#orgd5a7138)
  - [Header](#orgb6486aa)
  - [Data](#orgb02faf5)
    - [RLE](#org6b23078)
- [Source Code](#org328710f)



<a id="org9dddb4b"></a>

# Introduction

[Will's columnar format](https://wmedrano.dev/living-programs/wills-columnar-format) is a columnar format made by will.s.medrano@gmail.com. It is primarily implemented for educational purposes. If you are interested in using a well supported columnar format, consider using [Apache Parquet](https://parquet.apache.org/).


<a id="org5b870de"></a>

## Conventions

The following conventions are used:

-   All structs are encoded using [Bincode](https://github.com/bincode-org/bincode). Bincode is a binary encoding/decoding scheme implemented in Rust.
-   Source code snippets are presented for relatively high level constructs. Lower level details may be omitted from presentation.


<a id="orgcdbc1f4"></a>

# API


<a id="org9ae0653"></a>

## V0 Features

V0 is roughly implemented but still requires verification, testing, error handling, and bench-marking.

Supports:

-   Only a single column per encode/decode.
-   `i64` and `String` types.
-   Run length encoding.


<a id="orgdc63f79"></a>

## V1 Features Brainstorm

-   Dictionary encoding for better string compression.
-   Compression (like zstd or snappy) for data.
-   Multiple columns.
-   Push down filtering.
-   Split column data into blocks. Required by push down filtering.


<a id="orgd5b0f5c"></a>

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


<a id="org547a809"></a>

## Decoding

`decode_column` decodes data from a byte stream into a `Vec<T>`.

TODO: Decoding should return an iterator of `(element_count, element)` to support efficient reads of run-length-encoded data.

```rust
pub fn decode_column<T>(r: &mut impl std::io::Read) -> Vec<T>
where
    T: 'static + Clone + bincode::Decode {
    decode_column_impl(r)
}
```


<a id="orgd5a7138"></a>

# Format Specification

-   `magic-bytes` - The magic bytes are "wmedrano0".
-   `header` - The header.
-   `data` - The data.


<a id="orgb6486aa"></a>

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


<a id="orgb02faf5"></a>

## Data

The data consists of a sequence of encoded data. Encoding happens using the standard `bincode` package for all data types.


<a id="org6b23078"></a>

### RLE

[Run length encoding](https://en.wikipedia.org/wiki/Run-length_encoding#:~:text=Run%2Dlength%20encoding%20(RLE),than%20as%20the%20original%20run.) is a compression technique for repeated values. For RLE, the data is encoded as a tuple of `(u16, T)` where the first item contains the run length and the second contains the element.

TODO: Refactor type from `(u16, T)` to something cleaner like a new custom type, `RleElement<T>`.

```rust
fn rle_encode_data<T: Eq>(data: impl Iterator<Item = T>) -> Vec<(u16, T)> {
    let mut data = data;
    let mut element = match data.next() {
        Some(e) => e,
        None => return Vec::new(),
    };
    let mut count = 1;

    let mut ret = Vec::new();
    for next_element in data {
        if next_element != element || count == u16::MAX {
            ret.push((count, element));
            (element, count) = (next_element, 1);
        } else {
            count += 1;
        }
    }
    if count > 0 {
        ret.push((count, element));
    }
    ret
}

fn rle_decode_data<'a, T: 'static>(
    iter: impl 'a + Iterator<Item = &'a (u16, T)>,
) -> impl Iterator<Item = &'a T> {
    iter.flat_map(move |(run_length, element)| {
        std::iter::repeat(element).take(*run_length as usize)
    })
}
```


<a id="org328710f"></a>

# Source Code

The source code is stored at <https://github.com/wmedrano/wills-columnar-format>. The main source file is `wills-columnar-format.org` which is used to generate the `src/lib.rs`.