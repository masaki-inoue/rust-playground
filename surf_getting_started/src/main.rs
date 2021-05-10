use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Ip {
    ip: String,
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let uri = "https://httpbin.org/post";
    let data = &Ip {
        ip: "129.0.0.1".into(),
    };
    let res = surf::post(uri).body(surf::Body::from_json(data)?).await?;
    assert_eq!(res.status(), 200);

    Ok(())
}