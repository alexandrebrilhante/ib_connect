use super::{configuration, ContentType, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{de::Error as _, Deserialize, Serialize};

/// struct for typed errors of method [`get_chatbot_rooms`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetChatbotRoomsError {
    Status400(models::ErrorSchema),
    Status403(models::ErrorSchema),
    Status429(models::ErrorSchema),
    Status500(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_chatbot_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostChatbotMessageError {
    Status400(models::ErrorSchema),
    Status403(models::ErrorSchema),
    Status408(models::ErrorSchema),
    Status411(models::ErrorSchema),
    Status412(models::ErrorSchema),
    Status413(models::ErrorSchema),
    Status415(models::ErrorSchema),
    Status429(models::ErrorSchema),
    Status500(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}

/// Given a chatbot ID, Returns a list of Room IDs that the chatbot is a member of.
pub async fn get_chatbot_rooms(
    configuration: &configuration::Configuration,
    chatbot_id: i32,
) -> Result<models::RoomList, Error<GetChatbotRoomsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_chatbot_id = chatbot_id;

    let uri_str = format!(
        "{}/ib/v1/chatbot/{chatbot_id}/rooms",
        configuration.base_path,
        chatbot_id = p_chatbot_id
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::RoomList`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::RoomList`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetChatbotRoomsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// A Registered IB Chatbot Posts Unstructured Content to an IB chat Room.
pub async fn post_chatbot_message(
    configuration: &configuration::Configuration,
    chatbot_post_request: models::ChatbotPostRequest,
) -> Result<serde_json::Value, Error<PostChatbotMessageError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_chatbot_post_request = chatbot_post_request;

    let uri_str = format!("{}/ib/v1/chatbot", configuration.base_path);
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_chatbot_post_request);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `serde_json::Value`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `serde_json::Value`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PostChatbotMessageError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
