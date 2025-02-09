# \ApplicationApi

All URIs are relative to *http://localhost:9696*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_applications**](ApplicationApi.md#create_applications) | **POST** /api/v1/applications | 
[**create_applications_action_by_name**](ApplicationApi.md#create_applications_action_by_name) | **POST** /api/v1/applications/action/{name} | 
[**delete_applications**](ApplicationApi.md#delete_applications) | **DELETE** /api/v1/applications/{id} | 
[**delete_applications_bulk**](ApplicationApi.md#delete_applications_bulk) | **DELETE** /api/v1/applications/bulk | 
[**get_applications_by_id**](ApplicationApi.md#get_applications_by_id) | **GET** /api/v1/applications/{id} | 
[**list_applications**](ApplicationApi.md#list_applications) | **GET** /api/v1/applications | 
[**list_applications_schema**](ApplicationApi.md#list_applications_schema) | **GET** /api/v1/applications/schema | 
[**put_applications_bulk**](ApplicationApi.md#put_applications_bulk) | **PUT** /api/v1/applications/bulk | 
[**test_applications**](ApplicationApi.md#test_applications) | **POST** /api/v1/applications/test | 
[**testall_applications**](ApplicationApi.md#testall_applications) | **POST** /api/v1/applications/testall | 
[**update_applications**](ApplicationApi.md#update_applications) | **PUT** /api/v1/applications/{id} | 



## create_applications

> models::ApplicationResource create_applications(force_save, application_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force_save** | Option<**bool**> |  |  |[default to false]
**application_resource** | Option<[**ApplicationResource**](ApplicationResource.md)> |  |  |

### Return type

[**models::ApplicationResource**](ApplicationResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_applications_action_by_name

> create_applications_action_by_name(name, application_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**application_resource** | Option<[**ApplicationResource**](ApplicationResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_applications

> delete_applications(id)


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


## delete_applications_bulk

> delete_applications_bulk(application_bulk_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_bulk_resource** | Option<[**ApplicationBulkResource**](ApplicationBulkResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_applications_by_id

> models::ApplicationResource get_applications_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**models::ApplicationResource**](ApplicationResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_applications

> Vec<models::ApplicationResource> list_applications()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ApplicationResource>**](ApplicationResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_applications_schema

> Vec<models::ApplicationResource> list_applications_schema()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ApplicationResource>**](ApplicationResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_applications_bulk

> models::ApplicationResource put_applications_bulk(application_bulk_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_bulk_resource** | Option<[**ApplicationBulkResource**](ApplicationBulkResource.md)> |  |  |

### Return type

[**models::ApplicationResource**](ApplicationResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## test_applications

> test_applications(force_test, application_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force_test** | Option<**bool**> |  |  |[default to false]
**application_resource** | Option<[**ApplicationResource**](ApplicationResource.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## testall_applications

> testall_applications()


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


## update_applications

> models::ApplicationResource update_applications(id, force_save, application_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**force_save** | Option<**bool**> |  |  |[default to false]
**application_resource** | Option<[**ApplicationResource**](ApplicationResource.md)> |  |  |

### Return type

[**models::ApplicationResource**](ApplicationResource.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

