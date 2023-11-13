use actix_files::NamedFile;
use actix_web::{web, HttpRequest, Result, error};
use std::path::{Path, PathBuf};
use regex::Regex;

async fn index(req: HttpRequest) -> Result<NamedFile> {
    let filename = req.match_info().query("filename").parse::<PathBuf>().unwrap();

    let filename_str = filename.to_string_lossy();
    let filename_regex = Regex::new(r"^[\w\./_]+$").unwrap(); /* <- this is rules */

    if !filename_regex.is_match(&filename_str) {
        return Err(
            error::ErrorNotFound("invalid filename")
        );
    }

    let sanitized_filename = sanitize_filename::sanitize(&filename_str);

    let base_dir = "./static";
    let path = Path::new(&base_dir).join(&sanitized_filename);

    if !path.starts_with(&base_dir) {
        return Err(
            error::ErrorNotFound("invalid file path")
        )
    }

    match NamedFile::open(path) {
        Ok(file) => Ok(file),
        Err(err) => {
            if err.kind() == std::io::ErrorKind::NotFound {
                Err(
                    error::ErrorNotFound("file not found")
                )
            } else {
                Err(
                    error::ErrorInternalServerError("internal server error")
                )
            }
        }
    }
}

pub fn scoped_individual_file(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/individual_file/{filename}")
            .route(web::get().to(index))
    );
}
