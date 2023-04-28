use std::env;

use crate::error::Result;
use axum::http::StatusCode;

use crate::discord::{DiscordPayload, DiscordResponse, InteractionResponse, ResponseType};

use super::RosterItem;

pub async fn run(body: &DiscordPayload) -> Result<(DiscordResponse, StatusCode)> {
    println!("Processing delchar request");
    let options = match &body.data.options {
        Some(x) => x,
        None => {
            let res = InteractionResponse::new(
                ResponseType::ChannelMessageWithSource,
                "No options provided",
            );
            return Ok((res, StatusCode::OK));
        }
    };
    println!("Searching for character_name command parameter");
    let character_name = match &options.iter().find(|x| x.name == "character_name") {
        Some(character_name) => character_name
            .value
            .as_ref()
            .unwrap()
            .as_str()
            .unwrap()
            .to_string(),
        None => {
            let res = InteractionResponse::new(
                ResponseType::ChannelMessageWithSource,
                "Failed to find the option character_name in your request. Please include a valid character_name in the option",
            );
            return Ok((res, StatusCode::OK));
        }
    };
    println!("character_name parameter found: {}", character_name);

    let bucket_name = env::var("BUCKET_NAME").unwrap();
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_s3::Client::new(&config);

    println!("Getting roster data from S3...");
    let content = match client
        .get_object()
        .bucket(&bucket_name)
        .key("roster.json")
        .send()
        .await
    {
        Ok(content) => content,
        Err(_e) => {
            let res = InteractionResponse::new(
                ResponseType::ChannelMessageWithSource,
                "Failed to query the roster data",
            );
            return Ok((res, StatusCode::OK));
        }
    };
    println!("Got roster content");
    println!("Collecting bytes...");
    let bytes = match content.body.collect().await {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Failed to read roster ByteStream: {}", e);
            let res = InteractionResponse::new(
                ResponseType::ChannelMessageWithSource,
                "Failed to read roster ByteStream",
            );
            return Ok((res, StatusCode::OK));
        }
    }
    .into_bytes();
    println!("Serializing roster...");
    let mut roster: Vec<RosterItem> = serde_json::from_slice(&bytes).unwrap();

    println!("Getting char index");
    let index = match roster
        .iter()
        .position(|x| x.character_name.to_lowercase() == character_name.to_lowercase())
    {
        Some(index) => index,
        None => {
            let res = InteractionResponse::new(
                ResponseType::ChannelMessageWithSource,
                format!("The character {} was not found in the current roster. Please check the character name and try again.", character_name)
            );
            return Ok((res, StatusCode::OK));
        }
    };
    println!("Found the index. Removing character from vector...");
    roster.remove(index);

    let data = match serde_json::to_string(&roster) {
        Ok(data) => data.into_bytes(),
        Err(e) => {
            eprintln!("Failed to serialize roster to JSON: {}", e);
            let res = InteractionResponse::new(
                ResponseType::ChannelMessageWithSource,
                "Failed to serialize roster to JSON",
            );
            return Ok((res, StatusCode::OK));
        }
    };

    println!("Uploading new roster to S3...");
    match client
        .put_object()
        .bucket(&bucket_name)
        .key("roster.json")
        .body(data.into())
        .send()
        .await
    {
        Ok(_) => {
            println!("Successfully uploaded new roster to S3");
            let res = InteractionResponse::new(
                ResponseType::ChannelMessageWithSource,
                format!(
                    "Successfully removed character {} from the roster",
                    character_name
                ),
            );
            return Ok((res, StatusCode::OK));
        }
        Err(e) => {
            eprintln!("Failed to save roster to S3: {}", e);
            let res = InteractionResponse::new(
                ResponseType::ChannelMessageWithSource,
                "Failed to save roster to S3. Your character was not removed from the roster. Please try again",
            );
            return Ok((res, StatusCode::OK));
        }
    }
}
