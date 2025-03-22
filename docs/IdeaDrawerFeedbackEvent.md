# IdeaDrawerFeedbackEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**at_type** | **String** |  | 
**suggestion_id** | **String** | A unique identifier for a suggestion | 
**destinations** | Option<[**Vec<models::IdeaDrawerFeedbackEventDestinationsInner>**](IdeaDrawerFeedbackEvent_destinations_inner.md)> | The room ids that the suggestion was shared with | [optional]
**sender_id** | Option<[**models::UserId**](UserId.md)> |  | [optional]
**action** | Option<**String**> | Indicates whether the suggestion was sent as is or edited | [optional]
**backfill_id** | Option<**String**> | Can be used to resume streaming from after this event when a disconnection happens | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


