use std::time::Duration;

use reqwest::Client;
use serde_json::json;

use crate::modules::components::log_error::log_error_fl::log_error_fn;

// pub async fn get_token_from_employee_id_fn(employee_id: String) -> Result<String, Box<dyn std::error::Error>> {
//     let client = Client::builder()
//         .danger_accept_invalid_certs(true)
//         .connect_timeout(Duration::from_secs(15))
//         .timeout(Duration::from_secs(60))
//         .build()
//         .expect("Failed to create reqwest client");

//     let api_url = "https://app.trackforce.io/api/Auth/RegisterAndCheck";

//     let mut headers = reqwest::header::HeaderMap::new();
//     headers.insert("Content-Type", "application/json".parse()?);

//     let payload = json!({
//         "employeeId": employee_id,
//     });

//     let response = client
//         .post(api_url)
//         .headers(headers)
//         .json(&payload)
//         .send()
//         .await;

//     match response {
//         Ok(resp) if resp.status().is_success() => {
//             let response_json: serde_json::Value = resp.json().await?;
//             let token = response_json["employeeInfo"]["strToken"].as_str().unwrap().to_string();
//             Ok(token)
//         },
//         Ok(resp) => Err(Box::new(std::io::Error::new(
//             std::io::ErrorKind::Other,
//             format!("Failed with status: {:?}", resp.status()),
//         ))),
//         Err(e) => {
//             println!("Failed to get token from API: {:?}", e);
//             Err(Box::new(e))
//         },
//     }
// }

pub async fn get_token_from_employee_id_fn(
    employee_id: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .connect_timeout(Duration::from_secs(15))
        .timeout(Duration::from_secs(60))
        .build()
        .expect("Failed to create reqwest client");

    let api_url = "https://app.trackforce.io/api/Auth/RegisterAndCheck";
    // let api_url = "https://localhost:7020/api/Auth/RegisterAndCheck";

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse()?);

    let payload = json!({
        "employeeId": employee_id,
    });

    const MAX_ATTEMPTS: usize = 3;

    for attempt in 1..=MAX_ATTEMPTS {
        println!("Attempt {} to get token from API...", attempt);

        let response = client
            .post(api_url)
            .headers(headers.clone())
            .json(&payload)
            .send()
            .await;

        match response {
            Ok(resp) if resp.status().is_success() => {
                let response_json: serde_json::Value = resp.json().await?;
                if let Some(token) = response_json["employeeInfo"]["strToken"].as_str() {
                    return Ok(token.to_string());
                } else {
                    log_error_fn(&format!(
                        "Token not found in response (attempt {})",
                        attempt
                    ));
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Token not found in response",
                    )));
                }
            }
            Ok(resp) => {
                log_error_fn(&format!(
                    "Token get Failed with status code (attempt {}): {:?}",
                    attempt,
                    resp.status()
                ));
                println!("Failed with status: {:?}", resp.status());
            }
            Err(e) => {
                log_error_fn(&format!(
                    "Failed to get token from API (attempt {}): {:?}",
                    attempt, e
                ));
                println!("Failed to get token from API: {:?}", e);
            }
        }

        if attempt < MAX_ATTEMPTS {
            println!("Retrying after a delay...");
            tokio::time::sleep(Duration::from_secs(10)).await;
        }
    }
    log_error_fn("Failed to get token after multiple attempts");
    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Failed to get token after multiple attempts",
    )))
}
