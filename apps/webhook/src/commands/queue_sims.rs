use std::env;

use crate::error::Result;
use aws_sdk_lambda::types::ByteStream;
use axum::http::StatusCode;

use crate::discord::{
    DiscordPayload, DiscordResponse, InteractionResponse, ResponseType, SimRoles,
};

pub async fn run(body: &DiscordPayload) -> Result<(DiscordResponse, StatusCode)> {
    let user_roles = &body.member.roles;

    let allowed = user_roles
        .iter()
        .any(|role| match SimRoles::from_str(&role) {
            SimRoles::Everyone => false,
            _ => true,
        });

    let sim_creator_function_name =
        env::var("SIM_CREATOR_FUNCTION_NAME").expect("SIM_CREATOR_FUNCTION_NAME must be set");

    let message = match allowed {
        true => "Your sim will be posted in <#1045758066127282176> shortly (sometimes the sims get stuck in queues so it could take a few minutes)",
        false => "You are not allowed to queue for sims"
    };

    if allowed {
        let config = aws_config::load_from_env().await;
        let client = aws_sdk_lambda::Client::new(&config);

        let func_input = ByteStream::from("{}".as_bytes().to_vec());

        match client
            .invoke_async()
            .set_function_name(Some(sim_creator_function_name))
            .invoke_args(func_input)
            .send()
            .await
        {
            Ok(_) => (),
            Err(e) => {
                let res = InteractionResponse::new(
                    ResponseType::ChannelMessageWithSource,
                    format!("Failed to invoke the sim creator: {}", e),
                );
                return Ok((res, StatusCode::OK));
            }
        };
    }

    let res = InteractionResponse::new(ResponseType::ChannelMessageWithSource, message);

    Ok((res, StatusCode::OK))
}
