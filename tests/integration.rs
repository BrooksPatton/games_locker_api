use eyre::Result;

const BASE_URL: &str = "http://localhost:3000";

#[tokio::test]
async fn healthcheck() -> Result<()> {
    let url = format!("{BASE_URL}/healthcheck");
    let request = reqwest::get(url).await?;
    let status_code = request.status();

    assert_eq!(status_code, 200);

    Ok(())
}
