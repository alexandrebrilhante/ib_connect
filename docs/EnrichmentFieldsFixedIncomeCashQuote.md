# EnrichmentFieldsFixedIncomeCashQuote

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**benchmark** | Option<**String**> | FIGI of benchmark security | [optional]
**benchmark_yellow_key** | Option<**String**> | Yellowkey of benchmark security | [optional]
**confidence_type** | Option<**String**> | Confidence level | [optional]
**firmness** | Option<**String**> | How changeable the stated quote is | [optional]
**settlement_currency** | Option<**String**> | Three letter Currency identifier | [optional]
**settlement_date** | Option<**String**> | The text scraped for settlement date. | [optional]
**underlying_equity** | Option<[**models::EnrichmentFieldsFixedIncomeCashQuoteUnderlyingEquity**](enrichment_fields_fixed_income_cash_quote_underlying_equity.md)> |  | [optional]
**values** | Option<[**Vec<models::EnrichmentFieldsFixedIncomeCashQuoteValuesInner>**](enrichment_fields_fixed_income_cash_quote_values_inner.md)> | An array of different quote value types like price, yield, spread, discount margin | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


