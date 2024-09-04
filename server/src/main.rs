use axum::{extract::Multipart, http::Method, routing::post, Router};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    //CORS
    let cors = CorsLayer::new()
        .allow_methods([Method::POST])
        .allow_origin(Any);
    //Router
    let app = Router::new().route("/", post(upload_handler)).layer(cors);
    //Server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:5000")
        .await
        .unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// Handler
async fn upload_handler(mut multipart: Multipart) -> String {
    //multipartを一つづつ取り出す
    match multipart.next_field().await.unwrap() {
        Some(field) => {
            let file_name = field.file_name().unwrap().to_string();
            println!("Uploaded file: {}", file_name);
            let param_name = field.name().unwrap().to_string();
            println!("Param name: {}", param_name);
            //画像に関する部分
            let data = field.bytes().await.unwrap();
            convert_to_webp(&data, 75.0).unwrap();
            file_name
        }
        None => "Upload Failed".to_string(),
    }
}

//画像処理
use image::DynamicImage;
use std::{error::Error, fs};
use webp::{Encoder, WebPMemory};

fn quality_range_protector(quality: f32) -> Result<f32, String> {
    if (0.0..=100.0).contains(&quality) {
        Ok(quality)
    } else {
        Err("Quality must be between 0 and 100".to_string())
    }
}

pub fn convert_to_webp(binary: &[u8], quality: f32) -> Result<(), Box<dyn Error>> {
    let img: DynamicImage = image::load_from_memory(binary).unwrap();
    let encoder: Encoder<'_> =
        Encoder::from_image(&img).map_err(|e| format!("Failed to create a webp encoder: {}", e))?;
    let webp: WebPMemory = encoder.encode(quality_range_protector(quality)?);
    fs::write("./tmp/1.webp", webp.to_vec())?;
    Ok(())
}
