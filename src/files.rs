use actix_web::{Result, HttpRequest, get};
use actix_files::NamedFile;
use std::path::PathBuf;

#[get("/{filename}")]
pub async fn serve_static_file(req: HttpRequest) -> Result<NamedFile> {
    let static_path = format!("./static/{}", req.match_info().query("filename"));
    let file: PathBuf = static_path.parse().unwrap();
    Ok(NamedFile::open(file)?)
}