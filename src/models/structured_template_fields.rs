use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StructuredTemplateFields {
    #[serde(rename = "equity", skip_serializing_if = "Option::is_none")]
    pub equity: Option<Box<models::StructuredTemplateFieldsEquity>>,
    #[serde(rename = "fixed_income_cash", skip_serializing_if = "Option::is_none")]
    pub fixed_income_cash: Option<Box<models::StructuredTemplateFieldsFixedIncomeCash>>,
    #[serde(rename = "mortgage", skip_serializing_if = "Option::is_none")]
    pub mortgage: Option<Box<models::StructuredTemplateFieldsMortgage>>,
    #[serde(rename = "option", skip_serializing_if = "Option::is_none")]
    pub option: Option<Box<models::StructuredTemplateFieldsOption>>,
    #[serde(rename = "other", skip_serializing_if = "Option::is_none")]
    pub other: Option<Box<models::StructuredTemplateFieldsOther>>,
    #[serde(rename = "security_lending", skip_serializing_if = "Option::is_none")]
    pub security_lending: Option<Box<models::StructuredTemplateFieldsSecurityLending>>,
    /// Application meta-data
    #[serde(
        rename = "common_fields_app_meta_data",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_app_meta_data: Option<String>,
    /// The price of a security
    #[serde(
        rename = "common_fields_ask_price",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_ask_price: Option<f64>,
    /// Ask size (in security currency)
    #[serde(
        rename = "common_fields_ask_size",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_ask_size: Option<f64>,
    /// Ask value of a price_type for the quote
    #[serde(
        rename = "common_fields_ask_value",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_ask_value: Option<f64>,
    /// Bloomberg Company Identifier
    #[serde(rename = "common_fields_bbid", skip_serializing_if = "Option::is_none")]
    pub common_fields_bbid: Option<String>,
    /// FIGI of benchmark security
    #[serde(
        rename = "common_fields_benchmark",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_benchmark: Option<String>,
    /// Yellowkey of benchmark security
    #[serde(
        rename = "common_fields_benchmark_yellow_key",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_benchmark_yellow_key: Option<CommonFieldsBenchmarkYellowKey>,
    /// Bid price
    #[serde(
        rename = "common_fields_bid_price",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_bid_price: Option<f64>,
    /// Bid size (in security currency)
    #[serde(
        rename = "common_fields_bid_size",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_bid_size: Option<f64>,
    /// Bid value of a price_type for the quote
    #[serde(
        rename = "common_fields_bid_value",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_bid_value: Option<f64>,
    /// Three letter Currency identifier
    #[serde(
        rename = "common_fields_currency",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_currency: Option<String>,
    /// Current outstanding value
    #[serde(
        rename = "common_fields_current_face",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_current_face: Option<f64>,
    /// Cusip
    #[serde(
        rename = "common_fields_cusip",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_cusip: Option<String>,
    #[serde(rename = "common_fields_date", skip_serializing_if = "Option::is_none")]
    pub common_fields_date: Option<Box<models::StructuredTemplateFieldsCommonFieldsDate>>,
    /// Equity price associated with bond
    #[serde(
        rename = "common_fields_equity_price",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_equity_price: Option<f64>,
    /// Exchange code
    #[serde(
        rename = "common_fields_exchange",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_exchange: Option<String>,
    #[serde(
        rename = "common_fields_expiration_date",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_expiration_date:
        Option<Box<models::StructuredTemplateFieldsCommonFieldsExpirationDate>>,
    /// Primary security FIGI identifier
    #[serde(rename = "common_fields_figi", skip_serializing_if = "Option::is_none")]
    pub common_fields_figi: Option<String>,
    /// How changeable the stated quote is
    #[serde(
        rename = "common_fields_firmness",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_firmness: Option<CommonFieldsFirmness>,
    /// Natural Indication of Interest (see FINRA regulatory notice 11-43).
    #[serde(
        rename = "common_fields_if_natural",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_if_natural: Option<bool>,
    /// Ratio of an option leg
    #[serde(
        rename = "common_fields_leg_ratio",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_leg_ratio: Option<f64>,
    #[serde(
        rename = "common_fields_maturity_date",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_maturity_date: Option<Box<models::StructuredTemplateFieldsCommonFieldsDate>>,
    /// Represents a percentage and should always be normalized into a value between 0.00 and 1.00
    #[serde(
        rename = "common_fields_normalized_delta",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_normalized_delta: Option<f64>,
    /// The size of a deal in notional ammount
    #[serde(
        rename = "common_fields_notional_size",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_notional_size: Option<f64>,
    /// An option style
    #[serde(
        rename = "common_fields_option_style",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_option_style: Option<CommonFieldsOptionStyle>,
    /// The quote execution strategy
    #[serde(
        rename = "common_fields_order_type",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_order_type: Option<CommonFieldsOrderType>,
    /// The price of a security
    #[serde(
        rename = "common_fields_price",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_price: Option<f64>,
    /// Rate
    #[serde(rename = "common_fields_rate", skip_serializing_if = "Option::is_none")]
    pub common_fields_rate: Option<f64>,
    /// Reference price of underlying asset
    #[serde(
        rename = "common_fields_reference_price",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_reference_price: Option<f64>,
    /// Text describing the security, ex: THC 6.25 2018, AMNEAL PHARMA TL B
    #[serde(
        rename = "common_fields_security_description",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_security_description: Option<String>,
    /// The text scraped for settlement date.
    #[serde(
        rename = "common_fields_settlement_date",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_settlement_date: Option<String>,
    /// The size of a deal in shares
    #[serde(
        rename = "common_fields_share_size",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_share_size: Option<f64>,
    /// The size of a deal
    #[serde(rename = "common_fields_size", skip_serializing_if = "Option::is_none")]
    pub common_fields_size: Option<f64>,
    /// Free-form text for specifying additional details in SPW negotiations
    #[serde(rename = "common_fields_text", skip_serializing_if = "Option::is_none")]
    pub common_fields_text: Option<String>,
    /// Ticker associated with the security
    #[serde(
        rename = "common_fields_ticker",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_ticker: Option<String>,
    /// Version of the enrichment schema <major>.<minor>[.<patch>]
    #[serde(
        rename = "common_fields_version",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_version: Option<String>,
    /// Yellow key of security
    #[serde(
        rename = "common_fields_yellow_key",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_yellow_key: Option<CommonFieldsYellowKey>,
    #[serde(
        rename = "option_instrument_instrument_underlying",
        skip_serializing_if = "Option::is_none"
    )]
    pub option_instrument_instrument_underlying:
        Option<Box<models::StructuredTemplateFieldsOptionInstrumentInstrumentUnderlying>>,
    /// A leg side of the option
    #[serde(
        rename = "option_instrument_leg_side",
        skip_serializing_if = "Option::is_none"
    )]
    pub option_instrument_leg_side: Option<OptionInstrumentLegSide>,
    /// A leg type of the option
    #[serde(
        rename = "option_instrument_leg_type",
        skip_serializing_if = "Option::is_none"
    )]
    pub option_instrument_leg_type: Option<OptionInstrumentLegType>,
    /// Strike price of the option
    #[serde(
        rename = "option_instrument_strike_price",
        skip_serializing_if = "Option::is_none"
    )]
    pub option_instrument_strike_price: Option<f64>,
    #[serde(
        rename = "other_quote_value_type",
        skip_serializing_if = "Option::is_none"
    )]
    pub other_quote_value_type: Option<OtherQuoteValueType>,
    /// An array of different quote value types
    #[serde(rename = "other_quote_values", skip_serializing_if = "Option::is_none")]
    pub other_quote_values: Option<Vec<models::StructuredTemplateFieldsOtherQuoteValuesInner>>,
}

impl StructuredTemplateFields {
    pub fn new() -> StructuredTemplateFields {
        StructuredTemplateFields {
            equity: None,
            fixed_income_cash: None,
            mortgage: None,
            option: None,
            other: None,
            security_lending: None,
            common_fields_app_meta_data: None,
            common_fields_ask_price: None,
            common_fields_ask_size: None,
            common_fields_ask_value: None,
            common_fields_bbid: None,
            common_fields_benchmark: None,
            common_fields_benchmark_yellow_key: None,
            common_fields_bid_price: None,
            common_fields_bid_size: None,
            common_fields_bid_value: None,
            common_fields_currency: None,
            common_fields_current_face: None,
            common_fields_cusip: None,
            common_fields_date: None,
            common_fields_equity_price: None,
            common_fields_exchange: None,
            common_fields_expiration_date: None,
            common_fields_figi: None,
            common_fields_firmness: None,
            common_fields_if_natural: None,
            common_fields_leg_ratio: None,
            common_fields_maturity_date: None,
            common_fields_normalized_delta: None,
            common_fields_notional_size: None,
            common_fields_option_style: None,
            common_fields_order_type: None,
            common_fields_price: None,
            common_fields_rate: None,
            common_fields_reference_price: None,
            common_fields_security_description: None,
            common_fields_settlement_date: None,
            common_fields_share_size: None,
            common_fields_size: None,
            common_fields_text: None,
            common_fields_ticker: None,
            common_fields_version: None,
            common_fields_yellow_key: None,
            option_instrument_instrument_underlying: None,
            option_instrument_leg_side: None,
            option_instrument_leg_type: None,
            option_instrument_strike_price: None,
            other_quote_value_type: None,
            other_quote_values: None,
        }
    }
}
/// Yellowkey of benchmark security
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CommonFieldsBenchmarkYellowKey {
    #[serde(rename = "Govt")]
    Govt,
    #[serde(rename = "Corp")]
    Corp,
    #[serde(rename = "Mtge")]
    Mtge,
    #[serde(rename = "M-Mkt")]
    MMkt,
    #[serde(rename = "Muni")]
    Muni,
    #[serde(rename = "Pfd")]
    Pfd,
    #[serde(rename = "Equity")]
    Equity,
    #[serde(rename = "Comdty")]
    Comdty,
    #[serde(rename = "Index")]
    Index,
    #[serde(rename = "Curncy")]
    Curncy,
    #[serde(rename = "Client")]
    Client,
}

impl Default for CommonFieldsBenchmarkYellowKey {
    fn default() -> CommonFieldsBenchmarkYellowKey {
        Self::Govt
    }
}
/// How changeable the stated quote is
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CommonFieldsFirmness {
    #[serde(rename = "FIRM")]
    Firm,
    #[serde(rename = "INDICATIVE")]
    Indicative,
}

impl Default for CommonFieldsFirmness {
    fn default() -> CommonFieldsFirmness {
        Self::Firm
    }
}
/// An option style
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CommonFieldsOptionStyle {
    #[serde(rename = "AMERICAN")]
    American,
    #[serde(rename = "EUROPEAN")]
    European,
}

impl Default for CommonFieldsOptionStyle {
    fn default() -> CommonFieldsOptionStyle {
        Self::American
    }
}
/// The quote execution strategy
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CommonFieldsOrderType {
    #[serde(rename = "LIMIT")]
    Limit,
    #[serde(rename = "MARKET")]
    Market,
}

impl Default for CommonFieldsOrderType {
    fn default() -> CommonFieldsOrderType {
        Self::Limit
    }
}
/// Yellow key of security
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CommonFieldsYellowKey {
    #[serde(rename = "Govt")]
    Govt,
    #[serde(rename = "Corp")]
    Corp,
    #[serde(rename = "Mtge")]
    Mtge,
    #[serde(rename = "M-Mkt")]
    MMkt,
    #[serde(rename = "Muni")]
    Muni,
    #[serde(rename = "Pfd")]
    Pfd,
    #[serde(rename = "Equity")]
    Equity,
    #[serde(rename = "Comdty")]
    Comdty,
    #[serde(rename = "Index")]
    Index,
    #[serde(rename = "Curncy")]
    Curncy,
    #[serde(rename = "Client")]
    Client,
}

impl Default for CommonFieldsYellowKey {
    fn default() -> CommonFieldsYellowKey {
        Self::Govt
    }
}
/// A leg side of the option
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OptionInstrumentLegSide {
    #[serde(rename = "BUY")]
    Buy,
    #[serde(rename = "SELL")]
    Sell,
}

impl Default for OptionInstrumentLegSide {
    fn default() -> OptionInstrumentLegSide {
        Self::Buy
    }
}
/// A leg type of the option
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OptionInstrumentLegType {
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "CALL")]
    Call,
}

impl Default for OptionInstrumentLegType {
    fn default() -> OptionInstrumentLegType {
        Self::Put
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OtherQuoteValueType {
    #[serde(rename = "PRICE")]
    Price,
}

impl Default for OtherQuoteValueType {
    fn default() -> OtherQuoteValueType {
        Self::Price
    }
}
