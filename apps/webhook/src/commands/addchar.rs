use crate::commands::RosterItem;
use crate::discord::{DiscordPayload, DiscordResponse, InteractionResponse, ResponseType};
use crate::error::Result;
use axum::http::StatusCode;

pub async fn run(body: &DiscordPayload) -> Result<(DiscordResponse, StatusCode)> {
    let options = match body.data.options.as_ref() {
        Some(x) => x,
        None => {
            let res = InteractionResponse::new(
                ResponseType::ChannelMessageWithSource,
                "No options provided",
            );
            return Ok((res, StatusCode::OK));
        }
    };

    let command = &options.get(0).unwrap().clone();
    let command_options = command.options.as_ref().unwrap();

    let character_name = &command_options
        .iter()
        .find(|x| x.name == "character_name")
        .unwrap()
        .value
        .as_ref()
        .unwrap();

    let alt = &command_options
        .iter()
        .find(|x| x.name == "alt")
        .unwrap()
        .value
        .as_ref()
        .unwrap()
        .as_bool()
        .unwrap();

    let spec = &command_options
        .iter()
        .find(|x| x.name == "spec")
        .unwrap()
        .value
        .as_ref()
        .unwrap();

    let config = aws_config::load_from_env().await;
    let client = aws_sdk_s3::Client::new(&config);
    let content = match client
        .get_object()
        .bucket("dwarf-invasion")
        .key("roster.json")
        .send()
        .await
    {
        Ok(content) => content,
        Err(e) => {
            let res = InteractionResponse::new(
                ResponseType::ChannelMessageWithSource,
                format!("Failed to query the roster data: {}", e),
            );
            return Ok((res, StatusCode::OK));
        }
    };
    let bytes = content.body.collect().await.unwrap().into_bytes();
    let mut roster: Vec<RosterItem> = serde_json::from_slice(&bytes).unwrap();

    let user_id = &body.member.user.id;

    let character_name = character_name.as_str().unwrap().to_string();
    let exists = roster
        .iter()
        .any(|x| x.character_name.to_lowercase() == character_name.to_lowercase());

    if !alt {
        let user_chars = roster
            .iter()
            .filter(|x| x.user_id == *user_id)
            .collect::<Vec<&RosterItem>>();
        if user_chars.len() >= 1 {
            let res = InteractionResponse::new(
                ResponseType::ChannelMessageWithSource,
                format!(
                    "You already have a character marked as your main in the roster: {}",
                    user_chars[0].character_name
                ),
            );
            return Ok((res, StatusCode::OK));
        }
    }

    if exists {
        let res = InteractionResponse::new(
            ResponseType::ChannelMessageWithSource,
            format!(
                "The character {} already exists in the roster",
                character_name
            ),
        );
        return Ok((res, StatusCode::OK));
    }

    roster.push(RosterItem {
        class: command.name.clone().as_str().to_string(),
        spec: spec.clone().as_str().unwrap().to_string(),
        character_name: character_name.clone(),
        alt: alt.clone(),
        user_id: user_id.clone(),
    });

    match client
        .put_object()
        .bucket("dwarf-invasion")
        .key("roster.json")
        .body(serde_json::to_string(&roster).unwrap().into_bytes().into())
        .send()
        .await
    {
        Ok(_) => {
            let res = InteractionResponse::new(
                ResponseType::ChannelMessageWithSource,
                format!("Added {} to the roster!", character_name),
            );
            return Ok((res, StatusCode::OK));
        }
        Err(_) => {
            let res = InteractionResponse::new(
                ResponseType::ChannelMessageWithSource,
                "Failed to update the roster",
            );
            return Ok((res, StatusCode::OK));
        }
    }
}
