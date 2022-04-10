#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstanceGroup {
    /// ID of the instance group.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// ID of the folder that the instance group belongs to.
    #[prost(string, tag="2")]
    pub folder_id: ::prost::alloc::string::String,
    /// Creation timestamp in \[RFC3339\](<https://www.ietf.org/rfc/rfc3339.txt>) text format.
    #[prost(message, optional, tag="3")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    /// Name of the instance group.
    /// The name is unique within the folder.
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
    /// Description of the instance group.
    #[prost(string, tag="5")]
    pub description: ::prost::alloc::string::String,
    /// Resource labels as `key:value` pairs.
    #[prost(map="string, string", tag="6")]
    pub labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Instance template for creating the instance group.
    /// For more information, see [Instance Templates](/docs/compute/concepts/instance-groups/instance-template).
    #[prost(message, optional, tag="7")]
    pub instance_template: ::core::option::Option<InstanceTemplate>,
    /// [Scaling policy](/docs/compute/concepts/instance-groups/scale) of the instance group.
    #[prost(message, optional, tag="8")]
    pub scale_policy: ::core::option::Option<ScalePolicy>,
    /// Deployment policy of the instance group.
    #[prost(message, optional, tag="9")]
    pub deploy_policy: ::core::option::Option<DeployPolicy>,
    /// Allocation policy of the instance group by zones and regions.
    #[prost(message, optional, tag="10")]
    pub allocation_policy: ::core::option::Option<AllocationPolicy>,
    /// Status of the Network Load Balancer target group attributed to the instance group.
    #[prost(message, optional, tag="11")]
    pub load_balancer_state: ::core::option::Option<LoadBalancerState>,
    /// States of instances for this instance group.
    #[prost(message, optional, tag="12")]
    pub managed_instances_state: ::core::option::Option<ManagedInstancesState>,
    /// Settings for balancing load between instances via [Network Load Balancer](/docs/network-load-balancer/concepts)
    /// (OSI model layer 3).
    #[prost(message, optional, tag="14")]
    pub load_balancer_spec: ::core::option::Option<LoadBalancerSpec>,
    /// Health checking specification. For more information, see [Health check](/docs/network-load-balancer/concepts/health-check).
    #[prost(message, optional, tag="15")]
    pub health_checks_spec: ::core::option::Option<HealthChecksSpec>,
    /// ID of the service account. The service account will be used for all API calls
    /// made by the Instance Groups component on behalf of the user (for example, creating instances, adding them to load balancer target group, etc.). For more information, see [Service accounts](/docs/iam/concepts/users/service-accounts).
    /// To get the service account ID, use a \[yandex.cloud.iam.v1.ServiceAccountService.List\] request.
    #[prost(string, tag="16")]
    pub service_account_id: ::prost::alloc::string::String,
    /// Status of the instance group.
    #[prost(enumeration="instance_group::Status", tag="17")]
    pub status: i32,
    #[prost(message, repeated, tag="18")]
    pub variables: ::prost::alloc::vec::Vec<Variable>,
    /// Flag prohibiting deletion of the instance group.
    ///
    /// Allowed values:</br>- `false`: The instance group can be deleted.</br>- `true`: The instance group cannot be deleted.
    ///
    /// The default is `false`.
    #[prost(bool, tag="19")]
    pub deletion_protection: bool,
    /// Settings for balancing load between instances via [Application Load Balancer](/docs/application-load-balancer/concepts)
    /// (OSI model layer 7).
    #[prost(message, optional, tag="20")]
    pub application_load_balancer_spec: ::core::option::Option<ApplicationLoadBalancerSpec>,
    /// Status of the Application Load Balancer target group attributed to the instance group.
    ///
    /// Returned if there is a working load balancer that the target group is connected to.
    #[prost(message, optional, tag="21")]
    pub application_load_balancer_state: ::core::option::Option<ApplicationLoadBalancerState>,
}
/// Nested message and enum types in `InstanceGroup`.
pub mod instance_group {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        Unspecified = 0,
        /// Instance group is being started and will become active soon.
        Starting = 1,
        /// Instance group is active.
        /// In this state the group manages its instances and monitors their health,
        /// creating, deleting, stopping, updating and starting instances as needed.
        ///
        /// To stop the instance group, call \[yandex.cloud.compute.v1.instancegroup.InstanceGroupService.Stop\].
        /// To pause the processes in the instance group, i.e. scaling, checking instances' health,
        /// auto-healing and updating them, without stopping the instances,
        /// call \[yandex.cloud.compute.v1.instancegroup.InstanceGroupService.PauseProcesses\].
        Active = 2,
        /// Instance group is being stopped.
        /// Group's instances stop receiving traffic from the load balancer (if any) and are then stopped.
        Stopping = 3,
        /// Instance group is stopped.
        /// In this state the group cannot be updated and does not react to any changes made to its instances.
        /// To start the instance group, call \[yandex.cloud.compute.v1.instancegroup.InstanceGroupService.Start\].
        Stopped = 4,
        /// Instance group is being deleted.
        Deleting = 5,
        /// Instance group is paused.
        /// In this state all the processes regarding the group management, i.e. scaling, checking instances' health,
        /// auto-healing and updating them, are paused. The instances that were running prior to pausing the group, however,
        /// may still be running.
        ///
        /// To resume the processes in the instance group,
        /// call \[yandex.cloud.compute.v1.instancegroup.InstanceGroupService.ResumeProcesses\].
        /// The group status will change to `ACTIVE`.
        Paused = 6,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplicationLoadBalancerState {
    /// ID of the Application Load Balancer target group attributed to the instance group.
    #[prost(string, tag="1")]
    pub target_group_id: ::prost::alloc::string::String,
    /// Status message of the target group.
    #[prost(string, tag="2")]
    pub status_message: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Variable {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadBalancerState {
    /// ID of the Network Load Balancer target group attributed to the instance group.
    #[prost(string, tag="1")]
    pub target_group_id: ::prost::alloc::string::String,
    /// Status message of the target group.
    #[prost(string, tag="2")]
    pub status_message: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManagedInstancesState {
    /// Target number of instances for this instance group.
    #[prost(int64, tag="1")]
    pub target_size: i64,
    /// The number of running instances that match the current instance template. For more information, see \[ManagedInstance.Status.RUNNING_ACTUAL\].
    #[prost(int64, tag="4")]
    pub running_actual_count: i64,
    /// The number of running instances that does not match the current instance template. For more information, see \[ManagedInstance.Status.RUNNING_OUTDATED\].
    #[prost(int64, tag="5")]
    pub running_outdated_count: i64,
    /// The number of instances in flight (for example, updating, starting, deleting). For more information, see \[ManagedInstance.Status\].
    #[prost(int64, tag="6")]
    pub processing_count: i64,
}
/// Nested message and enum types in `ManagedInstancesState`.
pub mod managed_instances_state {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Statuses {
        /// Instance is being created.
        #[prost(int64, tag="1")]
        pub creating: i64,
        /// Instance is being started.
        #[prost(int64, tag="2")]
        pub starting: i64,
        /// Instance is being opened to receive traffic.
        #[prost(int64, tag="3")]
        pub opening: i64,
        /// Instance is being warmed.
        #[prost(int64, tag="4")]
        pub warming: i64,
        /// Instance is running normally.
        #[prost(int64, tag="5")]
        pub running: i64,
        /// Instance is being closed to traffic.
        #[prost(int64, tag="6")]
        pub closing: i64,
        /// Instance is being stopped.
        #[prost(int64, tag="7")]
        pub stopping: i64,
        /// Instance is being updated.
        #[prost(int64, tag="8")]
        pub updating: i64,
        /// Instance is being deleted.
        #[prost(int64, tag="9")]
        pub deleting: i64,
        /// Instance failed and needs to be recreated.
        #[prost(int64, tag="10")]
        pub failed: i64,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScalePolicy {
    /// Test spec for [automatic scaling policy](/docs/compute/concepts/instance-groups/scale#auto-scale) of the instance group.
    #[prost(message, optional, tag="3")]
    pub test_auto_scale: ::core::option::Option<scale_policy::AutoScale>,
    #[prost(oneof="scale_policy::ScaleType", tags="1, 2")]
    pub scale_type: ::core::option::Option<scale_policy::ScaleType>,
}
/// Nested message and enum types in `ScalePolicy`.
pub mod scale_policy {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AutoScale {
        /// Lower limit for instance count in each zone.
        #[prost(int64, tag="1")]
        pub min_zone_size: i64,
        /// Upper limit for total instance count (across all zones).
        /// 0 means maximum limit = 100.
        #[prost(int64, tag="2")]
        pub max_size: i64,
        /// Time in seconds allotted for averaging metrics.
        /// 1 minute by default.
        #[prost(message, optional, tag="3")]
        pub measurement_duration: ::core::option::Option<::prost_types::Duration>,
        /// The warmup time of the instance in seconds. During this time,
        /// traffic is sent to the instance, but instance metrics are not collected.
        #[prost(message, optional, tag="4")]
        pub warmup_duration: ::core::option::Option<::prost_types::Duration>,
        /// Minimum amount of time in seconds allotted for monitoring before
        /// Instance Groups can reduce the number of instances in the group.
        /// During this time, the group size doesn't decrease, even if the new metric values
        /// indicate that it should.
        #[prost(message, optional, tag="5")]
        pub stabilization_duration: ::core::option::Option<::prost_types::Duration>,
        /// Target group size.
        #[prost(int64, tag="6")]
        pub initial_size: i64,
        /// Defines an autoscaling rule based on the average CPU utilization of the instance group.
        ///
        /// If more than one rule is specified, e.g. CPU utilization and one or more Yandex Monitoring metrics (\[custom_rules\]),
        /// the size of the instance group will be equal to the maximum of sizes calculated according to each metric.
        #[prost(message, optional, tag="7")]
        pub cpu_utilization_rule: ::core::option::Option<CpuUtilizationRule>,
        /// Defines an autoscaling rule based on a [custom metric](/docs/monitoring/operations/metric/add) from Yandex Monitoring.
        ///
        /// If more than one rule is specified, e.g. CPU utilization (\[cpu_utilization_rule\]) and one or more Yandex Monitoring
        /// metrics, the size of the instance group will be equal to the maximum of sizes calculated according to each metric.
        #[prost(message, repeated, tag="8")]
        pub custom_rules: ::prost::alloc::vec::Vec<CustomRule>,
        /// Autoscaling type.
        #[prost(enumeration="auto_scale::AutoScaleType", tag="9")]
        pub auto_scale_type: i32,
    }
    /// Nested message and enum types in `AutoScale`.
    pub mod auto_scale {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum AutoScaleType {
            Unspecified = 0,
            /// Scale each zone independently. This is the default.
            Zonal = 1,
            /// Scale group as a whole.
            Regional = 2,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CpuUtilizationRule {
        /// Target CPU utilization level. Instance Groups maintains this level for each availability zone.
        #[prost(double, tag="1")]
        pub utilization_target: f64,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CustomRule {
        /// Custom metric rule type. This field affects which label from
        /// the custom metric should be used: `zone_id` or `instance_id`.
        #[prost(enumeration="custom_rule::RuleType", tag="1")]
        pub rule_type: i32,
        /// Type of custom metric. This field affects how Instance Groups calculates the average metric value.
        #[prost(enumeration="custom_rule::MetricType", tag="2")]
        pub metric_type: i32,
        /// Name of custom metric in Yandex Monitoring that should be used for scaling.
        #[prost(string, tag="3")]
        pub metric_name: ::prost::alloc::string::String,
        /// Labels of custom metric in Yandex Monitoring that should be used for scaling.
        #[prost(map="string, string", tag="5")]
        pub labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
        /// Target value for the custom metric. Instance Groups maintains this level for each availability zone.
        #[prost(double, tag="4")]
        pub target: f64,
        /// Folder id of custom metric in Yandex Monitoring that should be used for scaling.
        #[prost(string, tag="6")]
        pub folder_id: ::prost::alloc::string::String,
        /// Service of custom metric in Yandex Monitoring that should be used for scaling.
        #[prost(string, tag="7")]
        pub service: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `CustomRule`.
    pub mod custom_rule {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum RuleType {
            Unspecified = 0,
            /// This type means that the metric applies to one instance.
            /// First, Instance Groups calculates the average metric value for each instance,
            /// then averages the values for instances in one availability zone or in whole group depends on autoscaling type.
            /// This type of metric must have the `instance_id` label.
            Utilization = 1,
            /// This type means that the metric applies to instances in one availability zone or to whole group depends on autoscaling type.
            /// This type of metric must have the `zone_id` label if ZONAL autoscaling type is chosen.
            Workload = 2,
        }
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum MetricType {
            Unspecified = 0,
            /// This type is used for metrics that show the metric value at a certain point in time,
            /// such as requests per second to the server on an instance.
            ///
            /// Instance Groups calculates the average metric value for the period
            /// specified in the \[AutoScale.measurement_duration\] field.
            Gauge = 1,
            /// This type is used for metrics that monotonically increase over time,
            /// such as the total number of requests to the server on an instance.
            ///
            /// Instance Groups calculates the average value increase for the period
            /// specified in the \[AutoScale.measurement_duration\] field.
            Counter = 2,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FixedScale {
        /// Number of instances in the instance group.
        #[prost(int64, tag="1")]
        pub size: i64,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ScaleType {
        /// [Manual scaling policy](/docs/compute/concepts/instance-groups/scale#fixed-policy) of the instance group.
        #[prost(message, tag="1")]
        FixedScale(FixedScale),
        /// [Automatic scaling policy](/docs/compute/concepts/instance-groups/scale#auto-scale) of the instance group.
        #[prost(message, tag="2")]
        AutoScale(AutoScale),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeployPolicy {
    /// The maximum number of running instances that can be taken offline (i.e., stopped or deleted) at the same time
    /// during the update process.
    /// If \[max_expansion\] is not specified or set to zero, \[max_unavailable\] must be set to a non-zero value.
    #[prost(int64, tag="1")]
    pub max_unavailable: i64,
    /// The maximum number of instances that can be deleted at the same time.
    ///
    ///The value 0 is any number of virtual machines within the allowed values.
    #[prost(int64, tag="2")]
    pub max_deleting: i64,
    /// The maximum number of instances that can be created at the same time.
    ///
    ///The value 0 is any number of virtual machines within the allowed values.
    #[prost(int64, tag="3")]
    pub max_creating: i64,
    /// The maximum number of instances that can be temporarily allocated above the group's target size
    /// during the update process.
    /// If \[max_unavailable\] is not specified or set to zero, \[max_expansion\] must be set to a non-zero value.
    #[prost(int64, tag="6")]
    pub max_expansion: i64,
    /// Instance startup duration.
    /// Instance will be considered up and running (and start receiving traffic) only after startup_duration
    /// has elapsed and all health checks are passed.
    /// See \[yandex.cloud.compute.v1.instancegroup.ManagedInstance.Status\] for more information.
    #[prost(message, optional, tag="7")]
    pub startup_duration: ::core::option::Option<::prost_types::Duration>,
    /// Affects the lifecycle of the instance during deployment.
    #[prost(enumeration="deploy_policy::Strategy", tag="8")]
    pub strategy: i32,
}
/// Nested message and enum types in `DeployPolicy`.
pub mod deploy_policy {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Strategy {
        Unspecified = 0,
        /// Instance Groups can forcefully stop a running instance. This is the default.
        Proactive = 1,
        /// Instance Groups does not stop a running instance.
        /// Instead, it will wait until the instance stops itself or becomes unhealthy.
        Opportunistic = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllocationPolicy {
    /// List of availability zones.
    #[prost(message, repeated, tag="1")]
    pub zones: ::prost::alloc::vec::Vec<allocation_policy::Zone>,
}
/// Nested message and enum types in `AllocationPolicy`.
pub mod allocation_policy {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Zone {
        /// ID of the availability zone where the instance resides.
        #[prost(string, tag="1")]
        pub zone_id: ::prost::alloc::string::String,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstanceTemplate {
    /// Description of the instance template.
    #[prost(string, tag="1")]
    pub description: ::prost::alloc::string::String,
    /// Resource labels as `key:value` pairs.
    #[prost(map="string, string", tag="2")]
    pub labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// ID of the hardware platform configuration for the instance.
    /// Platforms allows you to create various types of instances: with a large amount of memory,
    /// with a large number of cores, with a burstable performance.
    /// For more information, see \[Platforms\](/docs/compute/concepts/vm-platforms).
    #[prost(string, tag="3")]
    pub platform_id: ::prost::alloc::string::String,
    /// Computing resources of the instance such as the amount of memory and number of cores.
    #[prost(message, optional, tag="4")]
    pub resources_spec: ::core::option::Option<ResourcesSpec>,
    /// The metadata `key:value` pairs assigned to this instance template. This includes custom metadata and predefined keys.
    ///
    /// Metadata values may contain one of the supported placeholders:
    ///   {instance_group.id}
    ///   {instance.short_id}
    ///   {instance.index}
    ///   {instance.index_in_zone}
    ///   {instance.zone_id}
    /// InstanceGroup and Instance labels may be copied to metadata following way:
    ///   {instance_group.labels.some_label_key}
    ///   {instance.labels.another_label_key}
    /// These placeholders will be substituted for each created instance anywhere in the value text.
    /// In the rare case the value requires to contain this placeholder explicitly,
    /// it must be escaped with double brackets, in example {instance.index}.
    ///
    /// For example, you may use the metadata in order to provide your public SSH key to the instance.
    /// For more information, see \[Metadata\](/docs/compute/concepts/vm-metadata).
    #[prost(map="string, string", tag="5")]
    pub metadata: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Boot disk specification that will be attached to the instance.
    #[prost(message, optional, tag="6")]
    pub boot_disk_spec: ::core::option::Option<AttachedDiskSpec>,
    /// Array of secondary disks that will be attached to the instance.
    #[prost(message, repeated, tag="7")]
    pub secondary_disk_specs: ::prost::alloc::vec::Vec<AttachedDiskSpec>,
    /// Array of network interfaces that will be attached to the instance.
    #[prost(message, repeated, tag="8")]
    pub network_interface_specs: ::prost::alloc::vec::Vec<NetworkInterfaceSpec>,
    /// Scheduling policy for the instance.
    #[prost(message, optional, tag="9")]
    pub scheduling_policy: ::core::option::Option<SchedulingPolicy>,
    /// Service account ID for the instance.
    #[prost(string, tag="10")]
    pub service_account_id: ::prost::alloc::string::String,
    /// Network settings for the instance.
    #[prost(message, optional, tag="11")]
    pub network_settings: ::core::option::Option<NetworkSettings>,
    /// Name of the instance.
    /// In order to be unique it must contain at least on of instance unique placeholders:
    ///   {instance.short_id}
    ///   {instance.index}
    ///   combination of {instance.zone_id} and {instance.index_in_zone}
    /// Example: my-instance-{instance.index}
    /// If not set, default is used: {instance_group.id}-{instance.short_id}
    /// It may also contain another placeholders, see metadata doc for full list.
    #[prost(string, tag="12")]
    pub name: ::prost::alloc::string::String,
    /// Host name for the instance.
    /// This field is used to generate the \[yandex.cloud.compute.v1.Instance.fqdn\] value.
    /// The host name must be unique within the network and region.
    /// If not specified, the host name will be equal to \[yandex.cloud.compute.v1.Instance.id\] of the instance
    /// and FQDN will be `<id>.auto.internal`. Otherwise FQDN will be `<hostname>.<region_id>.internal`.
    ///
    /// In order to be unique it must contain at least on of instance unique placeholders:
    ///   {instance.short_id}
    ///   {instance.index}
    ///   combination of {instance.zone_id} and {instance.index_in_zone}
    /// Example: my-instance-{instance.index}
    /// If not set, `name` value will be used
    /// It may also contain another placeholders, see metadata doc for full list.
    #[prost(string, tag="13")]
    pub hostname: ::prost::alloc::string::String,
    /// Placement Group
    #[prost(message, optional, tag="14")]
    pub placement_policy: ::core::option::Option<PlacementPolicy>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlacementPolicy {
    /// Identifier of placement group
    #[prost(string, tag="1")]
    pub placement_group_id: ::prost::alloc::string::String,
    /// List of affinity rules. Scheduler will attempt to allocate instances according to order of rules.
    #[prost(message, repeated, tag="2")]
    pub host_affinity_rules: ::prost::alloc::vec::Vec<placement_policy::HostAffinityRule>,
}
/// Nested message and enum types in `PlacementPolicy`.
pub mod placement_policy {
    /// Affinitity definition
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HostAffinityRule {
        /// Affinity label or one of reserved values - 'yc.hostId', 'yc.hostGroupId'
        #[prost(string, tag="1")]
        pub key: ::prost::alloc::string::String,
        /// Include or exclude action
        #[prost(enumeration="host_affinity_rule::Operator", tag="2")]
        pub op: i32,
        /// Affinity value or host ID or host group ID
        #[prost(string, repeated, tag="3")]
        pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// Nested message and enum types in `HostAffinityRule`.
    pub mod host_affinity_rule {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum Operator {
            Unspecified = 0,
            In = 1,
            NotIn = 2,
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourcesSpec {
    /// The amount of memory available to the instance, specified in bytes.
    #[prost(int64, tag="1")]
    pub memory: i64,
    /// The number of cores available to the instance.
    #[prost(int64, tag="2")]
    pub cores: i64,
    /// Baseline level of CPU performance with the ability to burst performance above that baseline level.
    /// This field sets baseline performance for each core.
    #[prost(int64, tag="3")]
    pub core_fraction: i64,
    /// The number of GPUs available to the instance.
    #[prost(int64, tag="4")]
    pub gpus: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttachedDiskSpec {
    /// Access mode to the Disk resource.
    #[prost(enumeration="attached_disk_spec::Mode", tag="1")]
    pub mode: i32,
    /// Serial number that is reflected in the /dev/disk/by-id/ tree
    /// of a Linux operating system running within the instance.
    ///
    /// This value can be used to reference the device for mounting, resizing, and so on, from within the instance.
    #[prost(string, tag="2")]
    pub device_name: ::prost::alloc::string::String,
    ///oneof disk_spec or disk_id
    /// Disk specification that is attached to the instance. For more information, see \[Disks\](/docs/compute/concepts/disk).
    #[prost(message, optional, tag="3")]
    pub disk_spec: ::core::option::Option<attached_disk_spec::DiskSpec>,
    /// Set to use an existing disk. To set use variables.
    #[prost(string, tag="4")]
    pub disk_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `AttachedDiskSpec`.
pub mod attached_disk_spec {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DiskSpec {
        /// Description of the disk.
        #[prost(string, tag="1")]
        pub description: ::prost::alloc::string::String,
        /// ID of the disk type.
        #[prost(string, tag="2")]
        pub type_id: ::prost::alloc::string::String,
        /// Size of the disk, specified in bytes.
        #[prost(int64, tag="3")]
        pub size: i64,
        /// When set to true, disk will not be deleted even after managed instance is deleted.
        /// It will be a user's responsibility to delete such disks.
        #[prost(bool, tag="6")]
        pub preserve_after_instance_delete: bool,
        #[prost(oneof="disk_spec::SourceOneof", tags="4, 5")]
        pub source_oneof: ::core::option::Option<disk_spec::SourceOneof>,
    }
    /// Nested message and enum types in `DiskSpec`.
    pub mod disk_spec {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum SourceOneof {
            /// ID of the image that will be used for disk creation.
            #[prost(string, tag="4")]
            ImageId(::prost::alloc::string::String),
            /// ID of the snapshot that will be used for disk creation.
            #[prost(string, tag="5")]
            SnapshotId(::prost::alloc::string::String),
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Mode {
        Unspecified = 0,
        /// Read-only access.
        ReadOnly = 1,
        /// Read/Write access.
        ReadWrite = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkInterfaceSpec {
    /// ID of the network.
    #[prost(string, tag="1")]
    pub network_id: ::prost::alloc::string::String,
    /// IDs of the subnets.
    #[prost(string, repeated, tag="2")]
    pub subnet_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Primary IPv4 address that is assigned to the instance for this network interface.
    #[prost(message, optional, tag="3")]
    pub primary_v4_address_spec: ::core::option::Option<PrimaryAddressSpec>,
    /// Primary IPv6 address that is assigned to the instance for this network interface. IPv6 not available yet.
    #[prost(message, optional, tag="4")]
    pub primary_v6_address_spec: ::core::option::Option<PrimaryAddressSpec>,
    /// IDs of security groups.
    #[prost(string, repeated, tag="5")]
    pub security_group_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrimaryAddressSpec {
    /// An external IP address configuration.
    /// If not specified, then this managed instance will have no external internet access.
    #[prost(message, optional, tag="1")]
    pub one_to_one_nat_spec: ::core::option::Option<OneToOneNatSpec>,
    /// Internal DNS configuration
    #[prost(message, repeated, tag="2")]
    pub dns_record_specs: ::prost::alloc::vec::Vec<DnsRecordSpec>,
    /// Optional. Manual set static internal IP. To set use variables.
    #[prost(string, tag="3")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneToOneNatSpec {
    /// IP version for the public IP address.
    #[prost(enumeration="IpVersion", tag="1")]
    pub ip_version: i32,
    /// Manual set static public IP. To set use variables. (optional)
    #[prost(string, tag="2")]
    pub address: ::prost::alloc::string::String,
    /// External DNS configuration
    #[prost(message, repeated, tag="3")]
    pub dns_record_specs: ::prost::alloc::vec::Vec<DnsRecordSpec>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DnsRecordSpec {
    /// FQDN (required)
    #[prost(string, tag="1")]
    pub fqdn: ::prost::alloc::string::String,
    /// DNS zone id (optional, if not set, private zone used)
    #[prost(string, tag="2")]
    pub dns_zone_id: ::prost::alloc::string::String,
    /// DNS record ttl, values in 0-86400 (optional)
    #[prost(int64, tag="3")]
    pub ttl: i64,
    /// When set to true, also create PTR DNS record (optional)
    #[prost(bool, tag="4")]
    pub ptr: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SchedulingPolicy {
    /// Preemptible instances are stopped at least once every 24 hours, and can be stopped at any time
    /// if their resources are needed by Compute.
    /// For more information, see [Preemptible Virtual Machines](/docs/compute/concepts/preemptible-vm).
    #[prost(bool, tag="1")]
    pub preemptible: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkSettings {
    /// Type of instance network.
    #[prost(enumeration="network_settings::Type", tag="1")]
    pub r#type: i32,
}
/// Nested message and enum types in `NetworkSettings`.
pub mod network_settings {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        Unspecified = 0,
        Standard = 1,
        SoftwareAccelerated = 2,
        HardwareAccelerated = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadBalancerSpec {
    /// Specification of the target group that the instance group will be added to. For more information, see [Target groups and resources](/docs/load-balancer/concepts/target-resources).
    #[prost(message, optional, tag="1")]
    pub target_group_spec: ::core::option::Option<TargetGroupSpec>,
    /// Timeout for waiting for the VM to be checked by the load balancer. If the timeout is exceeded,
    /// the VM will be turned off based on the deployment policy. Specified in seconds.
    #[prost(message, optional, tag="2")]
    pub max_opening_traffic_duration: ::core::option::Option<::prost_types::Duration>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetGroupSpec {
    /// Name of the target group.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Description of the target group.
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// Resource labels as `key:value` pairs.
    #[prost(map="string, string", tag="3")]
    pub labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplicationLoadBalancerSpec {
    /// Basic properties of the Application Load Balancer target group attributed to the instance group.
    #[prost(message, optional, tag="1")]
    pub target_group_spec: ::core::option::Option<ApplicationTargetGroupSpec>,
    /// Timeout for waiting for the VM to be checked by the load balancer. If the timeout is exceeded,
    /// the VM will be turned off based on the deployment policy. Specified in seconds.
    #[prost(message, optional, tag="2")]
    pub max_opening_traffic_duration: ::core::option::Option<::prost_types::Duration>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplicationTargetGroupSpec {
    /// Name of the target group.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Description of the target group.
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// Resource labels as `key:value` pairs.
    #[prost(map="string, string", tag="3")]
    pub labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthChecksSpec {
    /// Health checking specification. For more information, see [Health check](/docs/load-balancer/concepts/health-check).
    #[prost(message, repeated, tag="1")]
    pub health_check_specs: ::prost::alloc::vec::Vec<HealthCheckSpec>,
    /// Timeout for waiting for the VM to become healthy. If the timeout is exceeded,
    /// the VM will be turned off based on the deployment policy. Specified in seconds.
    #[prost(message, optional, tag="2")]
    pub max_checking_health_duration: ::core::option::Option<::prost_types::Duration>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthCheckSpec {
    /// The interval between health checks. The default is 2 seconds.
    #[prost(message, optional, tag="1")]
    pub interval: ::core::option::Option<::prost_types::Duration>,
    /// Timeout for the managed instance to return a response for the health check. The default is 1 second.
    #[prost(message, optional, tag="2")]
    pub timeout: ::core::option::Option<::prost_types::Duration>,
    /// The number of failed health checks for the managed instance to be considered unhealthy. The default (0) is 2.
    #[prost(int64, tag="3")]
    pub unhealthy_threshold: i64,
    /// The number of successful health checks required in order for the managed instance to be considered healthy. The default (0) is 2.
    #[prost(int64, tag="4")]
    pub healthy_threshold: i64,
    #[prost(oneof="health_check_spec::HealthCheckOptions", tags="5, 6")]
    pub health_check_options: ::core::option::Option<health_check_spec::HealthCheckOptions>,
}
/// Nested message and enum types in `HealthCheckSpec`.
pub mod health_check_spec {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TcpOptions {
        /// Port to use for TCP health checks.
        #[prost(int64, tag="1")]
        pub port: i64,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HttpOptions {
        /// Port to use for HTTP health checks.
        #[prost(int64, tag="1")]
        pub port: i64,
        /// URL path to set for health checking requests.
        #[prost(string, tag="2")]
        pub path: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum HealthCheckOptions {
        /// Configuration options for a TCP health check.
        #[prost(message, tag="5")]
        TcpOptions(TcpOptions),
        /// Configuration options for an HTTP health check.
        #[prost(message, tag="6")]
        HttpOptions(HttpOptions),
    }
}
/// A ManagedInstance resource. For more information, see [Instance Groups Concepts](/docs/compute/concepts/instance-groups/).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManagedInstance {
    /// ID of the managed instance.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Status of the managed instance.
    #[prost(enumeration="managed_instance::Status", tag="2")]
    pub status: i32,
    /// ID of the instance.
    #[prost(string, tag="3")]
    pub instance_id: ::prost::alloc::string::String,
    /// Fully Qualified Domain Name.
    #[prost(string, tag="4")]
    pub fqdn: ::prost::alloc::string::String,
    /// The name of the managed instance.
    #[prost(string, tag="5")]
    pub name: ::prost::alloc::string::String,
    /// Status message for the managed instance.
    #[prost(string, tag="6")]
    pub status_message: ::prost::alloc::string::String,
    /// ID of the availability zone where the instance resides.
    #[prost(string, tag="7")]
    pub zone_id: ::prost::alloc::string::String,
    /// Array of network interfaces that are attached to the managed instance.
    #[prost(message, repeated, tag="8")]
    pub network_interfaces: ::prost::alloc::vec::Vec<NetworkInterface>,
    /// The timestamp in \[RFC3339\](<https://www.ietf.org/rfc/rfc3339.txt>) text format when the status of the managed instance was last changed.
    #[prost(message, optional, tag="9")]
    pub status_changed_at: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `ManagedInstance`.
pub mod managed_instance {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        Unspecified = 0,
        /// Instance is being created.
        CreatingInstance = 11,
        /// Instance is being updated.
        UpdatingInstance = 12,
        /// Instance is being deleted.
        DeletingInstance = 13,
        /// Instance is being started.
        StartingInstance = 14,
        /// Instance is being stopped.
        StoppingInstance = 15,
        /// Instance has been created successfully, but startup duration has not elapsed yet.
        AwaitingStartupDuration = 16,
        /// Instance has been created successfully and startup duration has elapsed, but health checks have not passed yet and the managed instance is not ready to receive traffic.
        CheckingHealth = 17,
        /// Instance Groups is initiating health checks and routing traffic to the instances.
        OpeningTraffic = 18,
        /// Instance is now receiving traffic, but warmup duration has not elapsed yet.
        AwaitingWarmupDuration = 19,
        /// Instance Groups has initiated the process of stopping routing traffic to the instances.
        ClosingTraffic = 20,
        /// Instance is running normally and its attributes match the current InstanceTemplate.
        RunningActual = 21,
        /// Instance is running normally, but its attributes do not match the current InstanceTemplate.
        /// It will be updated, recreated or deleted shortly.
        RunningOutdated = 22,
        /// Instance was stopped.
        Stopped = 23,
        /// Instance was deleted.
        Deleted = 24,
        /// Instance Groups is preparing dependent resources.
        PreparingResources = 25,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkInterface {
    /// The index of the network interface, generated by the server, 0,1,2... etc.
    /// Currently only one network interface is supported per instance.
    #[prost(string, tag="1")]
    pub index: ::prost::alloc::string::String,
    /// MAC address that is assigned to the network interface.
    #[prost(string, tag="2")]
    pub mac_address: ::prost::alloc::string::String,
    /// ID of the subnet.
    #[prost(string, tag="3")]
    pub subnet_id: ::prost::alloc::string::String,
    /// Primary IPv4 address that is assigned to the instance for this network interface.
    #[prost(message, optional, tag="4")]
    pub primary_v4_address: ::core::option::Option<PrimaryAddress>,
    /// Primary IPv6 address that is assigned to the instance for this network interface. IPv6 is not available yet.
    #[prost(message, optional, tag="5")]
    pub primary_v6_address: ::core::option::Option<PrimaryAddress>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrimaryAddress {
    /// An IPv4 internal network address that is assigned to the managed instance for this network interface.
    /// If not specified by the user, an unused internal IP is assigned by the system.
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    /// One-to-one NAT configuration. If missing, NAT has not been set up.
    #[prost(message, optional, tag="2")]
    pub one_to_one_nat: ::core::option::Option<OneToOneNat>,
    /// Internal DNS configuration.
    #[prost(message, repeated, tag="3")]
    pub dns_records: ::prost::alloc::vec::Vec<DnsRecord>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneToOneNat {
    /// An IPv4 external network address that is assigned to the managed instance for this network interface.
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    /// External IP address version.
    #[prost(enumeration="IpVersion", tag="2")]
    pub ip_version: i32,
    /// External DNS configuration.
    #[prost(message, repeated, tag="3")]
    pub dns_records: ::prost::alloc::vec::Vec<DnsRecord>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DnsRecord {
    /// Name of the A/AAAA record as specified when creating the instance.
    /// Note that if `fqdn' has no trailing '.', it is specified relative to the zone (@see dns_zone_id).
    #[prost(string, tag="1")]
    pub fqdn: ::prost::alloc::string::String,
    /// DNS zone id (optional, if not set, some private zone is used).
    #[prost(string, tag="2")]
    pub dns_zone_id: ::prost::alloc::string::String,
    /// DNS record ttl (optional, if 0, a reasonable default is used).
    #[prost(int64, tag="3")]
    pub ttl: i64,
    /// When true, indicates there is a corresponding auto-created PTR DNS record.
    #[prost(bool, tag="4")]
    pub ptr: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogRecord {
    /// Log timestamp in \[RFC3339\](<https://www.ietf.org/rfc/rfc3339.txt>) text format.
    #[prost(message, optional, tag="1")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    /// The log message.
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IpVersion {
    Unspecified = 0,
    /// IPv4 address, for example 192.168.0.0.
    Ipv4 = 1,
    /// IPv6 address, not available yet.
    Ipv6 = 2,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResumeInstanceGroupProcessesRequest {
    /// ID of the instance group to resume processes in.
    ///
    /// The instance group must have a `PAUSED` status (\[InstanceGroup.status\]).
    ///
    /// To get the instance group ID, make a \[InstanceGroupService.List\] request.
    #[prost(string, tag="1")]
    pub instance_group_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResumeInstanceGroupProcessMetadata {
    /// ID of the instance group that processes are being resumed in.
    #[prost(string, tag="1")]
    pub instance_group_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PauseInstanceGroupProcessesRequest {
    /// ID of the instance group to pause processes in.
    ///
    /// The instance group must have an `ACTIVE` status (\[InstanceGroup.status\]).
    ///
    /// To get the instance group ID, make a \[InstanceGroupService.List\] request.
    #[prost(string, tag="1")]
    pub instance_group_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PauseInstanceGroupProcessMetadata {
    /// ID of the instance group that processes are being paused in.
    #[prost(string, tag="1")]
    pub instance_group_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceGroupRequest {
    /// ID of the InstanceGroup resource to return.
    /// To get the instance group ID, use a \[InstanceGroupService.List\] request.
    #[prost(string, tag="1")]
    pub instance_group_id: ::prost::alloc::string::String,
    /// Defines which information about the Instance template should be returned in the server response.
    #[prost(enumeration="InstanceGroupView", tag="2")]
    pub view: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInstanceGroupRequest {
    /// ID of the folder to create an instance group in.
    /// To get the folder ID, use a \[yandex.cloud.resourcemanager.v1.FolderService.List\] request.
    #[prost(string, tag="1")]
    pub folder_id: ::prost::alloc::string::String,
    /// Name of the instance group.
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    /// Description of the instance group.
    #[prost(string, tag="4")]
    pub description: ::prost::alloc::string::String,
    /// Resource labels as `key:value` pairs.
    #[prost(map="string, string", tag="5")]
    pub labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Instance template that the instance group belongs to.
    #[prost(message, optional, tag="6")]
    pub instance_template: ::core::option::Option<InstanceTemplate>,
    /// [Scaling policy](/docs/compute/concepts/instance-groups/scale) of the instance group.
    #[prost(message, optional, tag="7")]
    pub scale_policy: ::core::option::Option<ScalePolicy>,
    /// Deployment policy of the instance group.
    #[prost(message, optional, tag="8")]
    pub deploy_policy: ::core::option::Option<DeployPolicy>,
    /// Allocation policy of the instance group by zones and regions.
    #[prost(message, optional, tag="9")]
    pub allocation_policy: ::core::option::Option<AllocationPolicy>,
    /// Settings for balancing load between instances via [Network Load Balancer](/docs/network-load-balancer/concepts)
    /// (OSI model layer 3).
    ///
    /// If specified, a Network Load Balancer target group containing all instances from the instance group will be created
    /// and attributed to the instance group.
    #[prost(message, optional, tag="10")]
    pub load_balancer_spec: ::core::option::Option<LoadBalancerSpec>,
    /// Health checking specification. For more information, see [Health check](/docs/load-balancer/concepts/health-check).
    #[prost(message, optional, tag="11")]
    pub health_checks_spec: ::core::option::Option<HealthChecksSpec>,
    /// ID of the service account. The service account will be used for all API calls
    /// made by the Instance Groups component on behalf of the user (for example, creating instances, adding them to load balancer target group, etc.). For more information, see [Service accounts](/docs/iam/concepts/users/service-accounts).
    /// To get the service account ID, use a \[yandex.cloud.iam.v1.ServiceAccountService.List\] request.
    #[prost(string, tag="12")]
    pub service_account_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="13")]
    pub variables: ::prost::alloc::vec::Vec<Variable>,
    /// Flag prohibiting deletion of the instance group.
    ///
    /// Allowed values:</br>- `false`: The instance group can be deleted.</br>- `true`: The instance group cannot be deleted.
    ///
    /// The default is `false`.
    #[prost(bool, tag="14")]
    pub deletion_protection: bool,
    /// Settings for balancing load between instances via [Application Load Balancer](/docs/application-load-balancer/concepts)
    /// (OSI model layer 7).
    ///
    /// If specified, an Application Load Balancer target group containing all instances from the instance group will be created
    /// and attributed to the instance group.
    #[prost(message, optional, tag="15")]
    pub application_load_balancer_spec: ::core::option::Option<ApplicationLoadBalancerSpec>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInstanceGroupFromYamlRequest {
    /// ID of the folder to create an instance group in.
    /// To get the folder ID, use a \[yandex.cloud.resourcemanager.v1.FolderService.List\] request.
    #[prost(string, tag="1")]
    pub folder_id: ::prost::alloc::string::String,
    /// \[InstanceGroupService.Create\] request in YAML format.
    #[prost(string, tag="2")]
    pub instance_group_yaml: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInstanceGroupMetadata {
    /// ID of the instance group that is being created.
    #[prost(string, tag="1")]
    pub instance_group_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInstanceGroupRequest {
    /// ID of the instance group to update.
    /// To get the instance group ID, use a \[InstanceGroupService.List\] request.
    #[prost(string, tag="1")]
    pub instance_group_id: ::prost::alloc::string::String,
    /// Field mask that specifies which fields of the InstanceGroup resource are going to be updated.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Name of the instance group.
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    /// Description of the instance group.
    #[prost(string, tag="4")]
    pub description: ::prost::alloc::string::String,
    /// Resource labels as `key:value` pairs.
    ///
    /// The existing set of `labels` is completely replaced by the provided set.
    #[prost(map="string, string", tag="5")]
    pub labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Instance template that the instance group belongs to.
    #[prost(message, optional, tag="6")]
    pub instance_template: ::core::option::Option<InstanceTemplate>,
    /// [Scaling policy](/docs/compute/concepts/instance-groups/scale) of the instance group.
    #[prost(message, optional, tag="7")]
    pub scale_policy: ::core::option::Option<ScalePolicy>,
    /// Deployment policy of the instance group.
    #[prost(message, optional, tag="8")]
    pub deploy_policy: ::core::option::Option<DeployPolicy>,
    /// Allocation policy of the instance group by zones and regions.
    #[prost(message, optional, tag="9")]
    pub allocation_policy: ::core::option::Option<AllocationPolicy>,
    /// Health checking specification. For more information, see [Health check](/docs/load-balancer/concepts/health-check).
    #[prost(message, optional, tag="11")]
    pub health_checks_spec: ::core::option::Option<HealthChecksSpec>,
    /// ID of the service account. The service account will be used for all API calls
    /// made by the Instance Groups component on behalf of the user (for example, creating instances, adding them to load balancer target group, etc.). For more information, see [Service accounts](/docs/iam/concepts/users/service-accounts).
    /// To get the service account ID, use a \[yandex.cloud.iam.v1.ServiceAccountService.List\] request.
    #[prost(string, tag="12")]
    pub service_account_id: ::prost::alloc::string::String,
    /// Settings for balancing load between instances via [Network Load Balancer](/docs/network-load-balancer/concepts)
    /// (OSI model layer 3).
    #[prost(message, optional, tag="14")]
    pub load_balancer_spec: ::core::option::Option<LoadBalancerSpec>,
    #[prost(message, repeated, tag="15")]
    pub variables: ::prost::alloc::vec::Vec<Variable>,
    /// Flag that inhibits deletion of the instance group
    #[prost(bool, tag="16")]
    pub deletion_protection: bool,
    /// Settings for balancing load between instances via [Application Load Balancer](/docs/application-load-balancer/concepts)
    /// (OSI model layer 7).
    #[prost(message, optional, tag="17")]
    pub application_load_balancer_spec: ::core::option::Option<ApplicationLoadBalancerSpec>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInstanceGroupFromYamlRequest {
    /// ID of the instance group to update.
    /// To get the instance group ID, use a \[InstanceGroupService.List\] request.
    #[prost(string, tag="1")]
    pub instance_group_id: ::prost::alloc::string::String,
    /// \[InstanceGroupService.Update\] request in YAML format.
    #[prost(string, tag="2")]
    pub instance_group_yaml: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInstanceGroupMetadata {
    /// ID of the InstanceGroup resource that is being updated.
    /// To get the instance group ID, use a \[InstanceGroupService.List\] request.
    #[prost(string, tag="1")]
    pub instance_group_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartInstanceGroupRequest {
    /// ID of the instance group to start.
    /// To get the instance group ID, use a \[InstanceGroupService.List\] request.
    #[prost(string, tag="1")]
    pub instance_group_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartInstanceGroupMetadata {
    /// ID of the InstanceGroup resource that is being started.
    #[prost(string, tag="1")]
    pub instance_group_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopInstanceGroupRequest {
    /// ID of the instance group to stop.
    /// To get the instance group ID, use a \[InstanceGroupService.List\] request.
    #[prost(string, tag="1")]
    pub instance_group_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopInstanceGroupMetadata {
    /// ID of the InstanceGroup resource that is being stopped.
    #[prost(string, tag="1")]
    pub instance_group_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteInstanceGroupRequest {
    /// ID of the instance group to delete.
    /// To get the instance group ID, use a \[InstanceGroupService.List\] request.
    #[prost(string, tag="1")]
    pub instance_group_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteInstanceGroupMetadata {
    /// ID of the instance group that is being deleted.
    /// To get the instance group ID, use a \[InstanceGroupService.List\] request.
    #[prost(string, tag="1")]
    pub instance_group_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteInstancesMetadata {
    /// ID of the instance group that the instances are being deleted from.
    #[prost(string, tag="1")]
    pub instance_group_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopInstancesMetadata {
    /// ID of the instance group that the instances are being stopped from.
    #[prost(string, tag="1")]
    pub instance_group_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstanceGroupsRequest {
    /// ID of the folder to list instance groups in.
    /// To get the folder ID, use a \[yandex.cloud.resourcemanager.v1.FolderService.List\] request.
    #[prost(string, tag="1")]
    pub folder_id: ::prost::alloc::string::String,
    /// The maximum number of results per page to return. If the number of available
    /// results is larger than \[page_size\],
    /// the service returns a \[ListInstanceGroupsResponse.next_page_token\]
    /// that can be used to get the next page of results in subsequent list requests.
    #[prost(int64, tag="2")]
    pub page_size: i64,
    /// Page token. To get the next page of results,
    /// set \[page_token\] to the \[ListInstanceGroupsResponse.next_page_token\]
    /// returned by a previous list request.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// A filter expression that filters resources listed in the response.
    /// Currently you can use filtering only on the \[InstanceGroup.name\] field.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
    /// Defines which information about the Instance template should be returned in the server response.
    #[prost(enumeration="InstanceGroupView", tag="5")]
    pub view: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstanceGroupsResponse {
    /// Lists instance groups for the specified folder.
    #[prost(message, repeated, tag="1")]
    pub instance_groups: ::prost::alloc::vec::Vec<InstanceGroup>,
    /// This token allows you to get the next page of results for list requests. If the number of results
    /// is larger than \[ListInstanceGroupsRequest.page_size\], use
    /// \[next_page_token\] as the value
    /// for the \[ListInstanceGroupsRequest.page_token\] query parameter
    /// in the next list request. Each subsequent list request will have its own
    /// \[next_page_token\] to continue paging through the results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstanceGroupInstancesRequest {
    /// ID of the InstanceGroup resource to list instances for.
    /// To get the instance group ID, use a \[InstanceGroupService.List\] request.
    #[prost(string, tag="1")]
    pub instance_group_id: ::prost::alloc::string::String,
    /// The maximum number of results per page to return. If the number of available
    /// results is larger than \[page_size\],
    /// the service returns a \[ListInstanceGroupInstancesResponse.next_page_token\]
    /// that can be used to get the next page of results in subsequent list requests.
    #[prost(int64, tag="2")]
    pub page_size: i64,
    /// Page token. To get the next page of results,
    /// set \[page_token\] to the \[ListInstanceGroupInstancesResponse.next_page_token\]
    /// returned by a previous list request.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// A filter expression that filters resources listed in the response.
    /// Currently you can use filtering only on the \[ManagedInstance.name\] field.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstanceGroupInstancesResponse {
    /// Lists instances for the specified instance group.
    #[prost(message, repeated, tag="1")]
    pub instances: ::prost::alloc::vec::Vec<ManagedInstance>,
    /// This token allows you to get the next page of results for list requests. If the number of results
    /// is more than \[ListInstanceGroupInstancesRequest.page_size\], use
    /// \[next_page_token\] as the value
    /// for the \[ListInstanceGroupInstancesRequest.page_token\] query parameter
    /// in the next list request. Each subsequent list request will have its own
    /// \[next_page_token\] to continue paging through the results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteInstancesRequest {
    /// ID of the instance group that the instances are being deleted from.
    /// To get the ID of the instance group, use the \[InstanceGroupService.List\] request.
    #[prost(string, tag="1")]
    pub instance_group_id: ::prost::alloc::string::String,
    /// IDs of the instances to delete. Instances will be deleted along with all dependent resources.
    /// Only IDs from the ManagedInstance.id field are allowed, not ManagedInstance.instance_id.
    #[prost(string, repeated, tag="2")]
    pub managed_instance_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// If set to true, the target size of instance group will not be reduced and
    /// a new instance will be created instead of the deleted one. By default, the target size of instance group
    /// will be reduced by the specified number of instance IDs.
    #[prost(bool, tag="3")]
    pub create_another: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopInstancesRequest {
    /// ID of the instance group that the instances are being stopped from.
    /// To get the ID of the instance group, use the \[InstanceGroupService.List\] request.
    #[prost(string, tag="1")]
    pub instance_group_id: ::prost::alloc::string::String,
    /// IDs of the instances to stop. After stopping, the instance can be updated, started, or deleted
    /// according to scale and deploy policies.
    /// Only IDs from the ManagedInstance.id field are allowed, not ManagedInstance.instance_id.
    #[prost(string, repeated, tag="2")]
    pub managed_instance_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstanceGroupOperationsRequest {
    /// ID of the InstanceGroup resource to list operations for.
    /// To get the instance group ID, use a \[InstanceGroupService.List\] request.
    #[prost(string, tag="1")]
    pub instance_group_id: ::prost::alloc::string::String,
    /// The maximum number of results per page to return. If the number of available
    /// results is more than \[page_size\], the service returns a \[ListInstanceGroupOperationsResponse.next_page_token\]
    /// that can be used to get the next page of results in subsequent list requests.
    #[prost(int64, tag="2")]
    pub page_size: i64,
    /// Page token. To get the next page of results, set \[page_token\] to the
    /// \[ListInstanceGroupOperationsResponse.next_page_token\] returned by a previous list request.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// A filter expression that filters resources listed in the response.
    /// Currently you can use filtering only on the \[InstanceGroup.name\] field.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstanceGroupOperationsResponse {
    /// Lists operations for the specified instance group.
    #[prost(message, repeated, tag="1")]
    pub operations: ::prost::alloc::vec::Vec<super::super::super::operation::Operation>,
    /// This token allows you to get the next page of results for list requests. If the number of results
    /// is more than \[ListInstanceGroupOperationsRequest.page_size\], use the \[next_page_token\] as the value
    /// for the \[ListInstanceGroupOperationsRequest.page_token\] query parameter in the next list request.
    /// Each subsequent list request will have its own \[next_page_token\] to continue paging through the results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstanceGroupLogRecordsRequest {
    /// ID of the InstanceGroup resource to list logs for.
    /// To get the instance group ID, use a \[InstanceGroupService.List\] request.
    #[prost(string, tag="1")]
    pub instance_group_id: ::prost::alloc::string::String,
    /// The maximum number of results per page to return. If the number of available
    /// results is larger than \[page_size\],
    /// the service returns a \[ListInstanceGroupLogRecordsResponse.next_page_token\]
    /// that can be used to get the next page of results in subsequent list requests.
    #[prost(int64, tag="2")]
    pub page_size: i64,
    /// Page token. To get the next page of results,
    /// set \[page_token\] to the \[ListInstanceGroupLogRecordsResponse.next_page_token\]
    /// returned by a previous list request.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// A filter expression that filters resources listed in the response.
    /// Currently you can use filtering only on the \[InstanceGroup.name\] field.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstanceGroupLogRecordsResponse {
    /// Lists logs for the specified instance group.
    #[prost(message, repeated, tag="1")]
    pub log_records: ::prost::alloc::vec::Vec<LogRecord>,
    /// This token allows you to get the next page of results for list requests. If the number of results
    /// is larger than \[ListInstanceGroupLogRecordsRequest.page_size\], use
    /// \[next_page_token\] as the value
    /// for the \[ListInstanceGroupLogRecordsRequest.page_token\] query parameter
    /// in the next list request. Each subsequent list request will have its own
    /// \[next_page_token\] to continue paging through the results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InstanceGroupView {
    /// Doesn't include the metadata of the instance template in the server response.
    Basic = 0,
    /// Returns the metadata of the instance template in the server response.
    Full = 1,
}
/// Generated client implementations.
pub mod instance_group_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// A set of methods for managing InstanceGroup resources.
    #[derive(Debug, Clone)]
    pub struct InstanceGroupServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl InstanceGroupServiceClient<tonic::transport::Channel> {
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
    impl<T> InstanceGroupServiceClient<T>
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
        ) -> InstanceGroupServiceClient<InterceptedService<T, F>>
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
            InstanceGroupServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Returns the specified InstanceGroup resource.
        ///
        /// To get the list of available InstanceGroup resources, make a [List] request.
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInstanceGroupRequest>,
        ) -> Result<tonic::Response<super::InstanceGroup>, tonic::Status> {
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
                "/yandex.cloud.compute.v1.instancegroup.InstanceGroupService/Get",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Retrieves the list of InstanceGroup resources in the specified folder.
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInstanceGroupsRequest>,
        ) -> Result<tonic::Response<super::ListInstanceGroupsResponse>, tonic::Status> {
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
                "/yandex.cloud.compute.v1.instancegroup.InstanceGroupService/List",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates an instance group in the specified folder.
        /// This method starts an operation that can be cancelled by another operation.
        pub async fn create(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateInstanceGroupRequest>,
        ) -> Result<
                tonic::Response<super::super::super::super::operation::Operation>,
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
                "/yandex.cloud.compute.v1.instancegroup.InstanceGroupService/Create",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates an instance group in the specified folder from a YAML file.
        /// This method starts an operation that can be cancelled by another operation.
        pub async fn create_from_yaml(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateInstanceGroupFromYamlRequest>,
        ) -> Result<
                tonic::Response<super::super::super::super::operation::Operation>,
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
                "/yandex.cloud.compute.v1.instancegroup.InstanceGroupService/CreateFromYaml",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the specified instance group.
        /// This method starts an operation that can be cancelled by another operation.
        pub async fn update(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateInstanceGroupRequest>,
        ) -> Result<
                tonic::Response<super::super::super::super::operation::Operation>,
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
                "/yandex.cloud.compute.v1.instancegroup.InstanceGroupService/Update",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the specified instance group from a YAML file.
        /// This method starts an operation that can be cancelled by another operation.
        pub async fn update_from_yaml(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateInstanceGroupFromYamlRequest>,
        ) -> Result<
                tonic::Response<super::super::super::super::operation::Operation>,
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
                "/yandex.cloud.compute.v1.instancegroup.InstanceGroupService/UpdateFromYaml",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Stops the specified instance group.
        pub async fn stop(
            &mut self,
            request: impl tonic::IntoRequest<super::StopInstanceGroupRequest>,
        ) -> Result<
                tonic::Response<super::super::super::super::operation::Operation>,
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
                "/yandex.cloud.compute.v1.instancegroup.InstanceGroupService/Stop",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Starts the specified instance group.
        pub async fn start(
            &mut self,
            request: impl tonic::IntoRequest<super::StartInstanceGroupRequest>,
        ) -> Result<
                tonic::Response<super::super::super::super::operation::Operation>,
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
                "/yandex.cloud.compute.v1.instancegroup.InstanceGroupService/Start",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes the specified instance group.
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteInstanceGroupRequest>,
        ) -> Result<
                tonic::Response<super::super::super::super::operation::Operation>,
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
                "/yandex.cloud.compute.v1.instancegroup.InstanceGroupService/Delete",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists instances for the specified instance group.
        pub async fn list_instances(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInstanceGroupInstancesRequest>,
        ) -> Result<
                tonic::Response<super::ListInstanceGroupInstancesResponse>,
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
                "/yandex.cloud.compute.v1.instancegroup.InstanceGroupService/ListInstances",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Delete instances from the instance group.
        pub async fn delete_instances(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteInstancesRequest>,
        ) -> Result<
                tonic::Response<super::super::super::super::operation::Operation>,
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
                "/yandex.cloud.compute.v1.instancegroup.InstanceGroupService/DeleteInstances",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Stop instances from the instance group.
        pub async fn stop_instances(
            &mut self,
            request: impl tonic::IntoRequest<super::StopInstancesRequest>,
        ) -> Result<
                tonic::Response<super::super::super::super::operation::Operation>,
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
                "/yandex.cloud.compute.v1.instancegroup.InstanceGroupService/StopInstances",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists operations for the specified instance group.
        pub async fn list_operations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInstanceGroupOperationsRequest>,
        ) -> Result<
                tonic::Response<super::ListInstanceGroupOperationsResponse>,
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
                "/yandex.cloud.compute.v1.instancegroup.InstanceGroupService/ListOperations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists logs for the specified instance group.
        pub async fn list_log_records(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInstanceGroupLogRecordsRequest>,
        ) -> Result<
                tonic::Response<super::ListInstanceGroupLogRecordsResponse>,
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
                "/yandex.cloud.compute.v1.instancegroup.InstanceGroupService/ListLogRecords",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists existing access bindings for the specified instance group.
        pub async fn list_access_bindings(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::access::ListAccessBindingsRequest,
            >,
        ) -> Result<
                tonic::Response<
                    super::super::super::super::access::ListAccessBindingsResponse,
                >,
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
                "/yandex.cloud.compute.v1.instancegroup.InstanceGroupService/ListAccessBindings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets access bindings for the specified instance group.
        pub async fn set_access_bindings(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::access::SetAccessBindingsRequest,
            >,
        ) -> Result<
                tonic::Response<super::super::super::super::operation::Operation>,
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
                "/yandex.cloud.compute.v1.instancegroup.InstanceGroupService/SetAccessBindings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates access bindings for the specified instance group.
        pub async fn update_access_bindings(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::access::UpdateAccessBindingsRequest,
            >,
        ) -> Result<
                tonic::Response<super::super::super::super::operation::Operation>,
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
                "/yandex.cloud.compute.v1.instancegroup.InstanceGroupService/UpdateAccessBindings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Resumes all processes regarding management of the specified instance group,
        /// i.e. scaling, checking instances' health, auto-healing and updating them.
        pub async fn resume_processes(
            &mut self,
            request: impl tonic::IntoRequest<super::ResumeInstanceGroupProcessesRequest>,
        ) -> Result<
                tonic::Response<super::super::super::super::operation::Operation>,
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
                "/yandex.cloud.compute.v1.instancegroup.InstanceGroupService/ResumeProcesses",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Pauses all processes regarding management of the specified instance group,
        /// i.e. scaling, checking instances' health, auto-healing and updating them. Running instances are not stopped.
        pub async fn pause_processes(
            &mut self,
            request: impl tonic::IntoRequest<super::PauseInstanceGroupProcessesRequest>,
        ) -> Result<
                tonic::Response<super::super::super::super::operation::Operation>,
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
                "/yandex.cloud.compute.v1.instancegroup.InstanceGroupService/PauseProcesses",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
