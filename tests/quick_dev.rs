use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:3000")?;

    hc.do_get("/hello?name=foo").await?.print().await?;
    hc.do_get("/hello2/Mike").await?.print().await?;
    // hc.do_get("/foo/sss").await?.print().await?;
    // hc.do_get("/src/main.rs").await?.print().await?;
    hc.do_post(
        "/api/login",
        json!({
            "username": "admin",
            "pwd": "admin"
        }),
    )
    .await?
    .print()
    .await?;

    Ok(())
}
