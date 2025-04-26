use super::{configuration, ContentType, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{de::Error as _, Deserialize, Serialize};

/// struct for typed errors of method [`post_initiate_chat`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostInitiateChatError {
    Status400(models::ErrorSchema),
    Status401(models::ErrorSchema),
    Status408(models::ErrorSchema),
    Status411(models::ErrorSchema),
    Status412(models::ErrorSchema),
    Status413(models::ErrorSchema),
    Status415(models::ErrorSchema),
    Status429(models::ErrorSchema),
    Status500(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}

/// Post a chat message to one or more recipients in IB where a recipient can be a user or an existing chat room.
pub async fn post_initiate_chat(
    configuration: &configuration::Configuration,
    chat_initiation_request: models::ChatInitiationRequest,
) -> Result<models::PostInitiateChat202Response, Error<PostInitiateChatError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_chat_initiation_request = chat_initiation_request;

    let uri_str = format!("{}/ib/v1/initiateChat", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_chat_initiation_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PostInitiateChat202Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PostInitiateChat202Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PostInitiateChatError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
