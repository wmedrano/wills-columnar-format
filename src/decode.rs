// [[file:../wills-columnar-format.org::#IntroductionCargotoml-cqc696o03tj0][Dependencies:5]]
use std::io::{Read, Seek};

use crate::{rle, Footer, Result, BINCODE_DATA_CONFIG};
// Dependencies:5 ends here

// [[file:../wills-columnar-format.org::#FormatSpecificationFormatOverview-j3k696o03tj0][Format Overview:3]]
pub fn decode_column_impl<T: 'static + bincode::Decode>(
    r: impl Read + Seek,
) -> Result<impl Iterator<Item = Result<rle::Values<T>>>> {
    let mut r = r;
    let data_start = r.stream_position()?;
    r.seek(std::io::SeekFrom::End(-8))?;
    let footer_length_bytes = bincode::decode_from_std_read(&mut r, BINCODE_DATA_CONFIG)?;
    let footer_length = u64::from_le_bytes(footer_length_bytes);
    r.seek(std::io::SeekFrom::End(-8 - footer_length as i64))?;
    let footer: Footer = bincode::decode_from_std_read(&mut r, BINCODE_DATA_CONFIG)?;
    r.seek(std::io::SeekFrom::Start(data_start))?;
    // TODO: Return an error instead of panicking.
    assert!(
        footer.data_type.is_supported::<T>(),
        "Format of expected type {:?} does not support {:?}.",
        footer.data_type,
        std::any::type_name::<T>(),
    );

    let mut iter_pages = footer.pages.into_iter().peekable();
    let iter = std::iter::from_fn(move || -> Option<Result<rle::Values<T>>> {
        // TODO: Verify
        while iter_pages.next_if(|p| p.values_count == 0).is_some() {}
        let page = iter_pages.peek_mut()?;
        let rle_element_or_err = if footer.use_rle {
            bincode::decode_from_std_read(&mut r, BINCODE_DATA_CONFIG)
        } else {
            bincode::decode_from_std_read(&mut r, BINCODE_DATA_CONFIG).map(rle::Values::single)
        };
        if let Ok(e) = &rle_element_or_err {
            page.values_count -= e.run_length as usize;
        }
        Some(rle_element_or_err.map_err(std::convert::Into::into))
    });
    Ok(iter)
}
// Format Overview:3 ends here
