- [Introduction](#org444833b)
  - [Conventions](#orgb007b95)
- [API](#org9556ce4)
  - [V0 Features](#org4babdc8)
  - [Encoding](#org82f5bbe)
  - [Decoding](#org767ad18)
- [Format Specification](#orge0957e0)
  - [Header](#org141d738)
  - [Data](#org1cd4a04)
    - [RLE](#org61cb287)
    - [Dictionary Encoding](#org11f27b1)



<a id="org444833b"></a>

# Introduction

Will's columnar format is a columnar format made by will.s.medrano@gmail.com. It is primarily a research project. If you are interested in using a well supported columnar format, consider using [Apache Parquet](https://parquet.apache.org/).


<a id="orgb007b95"></a>

## Conventions

The following conventions are used:

-   All structs are encoded using `bincode`. `bincode` is a binary encoding/decoding scheme implemented in Rust.
-   Source code snippets are presented for relatively high level constructs. Lower level details may be omitted from presentation.


<a id="org9556ce4"></a>

# API


<a id="org4babdc8"></a>

## V0 Features

Supports:

-   Only a single column per encode/decode.
-   `i64` and `String` types.
-   Run length encoding.


<a id="org82f5bbe"></a>

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


<a id="org767ad18"></a>

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


<a id="orge0957e0"></a>

# Format Specification

-   `magic-string` - A magic string of "wmedrano0".
-   `header` - The header.
-   `data` - The data.


<a id="org141d738"></a>

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


<a id="org1cd4a04"></a>

## Data

The data consists of a sequence of encoded data. Encoding happens using the standard `bincode` package for all data types.


<a id="org61cb287"></a>

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


<a id="org11f27b1"></a>

### TODO Dictionary Encoding

Dictionary encoding is useful for string columns with few unique values.