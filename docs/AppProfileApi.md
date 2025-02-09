# \AppProfileApi

All URIs are relative to *http://localhost:9696*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_app_profile**](AppProfileApi.md#create_app_profile) | **POST** /api/v1/appprofile | 
[**delete_app_profile**](AppProfileApi.md#delete_app_profile) | **DELETE** /api/v1/appprofile/{id} | 
[**get_app_profile_by_id**](AppProfileApi.md#get_app_profile_by_id) | **GET** /api/v1/appprofile/{id} | 
[**get_app_profile_schema**](AppProfileApi.md#get_app_profile_schema) | **GET** /api/v1/appprofile/schema | 
[**list_app_profile**](AppProfileApi.md#list_app_profile) | **GET** /api/v1/appprofile | 
[**update_app_profile**](AppProfileApi.md#update_app_profile) | **PUT** /api/v1/appprofile/{id} | 



## create_app_profile

> models::AppProfileResource create_app_profile(app_profile_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_profile_resource** | Option<[**AppProfileResource**](AppProfileResource.md)> |  |  |

### Return type

[**models::AppProfileResource**](AppProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_app_profile

> delete_app_profile(id)


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


## get_app_profile_by_id

> models::AppProfileResource get_app_profile_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::AppProfileResource**](AppProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_app_profile_schema

> models::AppProfileResource get_app_profile_schema()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AppProfileResource**](AppProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_app_profile

> Vec<models::AppProfileResource> list_app_profile()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::AppProfileResource>**](AppProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_app_profile

> models::AppProfileResource update_app_profile(id, app_profile_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**app_profile_resource** | Option<[**AppProfileResource**](AppProfileResource.md)> |  |  |

### Return type

[**models::AppProfileResource**](AppProfileResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

