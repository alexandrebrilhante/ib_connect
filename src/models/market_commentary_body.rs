use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarketCommentaryBody {
    #[serde(rename = "rich-text", skip_serializing_if = "Option::is_none")]
    pub rich_text: Option<Box<models::MarketCommentaryBodyRichText>>,
    #[serde(rename = "block", skip_serializing_if = "Option::is_none")]
    pub block: Option<Box<models::MarketCommentaryBodyBlock>>,
    #[serde(rename = "paragraph", skip_serializing_if = "Option::is_none")]
    pub paragraph: Option<Box<models::Paragraph>>,
    #[serde(rename = "inline-token", skip_serializing_if = "Option::is_none")]
    pub inline_token: Option<Box<models::MarketCommentaryBodyInlineToken>>,
    #[serde(rename = "text-token", skip_serializing_if = "Option::is_none")]
    pub text_token: Option<Box<models::Text>>,
    #[serde(rename = "link-token", skip_serializing_if = "Option::is_none")]
    pub link_token: Option<Box<models::Link>>,
    #[serde(
        rename = "rich-text-attributes",
        skip_serializing_if = "Option::is_none"
    )]
    pub rich_text_attributes: Option<Box<models::TextFormattingOptions>>,
    #[serde(rename = "link", skip_serializing_if = "Option::is_none")]
    pub link: Option<Box<models::MarketCommentaryBodyLink>>,
    #[serde(rename = "bb-link", skip_serializing_if = "Option::is_none")]
    pub bb_link: Option<Box<models::BloombergLink>>,
    #[serde(rename = "web-link", skip_serializing_if = "Option::is_none")]
    pub web_link: Option<Box<models::WebLink>>,
}

impl MarketCommentaryBody {
    pub fn new() -> MarketCommentaryBody {
        MarketCommentaryBody {
            rich_text: None,
            block: None,
            paragraph: None,
            inline_token: None,
            text_token: None,
            link_token: None,
            rich_text_attributes: None,
            link: None,
            bb_link: None,
            web_link: None,
        }
    }
}
