# \DevelopmentConfigApi

All URIs are relative to *http://localhost:9696*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_development_config**](DevelopmentConfigApi.md#get_development_config) | **GET** /api/v1/config/development | 
[**get_development_config_by_id**](DevelopmentConfigApi.md#get_development_config_by_id) | **GET** /api/v1/config/development/{id} | 
[**update_development_config**](DevelopmentConfigApi.md#update_development_config) | **PUT** /api/v1/config/development/{id} | 



## get_development_config

> models::DevelopmentConfigResource get_development_config()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::DevelopmentConfigResource**](DevelopmentConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_development_config_by_id

> models::DevelopmentConfigResource get_development_config_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::DevelopmentConfigResource**](DevelopmentConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_development_config

> models::DevelopmentConfigResource update_development_config(id, development_config_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**development_config_resource** | Option<[**DevelopmentConfigResource**](DevelopmentConfigResource.md)> |  |  |

### Return type

[**models::DevelopmentConfigResource**](DevelopmentConfigResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

