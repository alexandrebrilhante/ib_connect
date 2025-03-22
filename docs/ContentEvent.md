# ContentEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**at_type** | **String** |  | 
**room_id** | **String** |  | 
**room_name** | Option<**String**> | Name of the IB Chatroom. It is only provided for persistent chat rooms. | [optional]
**event_id** | **String** |  | 
**backfill_id** | Option<**String**> | Can be used to resume streaming from after this event when a disconnection happens | [optional]
**requester** | Option<[**models::User**](User.md)> |  | [optional]
**participants** | [**Vec<models::User>**](User.md) |  | 
**trigger** | [**models::Trigger**](Trigger.md) |  | 
**enrichments** | Option<[**Vec<models::Enrichment>**](Enrichment.md)> |  | [optional]
**messages** | [**Vec<models::Message>**](Message.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


