# \IndexerProxyApi

All URIs are relative to *http://localhost:9696*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_indexer_proxy**](IndexerProxyApi.md#create_indexer_proxy) | **POST** /api/v1/indexerproxy | 
[**create_indexer_proxy_action_by_name**](IndexerProxyApi.md#create_indexer_proxy_action_by_name) | **POST** /api/v1/indexerproxy/action/{name} | 
[**delete_indexer_proxy**](IndexerProxyApi.md#delete_indexer_proxy) | **DELETE** /api/v1/indexerproxy/{id} | 
[**get_indexer_proxy_by_id**](IndexerProxyApi.md#get_indexer_proxy_by_id) | **GET** /api/v1/indexerproxy/{id} | 
[**list_indexer_proxy**](IndexerProxyApi.md#list_indexer_proxy) | **GET** /api/v1/indexerproxy | 
[**list_indexer_proxy_schema**](IndexerProxyApi.md#list_indexer_proxy_schema) | **GET** /api/v1/indexerproxy/schema | 
[**test_indexer_proxy**](IndexerProxyApi.md#test_indexer_proxy) | **POST** /api/v1/indexerproxy/test | 
[**testall_indexer_proxy**](IndexerProxyApi.md#testall_indexer_proxy) | **POST** /api/v1/indexerproxy/testall | 
[**update_indexer_proxy**](IndexerProxyApi.md#update_indexer_proxy) | **PUT** /api/v1/indexerproxy/{id} | 



## create_indexer_proxy

> models::IndexerProxyResource create_indexer_proxy(force_save, indexer_proxy_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force_save** | Option<**bool**> |  |  |[default to false]
**indexer_proxy_resource** | Option<[**IndexerProxyResource**](IndexerProxyResource.md)> |  |  |

### Return type

[**models::IndexerProxyResource**](IndexerProxyResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_indexer_proxy_action_by_name

> create_indexer_proxy_action_by_name(name, indexer_proxy_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**indexer_proxy_resource** | Option<[**IndexerProxyResource**](IndexerProxyResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_indexer_proxy

> delete_indexer_proxy(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_indexer_proxy_by_id

> models::IndexerProxyResource get_indexer_proxy_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::IndexerProxyResource**](IndexerProxyResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_indexer_proxy

> Vec<models::IndexerProxyResource> list_indexer_proxy()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::IndexerProxyResource>**](IndexerProxyResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_indexer_proxy_schema

> Vec<models::IndexerProxyResource> list_indexer_proxy_schema()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::IndexerProxyResource>**](IndexerProxyResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_indexer_proxy

> test_indexer_proxy(force_test, indexer_proxy_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force_test** | Option<**bool**> |  |  |[default to false]
**indexer_proxy_resource** | Option<[**IndexerProxyResource**](IndexerProxyResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## testall_indexer_proxy

> testall_indexer_proxy()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_indexer_proxy

> models::IndexerProxyResource update_indexer_proxy(id, force_save, indexer_proxy_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**force_save** | Option<**bool**> |  |  |[default to false]
**indexer_proxy_resource** | Option<[**IndexerProxyResource**](IndexerProxyResource.md)> |  |  |

### Return type

[**models::IndexerProxyResource**](IndexerProxyResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

