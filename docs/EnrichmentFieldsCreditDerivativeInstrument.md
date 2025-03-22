# EnrichmentFieldsCreditDerivativeInstrument

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cdx_group** | Option<**String**> | Brand name for credit default swap index | [optional]
**cdx_offer_date** | Option<[**models::StructuredTemplateFieldsSecurityLendingQuoteStartTermDate**](structured_template_fields_security_lending_quote_start_term_date.md)> |  | [optional]
**confidence_type** | Option<**String**> | Confidence level | [optional]
**coupon** | Option<**f64**> | Coupon | [optional]
**figi_underlying** | Option<**String**> | Primary security FIGI identifier | [optional]
**roll_from_series** | Option<**f64**> | Series rolling from (e.g. for CDX rolls) | [optional]
**sector** | Option<**String**> | Sector of security.  Currently one of: HY (high yield) or IG (investment grade) | [optional]
**security_description** | Option<**String**> | Text describing the security, ex: THC 6.25 2018, AMNEAL PHARMA TL B | [optional]
**series** | Option<**String**> | Series of security. e.g. 'REGS', '144A', '22', etc. | [optional]
**tenor** | Option<**String**> | Tenor associated with security. | [optional]
**ticker** | Option<**String**> | Ticker associated with the security | [optional]
**year_definition** | Option<**f64**> | Year definition of the security (e.g. CDS) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


