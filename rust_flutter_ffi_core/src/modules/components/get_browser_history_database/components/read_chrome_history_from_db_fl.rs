use crate::{
    models::tracforce_post_model::BrowserHistorySingle,
    modules::components::log_error::log_error_fl::log_error_fn,
};
use anyhow::Result;
use chrono::{DateTime, Duration, NaiveDateTime};
use rusqlite::Connection;
use std::path::Path;

pub async fn read_chrome_history_from_db(path: &Path) -> Vec<BrowserHistorySingle> {
    // let query =
    //     "SELECT url, title, last_visit_time FROM urls ORDER BY last_visit_time DESC LIMIT 10";
    let query = "SELECT 
    urls.url AS url,
    urls.title AS title,
    urls.last_visit_time AS last_visit_time,
    urls.visit_count AS visit_count,
    visits.visit_duration AS visit_duration
    FROM 
    urls
    LEFT JOIN 
    visits ON urls.id = visits.url
    ORDER BY 
    urls.last_visit_time DESC
    LIMIT 10
    ";
    let mut history = Vec::new();
    let path = path.to_path_buf();

    match tokio::task::spawn_blocking(move || {
        let conn = Connection::open(&path)?;
        let mut stmt = conn.prepare(query)?;
        let rows = stmt.query_map([], |row| {
            let url: String = row.get(0)?;
            let title: String = row.get(1)?;
            let lastvisittime: i64 = row.get(2)?;
            let visitcount: i32 = row.get(3)?;
            let visitduration: i64 = row.get(4)?;

            // Ok(format!(
            //     "URL: {:?}, Title: {:?}, LastVisitTime: {:?}, VisitCount: {:?}, VisitDuration: {:?}",
            //     url, title, lastvisittime, visitcount, visitduration
            // ))

            let utc_plus_6_time = convert_chrome_timestamp_to_utc_plus6(lastvisittime);

            Ok(BrowserHistorySingle {
                url,
                title,
                last_visit_time: utc_plus_6_time.to_string(),
                visit_count: visitcount,
                visit_duration: visitduration,
            })
        })?;

        let history: Vec<BrowserHistorySingle> = rows.filter_map(Result::ok).collect();
        Ok::<_, rusqlite::Error>(history)
    })
    .await
    {
        Ok(Ok(result)) => history = result,
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

fn convert_chrome_timestamp_to_utc_plus6(chrome_timestamp: i64) -> NaiveDateTime {
    let chrome_epoch = DateTime::from_timestamp(-11644473600, 0).unwrap();

    let seconds = chrome_timestamp / 1_000_000;
    let nanoseconds = (chrome_timestamp % 1_000_000) * 1_000;

    let utc_time = chrome_epoch + Duration::seconds(seconds) + Duration::nanoseconds(nanoseconds);

    (utc_time + Duration::hours(6)).naive_utc()
}
