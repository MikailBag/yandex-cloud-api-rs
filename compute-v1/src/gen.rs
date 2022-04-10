pub mod yandex {
    pub mod cloud {
        pub mod access {
            include!("gen/yandex.cloud.access.rs");
        }
        pub mod compute {
            pub mod v1 {
                pub mod instancegroup {
                    include!("gen/yandex.cloud.compute.v1.instancegroup.rs");
                }
                include!("gen/yandex.cloud.compute.v1.rs");
            }
        }
        pub mod operation {
            include!("gen/yandex.cloud.operation.rs");
        }
        include!("gen/yandex.cloud.rs");
    }
}
pub mod google {
    pub mod api {
        include!("gen/google.api.rs");
    }
    pub mod rpc {
        include!("gen/google.rpc.rs");
    }
}