use super::{configuration, ContentType, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{de::Error as _, Deserialize, Serialize};

/// struct for typed errors of method [`get_streams_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetStreamsListError {
    Status401(models::ErrorSchema),
    Status403(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`read_stream`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReadStreamError {
    Status400(models::ErrorSchema),
    Status401(models::ErrorSchema),
    Status403(models::ErrorSchema),
    Status404(models::ErrorSchema),
    Status410(models::ErrorSchema),
    Status421(models::ErrorSchema),
    Status429(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}

/// List IDs of IB feed streams the API caller has permission to access.
pub async fn get_streams_list(
    configuration: &configuration::Configuration,
) -> Result<models::StreamList, Error<GetStreamsListError>> {
    let uri_str = format!("{}/ib/v1/streams", configuration.base_path);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::StreamList`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::StreamList`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetStreamsListError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

/// Get events from an IB feed stream. For the Base Feed and Enriched Feed workflows, the 'messages' array will contain at most a single message. In the On Demand workflow, the 'messages' array may contain multiple messages.
pub async fn read_stream(
    configuration: &configuration::Configuration,
    stream_id: &str,
    send_content_events: Option<bool>,
    send_idea_drawer_feedback_events: Option<bool>,
    backfill_id: Option<&str>,
) -> Result<models::ReadStream200Response, Error<ReadStreamError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_stream_id = stream_id;
    let p_send_content_events = send_content_events;
    let p_send_idea_drawer_feedback_events = send_idea_drawer_feedback_events;
    let p_backfill_id = backfill_id;

    let uri_str = format!(
        "{}/ib/v1/streams/{stream_id}",
        configuration.base_path,
        stream_id = crate::apis::urlencode(p_stream_id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_send_content_events {
        req_builder = req_builder.query(&[("sendContentEvents", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_send_idea_drawer_feedback_events {
        req_builder =
            req_builder.query(&[("sendIdeaDrawerFeedbackEvents", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_backfill_id {
        req_builder = req_builder.query(&[("backfillId", &param_value.to_string())]);
    }
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ReadStream200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ReadStream200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ReadStreamError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
