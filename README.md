# ib_connect

Rust client for Instant Bloomberg Connect.

## Endpoints

All URIs are relative to *https://api.bloomberg.com*.

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*HealthCheckApi* | [**get_health_check**](docs/HealthCheckApi.md#get_health_check) | **GET** /ib/v1/check | Health Check
*IbChatbotsApi* | [**get_chatbot_rooms**](docs/IbChatbotsApi.md#get_chatbot_rooms) | **GET** /ib/v1/chatbot/{chatbot_id}/rooms | Get rooms that chatbot is a member of.
*IbChatbotsApi* | [**post_chatbot_message**](docs/IbChatbotsApi.md#post_chatbot_message) | **POST** /ib/v1/chatbot | A Registered IB Chatbot Posts Unstructured Content to an IB chat Room
*InitiatingChatsInIbApi* | [**post_initiate_chat**](docs/InitiatingChatsInIbApi.md#post_initiate_chat) | **POST** /ib/v1/initiateChat | Post a chat message to one or more recipients in IB.
*PostingToIdeaDrawerApi* | [**post_response_to_stream**](docs/PostingToIdeaDrawerApi.md#post_response_to_stream) | **POST** /ib/v1/streams/{stream_id} | Post Suggestion
*SubscribingToIbFeedsApi* | [**get_streams_list**](docs/SubscribingToIbFeedsApi.md#get_streams_list) | **GET** /ib/v1/streams | List IB Feed stream IDs
*SubscribingToIbFeedsApi* | [**read_stream**](docs/SubscribingToIbFeedsApi.md#read_stream) | **GET** /ib/v1/streams/{stream_id} | Subscribe to an IB Feed Stream
*UpdatingIdeaDrawerApi* | [**patch_response_to_stream**](docs/UpdatingIdeaDrawerApi.md#patch_response_to_stream) | **PATCH** /ib/v1/streams/{stream_id} | Patch Suggestion.
