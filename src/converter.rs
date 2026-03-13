use std::path::{Path, PathBuf};
use rayon::prelude::*;

/// Result of a single file conversion attempt.
pub struct ConversionResult {
    pub success: bool,
    pub message: String,
}

/// Find all `.icns` files in `input_dir` and convert them to `.ico` in `output_dir`
/// using Rayon for parallel processing.
pub fn convert_all(input_dir: &Path, output_dir: &Path) -> Vec<ConversionResult> {
    let files: Vec<PathBuf> = match std::fs::read_dir(input_dir) {
        Ok(entries) => entries
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| {
                p.extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| ext.eq_ignore_ascii_case("icns"))
                    .unwrap_or(false)
            })
            .collect(),
        Err(e) => {
            return vec![ConversionResult {
                success: false,
                message: format!("Could not read input directory: {}", e),
            }];
        }
    };

    if files.is_empty() {
        return Vec::new();
    }

    files
        .par_iter()
        .map(|path| convert_single(path, output_dir))
        .collect()
}

fn convert_single(input_path: &Path, output_dir: &Path) -> ConversionResult {
    let filename = input_path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();

    let stem = input_path.file_stem().unwrap_or_default();
    let output_filename = format!("{}.ico", stem.to_string_lossy());
    let output_path = output_dir.join(&output_filename);

    match do_convert(input_path, &output_path) {
        Ok(()) => ConversionResult {
            success: true,
            message: format!("{} → {}", filename, output_filename),
        },
        Err(e) => ConversionResult {
            success: false,
            message: format!("{}: {}", filename, e),
        },
    }
}

fn do_convert(
    input_path: &Path,
    output_path: &Path,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Primary strategy: icns crate for accurate Apple icon parsing
    if let Ok(file) = std::fs::File::open(input_path) {
        if let Ok(family) = icns::IconFamily::read(file) {
            let preferred = [
                icns::IconType::RGBA32_512x512_2x,
                icns::IconType::RGBA32_512x512,
                icns::IconType::RGBA32_256x256_2x,
                icns::IconType::RGBA32_256x256,
                icns::IconType::RGBA32_128x128_2x,
                icns::IconType::RGBA32_128x128,
                icns::IconType::RGBA32_64x64,
                icns::IconType::RGBA32_32x32_2x,
                icns::IconType::RGBA32_32x32,
                icns::IconType::RGBA32_16x16_2x,
                icns::IconType::RGBA32_16x16,
            ];
            if let Some(&icon_type) = preferred.iter().find(|&&t| family.has_icon_with_type(t)) {
                let icon = family.get_icon_with_type(icon_type)?;
                let rgba = image::RgbaImage::from_raw(
                    icon.width(),
                    icon.height(),
                    icon.data().to_vec(),
                )
                .ok_or("Failed to construct image buffer from ICNS data")?;
                return write_ico(&rgba, output_path);
            }
        }
    }

    // Fallback: generic image decoding
    let img = image::open(input_path)?.into_rgba8();
    write_ico(&img, output_path)
}

/// Resize `source` to standard ICO sizes and write a multi-resolution `.ico` file.
fn write_ico(
    source: &image::RgbaImage,
    output_path: &Path,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use std::io::BufWriter;

    let mut dir = ico::IconDir::new(ico::ResourceType::Icon);
    for &size in &[256u32, 128, 64, 48, 32, 16] {
        let frame = if source.width() == size && source.height() == size {
            source.clone()
        } else {
            image::imageops::resize(source, size, size, image::imageops::FilterType::Lanczos3)
        };
        let icon_image = ico::IconImage::from_rgba_data(frame.width(), frame.height(), frame.into_raw());
        dir.add_entry(ico::IconDirEntry::encode(&icon_image)?);
    }

    let file = std::fs::File::create(output_path)?;
    dir.write(BufWriter::new(file))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::RgbaImage;

    #[test]
    fn write_ico_produces_file() {
        let tmp = std::env::temp_dir().join("test_output.ico");
        let src = RgbaImage::new(256, 256);
        write_ico(&src, &tmp).expect("write_ico should succeed");
        assert!(tmp.exists(), "ICO file should be created");
        std::fs::remove_file(tmp).ok();
    }
}
