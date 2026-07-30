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
    let subtitles: Subtitles = response
        .json()
        .await
        .expect("Failed to deserialize JSON into Subtitles struct");

    // Snapshot the full output so you know when the API schema/data changes.
    // 
    // WARNING: Since we are hitting a live API, `download_count` and `new_download_count` 
    // will change every day, causing `cargo test` to fail CI.
    assert_json_snapshot!(subtitles, {
        ".data[].attributes.download_count" => insta::dynamic_redaction(|_, _| "REDACTED"),
        ".data[].attributes.new_download_count" => insta::dynamic_redaction(|_, _| "REDACTED"),
    });
}
