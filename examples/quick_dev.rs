use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello?name=Dima").await?.print().await?;
    hc.do_get("/hello2/Lucy").await?.print().await?;

    // hc.do_get("/src/main.rs").await?.print().await?;

    hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome",
        }),
    )
    .await?
    .print()
    .await?;

    hc.do_post(
        "/api/tickets",
        json!({
            "title": "Ticket 1",
        }),
    )
    .await?
    .print()
    .await?;

    // hc.do_delete("/api/tickets/1").await?.print().await?;

    hc.do_get("/api/tickets").await?.print().await?;

    Ok(())
}
