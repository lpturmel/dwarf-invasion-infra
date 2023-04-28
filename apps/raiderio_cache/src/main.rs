mod profile;

use std::collections::HashMap;
use std::env;
use std::time::Instant;

use aws_sdk_dynamodb::model::AttributeValue;
use lambda_runtime::{Context, LambdaEvent};
use serde::{Deserialize, Serialize};
use serde_dynamo::aws_sdk_dynamodb_0_22::to_item;
use serde_json::Value;

use profile::ProfileResponse;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct RosterItem {
    pub class: String,
    pub spec: String,
    pub character_name: String,
    pub alt: bool,
    pub user_id: String,
}
async fn handler(
    _e: lambda_runtime::LambdaEvent<Value>,
    _config: secrets::Config,
) -> Result<(), anyhow::Error> {
    let table_name = env::var("TABLE_NAME").expect("TABLE_NAME must be set");
    let bucket_name = env::var("BUCKET_NAME").expect("BUCKET_NAME must be set");
    let profile_url = "https://raider.io/api/v1/characters/profile";

    let config = aws_config::load_from_env().await;
    let client = aws_sdk_s3::Client::new(&config);
    let roster_res = client
        .get_object()
        .bucket(&bucket_name)
        .key("roster.json")
        .send()
        .await?;

    let roster_bytes = match roster_res.body.collect().await {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Failed to read roster ByteStream: {}", e);
            return Ok(());
        }
    }
    .into_bytes();
    let roster: Vec<RosterItem> = serde_json::from_slice(&roster_bytes).unwrap();

    let mut profiles: Vec<HashMap<String, AttributeValue>> = vec![];

    for char in roster {
        let char_now = Instant::now();
        let url = format!(
            "{}?region=us&realm=zuljin&name={}&fields=gear,mythic_plus_scores_by_season:current,mythic_plus_ranks,mythic_plus_recent_runs,mythic_plus_highest_level_runs,raid_progression",
            profile_url, char.character_name
        );
        let res = reqwest::get(url).await?;
        let json: ProfileResponse = match res.json().await {
            Ok(json) => json,
            Err(e) => {
                eprintln!(
                    "Failed to parse profile response for char {}: {}",
                    char.character_name, e
                );
                continue;
            }
        };
        println!(
            "Fetched profile for: {}: [took {}ms]",
            char.character_name,
            char_now.elapsed().as_millis()
        );
        let item = to_item(&json).unwrap();
        profiles.push(item);
    }

    let dynamo_client = aws_sdk_dynamodb::Client::new(&config);
    for profile in profiles {
        let put_item = dynamo_client
            .put_item()
            .table_name(&table_name)
            .set_item(Some(profile))
            .send()
            .await?;
        println!("{:?}", put_item);
    }
    Ok(())
}
