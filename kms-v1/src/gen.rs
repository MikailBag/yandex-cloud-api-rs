pub mod yandex {
    pub mod cloud {
        pub mod kms {
            pub mod v1 {
                include!("gen/yandex.cloud.kms.v1.rs");
            }
        }
        include!("gen/yandex.cloud.rs");
    }
}
pub mod google {
    pub mod api {
        include!("gen/google.api.rs");
    }
}
