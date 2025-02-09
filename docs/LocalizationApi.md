# \LocalizationApi

All URIs are relative to *http://localhost:9696*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_localization**](LocalizationApi.md#get_localization) | **GET** /api/v1/localization | 
[**list_localization_options**](LocalizationApi.md#list_localization_options) | **GET** /api/v1/localization/options | 



## get_localization

> get_localization()


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


## list_localization_options

> Vec<models::LocalizationOption> list_localization_options()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::LocalizationOption>**](LocalizationOption.md)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

