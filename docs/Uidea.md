# Uidea

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**at_type** | **String** |  | 
**recipients** | [**Vec<models::RetractSuggestionRecipientsInner>**](RetractSuggestion_recipients_inner.md) | The users whose idea drawers will receive these ideas. | 
**destinations** | [**models::UideaDestinations**](UIDEA_destinations.md) |  | 
**suggestions** | [**Vec<models::Suggestion>**](Suggestion.md) | Suggestions must contain at least one of: either a text or textElements (but not both), a structuredTemplate, or structuredElements. | 
**idea_header_properties** | Option<[**models::IdeaHeaderProperties**](IdeaHeaderProperties.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


