use super::{configuration, ContentType, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{de::Error as _, Deserialize, Serialize};

/// struct for typed errors of method [`patch_response_to_stream`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PatchResponseToStreamError {
    Status400(models::ErrorSchema),
    Status401(models::ErrorSchema),
    Status403(models::ErrorSchema),
    Status422(models::ErrorSchema),
    UnknownValue(serde_json::Value),
}

/// Update Suggestions data. This request can be used to update supported changes of Suggestion
pub async fn patch_response_to_stream(
    configuration: &configuration::Configuration,
    stream_id: &str,
    patch_response_to_stream_request: models::PatchResponseToStreamRequest,
) -> Result<(), Error<PatchResponseToStreamError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_stream_id = stream_id;
    let p_patch_response_to_stream_request = patch_response_to_stream_request;

    let uri_str = format!(
        "{}/ib/v1/streams/{stream_id}",
        configuration.base_path,
        stream_id = crate::apis::urlencode(p_stream_id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_patch_response_to_stream_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<PatchResponseToStreamError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
