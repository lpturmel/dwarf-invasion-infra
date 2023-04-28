use aws_config::profile::ProfileFileCredentialsProvider;
use aws_sdk_secretsmanager::Client;
use serde::de::DeserializeOwned;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub enum Error {
    DeserializationError,
    SecretsManagerError,
}
#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub discord_bot_token: String,
    pub discord_logs_webhook_url: String,
    pub discord_sims_webhook_url: String,
    pub raidbots_cookie: String,
    pub warcraftlogs_api_key: String,
}
pub async fn load<T>(secret_name: String) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    // run in debug
    #[cfg(debug_assertions)]
    let config = {
        // Load from local profile in dev and in prod use load_from_env
        // The name of the custom credentials profile you want to load
        let profile_name = "lpturmel";

        // This credentials provider will load credentials from ~/.aws/credentials.
        let credentials_provider = ProfileFileCredentialsProvider::builder()
            .profile_name(profile_name)
            .build();

        // Load the credentials
        let config = aws_config::from_env()
            .credentials_provider(credentials_provider)
            .load()
            .await;
        config
    };
    #[cfg(not(debug_assertions))]
    let config = {
        let config = aws_config::load_from_env().await;
        config
    };
    let client = Client::new(&config);

    let resp = client
        .get_secret_value()
        .secret_id(secret_name)
        .send()
        .await
        .map_err(|e| {
            println!("Error: {:#?}", e);
            return Error::SecretsManagerError;
        })?;

    let secret_string: String = resp.secret_string.expect("Secret string not found");

    let secret: T =
        serde_json::from_str(&secret_string).map_err(|_| Error::DeserializationError)?;
    Ok(secret)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    struct TestSecret {
        discord_bot_token: String,
        discord_logs_webhook_url: String,
        discord_sims_webhook_url: String,
        raidbots_cookie: String,
        warcraftlogs_api_key: String,
    }

    #[tokio::test]
    async fn test_load() {
        let secret = load::<TestSecret>("dwarf-invasion".to_string())
            .await
            .unwrap();

        assert!(secret.discord_bot_token.len() > 0);
    }
}
