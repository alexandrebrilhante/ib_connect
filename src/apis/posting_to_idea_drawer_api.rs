use super::{configuration, ContentType, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{de::Error as _, Deserialize, Serialize};

/// struct for typed errors of method [`post_response_to_stream`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostResponseToStreamError {
    Status400(models::ErrorSchema),
    Status401(models::ErrorSchema),
    Status403(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}

/// Posts Suggestions. Additional enablement may be required for one or multiple response types.
pub async fn post_response_to_stream(
    configuration: &configuration::Configuration,
    stream_id: &str,
    post_response: models::PostResponse,
) -> Result<models::PostResponseToStream200Response, Error<PostResponseToStreamError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_stream_id = stream_id;
    let p_post_response = post_response;

    let uri_str = format!(
        "{}/ib/v1/streams/{stream_id}",
        configuration.base_path,
        stream_id = crate::apis::urlencode(p_stream_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_post_response);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PostResponseToStream200Response`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PostResponseToStream200Response`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PostResponseToStreamError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
