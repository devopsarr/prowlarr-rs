# \HistoryApi

All URIs are relative to *http://localhost:9696*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_history**](HistoryApi.md#get_history) | **GET** /api/v1/history | 
[**list_history_indexer**](HistoryApi.md#list_history_indexer) | **GET** /api/v1/history/indexer | 
[**list_history_since**](HistoryApi.md#list_history_since) | **GET** /api/v1/history/since | 



## get_history

> models::HistoryResourcePagingResource get_history(page, page_size, sort_key, sort_direction, event_type, successful, download_id, indexer_ids)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> |  |  |[default to 1]
**page_size** | Option<**i32**> |  |  |[default to 10]
**sort_key** | Option<**String**> |  |  |
**sort_direction** | Option<[**SortDirection**](.md)> |  |  |
**event_type** | Option<[**Vec<i32>**](i32.md)> |  |  |
**successful** | Option<**bool**> |  |  |
**download_id** | Option<**String**> |  |  |
**indexer_ids** | Option<[**Vec<i32>**](i32.md)> |  |  |

### Return type

[**models::HistoryResourcePagingResource**](HistoryResourcePagingResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_history_indexer

> Vec<models::HistoryResource> list_history_indexer(indexer_id, event_type, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**indexer_id** | Option<**i32**> |  |  |
**event_type** | Option<[**HistoryEventType**](.md)> |  |  |
**limit** | Option<**i32**> |  |  |

### Return type

[**Vec<models::HistoryResource>**](HistoryResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_history_since

> Vec<models::HistoryResource> list_history_since(date, event_type)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date** | Option<**String**> |  |  |
**event_type** | Option<[**HistoryEventType**](.md)> |  |  |

### Return type

[**Vec<models::HistoryResource>**](HistoryResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

