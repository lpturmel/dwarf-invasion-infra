use crate::discord::{DiscordPayload, DiscordResponse, InteractionResponse, ResponseType};
use crate::error::Result;
use axum::http::StatusCode;
use rand::Rng;

pub async fn run(body: &DiscordPayload) -> Result<(DiscordResponse, StatusCode)> {
    let data = &body.data;

    let mut max = 100;

    if let Some(options) = &data.options {
        max = match options.iter().find(|x| x.name == "max") {
            Some(x) => x.value.as_ref().unwrap().as_u64().unwrap(),
            None => 100,
        };
    }

    let user_id = &body.member.user.id;

    let roll = rand::thread_rng().gen_range(1..=max);

    let res = InteractionResponse::new(
        ResponseType::ChannelMessageWithSource,
        format!("🎲 <@{}> {}", user_id, roll),
    );
    Ok((res, StatusCode::OK))
}
