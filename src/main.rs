use anyhow::{Context, Result};
use clap::Parser;
use csscolorparser::Color;
use image::codecs::jpeg::JpegEncoder;
use image::RgbImage;
use std::io::{BufWriter, Read};
use std::path::PathBuf;
use std::sync::Arc;
use tiny_skia::{Color as SkiaColor, Pixmap, Transform};
use usvg::{fontdb, Options, Tree};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input SVG file. If not provided, reads from Stdin.
    #[arg(short, long)]
    input: Option<PathBuf>,

    /// Output JPEG file. If not provided, writes to Stdout.
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Target width (maintains aspect ratio). If not set, uses original size.
    #[arg(short, long)]
    width: Option<u32>,

    /// JPEG Quality (1-100)
    #[arg(short, long, default_value_t = 80)]
    quality: u8,

    /// Background color "white" /"#FFFFFF"
    #[arg(short, long, default_value = "white")]
    background: String,

    /// Path to load custom fonts
    #[arg(long)]
    use_fonts_dir: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let quality = if args.quality == 0 || args.quality > 100 {
        eprintln!(
            "Warning: Invalid quality '{}'. Using default 80.",
            args.quality
        );
        80
    } else {
        args.quality
    };

    if let Some(w) = args.width {
        if w == 0 {
            anyhow::bail!("Width must be greater than 0");
        }
    }

    let bg_color_skia = match args.background.parse::<Color>() {
        Ok(bg_color) => SkiaColor::from_rgba(
            bg_color.r as f32,
            bg_color.g as f32,
            bg_color.b as f32,
            bg_color.a as f32,
        )
        .unwrap_or_else(|| {
            eprintln!("Warning: Invalid background color alpha. Using default white.");
            SkiaColor::WHITE
        }),
        Err(_) => {
            eprintln!(
                "Warning: Failed to parse background color '{}'. Using default white.",
                args.background
            );
            SkiaColor::WHITE
        }
    };

    let mut font_db = fontdb::Database::new();
    font_db.load_system_fonts();

    if let Some(dir) = args.use_fonts_dir {
        if dir.exists() {
            eprintln!("Loading fonts from: {}", dir.display());
            font_db.load_fonts_dir(dir);
        } else {
            eprintln!(
                "Warning: Fonts directory '{}' does not exist",
                dir.display()
            );
        }
    }

    let mut opt = Options::default();
    opt.fontdb = Arc::new(font_db);

    let svg_data = load_svg_data(args.input.as_ref())?;
    let rtree = Tree::from_data(&svg_data, &opt).context("Failed to parse SVG data")?;

    let (target_width, target_height, transform) = calculate_transform(&rtree, args.width);

    let mut pixmap =
        Pixmap::new(target_width, target_height).context("Failed to allocate memory for bitmap")?;

    pixmap.fill(bg_color_skia);

    resvg::render(&rtree, transform, &mut pixmap.as_mut());

    // Convert from tiny-skia premultiplied alpha to rgb (r*a, g*a, b*a, a) -> (r, g, b, 1)
    let rgb_image = convert_pixmap_to_rgb(&pixmap);

    write_output(args.output.as_ref(), &rgb_image, quality)?;

    Ok(())
}

fn load_svg_data(input: Option<&PathBuf>) -> Result<Vec<u8>> {
    match input {
        Some(path) => std::fs::read(path)
            .with_context(|| format!("Failed to read input file: {}", path.display())),
        None => {
            let mut buffer = Vec::new();
            std::io::stdin()
                .read_to_end(&mut buffer)
                .context("Failed to read from stdin")?;
            Ok(buffer)
        }
    }
}

fn calculate_transform(rtree: &Tree, width: Option<u32>) -> (u32, u32, Transform) {
    let original_size = rtree.size();
    let original_width = original_size.width();
    let original_height = original_size.height();

    match width {
        Some(w) => {
            let scale = w as f32 / original_width;
            let h = original_height * scale;
            (w, h.ceil() as u32, Transform::from_scale(scale, scale))
        }
        None => (
            original_width.ceil() as u32,
            original_height.ceil() as u32,
            Transform::default(),
        ),
    }
}

fn convert_pixmap_to_rgb(pixmap: &Pixmap) -> RgbImage {
    let width = pixmap.width();
    let height = pixmap.height();
    let data = pixmap.data();

    let mut rgb_data = Vec::with_capacity((width * height * 3) as usize);

    for chunk in data.chunks_exact(4) {
        let r = chunk[0];
        let g = chunk[1];
        let b = chunk[2];
        // ignore alpha channel
        rgb_data.push(r);
        rgb_data.push(g);
        rgb_data.push(b);
    }

    RgbImage::from_raw(width, height, rgb_data).expect("Container should have correct size")
}

fn write_output(output: Option<&PathBuf>, image: &RgbImage, quality: u8) -> Result<()> {
    match output {
        Some(path) => {
            let file = std::fs::File::create(path)
                .with_context(|| format!("Failed to create output file: {}", path.display()))?;
            let mut writer = BufWriter::new(file);
            let mut encoder = JpegEncoder::new_with_quality(&mut writer, quality);
            encoder
                .encode_image(image)
                .context("Failed to encode JPEG")?;
        }
        None => {
            let stdout = std::io::stdout();
            let lock = stdout.lock();
            let mut writer = BufWriter::new(lock);
            let mut encoder = JpegEncoder::new_with_quality(&mut writer, quality);
            encoder
                .encode_image(image)
                .context("Failed to encode JPEG to stdout")?;
        }
    }
    Ok(())
}
