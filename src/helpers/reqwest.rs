// useless at the moment
pub async fn get_request(req_url: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(
        reqwest::get(req_url).await?.text().await?
    ) 
}
