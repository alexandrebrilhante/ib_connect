# \PostingToIdeaDrawerApi

All URIs are relative to *https://api.bloomberg.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**post_response_to_stream**](PostingToIdeaDrawerApi.md#post_response_to_stream) | **POST** /ib/v1/streams/{stream_id} | Post Suggestion



## post_response_to_stream

> models::PostResponseToStream200Response post_response_to_stream(stream_id, post_response)
Post Suggestion

Posts Suggestions. Additional enablement may be required for one or multiple response types.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stream_id** | **String** | Stream Identifier | [required] |
**post_response** | [**PostResponse**](PostResponse.md) | The message to be posted to the given stream. | [required] |

### Return type

[**models::PostResponseToStream200Response**](postResponseToStream_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

