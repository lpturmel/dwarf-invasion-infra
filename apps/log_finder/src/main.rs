use lambda_runtime::{Context, LambdaEvent};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    let secrets: secrets::Config = secrets::load("dwarf-invasion".into())
        .await
        .expect("Could not retrieve config from Secrets Manager");
    #[cfg(debug_assertions)]
    {
        let secrets = secrets.clone();
        dotenv::dotenv().ok();
        let event = LambdaEvent::new(serde_json::Value::String("".into()), Context::default());
        handler(event, secrets).await?;
    }

    #[cfg(not(debug_assertions))]
    {
        tracing_subscriber::fmt()
            // .with_ansi(false)
            .without_time()
            .with_max_level(tracing::Level::INFO)
            .init();

        let func = lambda_runtime::service_fn(move |event: LambdaEvent<Value>| {
            handler(event, secrets.clone())
        });

        lambda_runtime::run(func).await?;
    }

    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
struct LogReport {
    id: String,
    title: String,
    owner: String,
    start: i64,
    end: i64,
    zone: i64,
}

async fn handler(
    _e: lambda_runtime::LambdaEvent<Value>,
    config: secrets::Config,
) -> Result<(), anyhow::Error> {
    let url = "https://www.warcraftlogs.com/v1/reports/guild/dwarf%20invasion/zuljin/us";
    let api_key = &config.warcraftlogs_api_key;
    let discord_webhook_url = &config.discord_logs_webhook_url;

    let poll_interval = env::var("POLL_INTERVAL")
        .expect("POLL_INTERVAL must be set")
        .parse::<i64>()
        .expect("POLL_INTERVAL must be a number");

    let client = reqwest::Client::new();

    let res = client
        .get(url)
        .query(&[("api_key", api_key)])
        .send()
        .await?;

    let reports: Vec<LogReport> = res.json().await?;

    let last = reports.iter().find(|report| {
        let now = chrono::Utc::now().timestamp();
        let start = report.start / 1000;
        let diff = now - start < poll_interval;
        diff
    });

    if let Some(report) = last {
        println!("[INFO] Found report: {:?}", report);
        let body = serde_json::json!({
            "content": format!("https://www.warcraftlogs.com/reports/{}", report.id),
        });
        client.post(discord_webhook_url).json(&body).send().await?;
    } else {
        println!("[INFO] No report found");
    }

    Ok(())
}
