# Rust API client for prowlarr

Prowlarr API docs


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: v1.32.2.4987
- Package version: 0.1.0 <!--- x-release-please-version -->
- Generator version: 7.12.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `prowlarr` and add the following to `Cargo.toml` under `[dependencies]`:

```
prowlarr = { path = "./prowlarr" }
```

## Documentation for API Endpoints

All URIs are relative to *http://localhost:9696*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*ApiInfoApi* | [**get_api**](docs/ApiInfoApi.md#get_api) | **GET** /api | 
*AppProfileApi* | [**create_app_profile**](docs/AppProfileApi.md#create_app_profile) | **POST** /api/v1/appprofile | 
*AppProfileApi* | [**delete_app_profile**](docs/AppProfileApi.md#delete_app_profile) | **DELETE** /api/v1/appprofile/{id} | 
*AppProfileApi* | [**get_app_profile_by_id**](docs/AppProfileApi.md#get_app_profile_by_id) | **GET** /api/v1/appprofile/{id} | 
*AppProfileApi* | [**get_app_profile_schema**](docs/AppProfileApi.md#get_app_profile_schema) | **GET** /api/v1/appprofile/schema | 
*AppProfileApi* | [**list_app_profile**](docs/AppProfileApi.md#list_app_profile) | **GET** /api/v1/appprofile | 
*AppProfileApi* | [**update_app_profile**](docs/AppProfileApi.md#update_app_profile) | **PUT** /api/v1/appprofile/{id} | 
*ApplicationApi* | [**create_applications**](docs/ApplicationApi.md#create_applications) | **POST** /api/v1/applications | 
*ApplicationApi* | [**create_applications_action_by_name**](docs/ApplicationApi.md#create_applications_action_by_name) | **POST** /api/v1/applications/action/{name} | 
*ApplicationApi* | [**delete_applications**](docs/ApplicationApi.md#delete_applications) | **DELETE** /api/v1/applications/{id} | 
*ApplicationApi* | [**delete_applications_bulk**](docs/ApplicationApi.md#delete_applications_bulk) | **DELETE** /api/v1/applications/bulk | 
*ApplicationApi* | [**get_applications_by_id**](docs/ApplicationApi.md#get_applications_by_id) | **GET** /api/v1/applications/{id} | 
*ApplicationApi* | [**list_applications**](docs/ApplicationApi.md#list_applications) | **GET** /api/v1/applications | 
*ApplicationApi* | [**list_applications_schema**](docs/ApplicationApi.md#list_applications_schema) | **GET** /api/v1/applications/schema | 
*ApplicationApi* | [**put_applications_bulk**](docs/ApplicationApi.md#put_applications_bulk) | **PUT** /api/v1/applications/bulk | 
*ApplicationApi* | [**test_applications**](docs/ApplicationApi.md#test_applications) | **POST** /api/v1/applications/test | 
*ApplicationApi* | [**testall_applications**](docs/ApplicationApi.md#testall_applications) | **POST** /api/v1/applications/testall | 
*ApplicationApi* | [**update_applications**](docs/ApplicationApi.md#update_applications) | **PUT** /api/v1/applications/{id} | 
*AuthenticationApi* | [**create_login**](docs/AuthenticationApi.md#create_login) | **POST** /login | 
*AuthenticationApi* | [**get_logout**](docs/AuthenticationApi.md#get_logout) | **GET** /logout | 
*BackupApi* | [**create_system_backup_restore_by_id**](docs/BackupApi.md#create_system_backup_restore_by_id) | **POST** /api/v1/system/backup/restore/{id} | 
*BackupApi* | [**create_system_backup_restore_upload**](docs/BackupApi.md#create_system_backup_restore_upload) | **POST** /api/v1/system/backup/restore/upload | 
*BackupApi* | [**delete_system_backup**](docs/BackupApi.md#delete_system_backup) | **DELETE** /api/v1/system/backup/{id} | 
*BackupApi* | [**list_system_backup**](docs/BackupApi.md#list_system_backup) | **GET** /api/v1/system/backup | 
*CommandApi* | [**create_command**](docs/CommandApi.md#create_command) | **POST** /api/v1/command | 
*CommandApi* | [**delete_command**](docs/CommandApi.md#delete_command) | **DELETE** /api/v1/command/{id} | 
*CommandApi* | [**get_command_by_id**](docs/CommandApi.md#get_command_by_id) | **GET** /api/v1/command/{id} | 
*CommandApi* | [**list_command**](docs/CommandApi.md#list_command) | **GET** /api/v1/command | 
*CustomFilterApi* | [**create_custom_filter**](docs/CustomFilterApi.md#create_custom_filter) | **POST** /api/v1/customfilter | 
*CustomFilterApi* | [**delete_custom_filter**](docs/CustomFilterApi.md#delete_custom_filter) | **DELETE** /api/v1/customfilter/{id} | 
*CustomFilterApi* | [**get_custom_filter_by_id**](docs/CustomFilterApi.md#get_custom_filter_by_id) | **GET** /api/v1/customfilter/{id} | 
*CustomFilterApi* | [**list_custom_filter**](docs/CustomFilterApi.md#list_custom_filter) | **GET** /api/v1/customfilter | 
*CustomFilterApi* | [**update_custom_filter**](docs/CustomFilterApi.md#update_custom_filter) | **PUT** /api/v1/customfilter/{id} | 
*DevelopmentConfigApi* | [**get_development_config**](docs/DevelopmentConfigApi.md#get_development_config) | **GET** /api/v1/config/development | 
*DevelopmentConfigApi* | [**get_development_config_by_id**](docs/DevelopmentConfigApi.md#get_development_config_by_id) | **GET** /api/v1/config/development/{id} | 
*DevelopmentConfigApi* | [**update_development_config**](docs/DevelopmentConfigApi.md#update_development_config) | **PUT** /api/v1/config/development/{id} | 
*DownloadClientApi* | [**create_download_client**](docs/DownloadClientApi.md#create_download_client) | **POST** /api/v1/downloadclient | 
*DownloadClientApi* | [**create_download_client_action_by_name**](docs/DownloadClientApi.md#create_download_client_action_by_name) | **POST** /api/v1/downloadclient/action/{name} | 
*DownloadClientApi* | [**delete_download_client**](docs/DownloadClientApi.md#delete_download_client) | **DELETE** /api/v1/downloadclient/{id} | 
*DownloadClientApi* | [**delete_download_client_bulk**](docs/DownloadClientApi.md#delete_download_client_bulk) | **DELETE** /api/v1/downloadclient/bulk | 
*DownloadClientApi* | [**get_download_client_by_id**](docs/DownloadClientApi.md#get_download_client_by_id) | **GET** /api/v1/downloadclient/{id} | 
*DownloadClientApi* | [**list_download_client**](docs/DownloadClientApi.md#list_download_client) | **GET** /api/v1/downloadclient | 
*DownloadClientApi* | [**list_download_client_schema**](docs/DownloadClientApi.md#list_download_client_schema) | **GET** /api/v1/downloadclient/schema | 
*DownloadClientApi* | [**put_download_client_bulk**](docs/DownloadClientApi.md#put_download_client_bulk) | **PUT** /api/v1/downloadclient/bulk | 
*DownloadClientApi* | [**test_download_client**](docs/DownloadClientApi.md#test_download_client) | **POST** /api/v1/downloadclient/test | 
*DownloadClientApi* | [**testall_download_client**](docs/DownloadClientApi.md#testall_download_client) | **POST** /api/v1/downloadclient/testall | 
*DownloadClientApi* | [**update_download_client**](docs/DownloadClientApi.md#update_download_client) | **PUT** /api/v1/downloadclient/{id} | 
*DownloadClientConfigApi* | [**get_download_client_config**](docs/DownloadClientConfigApi.md#get_download_client_config) | **GET** /api/v1/config/downloadclient | 
*DownloadClientConfigApi* | [**get_download_client_config_by_id**](docs/DownloadClientConfigApi.md#get_download_client_config_by_id) | **GET** /api/v1/config/downloadclient/{id} | 
*DownloadClientConfigApi* | [**update_download_client_config**](docs/DownloadClientConfigApi.md#update_download_client_config) | **PUT** /api/v1/config/downloadclient/{id} | 
*FileSystemApi* | [**get_file_system**](docs/FileSystemApi.md#get_file_system) | **GET** /api/v1/filesystem | 
*FileSystemApi* | [**get_file_system_type**](docs/FileSystemApi.md#get_file_system_type) | **GET** /api/v1/filesystem/type | 
*HealthApi* | [**list_health**](docs/HealthApi.md#list_health) | **GET** /api/v1/health | 
*HistoryApi* | [**get_history**](docs/HistoryApi.md#get_history) | **GET** /api/v1/history | 
*HistoryApi* | [**list_history_indexer**](docs/HistoryApi.md#list_history_indexer) | **GET** /api/v1/history/indexer | 
*HistoryApi* | [**list_history_since**](docs/HistoryApi.md#list_history_since) | **GET** /api/v1/history/since | 
*HostConfigApi* | [**get_host_config**](docs/HostConfigApi.md#get_host_config) | **GET** /api/v1/config/host | 
*HostConfigApi* | [**get_host_config_by_id**](docs/HostConfigApi.md#get_host_config_by_id) | **GET** /api/v1/config/host/{id} | 
*HostConfigApi* | [**update_host_config**](docs/HostConfigApi.md#update_host_config) | **PUT** /api/v1/config/host/{id} | 
*IndexerApi* | [**create_indexer**](docs/IndexerApi.md#create_indexer) | **POST** /api/v1/indexer | 
*IndexerApi* | [**create_indexer_action_by_name**](docs/IndexerApi.md#create_indexer_action_by_name) | **POST** /api/v1/indexer/action/{name} | 
*IndexerApi* | [**delete_indexer**](docs/IndexerApi.md#delete_indexer) | **DELETE** /api/v1/indexer/{id} | 
*IndexerApi* | [**delete_indexer_bulk**](docs/IndexerApi.md#delete_indexer_bulk) | **DELETE** /api/v1/indexer/bulk | 
*IndexerApi* | [**get_indexer_by_id**](docs/IndexerApi.md#get_indexer_by_id) | **GET** /api/v1/indexer/{id} | 
*IndexerApi* | [**list_indexer**](docs/IndexerApi.md#list_indexer) | **GET** /api/v1/indexer | 
*IndexerApi* | [**list_indexer_schema**](docs/IndexerApi.md#list_indexer_schema) | **GET** /api/v1/indexer/schema | 
*IndexerApi* | [**put_indexer_bulk**](docs/IndexerApi.md#put_indexer_bulk) | **PUT** /api/v1/indexer/bulk | 
*IndexerApi* | [**test_indexer**](docs/IndexerApi.md#test_indexer) | **POST** /api/v1/indexer/test | 
*IndexerApi* | [**testall_indexer**](docs/IndexerApi.md#testall_indexer) | **POST** /api/v1/indexer/testall | 
*IndexerApi* | [**update_indexer**](docs/IndexerApi.md#update_indexer) | **PUT** /api/v1/indexer/{id} | 
*IndexerDefaultCategoriesApi* | [**list_indexer_categories**](docs/IndexerDefaultCategoriesApi.md#list_indexer_categories) | **GET** /api/v1/indexer/categories | 
*IndexerProxyApi* | [**create_indexer_proxy**](docs/IndexerProxyApi.md#create_indexer_proxy) | **POST** /api/v1/indexerproxy | 
*IndexerProxyApi* | [**create_indexer_proxy_action_by_name**](docs/IndexerProxyApi.md#create_indexer_proxy_action_by_name) | **POST** /api/v1/indexerproxy/action/{name} | 
*IndexerProxyApi* | [**delete_indexer_proxy**](docs/IndexerProxyApi.md#delete_indexer_proxy) | **DELETE** /api/v1/indexerproxy/{id} | 
*IndexerProxyApi* | [**get_indexer_proxy_by_id**](docs/IndexerProxyApi.md#get_indexer_proxy_by_id) | **GET** /api/v1/indexerproxy/{id} | 
*IndexerProxyApi* | [**list_indexer_proxy**](docs/IndexerProxyApi.md#list_indexer_proxy) | **GET** /api/v1/indexerproxy | 
*IndexerProxyApi* | [**list_indexer_proxy_schema**](docs/IndexerProxyApi.md#list_indexer_proxy_schema) | **GET** /api/v1/indexerproxy/schema | 
*IndexerProxyApi* | [**test_indexer_proxy**](docs/IndexerProxyApi.md#test_indexer_proxy) | **POST** /api/v1/indexerproxy/test | 
*IndexerProxyApi* | [**testall_indexer_proxy**](docs/IndexerProxyApi.md#testall_indexer_proxy) | **POST** /api/v1/indexerproxy/testall | 
*IndexerProxyApi* | [**update_indexer_proxy**](docs/IndexerProxyApi.md#update_indexer_proxy) | **PUT** /api/v1/indexerproxy/{id} | 
*IndexerStatsApi* | [**get_indexer_stats**](docs/IndexerStatsApi.md#get_indexer_stats) | **GET** /api/v1/indexerstats | 
*IndexerStatusApi* | [**list_indexer_status**](docs/IndexerStatusApi.md#list_indexer_status) | **GET** /api/v1/indexerstatus | 
*LocalizationApi* | [**get_localization**](docs/LocalizationApi.md#get_localization) | **GET** /api/v1/localization | 
*LocalizationApi* | [**list_localization_options**](docs/LocalizationApi.md#list_localization_options) | **GET** /api/v1/localization/options | 
*LogApi* | [**get_log**](docs/LogApi.md#get_log) | **GET** /api/v1/log | 
*LogFileApi* | [**get_log_file_by_filename**](docs/LogFileApi.md#get_log_file_by_filename) | **GET** /api/v1/log/file/{filename} | 
*LogFileApi* | [**list_log_file**](docs/LogFileApi.md#list_log_file) | **GET** /api/v1/log/file | 
*NewznabApi* | [**get_indexer_download**](docs/NewznabApi.md#get_indexer_download) | **GET** /api/v1/indexer/{id}/download | 
*NewznabApi* | [**get_indexer_newznab**](docs/NewznabApi.md#get_indexer_newznab) | **GET** /api/v1/indexer/{id}/newznab | 
*NotificationApi* | [**create_notification**](docs/NotificationApi.md#create_notification) | **POST** /api/v1/notification | 
*NotificationApi* | [**create_notification_action_by_name**](docs/NotificationApi.md#create_notification_action_by_name) | **POST** /api/v1/notification/action/{name} | 
*NotificationApi* | [**delete_notification**](docs/NotificationApi.md#delete_notification) | **DELETE** /api/v1/notification/{id} | 
*NotificationApi* | [**get_notification_by_id**](docs/NotificationApi.md#get_notification_by_id) | **GET** /api/v1/notification/{id} | 
*NotificationApi* | [**list_notification**](docs/NotificationApi.md#list_notification) | **GET** /api/v1/notification | 
*NotificationApi* | [**list_notification_schema**](docs/NotificationApi.md#list_notification_schema) | **GET** /api/v1/notification/schema | 
*NotificationApi* | [**test_notification**](docs/NotificationApi.md#test_notification) | **POST** /api/v1/notification/test | 
*NotificationApi* | [**testall_notification**](docs/NotificationApi.md#testall_notification) | **POST** /api/v1/notification/testall | 
*NotificationApi* | [**update_notification**](docs/NotificationApi.md#update_notification) | **PUT** /api/v1/notification/{id} | 
*PingApi* | [**get_ping**](docs/PingApi.md#get_ping) | **GET** /ping | 
*PingApi* | [**head_ping**](docs/PingApi.md#head_ping) | **HEAD** /ping | 
*SearchApi* | [**create_search**](docs/SearchApi.md#create_search) | **POST** /api/v1/search | 
*SearchApi* | [**create_search_bulk**](docs/SearchApi.md#create_search_bulk) | **POST** /api/v1/search/bulk | 
*SearchApi* | [**list_search**](docs/SearchApi.md#list_search) | **GET** /api/v1/search | 
*StaticResourceApi* | [**get_by_path**](docs/StaticResourceApi.md#get_by_path) | **GET** /{path} | 
*StaticResourceApi* | [**get_content_by_path**](docs/StaticResourceApi.md#get_content_by_path) | **GET** /content/{path} | 
*StaticResourceApi* | [**get_login**](docs/StaticResourceApi.md#get_login) | **GET** /login | 
*SystemApi* | [**create_system_restart**](docs/SystemApi.md#create_system_restart) | **POST** /api/v1/system/restart | 
*SystemApi* | [**create_system_shutdown**](docs/SystemApi.md#create_system_shutdown) | **POST** /api/v1/system/shutdown | 
*SystemApi* | [**get_system_routes**](docs/SystemApi.md#get_system_routes) | **GET** /api/v1/system/routes | 
*SystemApi* | [**get_system_routes_duplicate**](docs/SystemApi.md#get_system_routes_duplicate) | **GET** /api/v1/system/routes/duplicate | 
*SystemApi* | [**get_system_status**](docs/SystemApi.md#get_system_status) | **GET** /api/v1/system/status | 
*TagApi* | [**create_tag**](docs/TagApi.md#create_tag) | **POST** /api/v1/tag | 
*TagApi* | [**delete_tag**](docs/TagApi.md#delete_tag) | **DELETE** /api/v1/tag/{id} | 
*TagApi* | [**get_tag_by_id**](docs/TagApi.md#get_tag_by_id) | **GET** /api/v1/tag/{id} | 
*TagApi* | [**list_tag**](docs/TagApi.md#list_tag) | **GET** /api/v1/tag | 
*TagApi* | [**update_tag**](docs/TagApi.md#update_tag) | **PUT** /api/v1/tag/{id} | 
*TagDetailsApi* | [**get_tag_detail_by_id**](docs/TagDetailsApi.md#get_tag_detail_by_id) | **GET** /api/v1/tag/detail/{id} | 
*TagDetailsApi* | [**list_tag_detail**](docs/TagDetailsApi.md#list_tag_detail) | **GET** /api/v1/tag/detail | 
*TaskApi* | [**get_system_task_by_id**](docs/TaskApi.md#get_system_task_by_id) | **GET** /api/v1/system/task/{id} | 
*TaskApi* | [**list_system_task**](docs/TaskApi.md#list_system_task) | **GET** /api/v1/system/task | 
*UiConfigApi* | [**get_ui_config**](docs/UiConfigApi.md#get_ui_config) | **GET** /api/v1/config/ui | 
*UiConfigApi* | [**get_ui_config_by_id**](docs/UiConfigApi.md#get_ui_config_by_id) | **GET** /api/v1/config/ui/{id} | 
*UiConfigApi* | [**update_ui_config**](docs/UiConfigApi.md#update_ui_config) | **PUT** /api/v1/config/ui/{id} | 
*UpdateApi* | [**list_update**](docs/UpdateApi.md#list_update) | **GET** /api/v1/update | 
*UpdateLogFileApi* | [**get_log_file_update_by_filename**](docs/UpdateLogFileApi.md#get_log_file_update_by_filename) | **GET** /api/v1/log/file/update/{filename} | 
*UpdateLogFileApi* | [**list_log_file_update**](docs/UpdateLogFileApi.md#list_log_file_update) | **GET** /api/v1/log/file/update | 


## Documentation For Models

 - [ApiInfoResource](docs/ApiInfoResource.md)
 - [AppProfileResource](docs/AppProfileResource.md)
 - [ApplicationBulkResource](docs/ApplicationBulkResource.md)
 - [ApplicationResource](docs/ApplicationResource.md)
 - [ApplicationSyncLevel](docs/ApplicationSyncLevel.md)
 - [ApplyTags](docs/ApplyTags.md)
 - [AuthenticationRequiredType](docs/AuthenticationRequiredType.md)
 - [AuthenticationType](docs/AuthenticationType.md)
 - [BackupResource](docs/BackupResource.md)
 - [BackupType](docs/BackupType.md)
 - [BookSearchParam](docs/BookSearchParam.md)
 - [CertificateValidationType](docs/CertificateValidationType.md)
 - [Command](docs/Command.md)
 - [CommandPriority](docs/CommandPriority.md)
 - [CommandResource](docs/CommandResource.md)
 - [CommandStatus](docs/CommandStatus.md)
 - [CommandTrigger](docs/CommandTrigger.md)
 - [CustomFilterResource](docs/CustomFilterResource.md)
 - [DatabaseType](docs/DatabaseType.md)
 - [DevelopmentConfigResource](docs/DevelopmentConfigResource.md)
 - [DownloadClientBulkResource](docs/DownloadClientBulkResource.md)
 - [DownloadClientCategory](docs/DownloadClientCategory.md)
 - [DownloadClientConfigResource](docs/DownloadClientConfigResource.md)
 - [DownloadClientResource](docs/DownloadClientResource.md)
 - [DownloadProtocol](docs/DownloadProtocol.md)
 - [Field](docs/Field.md)
 - [HealthCheckResult](docs/HealthCheckResult.md)
 - [HealthResource](docs/HealthResource.md)
 - [HistoryEventType](docs/HistoryEventType.md)
 - [HistoryResource](docs/HistoryResource.md)
 - [HistoryResourcePagingResource](docs/HistoryResourcePagingResource.md)
 - [HostConfigResource](docs/HostConfigResource.md)
 - [HostStatistics](docs/HostStatistics.md)
 - [IndexerBulkResource](docs/IndexerBulkResource.md)
 - [IndexerCapabilityResource](docs/IndexerCapabilityResource.md)
 - [IndexerCategory](docs/IndexerCategory.md)
 - [IndexerPrivacy](docs/IndexerPrivacy.md)
 - [IndexerProxyResource](docs/IndexerProxyResource.md)
 - [IndexerResource](docs/IndexerResource.md)
 - [IndexerStatistics](docs/IndexerStatistics.md)
 - [IndexerStatsResource](docs/IndexerStatsResource.md)
 - [IndexerStatusResource](docs/IndexerStatusResource.md)
 - [LocalizationOption](docs/LocalizationOption.md)
 - [LogFileResource](docs/LogFileResource.md)
 - [LogResource](docs/LogResource.md)
 - [LogResourcePagingResource](docs/LogResourcePagingResource.md)
 - [MovieSearchParam](docs/MovieSearchParam.md)
 - [MusicSearchParam](docs/MusicSearchParam.md)
 - [NotificationResource](docs/NotificationResource.md)
 - [PingResource](docs/PingResource.md)
 - [PrivacyLevel](docs/PrivacyLevel.md)
 - [ProviderMessage](docs/ProviderMessage.md)
 - [ProviderMessageType](docs/ProviderMessageType.md)
 - [ProxyType](docs/ProxyType.md)
 - [ReleaseResource](docs/ReleaseResource.md)
 - [RuntimeMode](docs/RuntimeMode.md)
 - [SearchParam](docs/SearchParam.md)
 - [SelectOption](docs/SelectOption.md)
 - [SortDirection](docs/SortDirection.md)
 - [SystemResource](docs/SystemResource.md)
 - [TagDetailsResource](docs/TagDetailsResource.md)
 - [TagResource](docs/TagResource.md)
 - [TaskResource](docs/TaskResource.md)
 - [TvSearchParam](docs/TvSearchParam.md)
 - [UiConfigResource](docs/UiConfigResource.md)
 - [UpdateChanges](docs/UpdateChanges.md)
 - [UpdateMechanism](docs/UpdateMechanism.md)
 - [UpdateResource](docs/UpdateResource.md)
 - [UserAgentStatistics](docs/UserAgentStatistics.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



