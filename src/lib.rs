#![windows_subsystem = "windows"]

use std::env;
use std::fs;
use std::io::BufReader;
use std::path::Path;

use image::ImageFormat;

pub enum OutputFormat {
    Jpg(u8),
    Png,
}

impl OutputFormat {
    pub fn extension(&self) -> &str {
        match self {
            OutputFormat::Jpg(_) => "jpg",
            OutputFormat::Png => "png",
        }
    }
}

pub fn convert_all(format: OutputFormat) {
    let dir = env::current_dir().expect("Impossible de lire le répertoire courant");

    let entries: Vec<_> = fs::read_dir(&dir)
        .expect("Impossible de lister le répertoire")
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| {
                    ext.eq_ignore_ascii_case("jif") || ext.eq_ignore_ascii_case("webp")
                })
                .unwrap_or(false)
        })
        .collect();

    if entries.is_empty() {
        return;
    }

    for entry in &entries {
        let input_path = entry.path();
        let output_path = input_path.with_extension(format.extension());

        if convert(&input_path, &output_path, &format).is_ok() {
            let _ = fs::remove_file(&input_path);
        }
    }
}

fn detect_format(path: &Path) -> ImageFormat {
    match path.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase().as_str() {
        "webp" => ImageFormat::WebP,
        _ => ImageFormat::Jpeg, // .jif = JPEG (JFIF)
    }
}

fn convert(input: &Path, output: &Path, format: &OutputFormat) -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::File::open(input)?;
    let reader = BufReader::new(file);

    let input_format = detect_format(input);
    let img = image::load(reader, input_format)?;

    match format {
        OutputFormat::Jpg(quality) => {
            let mut out = fs::File::create(output)?;
            let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out, *quality);
            img.write_with_encoder(encoder)?;
        }
        OutputFormat::Png => {
            img.save(output)?;
        }
    }

    Ok(())
}
