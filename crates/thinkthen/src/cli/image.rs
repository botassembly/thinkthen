//! Bounded file access for the branch-only scalar image spike.
use crate::args::Common;
use crate::core::image::ImageInput;
use crate::failure::Failure;
use std::fs::{self, File};
use std::io::Read as _;
use std::path::Path;

const MAX_BYTES: usize = 1024 * 1024;

pub(super) fn read(common: &Common) -> Result<Option<Vec<ImageInput>>, Failure> {
    if common.image.is_empty() {
        return Ok(None);
    }
    if common.image.len() > 2 {
        return Err(Failure::Usage("--image accepts at most two ordered images"));
    }
    if !crate::core::adapters::built_in::backends::supports_images(common.backend.as_deref()) {
        return Err(Failure::Usage(
            "--image requires explicit --backend naming a supported image backend",
        ));
    }
    if common.framing() != crate::core::Framing::Document || common.input.len() > 1 {
        return Err(Failure::Usage(
            "--image requires one document and no record framing or window",
        ));
    }
    if let Some(input) = common.input.first() {
        regular(input)?;
    }
    common
        .image
        .iter()
        .map(|path| read_file(path))
        .collect::<Result<Vec<_>, _>>()
        .map(Some)
}

fn read_file(path: &Path) -> Result<ImageInput, Failure> {
    regular(path)?;
    let file = File::open(path).map_err(|_| Failure::Usage("--image file could not be read"))?;
    if !file
        .metadata()
        .map_err(|_| Failure::Usage("--image file could not be read"))?
        .is_file()
    {
        return Err(Failure::Usage("--image requires a regular file"));
    }
    let mut bytes = Vec::new();
    file.take((MAX_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|_| Failure::Usage("--image file could not be read"))?;
    if bytes.len() > MAX_BYTES {
        return Err(Failure::Usage(
            "--image exceeds the spike limit of 1048576 bytes",
        ));
    }
    let media = super::image_format::media(&bytes).ok_or(Failure::Usage(
        "--image requires JPEG or PNG bytes; the suffix does not select the media type",
    ))?;
    Ok(ImageInput {
        media,
        bytes: bytes.into(),
    })
}

fn regular(path: &Path) -> Result<(), Failure> {
    let meta = fs::symlink_metadata(path)
        .map_err(|_| Failure::Usage("--image requires a readable regular file"))?;
    if !meta.is_file() {
        return Err(Failure::Usage(
            "--image requires a regular file, not a folder, symlink or special file",
        ));
    }
    Ok(())
}
