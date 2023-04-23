- [Introduction](#orgd5f1adb)
  - [Conventions](#org84accef)
- [API](#orgca0e12e)
  - [V0 Features](#org0aba4ba)
  - [Encoding](#org24748f6)
  - [Decoding](#org93a7602)
- [Format Specification](#org67f78f5)
  - [Header](#orgeb9cf54)
  - [Data](#orgba44769)
    - [RLE](#orgae53380)
    - [Dictionary Encoding](#org497d3b5)
- [Source Code](#orga5f5571)



<a id="orgd5f1adb"></a>

# Introduction

Will's columnar format is a columnar format made by will.s.medrano@gmail.com. It is primarily implemented for educational purposes. If you are interested in using a well supported columnar format, consider using [Apache Parquet](https://parquet.apache.org/).


<a id="org84accef"></a>

## Conventions

The following conventions are used:

-   All structs are encoded using `bincode`. `bincode` is a binary encoding/decoding scheme implemented in Rust.
-   Source code snippets are presented for relatively high level constructs. Lower level details may be omitted from presentation.


<a id="orgca0e12e"></a>

# API


<a id="org0aba4ba"></a>

## V0 Features

V0 is implemented but still requires verification, testing, and benchmarking.

Supports:

-   Only a single column per encode/decode.
-   `i64` and `String` types.
-   Run length encoding.


<a id="org24748f6"></a>

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


<a id="org93a7602"></a>

## Decoding

`decode_column` decodes data from a `Read` stream into a `Vec<T>`.

TODO: Decoding should return an iterator of `(element_count, element)` to support efficient reads of run-length-encoded data.

```rust
pub fn decode_column<T>(r: &mut impl std::io::Read) -> Vec<T>
where
    T: 'static + Clone + bincode::Decode {
    decode_column_impl(r)
}
```


<a id="org67f78f5"></a>

# Format Specification

-   `magic-string` - A magic string of "wmedrano0".
-   `header` - The header.
-   `data` - The data.


<a id="orgeb9cf54"></a>

## Header

The header contains an encoded struct:

```rust
#[derive(Encode, Decode, PartialEq, Eq, Copy, Clone, Debug)]
pub enum DataType {
    I64 = 0,
    String = 1,
}

#[derive(Encode, Decode, PartialEq, Eq, Copy, Clone, Debug)]
pub struct Header {
    pub data_type: DataType,
    pub is_rle: bool,
    pub elements: usize,
    pub data_size: usize,
}

impl Header {
    fn encode(&self) -> Vec<u8> {
        bincode::encode_to_vec(self, Self::CONFIGURATION).unwrap()
    }

    fn decode(r: &mut impl std::io::Read) -> Header {
        bincode::decode_from_std_read(r, Self::CONFIGURATION).unwrap()
    }
}
```


<a id="orgba44769"></a>

## Data

The data consists of a sequence of encoded data. Encoding happens using the standard `bincode` package for all data types.


<a id="orgae53380"></a>

### RLE

[Run length encoding](https://en.wikipedia.org/wiki/Run-length_encoding#:~:text=Run%2Dlength%20encoding%20(RLE),than%20as%20the%20original%20run.) is a compression technique for repeated values. For RLE, the data is encoded as a tuple of `(u16, T)` where the first item contains the run length and the second contains the element.

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


<a id="org497d3b5"></a>

### TODO Dictionary Encoding

Dictionary encoding is useful for string columns with few unique values. This is out of scope for V0.


<a id="orga5f5571"></a>

# Source Code

<https://github.com/wmedrano/wills-columnar-format>