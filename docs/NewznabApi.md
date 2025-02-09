# \NewznabApi

All URIs are relative to *http://localhost:9696*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_indexer_download**](NewznabApi.md#get_indexer_download) | **GET** /api/v1/indexer/{id}/download | 
[**get_indexer_newznab**](NewznabApi.md#get_indexer_newznab) | **GET** /api/v1/indexer/{id}/newznab | 



## get_indexer_download

> get_indexer_download(id, link, file)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**link** | Option<**String**> |  |  |
**file** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_indexer_newznab

> get_indexer_newznab(id, t, q, cat, imdbid, tmdbid, extended, limit, offset, minage, maxage, minsize, maxsize, rid, tvmazeid, traktid, tvdbid, doubanid, season, ep, album, artist, label, track, year, genre, author, title, publisher, configured, source, host, server)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**t** | Option<**String**> |  |  |
**q** | Option<**String**> |  |  |
**cat** | Option<**String**> |  |  |
**imdbid** | Option<**String**> |  |  |
**tmdbid** | Option<**i32**> |  |  |
**extended** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |
**offset** | Option<**i32**> |  |  |
**minage** | Option<**i32**> |  |  |
**maxage** | Option<**i32**> |  |  |
**minsize** | Option<**i64**> |  |  |
**maxsize** | Option<**i64**> |  |  |
**rid** | Option<**i32**> |  |  |
**tvmazeid** | Option<**i32**> |  |  |
**traktid** | Option<**i32**> |  |  |
**tvdbid** | Option<**i32**> |  |  |
**doubanid** | Option<**i32**> |  |  |
**season** | Option<**i32**> |  |  |
**ep** | Option<**String**> |  |  |
**album** | Option<**String**> |  |  |
**artist** | Option<**String**> |  |  |
**label** | Option<**String**> |  |  |
**track** | Option<**String**> |  |  |
**year** | Option<**i32**> |  |  |
**genre** | Option<**String**> |  |  |
**author** | Option<**String**> |  |  |
**title** | Option<**String**> |  |  |
**publisher** | Option<**String**> |  |  |
**configured** | Option<**String**> |  |  |
**source** | Option<**String**> |  |  |
**host** | Option<**String**> |  |  |
**server** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey), [X-Api-Key](../README.md#X-Api-Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

