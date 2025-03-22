# \UpdatingIdeaDrawerApi

All URIs are relative to *https://api.bloomberg.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**patch_response_to_stream**](UpdatingIdeaDrawerApi.md#patch_response_to_stream) | **PATCH** /ib/v1/streams/{stream_id} | Patch Suggestion.



## patch_response_to_stream

> patch_response_to_stream(stream_id, patch_response_to_stream_request)
Patch Suggestion.

Update Suggestions data. This request can be used to update supported changes of Suggestion

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stream_id** | **String** | Stream Identifier | [required] |
**patch_response_to_stream_request** | [**PatchResponseToStreamRequest**](PatchResponseToStreamRequest.md) | Suggestion to be updated on the given stream. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

