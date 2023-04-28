use crate::error::Result;
use ed25519_dalek::{Signature, Verifier};
use serde::{Deserialize, Serialize};

/// Verify an ed25519 signature
/// used for validating discord webhooks
pub fn verify_sig(
    body: String,
    signature: String,
    timestamp: String,
    public_key: String,
) -> Result<bool> {
    let sig_data = hex::decode(signature)?;
    let public_key_data = hex::decode(public_key)?;
    let signature = Signature::from_bytes(&sig_data)?;
    let public_key = ed25519_dalek::PublicKey::from_bytes(&public_key_data)?;
    let timestamp_data = timestamp.as_bytes();
    let body_data = body.as_bytes();
    let message = [timestamp_data, body_data].concat();
    Ok(public_key.verify(&message, &signature).is_ok())
}

pub enum SimRoles {
    Admin,
    Officer,
    Dev,
    Everyone,
}

impl SimRoles {
    pub fn from_str(s: &str) -> Self {
        match s {
            "1044039311638675568" => SimRoles::Admin,
            "1050534316104495104" => SimRoles::Officer,
            "1050585361362980884" => SimRoles::Dev,
            _ => SimRoles::Everyone,
        }
    }
}

pub enum ResponseType {
    Pong,
    ChannelMessageWithSource,
    DeferredChannelMessageWithSource,
    DeferredUpdateMessage,
    UpdateMessage,
    ApplicationCommandAutocompleteResult,
    Modal,
}

impl ResponseType {
    fn to_int(&self) -> u64 {
        match self {
            ResponseType::Pong => 1,
            ResponseType::ChannelMessageWithSource => 4,
            ResponseType::DeferredChannelMessageWithSource => 5,
            ResponseType::DeferredUpdateMessage => 6,
            ResponseType::UpdateMessage => 7,
            ResponseType::ApplicationCommandAutocompleteResult => 8,
            ResponseType::Modal => 9,
        }
    }
}
#[derive(Debug)]
pub struct InteractionResponse;

impl InteractionResponse {
    pub fn new<S: Into<String>>(r#type: ResponseType, content: S) -> DiscordResponse {
        DiscordResponse {
            r#type: r#type.to_int(),
            data: DiscordResponseData {
                content: content.into(),
                flags: 0,
                tts: false,
                embeds: None,
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordOption {
    pub name: String,
    pub r#type: u8,
    pub value: Option<serde_json::Value>,
    pub options: Option<Vec<DiscordOption>>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordData {
    pub id: String,
    pub name: String,
    pub r#type: u64,
    pub options: Option<Vec<DiscordOption>>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordUser {
    pub avatar: Option<String>,
    pub avatar_decoration: Option<String>,
    pub discriminator: String,
    pub id: String,
    pub public_flags: u64,
    pub username: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordMember {
    pub roles: Vec<String>,
    pub user: DiscordUser,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordPayload {
    pub application_id: String,
    pub channel_id: Option<String>,
    /// The data of the incoming integration command
    pub data: DiscordData,
    pub guild_id: Option<String>,
    pub member: DiscordMember,
    pub r#type: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordResponseData {
    content: String,
    flags: u64,
    tts: bool,
    embeds: Option<Vec<serde_json::Value>>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordResponse {
    r#type: u64,
    data: DiscordResponseData,
}
