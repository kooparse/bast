use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};
use std::path::PathBuf;

pub fn serve(req: HttpRequest) -> Result<NamedFile> {
    let filename = req.match_info().query("filename");
    let path: PathBuf = format!("static/{}", filename).parse()?;

    Ok(NamedFile::open(path)?)
}
