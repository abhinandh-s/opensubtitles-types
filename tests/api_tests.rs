use insta::assert_json_snapshot;
use opensubtitles_types::Subtitles;
use reqwest::Client;
use std::env;

#[tokio::test]
async fn test_fetch_subtitles_live_api() {
    let url = "https://api.opensubtitles.com/api/v1/subtitles?imdb_id=0133093&languages=en&page=1";
    let api_key = env::var("OPENSUBTITLES_API_KEY")
        .expect("Missing OPENSUBTITLES_API_KEY environment variable");

    let client = Client::new();
    let response = client
        .get(url)
        .header("Api-Key", api_key)
        .header("User-Agent", "opensubtitles-types-tester/1.0")
        .send()
        .await
        .expect("Failed to execute request");

    assert!(
        response.status().is_success(),
        "Expected success, got HTTP {}",
        response.status()
    );

    // validates struct matches the API schema
    let raw_json = response.text().await.expect("Failed to read response text");

    // Setup a JSON deserializer
    let mut deserializer = serde_json::Deserializer::from_str(&raw_json);

    // Attempt to deserialize, and if it fails, capture the exact path
    let subtitles: Subtitles = match serde_path_to_error::deserialize(&mut deserializer) {
        Ok(data) => data,
        Err(err) => {
             panic!("Deserialization failed at path: {}\nError: {}", err.path(), err);
        }
    };

    // Snapshot the full output so you know when the API schema/data changes.
    // 
    // WARNING: Since we are hitting a live API, `download_count` and `new_download_count` 
    // will change every day, causing `cargo test` to fail CI.
    assert_json_snapshot!(subtitles, {
        ".data[].attributes.img_url" => insta::dynamic_redaction(|_, _| "REDACTED"),
        ".data[].attributes.related_links[].img_url" => insta::dynamic_redaction(|_, _| "REDACTED"),
        ".data[].attributes.download_count" => insta::dynamic_redaction(|_, _| "REDACTED"),      
        ".data[].attributes.new_download_count" => insta::dynamic_redaction(|_, _| "REDACTED"),
    });
}
