// [[file:../wills-columnar-format.org::#IntroductionCargotoml-cqc696o03tj0][Dependencies:6]]
use std::io::Write;

use crate::{rle, DataType, Footer, PageInfo, Result, BINCODE_DATA_CONFIG};
// Dependencies:6 ends here

// [[file:../wills-columnar-format.org::#FormatSpecificationFormatOverview-j3k696o03tj0][Format Overview:2]]
pub fn encode_column_impl<T>(
    w: &mut impl Write,
    values_iter: impl Iterator<Item = T>,
    use_rle: bool,
) -> Result<Footer>
where
    T: 'static + bincode::Encode + Eq,
{
    // TODO: Return an error.
    let data_type = DataType::from_type::<T>().expect("unsupported data type");
    let mut values_iter = values_iter;

    let mut pages = Vec::new();
    let mut file_offset = 0;
    loop {
        let encoding =
            encode_values_as_bincode(&mut values_iter, file_offset, MIN_TARGET_PAGE_SIZE, use_rle)?;
        if encoding.encoded_values.is_empty() {
            break;
        } else {
            file_offset += w.write(encoding.encoded_values.as_slice())? as i64;
            pages.push(encoding.page_info);
        }
    }
    let footer = Footer {
        data_type,
        use_rle,
        pages,
    };
    let footer_size = bincode::encode_into_std_write(&footer, w, BINCODE_DATA_CONFIG)? as u64;
    w.write(&footer_size.to_le_bytes())?;
    Ok(footer)
}
// Format Overview:2 ends here

// [[file:../wills-columnar-format.org::#FormatSpecificationPages-b9u4ccg05tj0][Pages:2]]
const MIN_TARGET_PAGE_SIZE: usize = 2048;
// Pages:2 ends here

// [[file:../wills-columnar-format.org::#DataEncodingBasicEncoding-e4m696o03tj0][Basic Encoding:2]]
struct Encoding {
    pub encoded_values: Vec<u8>,
    pub page_info: PageInfo,
}

fn encode_values_as_bincode<T: 'static + bincode::Encode>(
    values: &mut impl Iterator<Item = T>,
    file_offset: i64,
    target_encoded_size: usize,
    use_rle: bool,
) -> Result<Encoding>
where
    T: 'static + bincode::Encode + Eq,
{
    let mut encoded_values = Vec::new();
    if use_rle {
        let mut values_count = 0;
        let mut encoded_values_count = 0;
        for rle in rle::encode_iter(values) {
            values_count += rle.run_length as usize;
            encoded_values_count += 1;
            bincode::encode_into_std_write(rle, &mut encoded_values, BINCODE_DATA_CONFIG)?;
            if encoded_values.len() >= target_encoded_size {
                break;
            }
        }
        Ok(Encoding {
            encoded_values,
            page_info: PageInfo {
                file_offset,
                values_count,
                encoded_values_count,
            },
        })
    } else {
        let mut values_count = 0;
        for value in values {
            values_count += 1;
            bincode::encode_into_std_write(value, &mut encoded_values, BINCODE_DATA_CONFIG)?;
            if encoded_values.len() >= target_encoded_size {
                break;
            }
        }
        Ok(Encoding {
            encoded_values,
            page_info: PageInfo {
                file_offset,
                values_count,
                encoded_values_count: values_count,
            },
        })
    }
}
// Basic Encoding:2 ends here
