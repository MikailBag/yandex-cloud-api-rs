pub mod google {
    pub mod api {
        include!("gen/google.api.rs");
    }
    pub mod r#type {
        include!("gen/google.r#type.rs");
    }
    pub mod rpc {
        include!("gen/google.rpc.rs");
    }
}
pub mod yandex {
    pub mod cloud {
        include!("gen/yandex.cloud.rs");
        pub mod api {
            include!("gen/yandex.cloud.api.rs");
        }
        pub mod operation {
            include!("gen/yandex.cloud.operation.rs");
        }
        pub mod ydb {
            pub mod v1 {
                include!("gen/yandex.cloud.ydb.v1.rs");
            }
        }
    }
}
