//! Generated protobuf code for BuildKit gRPC API

pub mod moby {
    pub mod buildkit {
        pub mod v1 {
            pub mod frontend {
                tonic::include_proto!("moby.buildkit.v1.frontend");
            }

            pub mod apicaps {
                tonic::include_proto!("moby.buildkit.v1.apicaps");
            }

            pub mod types {
                tonic::include_proto!("moby.buildkit.v1.types");
            }
            pub mod sourcepolicy {
                tonic::include_proto!("moby.buildkit.v1.sourcepolicy");
            }
        }
    }
}

pub mod fsutil {
    pub mod types {
        tonic::include_proto!("fsutil.types");
    }
}

pub mod google {
    pub mod rpc {
        tonic::include_proto!("google.rpc");
    }
}

pub mod pb {
    tonic::include_proto!("pb");
}
