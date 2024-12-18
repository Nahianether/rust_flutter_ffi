use crate::models::tracforce_post_model::{SearchKeyFromUrl, SearchKeyFromUrlPayload};
use crate::modules::components::log_error::log_error_fl::log_error_fn;
use anyhow::Result;
use chrono::{DateTime, Duration, NaiveDateTime};

use rusqlite::Connection;
use std::path::Path;

use url::Url;

pub async fn read_chrome_history_for_search_key_from_db_fn(path: &Path) -> Vec<SearchKeyFromUrl> {
    let query = "SELECT url, last_visit_time FROM urls ORDER BY last_visit_time DESC LIMIT 10";
    let mut history = Vec::new();
    let path = path.to_path_buf();

    match tokio::task::spawn_blocking(move || {
        let conn = Connection::open(&path)?;
        let mut stmt = conn.prepare(query)?;
        let rows = stmt.query_map([], |row| {
            let url: String = row.get(0)?;
            let last_visit_time: i64 = row.get(1)?;

            Ok(SearchKeyFromUrlPayload {
                url,
                date_time: last_visit_time,
            })
        })?;

        let payload: Vec<SearchKeyFromUrlPayload> = rows.filter_map(Result::ok).collect();
        Ok::<_, rusqlite::Error>(payload)
    })
    .await
    {
        Ok(Ok(payload)) => {
            for payload in payload {
                if let Some(search_key) = extract_search_key(&payload.url) {
                    let utc_plus_6_time = convert_chrome_timestamp_to_utc_plus6(payload.date_time);
                    let trim_url_data = trim_url(&payload.url);
                    history.push(SearchKeyFromUrl {
                        url: trim_url_data.unwrap_or_default(),
                        search_key: Some(search_key),
                        date_time: utc_plus_6_time.to_string(),
                    });
                } else {
                    let utc_plus_6_time = convert_chrome_timestamp_to_utc_plus6(payload.date_time);
                    let trim_url_data = trim_url(&payload.url);
                    history.push(SearchKeyFromUrl {
                        url: trim_url_data.unwrap_or_default(),
                        search_key: None,
                        date_time: utc_plus_6_time.to_string(),
                    });
                }
            }
        }
        Ok(Err(e)) => {
            println!("Failed to retrieve Chrome history data");
            log_error_fn(&format!("Failed to retrieve Chrome history data: {:?}", e));
        }
        Err(e) => {
            println!("Failed to join the blocking task");
            log_error_fn(&format!("Failed to join the blocking task: {:?}", e));
        }
    }

    history
}

fn extract_search_key(url: &str) -> Option<String> {
    if let Ok(parsed_url) = Url::parse(url) {
        for (key, value) in parsed_url.query_pairs() {
            if key == "q" || key == "p" {
                return Some(value.to_string());
            }
        }
    }
    None
}

fn convert_chrome_timestamp_to_utc_plus6(chrome_timestamp: i64) -> NaiveDateTime {
    let chrome_epoch = DateTime::from_timestamp(-11644473600, 0).unwrap();

    let seconds = chrome_timestamp / 1_000_000;
    let nanoseconds = (chrome_timestamp % 1_000_000) * 1_000;

    let utc_time = chrome_epoch + Duration::seconds(seconds) + Duration::nanoseconds(nanoseconds);

    (utc_time + Duration::hours(6)).naive_utc()
}

fn trim_url(url: &str) -> Option<String> {
    if let Ok(parsed_url) = Url::parse(url) {
        if let Some(host) = parsed_url.host_str() {
            return Some(format!("{}://{}", parsed_url.scheme(), host));
        }
    }
    None
}
