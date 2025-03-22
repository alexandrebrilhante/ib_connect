# Suggestion

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**text** | Option<**String**> | Message 6000 characters or less in length. The text element is planned to be deprecated in favour of textElements. Cannot be used with the textElements property. | [optional]
**action** | Option<**String**> | The type of action which can be taken on this suggestion. VIEW_ONLY means the suggestion cannot be shared with anyone else; therefore hiding any sharing capabilities. ORIGINAL_ROOM allows sharing of a suggestion into the chat room which contains the triggering message. For (solicited) ideas, ORIGINAL_ROOM is the default action, if no property is explicitly provided. For unsolicited ideas, the default action is to allow sharing of a suggestion to a chat room if VIEW_ONLY is not provided. ORIGINAL_ROOM does not apply to unsolicited ideas. If ORIGINAL_ROOM is provided for unsolicited ideas, it will be ignored and the default action of unsolicited ideas will be performed instead. | [optional]
**structured_template** | Option<[**Vec<models::StructuredTemplate>**](StructuredTemplate.md)> |  | [optional]
**text_elements** | Option<[**Vec<models::IdeaTextElement>**](IdeaTextElement.md)> | An array containing text or link elements that will be concatenated in the idea. Cannot be used with the text property. | [optional]
**structured_elements** | Option<[**Vec<models::IdeaStructuredElement>**](IdeaStructuredElement.md)> | An array containing structured content elements that will be appended to the end of the message. See IdeaStructuredElement for supported types. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


