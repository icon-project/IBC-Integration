// @generated
impl serde::Serialize for ConnectionEnd {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.client_id.is_empty() {
            len += 1;
        }
        if !self.versions.is_empty() {
            len += 1;
        }
        if self.state != 0 {
            len += 1;
        }
        if self.counterparty.is_some() {
            len += 1;
        }
        if self.delay_period != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("icon.proto.core.connection.ConnectionEnd", len)?;
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("client_id", &self.client_id)?;
        }
        if !self.versions.is_empty() {
            struct_ser.serialize_field("versions", &self.versions)?;
        }
        if self.state != 0 {
            let v = connection_end::State::from_i32(self.state)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if let Some(v) = self.counterparty.as_ref() {
            struct_ser.serialize_field("counterparty", v)?;
        }
        if self.delay_period != 0 {
            struct_ser.serialize_field("delay_period", ToString::to_string(&self.delay_period).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConnectionEnd {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "client_id",
            "clientId",
            "versions",
            "state",
            "counterparty",
            "delay_period",
            "delayPeriod",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClientId,
            Versions,
            State,
            Counterparty,
            DelayPeriod,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
                            "versions" => Ok(GeneratedField::Versions),
                            "state" => Ok(GeneratedField::State),
                            "counterparty" => Ok(GeneratedField::Counterparty),
                            "delayPeriod" | "delay_period" => Ok(GeneratedField::DelayPeriod),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConnectionEnd;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct icon.proto.core.connection.ConnectionEnd")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ConnectionEnd, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut client_id__ = None;
                let mut versions__ = None;
                let mut state__ = None;
                let mut counterparty__ = None;
                let mut delay_period__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Versions => {
                            if versions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versions"));
                            }
                            versions__ = Some(map.next_value()?);
                        }
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map.next_value::<connection_end::State>()? as i32);
                        }
                        GeneratedField::Counterparty => {
                            if counterparty__.is_some() {
                                return Err(serde::de::Error::duplicate_field("counterparty"));
                            }
                            counterparty__ = map.next_value()?;
                        }
                        GeneratedField::DelayPeriod => {
                            if delay_period__.is_some() {
                                return Err(serde::de::Error::duplicate_field("delayPeriod"));
                            }
                            delay_period__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ConnectionEnd {
                    client_id: client_id__.unwrap_or_default(),
                    versions: versions__.unwrap_or_default(),
                    state: state__.unwrap_or_default(),
                    counterparty: counterparty__,
                    delay_period: delay_period__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("icon.proto.core.connection.ConnectionEnd", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for connection_end::State {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::StateUninitializedUnspecified => "STATE_UNINITIALIZED_UNSPECIFIED",
            Self::StateInit => "STATE_INIT",
            Self::StateTryopen => "STATE_TRYOPEN",
            Self::StateOpen => "STATE_OPEN",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for connection_end::State {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "STATE_UNINITIALIZED_UNSPECIFIED",
            "STATE_INIT",
            "STATE_TRYOPEN",
            "STATE_OPEN",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = connection_end::State;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(connection_end::State::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(connection_end::State::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "STATE_UNINITIALIZED_UNSPECIFIED" => Ok(connection_end::State::StateUninitializedUnspecified),
                    "STATE_INIT" => Ok(connection_end::State::StateInit),
                    "STATE_TRYOPEN" => Ok(connection_end::State::StateTryopen),
                    "STATE_OPEN" => Ok(connection_end::State::StateOpen),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Counterparty {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.client_id.is_empty() {
            len += 1;
        }
        if !self.connection_id.is_empty() {
            len += 1;
        }
        if self.prefix.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("icon.proto.core.connection.Counterparty", len)?;
        if !self.client_id.is_empty() {
            struct_ser.serialize_field("client_id", &self.client_id)?;
        }
        if !self.connection_id.is_empty() {
            struct_ser.serialize_field("connection_id", &self.connection_id)?;
        }
        if let Some(v) = self.prefix.as_ref() {
            struct_ser.serialize_field("prefix", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Counterparty {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "client_id",
            "clientId",
            "connection_id",
            "connectionId",
            "prefix",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClientId,
            ConnectionId,
            Prefix,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "clientId" | "client_id" => Ok(GeneratedField::ClientId),
                            "connectionId" | "connection_id" => Ok(GeneratedField::ConnectionId),
                            "prefix" => Ok(GeneratedField::Prefix),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Counterparty;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct icon.proto.core.connection.Counterparty")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Counterparty, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut client_id__ = None;
                let mut connection_id__ = None;
                let mut prefix__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ClientId => {
                            if client_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientId"));
                            }
                            client_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::ConnectionId => {
                            if connection_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("connectionId"));
                            }
                            connection_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Prefix => {
                            if prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefix"));
                            }
                            prefix__ = map.next_value()?;
                        }
                    }
                }
                Ok(Counterparty {
                    client_id: client_id__.unwrap_or_default(),
                    connection_id: connection_id__.unwrap_or_default(),
                    prefix: prefix__,
                })
            }
        }
        deserializer.deserialize_struct("icon.proto.core.connection.Counterparty", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Version {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.identifier.is_empty() {
            len += 1;
        }
        if !self.features.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("icon.proto.core.connection.Version", len)?;
        if !self.identifier.is_empty() {
            struct_ser.serialize_field("identifier", &self.identifier)?;
        }
        if !self.features.is_empty() {
            struct_ser.serialize_field("features", &self.features)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Version {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "identifier",
            "features",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Identifier,
            Features,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "identifier" => Ok(GeneratedField::Identifier),
                            "features" => Ok(GeneratedField::Features),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Version;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct icon.proto.core.connection.Version")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Version, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut identifier__ = None;
                let mut features__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Identifier => {
                            if identifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            identifier__ = Some(map.next_value()?);
                        }
                        GeneratedField::Features => {
                            if features__.is_some() {
                                return Err(serde::de::Error::duplicate_field("features"));
                            }
                            features__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Version {
                    identifier: identifier__.unwrap_or_default(),
                    features: features__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("icon.proto.core.connection.Version", FIELDS, GeneratedVisitor)
    }
}
