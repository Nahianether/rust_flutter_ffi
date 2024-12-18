use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use image::io::Reader as ImageReader;
use std::path::Path;
use webp::Encoder;

// pub async fn image_to_base64_with_resize(file_path: &Path) -> Result<String> {
//     let image = ImageReader::open(file_path)?.decode()?.into_rgb8();
//     let mut buffer = Vec::new();
//     let mut encoder = JpegEncoder::new_with_quality(&mut buffer, 5);
//     encoder.encode_image(&image)?;

//     Ok(general_purpose::STANDARD.encode(&buffer))
// }

pub async fn image_to_base64_with_resize(file_path: &Path) -> Result<String> {
    let image = ImageReader::open(file_path)?.decode()?;
    let encoder = Encoder::from_image(&image);
    let webp_data = encoder.encode(15.0);

    let data = general_purpose::STANDARD.encode(webp_data.as_ref());

    Ok(data)
}
