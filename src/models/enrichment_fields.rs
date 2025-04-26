use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnrichmentFields {
    #[serde(rename = "credit_derivative", skip_serializing_if = "Option::is_none")]
    pub credit_derivative: Option<Box<models::EnrichmentFieldsCreditDerivative>>,
    #[serde(rename = "entity_org", skip_serializing_if = "Option::is_none")]
    pub entity_org: Option<Box<models::EnrichmentFieldsEntityOrg>>,
    #[serde(rename = "entity_person", skip_serializing_if = "Option::is_none")]
    pub entity_person: Option<Box<models::EnrichmentFieldsEntityPerson>>,
    #[serde(rename = "equity", skip_serializing_if = "Option::is_none")]
    pub equity: Option<Box<models::EnrichmentFieldsEquity>>,
    #[serde(rename = "fixed_income_cash", skip_serializing_if = "Option::is_none")]
    pub fixed_income_cash: Option<Box<models::EnrichmentFieldsFixedIncomeCash>>,
    #[serde(rename = "fx", skip_serializing_if = "Option::is_none")]
    pub fx: Option<Box<models::EnrichmentFieldsFx>>,
    #[serde(rename = "fx_cash", skip_serializing_if = "Option::is_none")]
    pub fx_cash: Option<Box<models::EnrichmentFieldsFxCash>>,
    #[serde(rename = "fx_derivative", skip_serializing_if = "Option::is_none")]
    pub fx_derivative: Option<Box<models::EnrichmentFieldsFxDerivative>>,
    #[serde(rename = "intent", skip_serializing_if = "Option::is_none")]
    pub intent: Option<Box<models::EnrichmentFieldsIntent>>,
    #[serde(rename = "mortgage", skip_serializing_if = "Option::is_none")]
    pub mortgage: Option<Box<models::EnrichmentFieldsMortgage>>,
    #[serde(rename = "option", skip_serializing_if = "Option::is_none")]
    pub option: Option<Box<models::EnrichmentFieldsOption>>,
    #[serde(rename = "other", skip_serializing_if = "Option::is_none")]
    pub other: Option<Box<models::EnrichmentFieldsOther>>,
    #[serde(rename = "repo", skip_serializing_if = "Option::is_none")]
    pub repo: Option<Box<models::EnrichmentFieldsRepo>>,
    #[serde(rename = "security_lending", skip_serializing_if = "Option::is_none")]
    pub security_lending: Option<Box<models::EnrichmentFieldsSecurityLending>>,
    /// Quantity
    #[serde(
        rename = "common_fields_amount",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_amount: Option<f64>,
    /// Currency of amount being traded
    #[serde(
        rename = "common_fields_amount_currency",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_amount_currency: Option<String>,
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
    /// Brand name for credit default swap index
    #[serde(
        rename = "common_fields_cdx_group",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_cdx_group: Option<String>,
    /// Bloomberg Company Identifier
    #[serde(
        rename = "common_fields_collateral",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_collateral: Option<String>,
    /// Confidence level
    #[serde(
        rename = "common_fields_confidence_type",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_confidence_type: Option<CommonFieldsConfidenceType>,
    /// Coupon
    #[serde(
        rename = "common_fields_coupon",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_coupon: Option<f64>,
    /// Three letter Currency identifier
    #[serde(
        rename = "common_fields_currency",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_currency: Option<String>,
    /// Currency pair
    #[serde(
        rename = "common_fields_currency_pair",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_currency_pair: Option<String>,
    /// Current outstanding value
    #[serde(
        rename = "common_fields_current_face",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_current_face: Option<f64>,
    #[serde(rename = "common_fields_date", skip_serializing_if = "Option::is_none")]
    pub common_fields_date: Option<Box<models::StructuredTemplateFieldsCommonFieldsDate>>,
    /// Option delta
    #[serde(
        rename = "common_fields_delta",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_delta: Option<f64>,
    #[serde(
        rename = "common_fields_delta_hedge_notional",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_delta_hedge_notional:
        Option<Box<models::EnrichmentFieldsCommonFieldsDeltaHedgeNotional>>,
    /// Type of FX derivative option
    #[serde(
        rename = "common_fields_derivative_type",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_derivative_type: Option<CommonFieldsDerivativeType>,
    /// Direction of quote
    #[serde(
        rename = "common_fields_direction",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_direction: Option<CommonFieldsDirection>,
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
    /// Option expiry cut-off
    #[serde(
        rename = "common_fields_expiration_cut",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_expiration_cut: Option<String>,
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
    /// True if the repo does not have a fixed maturity date
    #[serde(
        rename = "common_fields_is_open",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_is_open: Option<bool>,
    /// True if the repo trade is carried over from a prior trade
    #[serde(
        rename = "common_fields_is_roll",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_is_roll: Option<bool>,
    /// Free-form string to identify leg information in various sections
    #[serde(
        rename = "common_fields_leg_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_leg_id: Option<String>,
    /// Ratio of an option leg
    #[serde(
        rename = "common_fields_leg_ratio",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_leg_ratio: Option<f64>,
    /// Company id of loan
    #[serde(
        rename = "common_fields_loan_company_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_loan_company_id: Option<String>,
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
    #[serde(
        rename = "common_fields_notional",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_notional: Option<Box<models::EnrichmentFieldsCommonFieldsNotional>>,
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
    /// Premium for security over the base price
    #[serde(
        rename = "common_fields_payup",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_payup: Option<f64>,
    /// Quoted points for the leg
    #[serde(
        rename = "common_fields_points",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_points: Option<f64>,
    /// Currently supported intent names
    #[serde(
        rename = "common_fields_post_intent_name",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_post_intent_name: Option<CommonFieldsPostIntentName>,
    #[serde(
        rename = "common_fields_premium",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_premium: Option<Box<models::EnrichmentFieldsCommonFieldsPremium>>,
    #[serde(
        rename = "common_fields_premium_type",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_premium_type: Option<CommonFieldsPremiumType>,
    /// Rate at which loan can be prepaid
    #[serde(
        rename = "common_fields_prepay",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_prepay: Option<String>,
    /// The price of a security
    #[serde(
        rename = "common_fields_price",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_price: Option<f64>,
    /// Put call code of the option
    #[serde(
        rename = "common_fields_put_call_code",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_put_call_code: Option<CommonFieldsPutCallCode>,
    /// Rate
    #[serde(rename = "common_fields_rate", skip_serializing_if = "Option::is_none")]
    pub common_fields_rate: Option<f64>,
    /// Reference price of underlying asset
    #[serde(
        rename = "common_fields_reference_price",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_reference_price: Option<f64>,
    /// Direction of the repo offer
    #[serde(
        rename = "common_fields_repo_side",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_repo_side: Option<CommonFieldsRepoSide>,
    /// Series rolling from (e.g. for CDX rolls)
    #[serde(
        rename = "common_fields_roll_from_series",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_roll_from_series: Option<f64>,
    /// Sector of security.  Currently one of: HY (high yield) or IG (investment grade)
    #[serde(
        rename = "common_fields_sector",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_sector: Option<String>,
    /// Text describing the security, ex: THC 6.25 2018, AMNEAL PHARMA TL B
    #[serde(
        rename = "common_fields_security_description",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_security_description: Option<String>,
    /// Series of security. e.g. 'REGS', '144A', '22', etc.
    #[serde(
        rename = "common_fields_series",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_series: Option<String>,
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
    /// Side of the quote or security mention
    #[serde(rename = "common_fields_side", skip_serializing_if = "Option::is_none")]
    pub common_fields_side: Option<CommonFieldsSide>,
    /// The size of a deal
    #[serde(rename = "common_fields_size", skip_serializing_if = "Option::is_none")]
    pub common_fields_size: Option<f64>,
    /// Strike price of option
    #[serde(
        rename = "common_fields_strike_price",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_strike_price: Option<String>,
    /// Tenor associated with security.
    #[serde(
        rename = "common_fields_tenor",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_tenor: Option<String>,
    /// Free-form text for specifying additional details in SPW negotiations
    #[serde(rename = "common_fields_text", skip_serializing_if = "Option::is_none")]
    pub common_fields_text: Option<String>,
    /// Ticker associated with the security
    #[serde(
        rename = "common_fields_ticker",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_ticker: Option<String>,
    #[serde(
        rename = "common_fields_underlying",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_underlying: Option<Box<models::EnrichmentFieldsCommonFieldsUnderlying>>,
    /// UUID
    #[serde(rename = "common_fields_uuid", skip_serializing_if = "Option::is_none")]
    pub common_fields_uuid: Option<f64>,
    #[serde(
        rename = "common_fields_volatility",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_volatility: Option<Box<models::EnrichmentFieldsCommonFieldsVolatility>>,
    /// Year definition of the security (e.g. CDS)
    #[serde(
        rename = "common_fields_year_definition",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_year_definition: Option<f64>,
    /// Yellow key of security
    #[serde(
        rename = "common_fields_yellow_key",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_fields_yellow_key: Option<CommonFieldsYellowKey>,
    #[serde(rename = "common_groups_note", skip_serializing_if = "Option::is_none")]
    pub common_groups_note: Option<Box<models::EnrichmentFieldsCommonGroupsNote>>,
    #[serde(rename = "common_groups_size", skip_serializing_if = "Option::is_none")]
    pub common_groups_size: Option<Box<models::EnrichmentFieldsCommonGroupsSize>>,
    #[serde(
        rename = "credit_derivative_quote_quote",
        skip_serializing_if = "Option::is_none"
    )]
    pub credit_derivative_quote_quote:
        Option<Box<models::EnrichmentFieldsCreditDerivativeQuoteQuote>>,
    #[serde(
        rename = "credit_derivative_quote_value_type",
        skip_serializing_if = "Option::is_none"
    )]
    pub credit_derivative_quote_value_type: Option<CreditDerivativeQuoteValueType>,
    #[serde(
        rename = "credit_derivative_quote_values",
        skip_serializing_if = "Option::is_none"
    )]
    pub credit_derivative_quote_values:
        Option<Vec<models::EnrichmentFieldsCreditDerivativeQuoteValuesInner>>,
    #[serde(
        rename = "entity_person_instrument_instrument",
        skip_serializing_if = "Option::is_none"
    )]
    pub entity_person_instrument_instrument:
        Option<Box<models::EnrichmentFieldsEntityPersonInstrumentInstrument>>,
    #[serde(rename = "equity_quote_quote", skip_serializing_if = "Option::is_none")]
    pub equity_quote_quote: Option<Box<models::EnrichmentFieldsEquityQuoteQuote>>,
    #[serde(
        rename = "fixed_income_cash_quote_value_type",
        skip_serializing_if = "Option::is_none"
    )]
    pub fixed_income_cash_quote_value_type: Option<FixedIncomeCashQuoteValueType>,
    /// An array of different quote value types like price, yield, spread, discount margin
    #[serde(
        rename = "fixed_income_cash_quote_values",
        skip_serializing_if = "Option::is_none"
    )]
    pub fixed_income_cash_quote_values:
        Option<Vec<models::EnrichmentFieldsFixedIncomeCashQuoteValuesInner>>,
    #[serde(
        rename = "fx_cash_instrument_instrument",
        skip_serializing_if = "Option::is_none"
    )]
    pub fx_cash_instrument_instrument:
        Option<Box<models::EnrichmentFieldsFxCashInstrumentInstrument>>,
    #[serde(
        rename = "fx_cash_instrument_leg",
        skip_serializing_if = "Option::is_none"
    )]
    pub fx_cash_instrument_leg: Option<Box<models::EnrichmentFieldsFxCashInstrumentLeg>>,
    #[serde(
        rename = "fx_cash_quote_quote",
        skip_serializing_if = "Option::is_none"
    )]
    pub fx_cash_quote_quote: Option<Box<models::EnrichmentFieldsFxCashQuoteQuote>>,
    #[serde(rename = "fx_cash_size_size", skip_serializing_if = "Option::is_none")]
    pub fx_cash_size_size: Option<Box<models::EnrichmentFieldsFxCashSizeSize>>,
    #[serde(
        rename = "fx_derivative_instrument_instrument",
        skip_serializing_if = "Option::is_none"
    )]
    pub fx_derivative_instrument_instrument:
        Option<Box<models::EnrichmentFieldsFxDerivativeInstrumentInstrument>>,
    #[serde(
        rename = "fx_derivative_instrument_leg",
        skip_serializing_if = "Option::is_none"
    )]
    pub fx_derivative_instrument_leg:
        Option<Box<models::EnrichmentFieldsFxDerivativeInstrumentLeg>>,
    #[serde(
        rename = "fx_derivative_instrument_legs",
        skip_serializing_if = "Option::is_none"
    )]
    pub fx_derivative_instrument_legs: Option<Vec<models::FxDerivativeInstrumentLeg>>,
    #[serde(
        rename = "intent_intent_post_intent",
        skip_serializing_if = "Option::is_none"
    )]
    pub intent_intent_post_intent: Option<Box<models::EnrichmentFieldsIntentIntentPostIntent>>,
    #[serde(
        rename = "mortgage_quote_value_type",
        skip_serializing_if = "Option::is_none"
    )]
    pub mortgage_quote_value_type: Option<MortgageQuoteValueType>,
    #[serde(
        rename = "mortgage_quote_values",
        skip_serializing_if = "Option::is_none"
    )]
    pub mortgage_quote_values: Option<Vec<models::EnrichmentFieldsMortgageQuoteValuesInner>>,
    #[serde(
        rename = "option_instrument_instrument_underlying",
        skip_serializing_if = "Option::is_none"
    )]
    pub option_instrument_instrument_underlying:
        Option<Box<models::EnrichmentFieldsOptionInstrumentInstrumentUnderlying>>,
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
        rename = "other_instrument_instrument",
        skip_serializing_if = "Option::is_none"
    )]
    pub other_instrument_instrument: Option<Box<models::EnrichmentFieldsOtherInstrumentInstrument>>,
    #[serde(rename = "other_side_side", skip_serializing_if = "Option::is_none")]
    pub other_side_side: Option<Box<models::EnrichmentFieldsOtherSideSide>>,
    #[serde(rename = "repo_size_size", skip_serializing_if = "Option::is_none")]
    pub repo_size_size: Option<Box<models::EnrichmentFieldsRepoSizeSize>>,
}

impl EnrichmentFields {
    pub fn new() -> EnrichmentFields {
        EnrichmentFields {
            credit_derivative: None,
            entity_org: None,
            entity_person: None,
            equity: None,
            fixed_income_cash: None,
            fx: None,
            fx_cash: None,
            fx_derivative: None,
            intent: None,
            mortgage: None,
            option: None,
            other: None,
            repo: None,
            security_lending: None,
            common_fields_amount: None,
            common_fields_amount_currency: None,
            common_fields_ask_price: None,
            common_fields_ask_size: None,
            common_fields_ask_value: None,
            common_fields_bbid: None,
            common_fields_benchmark: None,
            common_fields_benchmark_yellow_key: None,
            common_fields_bid_price: None,
            common_fields_bid_size: None,
            common_fields_bid_value: None,
            common_fields_cdx_group: None,
            common_fields_collateral: None,
            common_fields_confidence_type: None,
            common_fields_coupon: None,
            common_fields_currency: None,
            common_fields_currency_pair: None,
            common_fields_current_face: None,
            common_fields_date: None,
            common_fields_delta: None,
            common_fields_delta_hedge_notional: None,
            common_fields_derivative_type: None,
            common_fields_direction: None,
            common_fields_equity_price: None,
            common_fields_exchange: None,
            common_fields_expiration_cut: None,
            common_fields_expiration_date: None,
            common_fields_figi: None,
            common_fields_firmness: None,
            common_fields_if_natural: None,
            common_fields_is_open: None,
            common_fields_is_roll: None,
            common_fields_leg_id: None,
            common_fields_leg_ratio: None,
            common_fields_loan_company_id: None,
            common_fields_maturity_date: None,
            common_fields_normalized_delta: None,
            common_fields_notional: None,
            common_fields_notional_size: None,
            common_fields_option_style: None,
            common_fields_order_type: None,
            common_fields_payup: None,
            common_fields_points: None,
            common_fields_post_intent_name: None,
            common_fields_premium: None,
            common_fields_premium_type: None,
            common_fields_prepay: None,
            common_fields_price: None,
            common_fields_put_call_code: None,
            common_fields_rate: None,
            common_fields_reference_price: None,
            common_fields_repo_side: None,
            common_fields_roll_from_series: None,
            common_fields_sector: None,
            common_fields_security_description: None,
            common_fields_series: None,
            common_fields_settlement_date: None,
            common_fields_share_size: None,
            common_fields_side: None,
            common_fields_size: None,
            common_fields_strike_price: None,
            common_fields_tenor: None,
            common_fields_text: None,
            common_fields_ticker: None,
            common_fields_underlying: None,
            common_fields_uuid: None,
            common_fields_volatility: None,
            common_fields_year_definition: None,
            common_fields_yellow_key: None,
            common_groups_note: None,
            common_groups_size: None,
            credit_derivative_quote_quote: None,
            credit_derivative_quote_value_type: None,
            credit_derivative_quote_values: None,
            entity_person_instrument_instrument: None,
            equity_quote_quote: None,
            fixed_income_cash_quote_value_type: None,
            fixed_income_cash_quote_values: None,
            fx_cash_instrument_instrument: None,
            fx_cash_instrument_leg: None,
            fx_cash_quote_quote: None,
            fx_cash_size_size: None,
            fx_derivative_instrument_instrument: None,
            fx_derivative_instrument_leg: None,
            fx_derivative_instrument_legs: None,
            intent_intent_post_intent: None,
            mortgage_quote_value_type: None,
            mortgage_quote_values: None,
            option_instrument_instrument_underlying: None,
            option_instrument_leg_side: None,
            option_instrument_leg_type: None,
            option_instrument_strike_price: None,
            other_instrument_instrument: None,
            other_side_side: None,
            repo_size_size: None,
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
/// Confidence level
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CommonFieldsConfidenceType {
    #[serde(rename = "USER")]
    User,
    #[serde(rename = "MACHINE")]
    Machine,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl Default for CommonFieldsConfidenceType {
    fn default() -> CommonFieldsConfidenceType {
        Self::User
    }
}
/// Type of FX derivative option
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CommonFieldsDerivativeType {
    #[serde(rename = "VANILLA")]
    Vanilla,
}

impl Default for CommonFieldsDerivativeType {
    fn default() -> CommonFieldsDerivativeType {
        Self::Vanilla
    }
}
/// Direction of quote
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CommonFieldsDirection {
    #[serde(rename = "BORROW")]
    Borrow,
    #[serde(rename = "BUY")]
    Buy,
    #[serde(rename = "DEPOSIT")]
    Deposit,
    #[serde(rename = "SELL")]
    Sell,
}

impl Default for CommonFieldsDirection {
    fn default() -> CommonFieldsDirection {
        Self::Borrow
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
/// Currently supported intent names
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CommonFieldsPostIntentName {
    #[serde(rename = "Axe")]
    Axe,
    #[serde(rename = "Inquiry")]
    Inquiry,
}

impl Default for CommonFieldsPostIntentName {
    fn default() -> CommonFieldsPostIntentName {
        Self::Axe
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CommonFieldsPremiumType {
    #[serde(rename = "PERCENT")]
    Percent,
    #[serde(rename = "PIPS")]
    Pips,
    #[serde(rename = "AMOUNT")]
    Amount,
}

impl Default for CommonFieldsPremiumType {
    fn default() -> CommonFieldsPremiumType {
        Self::Percent
    }
}
/// Put call code of the option
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CommonFieldsPutCallCode {
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "CALL")]
    Call,
}

impl Default for CommonFieldsPutCallCode {
    fn default() -> CommonFieldsPutCallCode {
        Self::Put
    }
}
/// Direction of the repo offer
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CommonFieldsRepoSide {
    #[serde(rename = "REPO")]
    Repo,
    #[serde(rename = "REVERSE_REPO")]
    ReverseRepo,
    #[serde(rename = "TWO-SIDED")]
    TwoSided,
}

impl Default for CommonFieldsRepoSide {
    fn default() -> CommonFieldsRepoSide {
        Self::Repo
    }
}
/// Side of the quote or security mention
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CommonFieldsSide {
    #[serde(rename = "LEND")]
    Lend,
    #[serde(rename = "BORROW")]
    Borrow,
    #[serde(rename = "BUY")]
    Buy,
    #[serde(rename = "SELL")]
    Sell,
    #[serde(rename = "TWO-SIDED")]
    TwoSided,
    #[serde(rename = "SELL-SHORT")]
    SellShort,
    #[serde(rename = "BUY-TO-COVER")]
    BuyToCover,
}

impl Default for CommonFieldsSide {
    fn default() -> CommonFieldsSide {
        Self::Lend
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
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CreditDerivativeQuoteValueType {
    #[serde(rename = "PRICE")]
    Price,
    #[serde(rename = "UPFRONT")]
    Upfront,
    #[serde(rename = "SPREAD")]
    Spread,
}

impl Default for CreditDerivativeQuoteValueType {
    fn default() -> CreditDerivativeQuoteValueType {
        Self::Price
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FixedIncomeCashQuoteValueType {
    #[serde(rename = "PRICE")]
    Price,
    #[serde(rename = "YIELD")]
    Yield,
    #[serde(rename = "DISCOUNT")]
    Discount,
    #[serde(rename = "DISCOUNT_MARGIN")]
    DiscountMargin,
    #[serde(rename = "SPREAD_TO_BENCHMARK")]
    SpreadToBenchmark,
    #[serde(rename = "G_SPREAD")]
    GSpread,
    #[serde(rename = "I_SPREAD")]
    ISpread,
    #[serde(rename = "Z_SPREAD")]
    ZSpread,
    #[serde(rename = "ASW_SPREAD")]
    AswSpread,
    #[serde(rename = "OAS_SPREAD")]
    OasSpread,
    #[serde(rename = "CDS_BASIS")]
    CdsBasis,
}

impl Default for FixedIncomeCashQuoteValueType {
    fn default() -> FixedIncomeCashQuoteValueType {
        Self::Price
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MortgageQuoteValueType {
    #[serde(rename = "PAYUP")]
    Payup,
    #[serde(rename = "PRICE")]
    Price,
    #[serde(rename = "YIELD")]
    Yield,
    #[serde(rename = "DISCOUNT_MARGIN")]
    DiscountMargin,
    #[serde(rename = "A_SPREAD")]
    ASpread,
    #[serde(rename = "E_SPREAD")]
    ESpread,
    #[serde(rename = "I_SPREAD")]
    ISpread,
    #[serde(rename = "J_SPREAD")]
    JSpread,
    #[serde(rename = "N_SPREAD")]
    NSpread,
    #[serde(rename = "S_SPREAD")]
    SSpread,
    #[serde(rename = "U_SPREAD")]
    USpread,
    #[serde(rename = "Z_SPREAD")]
    ZSpread,
}

impl Default for MortgageQuoteValueType {
    fn default() -> MortgageQuoteValueType {
        Self::Payup
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
