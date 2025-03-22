# Message

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**message** | Option<**String**> |  | [optional]
**message_elements** | Option<[**Vec<models::FeedMessageElement>**](FeedMessageElement.md)> |  | [optional]
**timestamp** | **String** |  | 
**sender** | [**models::User**](User.md) |  | 
**blast_info** | Option<[**models::BlastInfo**](BlastInfo.md)> |  | [optional]
**ib_post_id** | Option<**String**> | Unique identifier of the IB post. For Base Feed or Enriched Feed, this value will be the same as eventId. For On Demand Feed each IB post included in the event has a separate post Id. | [optional]
**micro_timestamp** | Option<**i64**> | Timestamp in microseconds. | [optional]
**reply_to** | Option<[**models::ReplyTo**](ReplyTo.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


