#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupSchedule {
    /// output only field: when next backup will be executed
    /// using provided schedule.
    #[prost(message, optional, tag="3")]
    pub next_execute_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(oneof="backup_schedule::Policy", tags="1, 2, 4")]
    pub policy: ::core::option::Option<backup_schedule::Policy>,
}
/// Nested message and enum types in `BackupSchedule`.
pub mod backup_schedule {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Policy {
        #[prost(message, tag="1")]
        DailyBackupSchedule(super::DailyBackupSchedule),
        #[prost(message, tag="2")]
        WeeklyBackupSchedule(super::WeeklyBackupSchedule),
        #[prost(message, tag="4")]
        RecurringBackupSchedule(super::RecurringBackupSchedule),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecurringBackupSchedule {
    /// Timestamp of the first recurrence.
    #[prost(message, optional, tag="1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// An RRULE (<https://tools.ietf.org/html/rfc5545#section-3.8.5.3>) for how
    /// this backup reccurs.
    /// The FREQ values of MINUTELY, and SECONDLY are not supported.
    #[prost(string, tag="2")]
    pub recurrence: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DaysOfWeekBackupSchedule {
    #[prost(enumeration="super::super::super::super::google::r#type::DayOfWeek", repeated, packed="false", tag="1")]
    pub days: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, optional, tag="2")]
    pub execute_time: ::core::option::Option<super::super::super::super::google::r#type::TimeOfDay>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WeeklyBackupSchedule {
    #[prost(message, repeated, tag="1")]
    pub days_of_week: ::prost::alloc::vec::Vec<DaysOfWeekBackupSchedule>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyBackupSchedule {
    #[prost(message, optional, tag="1")]
    pub execute_time: ::core::option::Option<super::super::super::super::google::r#type::TimeOfDay>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupSettings {
    /// name of backup settings
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// human readable description.
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// provide schedule. if empty, backup will be disabled.
    #[prost(message, optional, tag="3")]
    pub backup_schedule: ::core::option::Option<BackupSchedule>,
    /// provide time to live of backup.
    #[prost(message, optional, tag="4")]
    pub backup_time_to_live: ::core::option::Option<::prost_types::Duration>,
    /// provide a list of source paths. Each path can be directory, table or even database itself.
    /// Each directory (or database) will be traversed recursively and all childs of directory will be included to backup.
    /// By default, backup will be created for full database.
    #[prost(string, repeated, tag="5")]
    pub source_paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// provide a list of paths to exclude from backup.
    /// Each path is a directory, table, or database.
    /// Each directory (or database) will be traversed recursively and all childs of directory will be excluded.
    #[prost(string, repeated, tag="6")]
    pub source_paths_to_exclude: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration="backup_settings::Type", tag="7")]
    pub r#type: i32,
    #[prost(enumeration="backup_settings::StorageClass", tag="8")]
    pub storage_class: i32,
}
/// Nested message and enum types in `BackupSettings`.
pub mod backup_settings {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        Unspecified = 0,
        System = 1,
        User = 2,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum StorageClass {
        Unspecified = 0,
        Standard = 1,
        ReducedRedundancy = 2,
        StandardIa = 3,
        OnezoneIa = 4,
        IntelligentTiering = 5,
        Glacier = 6,
        DeepArchive = 7,
        Outposts = 8,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupConfig {
    #[prost(message, repeated, tag="1")]
    pub backup_settings: ::prost::alloc::vec::Vec<BackupSettings>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Backup {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// human readable backup name.
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub folder_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub database_id: ::prost::alloc::string::String,
    /// description of backup.
    #[prost(string, tag="5")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag="6")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    /// indicates when backup started.
    #[prost(message, optional, tag="7")]
    pub started_at: ::core::option::Option<::prost_types::Timestamp>,
    /// indicates when backup completed.
    #[prost(message, optional, tag="8")]
    pub completed_at: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(enumeration="backup::Status", tag="9")]
    pub status: i32,
    /// settings used to make backup.
    #[prost(message, optional, tag="10")]
    pub backup_settings: ::core::option::Option<BackupSettings>,
    #[prost(enumeration="backup::Type", tag="11")]
    pub r#type: i32,
    /// size of backup in bytes.
    #[prost(int64, tag="12")]
    pub size: i64,
}
/// Nested message and enum types in `Backup`.
pub mod backup {
    /// id of backup
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        Unspecified = 0,
        Creating = 1,
        Ready = 2,
        Error = 3,
        Cancelled = 4,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        Unspecified = 0,
        /// indicates that backup started by the system.
        System = 1,
        User = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPathsRequest {
    /// Required. ID of the YDB backup.
    #[prost(string, tag="1")]
    pub backup_id: ::prost::alloc::string::String,
    /// The maximum number of results per page that should be returned. If the number of available
    /// results is larger than `page_size`, the service returns a `next_page_token` that can be used
    /// to get the next page of results in subsequent ListPaths requests.
    /// Acceptable values are 0 to 1000, inclusive. Default value: 100.
    #[prost(int64, tag="2")]
    pub page_size: i64,
    /// Page token. Set `page_token` to the `next_page_token` returned by a previous ListPaths
    /// request to get the next page of results.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPathsResponse {
    #[prost(string, repeated, tag="1")]
    pub paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBackupRequest {
    /// Required. ID of the YDB backup.
    #[prost(string, tag="1")]
    pub backup_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBackupsRequest {
    #[prost(string, tag="1")]
    pub folder_id: ::prost::alloc::string::String,
    /// The maximum number of results per page that should be returned. If the number of available
    /// results is larger than `page_size`, the service returns a `next_page_token` that can be used
    /// to get the next page of results in subsequent ListBackups requests.
    /// Acceptable values are 0 to 1000, inclusive. Default value: 100.
    #[prost(int64, tag="2")]
    pub page_size: i64,
    /// Page token. Set `page_token` to the `next_page_token` returned by a previous ListBackups
    /// request to get the next page of results.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBackupsResponse {
    #[prost(message, repeated, tag="1")]
    pub backups: ::prost::alloc::vec::Vec<Backup>,
    /// This token allows you to get the next page of results for ListBackups requests,
    /// if the number of results is larger than `page_size` specified in the request.
    /// To get the next page, specify the value of `next_page_token` as a value for
    /// the `page_token` parameter in the next ListBackups request. Subsequent ListBackups
    /// requests will have their own `next_page_token` to continue paging through the results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBackupRequest {
    #[prost(string, tag="1")]
    pub backup_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBackupMetadata {
    #[prost(string, tag="1")]
    pub backup_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub database_id: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod backup_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// A set of methods for managing backups.
    #[derive(Debug, Clone)]
    pub struct BackupServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BackupServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> BackupServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Default + Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> BackupServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            BackupServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// Returns the specified backup.
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBackupRequest>,
        ) -> Result<tonic::Response<super::Backup>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/yandex.cloud.ydb.v1.BackupService/Get",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_paths(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPathsRequest>,
        ) -> Result<tonic::Response<super::ListPathsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/yandex.cloud.ydb.v1.BackupService/ListPaths",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Retrieves a list of backups.
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBackupsRequest>,
        ) -> Result<tonic::Response<super::ListBackupsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/yandex.cloud.ydb.v1.BackupService/List",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes the specified backup.
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteBackupRequest>,
        ) -> Result<
                tonic::Response<super::super::super::operation::Operation>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/yandex.cloud.ydb.v1.BackupService/Delete",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// YDB database.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Database {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub folder_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub description: ::prost::alloc::string::String,
    #[prost(enumeration="database::Status", tag="6")]
    pub status: i32,
    #[prost(string, tag="8")]
    pub endpoint: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub resource_preset_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="10")]
    pub storage_config: ::core::option::Option<StorageConfig>,
    #[prost(message, optional, tag="11")]
    pub scale_policy: ::core::option::Option<ScalePolicy>,
    #[prost(string, tag="12")]
    pub network_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="13")]
    pub subnet_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag="16")]
    pub assign_public_ips: bool,
    #[prost(string, tag="17")]
    pub location_id: ::prost::alloc::string::String,
    #[prost(map="string, string", tag="20")]
    pub labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(message, optional, tag="21")]
    pub backup_config: ::core::option::Option<BackupConfig>,
    #[prost(string, tag="22")]
    pub document_api_endpoint: ::prost::alloc::string::String,
    #[prost(string, tag="23")]
    pub kinesis_api_endpoint: ::prost::alloc::string::String,
    #[prost(message, optional, tag="24")]
    pub monitoring_config: ::core::option::Option<MonitoringConfig>,
    #[prost(oneof="database::DatabaseType", tags="14, 15, 18, 19")]
    pub database_type: ::core::option::Option<database::DatabaseType>,
}
/// Nested message and enum types in `Database`.
pub mod database {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        Unspecified = 0,
        Provisioning = 1,
        Running = 2,
        Updating = 4,
        Error = 5,
        Deleting = 6,
        Starting = 7,
        Stopped = 8,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DatabaseType {
        /// deprecated field
        #[prost(message, tag="14")]
        ZonalDatabase(super::ZonalDatabase),
        /// deprecated field
        #[prost(message, tag="15")]
        RegionalDatabase(super::RegionalDatabase),
        #[prost(message, tag="18")]
        DedicatedDatabase(super::DedicatedDatabase),
        #[prost(message, tag="19")]
        ServerlessDatabase(super::ServerlessDatabase),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlertParameter {
    #[prost(oneof="alert_parameter::Parameter", tags="1, 2, 3, 4, 5")]
    pub parameter: ::core::option::Option<alert_parameter::Parameter>,
}
/// Nested message and enum types in `AlertParameter`.
pub mod alert_parameter {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DoubleParameterValue {
        /// Required. Parameter name
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        /// Required. Parameter value
        #[prost(double, tag="2")]
        pub value: f64,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IntegerParameterValue {
        /// Required. Parameter name
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        /// Required. Parameter value
        #[prost(int64, tag="2")]
        pub value: i64,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TextParameterValue {
        /// Required. Parameter name
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        /// Required. Parameter value
        #[prost(string, tag="2")]
        pub value: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TextListParameterValue {
        /// Required. Parameter name
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        /// Required. Parameter value
        #[prost(string, repeated, tag="2")]
        pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LabelListParameterValue {
        /// Required. Parameter name
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        /// Required. Parameter value
        #[prost(string, repeated, tag="2")]
        pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Parameter {
        #[prost(message, tag="1")]
        DoubleParameterValue(DoubleParameterValue),
        #[prost(message, tag="2")]
        IntegerParameterValue(IntegerParameterValue),
        #[prost(message, tag="3")]
        TextParameterValue(TextParameterValue),
        #[prost(message, tag="4")]
        TextListParameterValue(TextListParameterValue),
        #[prost(message, tag="5")]
        LabelListParameterValue(LabelListParameterValue),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationChannel {
    #[prost(string, tag="1")]
    pub notification_channel_id: ::prost::alloc::string::String,
    #[prost(enumeration="AlertEvaluationStatus", repeated, tag="2")]
    pub notify_about_statuses: ::prost::alloc::vec::Vec<i32>,
    #[prost(int64, tag="3")]
    pub repeate_notify_delay_ms: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Alert {
    /// output only field.
    #[prost(string, tag="1")]
    pub alert_id: ::prost::alloc::string::String,
    /// template of the alert.
    #[prost(string, tag="2")]
    pub alert_template_id: ::prost::alloc::string::String,
    /// name of the alert.
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    /// human readable description of the alert.
    #[prost(string, tag="4")]
    pub description: ::prost::alloc::string::String,
    /// the notification channels of the alert.
    #[prost(message, repeated, tag="5")]
    pub notification_channels: ::prost::alloc::vec::Vec<NotificationChannel>,
    /// alert parameters to override.
    #[prost(message, repeated, tag="6")]
    pub alert_parameters: ::prost::alloc::vec::Vec<AlertParameter>,
    /// alert paratemers to override.
    #[prost(message, repeated, tag="7")]
    pub alert_thresholds: ::prost::alloc::vec::Vec<AlertParameter>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MonitoringConfig {
    #[prost(message, repeated, tag="1")]
    pub alerts: ::prost::alloc::vec::Vec<Alert>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DedicatedDatabase {
    #[prost(string, tag="1")]
    pub resource_preset_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub storage_config: ::core::option::Option<StorageConfig>,
    #[prost(message, optional, tag="3")]
    pub scale_policy: ::core::option::Option<ScalePolicy>,
    #[prost(string, tag="4")]
    pub network_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="5")]
    pub subnet_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag="6")]
    pub assign_public_ips: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerlessDatabase {
    /// Let's define 1 RU  - 1 request unit
    /// Let's define 1 RCU - 1 request capacity unit, which is 1 RU per second.
    /// If `enable_throttling_rcu_limit` flag is true, the database will be throttled using `throttling_rcu_limit` value.
    /// Otherwise, the database is throttled using the cloud quotas.
    /// If zero, all requests will be blocked until non zero value is set.
    #[prost(int64, tag="1")]
    pub throttling_rcu_limit: i64,
    /// Specify serverless database storage size limit. If zero, default value is applied.
    #[prost(int64, tag="2")]
    pub storage_size_limit: i64,
    /// If false, the database is throttled by cloud value.
    #[prost(bool, tag="3")]
    pub enable_throttling_rcu_limit: bool,
    /// Specify the number of provisioned RCUs to pay less if the database has predictable load.
    /// You will be charged for the provisioned capacity regularly even if this capacity is not fully consumed.
    /// You will be charged for the on-demand consumption only if provisioned capacity is consumed.
    #[prost(int64, tag="4")]
    pub provisioned_rcu_limit: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ZonalDatabase {
    #[prost(string, tag="1")]
    pub zone_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionalDatabase {
    #[prost(string, tag="1")]
    pub region_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScalePolicy {
    #[prost(oneof="scale_policy::ScaleType", tags="1")]
    pub scale_type: ::core::option::Option<scale_policy::ScaleType>,
}
/// Nested message and enum types in `ScalePolicy`.
pub mod scale_policy {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FixedScale {
        #[prost(int64, tag="1")]
        pub size: i64,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ScaleType {
        #[prost(message, tag="1")]
        FixedScale(FixedScale),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageConfig {
    #[prost(message, repeated, tag="1")]
    pub storage_options: ::prost::alloc::vec::Vec<StorageOption>,
    /// output only field: storage size limit of dedicated database.
    #[prost(int64, tag="2")]
    pub storage_size_limit: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageOption {
    #[prost(string, tag="1")]
    pub storage_type_id: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub group_count: i64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AlertEvaluationStatus {
    Unspecified = 0,
    Ok = 1,
    NoData = 2,
    Error = 3,
    Alarm = 4,
    Warn = 5,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreBackupRequest {
    /// Required. ID of the YDB backup.
    #[prost(string, tag="1")]
    pub backup_id: ::prost::alloc::string::String,
    /// Required. ID of the YDB database.
    #[prost(string, tag="2")]
    pub database_id: ::prost::alloc::string::String,
    /// Specify paths to restore.
    /// If empty, all paths will restored by default.
    #[prost(string, repeated, tag="3")]
    pub paths_to_restore: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Specify target path.
    #[prost(string, tag="4")]
    pub target_path: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreBackupMetadata {
    #[prost(string, tag="1")]
    pub backup_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub database_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupDatabaseRequest {
    #[prost(string, tag="1")]
    pub database_id: ::prost::alloc::string::String,
    /// custom backup options, if required.
    #[prost(message, optional, tag="2")]
    pub backup_settings: ::core::option::Option<BackupSettings>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupDatabaseMetadata {
    #[prost(string, tag="1")]
    pub backup_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub database_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartDatabaseRequest {
    #[prost(string, tag="1")]
    pub database_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartDatabaseMetadata {
    #[prost(string, tag="1")]
    pub database_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub database_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopDatabaseRequest {
    #[prost(string, tag="1")]
    pub database_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopDatabaseMetadata {
    #[prost(string, tag="1")]
    pub database_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub database_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatabaseRequest {
    /// Required. ID of the YDB cluster.
    #[prost(string, tag="1")]
    pub database_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDatabasesRequest {
    #[prost(string, tag="1")]
    pub folder_id: ::prost::alloc::string::String,
    /// The maximum number of results per page that should be returned. If the number of available
    /// results is larger than `page_size`, the service returns a `next_page_token` that can be used
    /// to get the next page of results in subsequent ListDatabases requests.
    /// Acceptable values are 0 to 1000, inclusive. Default value: 100.
    #[prost(int64, tag="2")]
    pub page_size: i64,
    /// Page token. Set `page_token` to the `next_page_token` returned by a previous ListDatabases
    /// request to get the next page of results.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDatabasesResponse {
    #[prost(message, repeated, tag="1")]
    pub databases: ::prost::alloc::vec::Vec<Database>,
    /// This token allows you to get the next page of results for ListDatabases requests,
    /// if the number of results is larger than `page_size` specified in the request.
    /// To get the next page, specify the value of `next_page_token` as a value for
    /// the `page_token` parameter in the next ListDatabases request. Subsequent ListDatabases
    /// requests will have their own `next_page_token` to continue paging through the results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDatabaseRequest {
    #[prost(string, tag="1")]
    pub folder_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub resource_preset_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="5")]
    pub storage_config: ::core::option::Option<StorageConfig>,
    #[prost(message, optional, tag="6")]
    pub scale_policy: ::core::option::Option<ScalePolicy>,
    #[prost(string, tag="7")]
    pub network_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="8")]
    pub subnet_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag="11")]
    pub assign_public_ips: bool,
    #[prost(string, tag="12")]
    pub location_id: ::prost::alloc::string::String,
    #[prost(map="string, string", tag="15")]
    pub labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(message, optional, tag="16")]
    pub backup_config: ::core::option::Option<BackupConfig>,
    #[prost(message, optional, tag="17")]
    pub monitoring_config: ::core::option::Option<MonitoringConfig>,
    #[prost(oneof="create_database_request::DatabaseType", tags="9, 10, 13, 14")]
    pub database_type: ::core::option::Option<create_database_request::DatabaseType>,
}
/// Nested message and enum types in `CreateDatabaseRequest`.
pub mod create_database_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DatabaseType {
        /// deprecated field
        #[prost(message, tag="9")]
        ZonalDatabase(super::ZonalDatabase),
        /// deprecated field
        #[prost(message, tag="10")]
        RegionalDatabase(super::RegionalDatabase),
        #[prost(message, tag="13")]
        DedicatedDatabase(super::DedicatedDatabase),
        #[prost(message, tag="14")]
        ServerlessDatabase(super::ServerlessDatabase),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDatabaseMetadata {
    /// Required. ID of the YDB cluster.
    #[prost(string, tag="1")]
    pub database_id: ::prost::alloc::string::String,
    /// Required. Name of the creating database.
    #[prost(string, tag="2")]
    pub database_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDatabaseRequest {
    #[prost(string, tag="1")]
    pub folder_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    #[prost(string, tag="3")]
    pub database_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub resource_preset_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="7")]
    pub storage_config: ::core::option::Option<StorageConfig>,
    #[prost(message, optional, tag="8")]
    pub scale_policy: ::core::option::Option<ScalePolicy>,
    #[prost(string, tag="9")]
    pub network_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="10")]
    pub subnet_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag="13")]
    pub assign_public_ips: bool,
    #[prost(string, tag="14")]
    pub location_id: ::prost::alloc::string::String,
    #[prost(map="string, string", tag="17")]
    pub labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(message, optional, tag="18")]
    pub backup_config: ::core::option::Option<BackupConfig>,
    #[prost(message, optional, tag="19")]
    pub monitoring_config: ::core::option::Option<MonitoringConfig>,
    #[prost(oneof="update_database_request::DatabaseType", tags="11, 12, 15, 16")]
    pub database_type: ::core::option::Option<update_database_request::DatabaseType>,
}
/// Nested message and enum types in `UpdateDatabaseRequest`.
pub mod update_database_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DatabaseType {
        #[prost(message, tag="11")]
        ZonalDatabase(super::ZonalDatabase),
        #[prost(message, tag="12")]
        RegionalDatabase(super::RegionalDatabase),
        #[prost(message, tag="15")]
        DedicatedDatabase(super::DedicatedDatabase),
        #[prost(message, tag="16")]
        ServerlessDatabase(super::ServerlessDatabase),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDatabaseMetadata {
    #[prost(string, tag="1")]
    pub database_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub database_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDatabaseRequest {
    #[prost(string, tag="1")]
    pub database_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDatabaseMetadata {
    #[prost(string, tag="1")]
    pub database_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub database_name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod database_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// A set of methods for managing databases.
    #[derive(Debug, Clone)]
    pub struct DatabaseServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DatabaseServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DatabaseServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Default + Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> DatabaseServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            DatabaseServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// Returns the specified database.
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDatabaseRequest>,
        ) -> Result<tonic::Response<super::Database>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/yandex.cloud.ydb.v1.DatabaseService/Get",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Retrieves a list of databases.
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDatabasesRequest>,
        ) -> Result<tonic::Response<super::ListDatabasesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/yandex.cloud.ydb.v1.DatabaseService/List",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new database.
        pub async fn create(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDatabaseRequest>,
        ) -> Result<
                tonic::Response<super::super::super::operation::Operation>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/yandex.cloud.ydb.v1.DatabaseService/Create",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Modifies the specified database.
        pub async fn update(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDatabaseRequest>,
        ) -> Result<
                tonic::Response<super::super::super::operation::Operation>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/yandex.cloud.ydb.v1.DatabaseService/Update",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Starts the specified database.
        pub async fn start(
            &mut self,
            request: impl tonic::IntoRequest<super::StartDatabaseRequest>,
        ) -> Result<
                tonic::Response<super::super::super::operation::Operation>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/yandex.cloud.ydb.v1.DatabaseService/Start",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Stops the specified database.
        pub async fn stop(
            &mut self,
            request: impl tonic::IntoRequest<super::StopDatabaseRequest>,
        ) -> Result<
                tonic::Response<super::super::super::operation::Operation>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/yandex.cloud.ydb.v1.DatabaseService/Stop",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes the specified database.
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDatabaseRequest>,
        ) -> Result<
                tonic::Response<super::super::super::operation::Operation>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/yandex.cloud.ydb.v1.DatabaseService/Delete",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Restores the specified backup
        pub async fn restore(
            &mut self,
            request: impl tonic::IntoRequest<super::RestoreBackupRequest>,
        ) -> Result<
                tonic::Response<super::super::super::operation::Operation>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/yandex.cloud.ydb.v1.DatabaseService/Restore",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn backup(
            &mut self,
            request: impl tonic::IntoRequest<super::BackupDatabaseRequest>,
        ) -> Result<
                tonic::Response<super::super::super::operation::Operation>,
                tonic::Status,
            > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/yandex.cloud.ydb.v1.DatabaseService/Backup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Location {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLocationRequest {
    /// Required. ID of the location to return.
    #[prost(string, tag="1")]
    pub location_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLocationsRequest {
    /// The maximum number of results per page that should be returned. If the number of available
    /// results is larger than `page_size`, the service returns a `next_page_token` that can be used
    /// to get the next page of results in subsequent ListLocations requests.
    /// Acceptable values are 0 to 1000, inclusive. Default value: 100.
    #[prost(int64, tag="1")]
    pub page_size: i64,
    /// Page token. Set `page_token` to the `next_page_token` returned by a previous ListLocations
    /// request to get the next page of results.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLocationsResponse {
    /// Requested list of locations.
    #[prost(message, repeated, tag="1")]
    pub locations: ::prost::alloc::vec::Vec<Location>,
    /// This token allows you to get the next page of results for ListLocations requests,
    /// if the number of results is larger than `page_size` specified in the request.
    /// To get the next page, specify the value of `next_page_token` as a value for
    /// the `page_token` parameter in the next ListLocations request. Subsequent ListLocations
    /// requests will have their own `next_page_token` to continue paging through the results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod location_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct LocationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl LocationServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> LocationServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Default + Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> LocationServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            LocationServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// Returns the specified location.
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLocationRequest>,
        ) -> Result<tonic::Response<super::Location>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/yandex.cloud.ydb.v1.LocationService/Get",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the list of available locations.
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLocationsRequest>,
        ) -> Result<tonic::Response<super::ListLocationsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/yandex.cloud.ydb.v1.LocationService/List",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
