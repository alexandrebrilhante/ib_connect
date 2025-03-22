# \InitiatingChatsInIbApi

All URIs are relative to *https://api.bloomberg.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**post_initiate_chat**](InitiatingChatsInIbApi.md#post_initiate_chat) | **POST** /ib/v1/initiateChat | Post a chat message to one or more recipients in IB.



## post_initiate_chat

> models::PostInitiateChat202Response post_initiate_chat(chat_initiation_request)
Post a chat message to one or more recipients in IB.

Post a chat message to one or more recipients in IB where a recipient can be a user or an existing chat room.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chat_initiation_request** | [**ChatInitiationRequest**](ChatInitiationRequest.md) |  | [required] |

### Return type

[**models::PostInitiateChat202Response**](postInitiateChat_202_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

