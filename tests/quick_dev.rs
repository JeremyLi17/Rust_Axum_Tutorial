#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:9000")?;

    // hc.do_get("/hello?name=jeremy").await?.print().await?;
    hc.do_get("/hello2/jeremy").await?.print().await?;

    hc.do_get("/src/main.rs").await?.print().await?;

    Ok(())
}
