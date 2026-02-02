use libvips_rs::VipsImage;

pub fn load_pdf_page(path: &str, page: i32) -> Result<VipsImage, libvips_rs::error::Error> {
    // libvips pdfload supports [page=N] suffix
    let path_with_page = format!("{}[page={}]", path, page);
    VipsImage::new_from_file(&path_with_page)
}
