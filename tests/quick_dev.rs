#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:5353")?;

    // hc.do_get("/hello?name=msa").await?.print().await?;
    hc.do_get("/hello2/msa").await?.print().await?;

    // hc.do_get("/src/main.rs").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!(
            {
                "username": "msa",
                "pwd": "123",
            }
        ),
    );

    req_login.await?.print().await?;

    Ok(())
}
