use insta::assert_json_snapshot;
use opensubtitles_types::{DownloadRequest, DownloadResponse, DOWNLOAD_URL};
use reqwest::Client;
use std::env;

const API_KEY: &str = env!("OPENSUBTITLES_API_KEY");

#[tokio::test]
async fn test_download_subtitle_live_api() {
    let api_key = env::var("OPENSUBTITLES_API_KEY")
        .expect("Missing OPENSUBTITLES_API_KEY environment variable");
    
   // let bearer_token = env::var("OPENSUBTITLES_BEARER_TOKEN")
       // .expect("Missing OPENSUBTITLES_BEARER_TOKEN environment variable");

    let client = Client::new();
    
    let payload = DownloadRequest::new(7421118);

    let response = client
        .post(DOWNLOAD_URL)
        .header("Api-Key", api_key)
       // .header("Authorization", format!("Bearer {}", bearer_token))
        .header("User-Agent", "opensubtitles-types-tester/1.0")
        .header("Accept", "application/json")
        .json(&payload)
        .send()
        .await
        .expect("Failed to execute request");

    // Print error details if the API rejects us (e.g., invalid token or file_id)
    if !response.status().is_success() {
        let status = response.status();
        let error_body = response.text().await.unwrap_or_default();
        panic!("API Request failed with status {}:\n{}", status, error_body);
    }

    let raw_json = response.text().await.expect("Failed to read response text");
    let mut deserializer = serde_json::Deserializer::from_str(&raw_json);
    
    let download_resp: DownloadResponse = match serde_path_to_error::deserialize(&mut deserializer) {
        Ok(data) => data,
        Err(err) => {
            panic!("Deserialization failed at path: {}\nError: {}", err.path(), err);
        }
    };

    // We must redact almost everything because quotas, times, and temporary links 
    // will change every single time the test is run, which would break the snapshot.
    assert_json_snapshot!(download_resp, {
        ".link" => insta::dynamic_redaction(|_, _| "REDACTED_LINK"),
        ".requests" => insta::dynamic_redaction(|_, _| "REDACTED_QUOTA"),
        ".remaining" => insta::dynamic_redaction(|_, _| "REDACTED_QUOTA"),
        ".message" => insta::dynamic_redaction(|_, _| "REDACTED_MESSAGE"),
        ".reset_time" => insta::dynamic_redaction(|_, _| "REDACTED_TIME"),
        ".reset_time_utc" => insta::dynamic_redaction(|_, _| "REDACTED_TIME"),
    });
}

/*
curl --request POST \
  --url https://api.opensubtitles.com/api/v1/download \
  --header 'Accept: application/json' \
  --header 'Authorization: Bearer 123' \
  --header 'Content-Type: application/json' \
  --header 'User-Agent: ' \
  --data '{
  "file_id": 123
}'
*/
pub async fn request_download_link(client: &Client,  payload: &DownloadResponse) -> Option<DownloadResponse> {
    let response = client.post(DOWNLOAD_URL)
        .header("Accept", "application/json")
        .header("Api-Key", API_KEY)
        .header("Content-Type", "application/json")
        .header("User-Agent", "opensubtitles-types v1.0.0")
        .json(payload)
        .send().await.ok()?;

    response.json().await.ok()
}
