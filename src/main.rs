use axum::{Json, Router, extract::Query, routing::get};
use chrono::Utc;
use dotenvy::dotenv;
use hmac::{Hmac, Mac};
use serde::Deserialize;
use sha1::Sha1;
use std::env;

type HmacSha1 = Hmac<Sha1>;

#[derive(Deserialize)]
struct Params {
    key: String,
}

fn generate_presigned_url(
    ak: &str,
    sk: &str,
    bucket: &str,
    key: &str,
    endpoint: &str,
    expires: i64,
) -> String {
    let resource = format!("/{}/{}", bucket, key);

    let string_to_sign = format!("GET\n\n\n{}\n{}", expires, resource);

    let mut mac = HmacSha1::new_from_slice(sk.as_bytes()).unwrap();
    mac.update(string_to_sign.as_bytes());

    let signature = base64::encode(mac.finalize().into_bytes());
    let signature = urlencoding::encode(&signature);

    format!(
        "https://{}.{}{}?AccessKeyId={}&Expires={}&Signature={}",
        bucket,
        endpoint,
        format!("/{}", key),
        ak,
        expires,
        signature
    )
}

async fn presign(Query(params): Query<Params>) -> Json<serde_json::Value> {
    let ak = env::var("OBS_AK").unwrap();
    let sk = env::var("OBS_SK").unwrap();
    let bucket = env::var("OBS_BUCKET").unwrap();
    let endpoint = env::var("OBS_ENDPOINT").unwrap();

    let expires = Utc::now().timestamp() + 3600;

    let url = generate_presigned_url(&ak, &sk, &bucket, &params.key, &endpoint, expires);

    Json(serde_json::json!({ "url": url }))
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new().route("/presign", get(presign));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("server running on {:?}", listener);

    axum::serve(listener, app).await.unwrap();
}
