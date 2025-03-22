# \SubscribingToIbFeedsApi

All URIs are relative to *https://api.bloomberg.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_streams_list**](SubscribingToIbFeedsApi.md#get_streams_list) | **GET** /ib/v1/streams | List IB Feed stream IDs
[**read_stream**](SubscribingToIbFeedsApi.md#read_stream) | **GET** /ib/v1/streams/{stream_id} | Subscribe to an IB Feed Stream



## get_streams_list

> models::StreamList get_streams_list()
List IB Feed stream IDs

List IDs of IB feed streams the API caller has permission to access.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::StreamList**](StreamList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_stream

> models::ReadStream200Response read_stream(stream_id, send_content_events, send_idea_drawer_feedback_events, backfill_id)
Subscribe to an IB Feed Stream

Get events from an IB feed stream. For the Base Feed and Enriched Feed workflows, the 'messages' array will contain at most a single message. In the On Demand workflow, the 'messages' array may contain multiple messages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stream_id** | **String** | Stream Identifier | [required] |
**send_content_events** | Option<**bool**> | Determines if Content Events are being sent in stream. Accepts truthy and falsy values, for example \"true\" and \"false\" respectively. If a value that can't be interpreted as truthy or falsy is supplied, an HTTP response with error code 400 will be delivered. |  |[default to true]
**send_idea_drawer_feedback_events** | Option<**bool**> | Determines if Idea Drawer Feedback Events are being sent in stream. Information regarding Idea Drawer can be found on this [page](https://developer.bloomberg.com/portal/documents/terminal_connect/ib_connect_on-demand_rest#ib_connect_on_demand_rest_api-on_demand_feed-posting_to_the_idea_drawer). Accepts truthy and falsy values, for example \"true\" and \"false\" respectively. If a value that can't be interpreted as truthy or falsy is supplied, an HTTP response with error code 400 will be delivered. |  |[default to false]
**backfill_id** | Option<**String**> | Specifies the event after which streaming will resume. Every message will have a backfillId field. Use this ID to request for a backfill. If the backfill ID is older than 5 minutes an HTTP response with error code 410 will be delivered. In this case you can try reconnecting without a backfillId query parameter to receive real-time events. If the backfill ID is invalid an HTTP response with error code 400 will be delivered. |  |

### Return type

[**models::ReadStream200Response**](readStream_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

