# \DefaultApi

All URIs are relative to *https://virtserver.swaggerhub.com/brainkod/htsget/1.1.1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**search_read_id**](DefaultApi.md#search_read_id) | **Get** /reads/{id} | Gets the reads from a pre-indexed id
[**search_variant_id**](DefaultApi.md#search_variant_id) | **Get** /variants/{id} | Gets the variants from a pre-indexed id


# **search_read_id**
> ::models::HtsgetResponse search_read_id(ctx, id, optional)
Gets the reads from a pre-indexed id

Searches a pre-indexed object id. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**| identifier of the read object | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**| identifier of the read object | 
 **format** | **String**| File format, BAM (default), CRAM. | 
 **reference_name** | **String**| VReference sequence name | 
 **start** | **i64**| The start position of the range on the reference, 0-based, inclusive. | 
 **end** | **i64**| The end position of the range on the reference, 0-based exclusive. | 
 **fields** | **String**| A list of fields to include, such as QNAME, FLAG, RNAME, etc... | 
 **tags** | **String**| A comma separated list of tags to include, by default all. | 
 **notags** | **String**| A comma separated list of tags to exclude, default none. | 

### Return type

[**::models::HtsgetResponse**](htsgetResponse.md)

### Authorization

[htsget_auth](../README.md#htsget_auth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **search_variant_id**
> ::models::HtsgetResponse search_variant_id(ctx, id, optional)
Gets the variants from a pre-indexed id

Searches a pre-indexed object id. 

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**| identifier of the variant object | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**| identifier of the variant object | 
 **format** | **String**| File format, VCF (default), BCF. | 
 **reference_name** | **String**| VReference sequence name | 
 **start** | **i64**| The start position of the range on the reference, 0-based, inclusive. | 
 **end** | **i64**| The end position of the range on the reference, 0-based exclusive. | 
 **fields** | **String**| A list of variant fields to include, such as INFO, SAMPLE, FORMAT, etc. | 
 **tags** | **String**| A comma separated list of tags to include, by default all. | 
 **notags** | **String**| A comma separated list of tags to exclude, default none. | 

### Return type

[**::models::HtsgetResponse**](htsgetResponse.md)

### Authorization

[htsget_auth](../README.md#htsget_auth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

