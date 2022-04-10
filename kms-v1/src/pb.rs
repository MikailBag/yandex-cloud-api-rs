pub mod google {pub mod api {include!("gen/google.api.rs");}
pub mod rpc {include!("gen/google.rpc.rs");}
}
pub mod yandex {pub mod cloud {include!("gen/yandex.cloud.rs");pub mod access {include!("gen/yandex.cloud.access.rs");}
pub mod api {include!("gen/yandex.cloud.api.rs");}
pub mod kms {pub mod v1 {include!("gen/yandex.cloud.kms.v1.rs");}
}
pub mod operation {include!("gen/yandex.cloud.operation.rs");}
}
}
