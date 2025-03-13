use domain::value_object::error::object_strage::webp::WebpError;
use image::load_from_memory;
use webp::Encoder;

fn quality_range_protector(quality: f32) -> Result<f32, WebpError> {
    if (0.0..=100.0).contains(&quality) {
        Ok(quality)
    } else {
        Err(WebpError::OutOfRangeError)
    }
}

pub fn convert_to_webp(binary: &[u8], quality: f32) -> Result<Vec<u8>, WebpError> {
    let img = load_from_memory(binary)?;
    let encoder = Encoder::from_image(&img).map_err(|e| WebpError::EncoderError(e.to_string()))?;
    let webp = encoder.encode(quality_range_protector(quality)?).to_vec();
    Ok(webp)
}
