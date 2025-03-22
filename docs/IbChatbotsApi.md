# \IbChatbotsApi

All URIs are relative to *https://api.bloomberg.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_chatbot_rooms**](IbChatbotsApi.md#get_chatbot_rooms) | **GET** /ib/v1/chatbot/{chatbot_id}/rooms | Get rooms that chatbot is a member of.
[**post_chatbot_message**](IbChatbotsApi.md#post_chatbot_message) | **POST** /ib/v1/chatbot | A Registered IB Chatbot Posts Unstructured Content to an IB chat Room



## get_chatbot_rooms

> models::RoomList get_chatbot_rooms(chatbot_id)
Get rooms that chatbot is a member of.

Given a chatbot ID, Returns a list of Room IDs that the chatbot is a member of.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chatbot_id** | **i32** | Chatbot Identifier | [required] |

### Return type

[**models::RoomList**](RoomList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_chatbot_message

> serde_json::Value post_chatbot_message(chatbot_post_request)
A Registered IB Chatbot Posts Unstructured Content to an IB chat Room

A Registered IB Chatbot Posts Unstructured Content to an IB chat Room.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chatbot_post_request** | [**ChatbotPostRequest**](ChatbotPostRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

