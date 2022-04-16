pub mod google {
    pub mod api {
        include!("gen/google.api.rs");
    }
    pub mod rpc {
        include!("gen/google.rpc.rs");
    }
}
pub mod yandex {
    pub mod cloud {
        include!("gen/yandex.cloud.rs");
        pub mod access {
            include!("gen/yandex.cloud.access.rs");
        }
        pub mod api {
            include!("gen/yandex.cloud.api.rs");
        }
        pub mod compute {
            pub mod v1 {
                include!("gen/yandex.cloud.compute.v1.rs");
                pub mod instancegroup {
                    include!("gen/yandex.cloud.compute.v1.instancegroup.rs");
                }
            }
        }
        pub mod operation {
            include!("gen/yandex.cloud.operation.rs");
        }
    }
}
