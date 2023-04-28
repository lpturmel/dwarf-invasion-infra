use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::Extension;
use axum::Json;
use axum::{routing::get, Router};
use error::{Error, Result};
use http::{HeaderMap, Request};
use hyper::Body;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

use self::discord::DiscordResponse;
use self::discord::InteractionResponse;
use self::discord::ResponseType;
use self::discord::{verify_sig, DiscordPayload};

pub mod commands;
pub mod discord;
pub mod error;
pub mod responses;

async fn ping() -> impl IntoResponse {
    "pong"
}
enum Command {
    Roster,
    AddChar,
    DelChar,
    Roll,
    QueueSims,
}
impl Command {
    fn from_id_str(s: &str) -> Option<Command> {
        match s {
            "1044441548277948436" => Some(Command::Roster),
            "1044740904235319377" => Some(Command::AddChar),
            "1046156516266352701" => Some(Command::DelChar),
            "1045885525703270460" => Some(Command::Roll),
            "1050585792570986537" => Some(Command::QueueSims),
            _ => None,
        }
    }
}
async fn integration(
    Json(body): Json<serde_json::Value>,
    Extension(config): Extension<Arc<secrets::Config>>,
    headers: HeaderMap,
) -> Result<(StatusCode, Json<DiscordResponse>)> {
    let signature = headers
        .get("x-signature-ed25519")
        .ok_or(Error::MissingSignature)?
        .to_str()
        .unwrap();
    let timestamp = headers
        .get("x-signature-timestamp")
        .ok_or(Error::MissingTimestamp)?
        .to_str()
        .unwrap();

    let body_str = serde_json::to_string(&body).map_err(Error::SerializeError)?;

    let valid_req = verify_sig(
        body_str,
        signature.to_string(),
        timestamp.to_string(),
        // public key
        "a5d1148be5d078b851d180a46134f24bceb1e6a02ff884c0c5bf2fc2ea85f408".to_string(),
    )?;

    if !valid_req {
        return Err(Error::BadSignature);
    }

    let body = serde_json::from_value::<DiscordPayload>(body)?;

    let (res, status) = match body.r#type {
        1 => commands::ping(&body).await.unwrap(),
        2 => {
            println!("Received command");
            let int_data = &body.data;
            let command = Command::from_id_str(&int_data.id).ok_or(Error::BadCommand)?;
            match command {
                Command::Roster => commands::roster::run(&body, config.clone()).await.unwrap(),
                Command::AddChar => commands::addchar::run(&body).await.unwrap(),
                Command::DelChar => commands::delchar::run(&body).await.unwrap(),
                Command::Roll => commands::roll::run(&body).await.unwrap(),
                Command::QueueSims => commands::queue_sims::run(&body).await.unwrap(),
            }
        }
        _ => (
            InteractionResponse::new(ResponseType::Pong, "Bad request type"),
            StatusCode::BAD_REQUEST,
        ),
    };

    Ok((status, Json(res)))
}
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        // .with_ansi(false)
        .without_time()
        .with_max_level(tracing::Level::INFO)
        .init();

    let secrets: secrets::Config = secrets::load("dwarf-invasion".into())
        .await
        .expect("Could not retrieve config from Secrets Manager");

    let secrets = Arc::new(secrets);

    let trace_layer =
        TraceLayer::new_for_http().on_request(|req: &Request<Body>, _: &tracing::Span| {
            let method = req.method();
            let url = req.uri().path();
            let msg = format!("{}  {}", method, url);
            tracing::info!(message = msg);
        });
    // Wrap an `axum::Router` with our state, CORS, Tracing, & Compression layers
    let app = Router::new()
        .route("/api/ping", get(ping))
        .route("/api/integration", post(integration))
        .layer(Extension(secrets))
        .layer(trace_layer);

    #[cfg(debug_assertions)]
    {
        dotenv::dotenv().ok();
        let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }

    // If we compile in release mode, use the Lambda Runtime
    #[cfg(not(debug_assertions))]
    {
        // To run with AWS Lambda runtime, wrap in our `LambdaLayer`
        let app = tower::ServiceBuilder::new()
            .layer(axum_aws_lambda::LambdaLayer::default())
            .service(app);

        lambda_http::run(app).await.unwrap();
    }
}
