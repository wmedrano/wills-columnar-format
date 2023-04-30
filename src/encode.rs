// [[file:../wills-columnar-format.org::#IntroductionCargotoml-cqc696o03tj0][Dependencies:6]]
use std::io::Write;

use crate::{rle, DataType, Footer, Header, PageInfo, Result, BINCODE_DATA_CONFIG, MAGIC_BYTES};
// Dependencies:6 ends here

// [[file:../wills-columnar-format.org::#FormatSpecificationFormatOverview-j3k696o03tj0][Format Overview:2]]
pub fn encode_column_impl<T>(
    w: &mut impl Write,
    values_iter: impl ExactSizeIterator + Iterator<Item = T>,
    use_rle: bool,
) -> Result<()>
where
    T: 'static + bincode::Encode + Eq,
{
    let values = values_iter.len();
    let mut file_offset = w.write(MAGIC_BYTES)?;
    file_offset += bincode::encode_into_std_write(
        Header {
            data_type: DataType::from_type::<T>().unwrap(),
            use_rle,
        },
        w,
        BINCODE_DATA_CONFIG,
    )?;
    // TODO: Use multiple pages instead of writing to a single page.
    let encoding = if use_rle {
        let rle_data /*: impl Iterator<Item=rle::Values<T>>*/ = rle::encode_iter(values_iter);
        encode_values_as_bincode(rle_data)?
    } else {
        encode_values_as_bincode(values_iter)?
    };
    file_offset += w.write(encoding.encoded_values.as_slice())?;
    let page_offset = file_offset;
    let footer_size = bincode::encode_into_std_write(
        Footer {
            pages: vec![PageInfo {
                file_offset: page_offset as i64,
                values_count: values,
                encoded_values_count: encoding.values_count,
            }],
        },
        w,
        BINCODE_DATA_CONFIG,
    )? as u64;
    w.write(&footer_size.to_le_bytes())?;
    Ok(())
}
// Format Overview:2 ends here

// [[file:../wills-columnar-format.org::#DataEncodingBasicEncoding-e4m696o03tj0][Basic Encoding:2]]
struct Encoding {
    pub encoded_values: Vec<u8>,
    pub values_count: usize,
}

fn encode_values_as_bincode<T: 'static + bincode::Encode>(
    values: impl Iterator<Item = T>,
) -> Result<Encoding> {
    let mut encoded_values = Vec::new();
    let mut values_count = 0;
    for element in values {
        bincode::encode_into_std_write(element, &mut encoded_values, BINCODE_DATA_CONFIG)?;
        values_count += 1;
    }
    Ok(Encoding {
        encoded_values,
        values_count,
    })
}
// Basic Encoding:2 ends here
