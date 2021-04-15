//! HTTP Request/Response Management, Route Validation & Serving
//! 
//! Validates routes & serves HTTP responses with the corresponding route files

use actix_files::NamedFile;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, Result};
use actix_multipart::Multipart;
use actix_web::{web, Error, HttpResponse};

use std::io::Write;
use std::path::Path;
use std::fs::create_dir;
use futures::{StreamExt, TryStreamExt};
use serde_json::json;
use uuid::Uuid;

use crate::config::get_config;

/// Save image file for upload (POST) requests
pub async fn serve_upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let config = get_config();
    let save_files_dir = config["uploaded_files"]["raw_directory"].to_string().replace("\"", "");

    if !Path::new(save_files_dir.as_str()).exists() {
        create_dir(save_files_dir.as_str()).unwrap();
    }

    let mut filename = String::from("");
    let mut fileuuidname = String::from("");
    
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        if config["template_variables"]["upload_content_name"].as_str() == content_type.get_name() {
            filename = content_type.get_filename().unwrap().to_string();
            fileuuidname = format!(
                "{}.{}",
                Uuid::new_v4(),
                Path::new(filename.as_str()).extension().unwrap().to_str().unwrap()
            );
            let filepath = format!(
                "{}/{}",
                save_files_dir,
                sanitize_filename::sanitize(fileuuidname.as_str())
            );
            
            // File::create is blocking operation, use threadpool
            let mut f = web::block(|| std::fs::File::create(filepath))
                .await
                .unwrap();
            
            // Field in turn is stream of *Bytes* object
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                // filesystem operations are blocking, we have to use threadpool
                f = web::block(move || f.write_all(&data).map(|_| f)).await?;
            }
        }
    }

    match filename.as_str() {
        "" => Ok(HttpResponse::Ok().json(
            json!({
                "error": format!(
                    "Ooops! Something has occured to the server...! {}",
                    "Maybe the upload API setting is wrong."
                ),
            })
        )),
        _ => Ok(HttpResponse::Ok().json(
            json!({
                "pic_url": format!(
                    "/{}/{}",
                    config["uploaded_files"]["directed_to"].as_str().unwrap(),
                    fileuuidname,
                )
            })
        ))
    }
}

/// Return routed files for static file (GET) requests
pub async fn serve_routed(req: HttpRequest) -> Result<NamedFile> {
    // get the HTTP request path
    let req_path = format!("/{}", req.match_info().query("route"));

    let config = get_config();
    let template_dir = config["template_directory"].to_string().replace("\"", "");
    let routes = config["routes"][req_path].to_string().replace("\"", "");

    let status_code;
    /*
        404 Not Found Handler
        `routes` returns 'null' if the route entry doesn't exist
    */
    let response_file = if routes == "null" {
        let page_404 = config["routes"]["404"].to_string().replace("\"", "");
        status_code = StatusCode::NOT_FOUND;
        format!("{}/{}", template_dir, page_404)
    } else {
        status_code = StatusCode::OK;
        format!("{}/{}", template_dir, routes)
    };

    Ok(NamedFile::open(response_file)?
        .set_status_code(status_code)
        .prefer_utf8(true)
        .use_last_modified(true)
    )
}
