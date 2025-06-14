# StructuredTemplateFields

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**equity** | Option<[**models::StructuredTemplateFieldsEquity**](structured_template_fields_equity.md)> |  | [optional]
**fixed_income_cash** | Option<[**models::StructuredTemplateFieldsFixedIncomeCash**](structured_template_fields_fixed_income_cash.md)> |  | [optional]
**mortgage** | Option<[**models::StructuredTemplateFieldsMortgage**](structured_template_fields_mortgage.md)> |  | [optional]
**option** | Option<[**models::StructuredTemplateFieldsOption**](structured_template_fields_option.md)> |  | [optional]
**other** | Option<[**models::StructuredTemplateFieldsOther**](structured_template_fields_other.md)> |  | [optional]
**security_lending** | Option<[**models::StructuredTemplateFieldsSecurityLending**](structured_template_fields_security_lending.md)> |  | [optional]
**common_fields_app_meta_data** | Option<**String**> | Application meta-data | [optional]
**common_fields_ask_price** | Option<**f64**> | The price of a security | [optional]
**common_fields_ask_size** | Option<**f64**> | Ask size (in security currency) | [optional]
**common_fields_ask_value** | Option<**f64**> | Ask value of a price_type for the quote | [optional]
**common_fields_bbid** | Option<**String**> | Bloomberg Company Identifier | [optional]
**common_fields_benchmark** | Option<**String**> | FIGI of benchmark security | [optional]
**common_fields_benchmark_yellow_key** | Option<**String**> | Yellowkey of benchmark security | [optional]
**common_fields_bid_price** | Option<**f64**> | Bid price | [optional]
**common_fields_bid_size** | Option<**f64**> | Bid size (in security currency) | [optional]
**common_fields_bid_value** | Option<**f64**> | Bid value of a price_type for the quote | [optional]
**common_fields_currency** | Option<**String**> | Three letter Currency identifier | [optional]
**common_fields_current_face** | Option<**f64**> | Current outstanding value | [optional]
**common_fields_cusip** | Option<**String**> | Cusip | [optional]
**common_fields_date** | Option<[**models::StructuredTemplateFieldsCommonFieldsDate**](structured_template_fields_common_fields_date.md)> |  | [optional]
**common_fields_equity_price** | Option<**f64**> | Equity price associated with bond | [optional]
**common_fields_exchange** | Option<**String**> | Exchange code | [optional]
**common_fields_expiration_date** | Option<[**models::StructuredTemplateFieldsCommonFieldsExpirationDate**](structured_template_fields_common_fields_expiration_date.md)> |  | [optional]
**common_fields_figi** | Option<**String**> | Primary security FIGI identifier | [optional]
**common_fields_firmness** | Option<**String**> | How changeable the stated quote is | [optional]
**common_fields_if_natural** | Option<**bool**> | Natural Indication of Interest (see FINRA regulatory notice 11-43). | [optional]
**common_fields_leg_ratio** | Option<**f64**> | Ratio of an option leg | [optional]
**common_fields_maturity_date** | Option<[**models::StructuredTemplateFieldsCommonFieldsDate**](structured_template_fields_common_fields_date.md)> |  | [optional]
**common_fields_normalized_delta** | Option<**f64**> | Represents a percentage and should always be normalized into a value between 0.00 and 1.00 | [optional]
**common_fields_notional_size** | Option<**f64**> | The size of a deal in notional ammount | [optional]
**common_fields_option_style** | Option<**String**> | An option style | [optional]
**common_fields_order_type** | Option<**String**> | The quote execution strategy | [optional]
**common_fields_price** | Option<**f64**> | The price of a security | [optional]
**common_fields_rate** | Option<**f64**> | Rate | [optional]
**common_fields_reference_price** | Option<**f64**> | Reference price of underlying asset | [optional]
**common_fields_security_description** | Option<**String**> | Text describing the security, ex: THC 6.25 2018, AMNEAL PHARMA TL B | [optional]
**common_fields_settlement_date** | Option<**String**> | The text scraped for settlement date. | [optional]
**common_fields_share_size** | Option<**f64**> | The size of a deal in shares | [optional]
**common_fields_size** | Option<**f64**> | The size of a deal | [optional]
**common_fields_text** | Option<**String**> | Free-form text for specifying additional details in SPW negotiations | [optional]
**common_fields_ticker** | Option<**String**> | Ticker associated with the security | [optional]
**common_fields_version** | Option<**String**> | Version of the enrichment schema <major>.<minor>[.<patch>] | [optional]
**common_fields_yellow_key** | Option<**String**> | Yellow key of security | [optional]
**option_instrument_instrument_underlying** | Option<[**models::StructuredTemplateFieldsOptionInstrumentInstrumentUnderlying**](structured_template_fields_option_instrument_instrument_underlying.md)> |  | [optional]
**option_instrument_leg_side** | Option<**String**> | A leg side of the option | [optional]
**option_instrument_leg_type** | Option<**String**> | A leg type of the option | [optional]
**option_instrument_strike_price** | Option<**f64**> | Strike price of the option | [optional]
**other_quote_value_type** | Option<**String**> |  | [optional]
**other_quote_values** | Option<[**Vec<models::StructuredTemplateFieldsOtherQuoteValuesInner>**](structured_template_fields_other_quote_values_inner.md)> | An array of different quote value types | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


