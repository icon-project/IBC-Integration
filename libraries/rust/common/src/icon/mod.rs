// @generated
pub mod icon {
    pub mod lightclient {
        // @@protoc_insertion_point(attribute:icon.lightclient.v1)
        pub mod v1 {
            include!("icon.lightclient.v1.rs");
            // @@protoc_insertion_point(icon.lightclient.v1)
        }
    }
    pub mod proto {
        pub mod core {
            // @@protoc_insertion_point(attribute:icon.proto.core.channel)
            pub mod channel {
                include!("icon.proto.core.channel.rs");
                // @@protoc_insertion_point(icon.proto.core.channel)
            }
            // @@protoc_insertion_point(attribute:icon.proto.core.client)
            pub mod client {
                include!("icon.proto.core.client.rs");
                // @@protoc_insertion_point(icon.proto.core.client)
            }
            // @@protoc_insertion_point(attribute:icon.proto.core.connection)
            pub mod connection {
                include!("icon.proto.core.connection.rs");
                // @@protoc_insertion_point(icon.proto.core.connection)
            }
        }
    }
    pub mod types {
        // @@protoc_insertion_point(attribute:icon.types.v1)
        pub mod v1 {
            include!("icon.types.v1.rs");
            // @@protoc_insertion_point(icon.types.v1)
        }
    }
}
pub mod tendermint {
    // @@protoc_insertion_point(attribute:tendermint.light)
    pub mod light {
        include!("tendermint.light.rs");
        // @@protoc_insertion_point(tendermint.light)
    }
}
