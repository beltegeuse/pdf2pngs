/// Original code from:
/// https://codereview.stackexchange.com/questions/234028/creating-a-thumbnail-of-a-pdf-using-cairo-poppler-and-rust

#[macro_use]
extern crate clap;

use std::convert::From;
use std::fs::File;
use std::result::Result;

use cairo::{Context, Format, ImageSurface};
use poppler::{PopplerDocument, PopplerPage};
use clap::{Arg, App};

#[derive(Debug)]
pub enum ThumbnailError {
    GlibError(glib::error::Error),
    NoPagesError,
    CairoError(cairo::Status),
    CairoIoError(cairo::IoError),
    IoError(std::io::Error),
}

impl From<glib::error::Error> for ThumbnailError {
    fn from(err: glib::error::Error) -> Self {
        ThumbnailError::GlibError(err)
    }
}

impl From<cairo::Status> for ThumbnailError {
    fn from(status: cairo::Status) -> Self {
        ThumbnailError::CairoError(status)
    }
}

impl From<cairo::IoError> for ThumbnailError {
    fn from(err: cairo::IoError) -> Self {
        ThumbnailError::CairoIoError(err)
    }
}

impl From<std::io::Error> for ThumbnailError {
    fn from(err: std::io::Error) -> Self {
        ThumbnailError::IoError(err)
    }
}

/// Create a JPEG thumbnail from the first page of a PDF.
fn create_thumbnail(
    pdf_path: &str,
    out_path: &str,
    scale: f64,
    max_pages: Option<usize>,
) -> Result<usize, ThumbnailError> {
    // Assume the PDF is not password protected.
    let doc: PopplerDocument = PopplerDocument::new_from_file(pdf_path, "")?;

    let nb_pages = match max_pages {
        None => doc.get_n_pages(),
        Some(v) => v.min(doc.get_n_pages()),
    };

    // Note: PDF pages are 0-indexed
    for page_id in 0..nb_pages {
        let page: PopplerPage = match doc.get_page(page_id) {
            Some(p) => p,
            None => return Err(ThumbnailError::NoPagesError),
        };
        let (width, height) = page.get_size();
        let mut surface = ImageSurface::create(
            Format::Rgb24,
            (width * scale) as i32,
            (height * scale) as i32,
        )?;
        // Draw a white background to start with.  If you don't, any transparent
        // regions in the PDF will be rendered as black in the final image.
        let ctxt = Context::new(&mut surface);
        ctxt.set_source_rgb(1.0 as f64, 1.0 as f64, 1.0 as f64);
        ctxt.scale(scale, scale);
        ctxt.paint();
        // Draw the contents of the PDF onto the page.
        page.render(&ctxt);
        let filename = format!("{}_{}.png", out_path, page_id);
        println!("Exporting {} ...", filename);
        let mut f: File = File::create(filename)?;
        surface.write_to_png(&mut f)?;
    }
    Ok(nb_pages)
}

fn main() {
    let matches = App::new("PDF2PNGs")
        .version("1.0")
        .author("beltegeuse <adrien.gruson@gmail.com>")
        .about("Convert PDF to multiple PNG images")
        .arg(
            Arg::with_name("input")
                .index(1)
                .required(true)
                .help("PDF input filepath")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("max_pages")
                .help("Sets the maximum pages to export")
                .short("p")
                .default_value("0"),
        )
        .arg(
            Arg::with_name("scale")
                .help("Sets image scale")
                .short("s")
                .default_value("1.0"),
        )
        .arg(
            Arg::with_name("output")
                .index(2)
                .required(true)
                .help("Output basename")
                .takes_value(true),
        )
        .get_matches();

    let pdf_path = value_t_or_exit!(matches.value_of("input"), String);
    let out_path = value_t_or_exit!(matches.value_of("output"), String);
    let max_pages = value_t_or_exit!(matches.value_of("max_pages"), i32);
    let max_pages = if max_pages <= 0 { None } else { Some(max_pages as usize) };
    let scale = value_t_or_exit!(matches.value_of("scale"), f64);

    match create_thumbnail(&pdf_path, &out_path, scale, max_pages) {
        Ok(nb_pages) => println!("Created thumbnail of {} with {} name ({} pages)", pdf_path, out_path, nb_pages),
        Err(err) => println!("Something went wrong: {:?}", err),
    };
}
