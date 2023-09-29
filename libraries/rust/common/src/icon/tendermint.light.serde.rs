// @generated
impl serde::Serialize for BlockId {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.hash.is_empty() {
            len += 1;
        }
        if self.part_set_header.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tendermint.light.BlockID", len)?;
        if !self.hash.is_empty() {
            struct_ser.serialize_field("hash", pbjson::private::base64::encode(&self.hash).as_str())?;
        }
        if let Some(v) = self.part_set_header.as_ref() {
            struct_ser.serialize_field("part_set_header", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BlockId {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "hash",
            "part_set_header",
            "partSetHeader",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Hash,
            PartSetHeader,
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
                            "hash" => Ok(GeneratedField::Hash),
                            "partSetHeader" | "part_set_header" => Ok(GeneratedField::PartSetHeader),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BlockId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tendermint.light.BlockID")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BlockId, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut hash__ = None;
                let mut part_set_header__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PartSetHeader => {
                            if part_set_header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("partSetHeader"));
                            }
                            part_set_header__ = map.next_value()?;
                        }
                    }
                }
                Ok(BlockId {
                    hash: hash__.unwrap_or_default(),
                    part_set_header: part_set_header__,
                })
            }
        }
        deserializer.deserialize_struct("tendermint.light.BlockID", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BlockIdFlag {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::BlockIdFlagUnknown => "BLOCK_ID_FLAG_UNKNOWN",
            Self::BlockIdFlagAbsent => "BLOCK_ID_FLAG_ABSENT",
            Self::BlockIdFlagCommit => "BLOCK_ID_FLAG_COMMIT",
            Self::BlockIdFlagNil => "BLOCK_ID_FLAG_NIL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for BlockIdFlag {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "BLOCK_ID_FLAG_UNKNOWN",
            "BLOCK_ID_FLAG_ABSENT",
            "BLOCK_ID_FLAG_COMMIT",
            "BLOCK_ID_FLAG_NIL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BlockIdFlag;

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
                    .and_then(BlockIdFlag::from_i32)
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
                    .and_then(BlockIdFlag::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "BLOCK_ID_FLAG_UNKNOWN" => Ok(BlockIdFlag::BlockIdFlagUnknown),
                    "BLOCK_ID_FLAG_ABSENT" => Ok(BlockIdFlag::BlockIdFlagAbsent),
                    "BLOCK_ID_FLAG_COMMIT" => Ok(BlockIdFlag::BlockIdFlagCommit),
                    "BLOCK_ID_FLAG_NIL" => Ok(BlockIdFlag::BlockIdFlagNil),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for CanonicalBlockId {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.hash.is_empty() {
            len += 1;
        }
        if self.part_set_header.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tendermint.light.CanonicalBlockID", len)?;
        if !self.hash.is_empty() {
            struct_ser.serialize_field("hash", pbjson::private::base64::encode(&self.hash).as_str())?;
        }
        if let Some(v) = self.part_set_header.as_ref() {
            struct_ser.serialize_field("part_set_header", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CanonicalBlockId {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "hash",
            "part_set_header",
            "partSetHeader",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Hash,
            PartSetHeader,
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
                            "hash" => Ok(GeneratedField::Hash),
                            "partSetHeader" | "part_set_header" => Ok(GeneratedField::PartSetHeader),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CanonicalBlockId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tendermint.light.CanonicalBlockID")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CanonicalBlockId, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut hash__ = None;
                let mut part_set_header__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PartSetHeader => {
                            if part_set_header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("partSetHeader"));
                            }
                            part_set_header__ = map.next_value()?;
                        }
                    }
                }
                Ok(CanonicalBlockId {
                    hash: hash__.unwrap_or_default(),
                    part_set_header: part_set_header__,
                })
            }
        }
        deserializer.deserialize_struct("tendermint.light.CanonicalBlockID", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CanonicalPartSetHeader {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.total != 0 {
            len += 1;
        }
        if !self.hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tendermint.light.CanonicalPartSetHeader", len)?;
        if self.total != 0 {
            struct_ser.serialize_field("total", &self.total)?;
        }
        if !self.hash.is_empty() {
            struct_ser.serialize_field("hash", pbjson::private::base64::encode(&self.hash).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CanonicalPartSetHeader {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "total",
            "hash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Total,
            Hash,
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
                            "total" => Ok(GeneratedField::Total),
                            "hash" => Ok(GeneratedField::Hash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CanonicalPartSetHeader;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tendermint.light.CanonicalPartSetHeader")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CanonicalPartSetHeader, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut total__ = None;
                let mut hash__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Total => {
                            if total__.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            total__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(CanonicalPartSetHeader {
                    total: total__.unwrap_or_default(),
                    hash: hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("tendermint.light.CanonicalPartSetHeader", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CanonicalVote {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type != 0 {
            len += 1;
        }
        if self.height != 0 {
            len += 1;
        }
        if self.round != 0 {
            len += 1;
        }
        if self.block_id.is_some() {
            len += 1;
        }
        if self.timestamp.is_some() {
            len += 1;
        }
        if !self.chain_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tendermint.light.CanonicalVote", len)?;
        if self.r#type != 0 {
            let v = SignedMsgType::from_i32(self.r#type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if self.height != 0 {
            struct_ser.serialize_field("height", ToString::to_string(&self.height).as_str())?;
        }
        if self.round != 0 {
            struct_ser.serialize_field("round", ToString::to_string(&self.round).as_str())?;
        }
        if let Some(v) = self.block_id.as_ref() {
            struct_ser.serialize_field("block_id", v)?;
        }
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", v)?;
        }
        if !self.chain_id.is_empty() {
            struct_ser.serialize_field("chain_id", &self.chain_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CanonicalVote {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "height",
            "round",
            "block_id",
            "blockId",
            "timestamp",
            "chain_id",
            "chainId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            Height,
            Round,
            BlockId,
            Timestamp,
            ChainId,
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
                            "type" => Ok(GeneratedField::Type),
                            "height" => Ok(GeneratedField::Height),
                            "round" => Ok(GeneratedField::Round),
                            "blockId" | "block_id" => Ok(GeneratedField::BlockId),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CanonicalVote;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tendermint.light.CanonicalVote")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CanonicalVote, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut height__ = None;
                let mut round__ = None;
                let mut block_id__ = None;
                let mut timestamp__ = None;
                let mut chain_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map.next_value::<SignedMsgType>()? as i32);
                        }
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Round => {
                            if round__.is_some() {
                                return Err(serde::de::Error::duplicate_field("round"));
                            }
                            round__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BlockId => {
                            if block_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockId"));
                            }
                            block_id__ = map.next_value()?;
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = map.next_value()?;
                        }
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CanonicalVote {
                    r#type: r#type__.unwrap_or_default(),
                    height: height__.unwrap_or_default(),
                    round: round__.unwrap_or_default(),
                    block_id: block_id__,
                    timestamp: timestamp__,
                    chain_id: chain_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("tendermint.light.CanonicalVote", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClientState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.chain_id.is_empty() {
            len += 1;
        }
        if self.trust_level.is_some() {
            len += 1;
        }
        if self.trusting_period.is_some() {
            len += 1;
        }
        if self.unbonding_period.is_some() {
            len += 1;
        }
        if self.max_clock_drift.is_some() {
            len += 1;
        }
        if self.frozen_height != 0 {
            len += 1;
        }
        if self.latest_height != 0 {
            len += 1;
        }
        if self.allow_update_after_expiry {
            len += 1;
        }
        if self.allow_update_after_misbehaviour {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tendermint.light.ClientState", len)?;
        if !self.chain_id.is_empty() {
            struct_ser.serialize_field("chain_id", &self.chain_id)?;
        }
        if let Some(v) = self.trust_level.as_ref() {
            struct_ser.serialize_field("trust_level", v)?;
        }
        if let Some(v) = self.trusting_period.as_ref() {
            struct_ser.serialize_field("trusting_period", v)?;
        }
        if let Some(v) = self.unbonding_period.as_ref() {
            struct_ser.serialize_field("unbonding_period", v)?;
        }
        if let Some(v) = self.max_clock_drift.as_ref() {
            struct_ser.serialize_field("max_clock_drift", v)?;
        }
        if self.frozen_height != 0 {
            struct_ser.serialize_field("frozen_height", ToString::to_string(&self.frozen_height).as_str())?;
        }
        if self.latest_height != 0 {
            struct_ser.serialize_field("latest_height", ToString::to_string(&self.latest_height).as_str())?;
        }
        if self.allow_update_after_expiry {
            struct_ser.serialize_field("allow_update_after_expiry", &self.allow_update_after_expiry)?;
        }
        if self.allow_update_after_misbehaviour {
            struct_ser.serialize_field("allow_update_after_misbehaviour", &self.allow_update_after_misbehaviour)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClientState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "trust_level",
            "trustLevel",
            "trusting_period",
            "trustingPeriod",
            "unbonding_period",
            "unbondingPeriod",
            "max_clock_drift",
            "maxClockDrift",
            "frozen_height",
            "frozenHeight",
            "latest_height",
            "latestHeight",
            "allow_update_after_expiry",
            "allowUpdateAfterExpiry",
            "allow_update_after_misbehaviour",
            "allowUpdateAfterMisbehaviour",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            TrustLevel,
            TrustingPeriod,
            UnbondingPeriod,
            MaxClockDrift,
            FrozenHeight,
            LatestHeight,
            AllowUpdateAfterExpiry,
            AllowUpdateAfterMisbehaviour,
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
                            "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
                            "trustLevel" | "trust_level" => Ok(GeneratedField::TrustLevel),
                            "trustingPeriod" | "trusting_period" => Ok(GeneratedField::TrustingPeriod),
                            "unbondingPeriod" | "unbonding_period" => Ok(GeneratedField::UnbondingPeriod),
                            "maxClockDrift" | "max_clock_drift" => Ok(GeneratedField::MaxClockDrift),
                            "frozenHeight" | "frozen_height" => Ok(GeneratedField::FrozenHeight),
                            "latestHeight" | "latest_height" => Ok(GeneratedField::LatestHeight),
                            "allowUpdateAfterExpiry" | "allow_update_after_expiry" => Ok(GeneratedField::AllowUpdateAfterExpiry),
                            "allowUpdateAfterMisbehaviour" | "allow_update_after_misbehaviour" => Ok(GeneratedField::AllowUpdateAfterMisbehaviour),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClientState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tendermint.light.ClientState")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ClientState, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut trust_level__ = None;
                let mut trusting_period__ = None;
                let mut unbonding_period__ = None;
                let mut max_clock_drift__ = None;
                let mut frozen_height__ = None;
                let mut latest_height__ = None;
                let mut allow_update_after_expiry__ = None;
                let mut allow_update_after_misbehaviour__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::TrustLevel => {
                            if trust_level__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trustLevel"));
                            }
                            trust_level__ = map.next_value()?;
                        }
                        GeneratedField::TrustingPeriod => {
                            if trusting_period__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trustingPeriod"));
                            }
                            trusting_period__ = map.next_value()?;
                        }
                        GeneratedField::UnbondingPeriod => {
                            if unbonding_period__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unbondingPeriod"));
                            }
                            unbonding_period__ = map.next_value()?;
                        }
                        GeneratedField::MaxClockDrift => {
                            if max_clock_drift__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxClockDrift"));
                            }
                            max_clock_drift__ = map.next_value()?;
                        }
                        GeneratedField::FrozenHeight => {
                            if frozen_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("frozenHeight"));
                            }
                            frozen_height__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::LatestHeight => {
                            if latest_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("latestHeight"));
                            }
                            latest_height__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AllowUpdateAfterExpiry => {
                            if allow_update_after_expiry__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowUpdateAfterExpiry"));
                            }
                            allow_update_after_expiry__ = Some(map.next_value()?);
                        }
                        GeneratedField::AllowUpdateAfterMisbehaviour => {
                            if allow_update_after_misbehaviour__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowUpdateAfterMisbehaviour"));
                            }
                            allow_update_after_misbehaviour__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ClientState {
                    chain_id: chain_id__.unwrap_or_default(),
                    trust_level: trust_level__,
                    trusting_period: trusting_period__,
                    unbonding_period: unbonding_period__,
                    max_clock_drift: max_clock_drift__,
                    frozen_height: frozen_height__.unwrap_or_default(),
                    latest_height: latest_height__.unwrap_or_default(),
                    allow_update_after_expiry: allow_update_after_expiry__.unwrap_or_default(),
                    allow_update_after_misbehaviour: allow_update_after_misbehaviour__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("tendermint.light.ClientState", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Commit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.height != 0 {
            len += 1;
        }
        if self.round != 0 {
            len += 1;
        }
        if self.block_id.is_some() {
            len += 1;
        }
        if !self.signatures.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tendermint.light.Commit", len)?;
        if self.height != 0 {
            struct_ser.serialize_field("height", ToString::to_string(&self.height).as_str())?;
        }
        if self.round != 0 {
            struct_ser.serialize_field("round", &self.round)?;
        }
        if let Some(v) = self.block_id.as_ref() {
            struct_ser.serialize_field("block_id", v)?;
        }
        if !self.signatures.is_empty() {
            struct_ser.serialize_field("signatures", &self.signatures)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Commit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "height",
            "round",
            "block_id",
            "blockId",
            "signatures",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Height,
            Round,
            BlockId,
            Signatures,
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
                            "height" => Ok(GeneratedField::Height),
                            "round" => Ok(GeneratedField::Round),
                            "blockId" | "block_id" => Ok(GeneratedField::BlockId),
                            "signatures" => Ok(GeneratedField::Signatures),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Commit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tendermint.light.Commit")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Commit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut height__ = None;
                let mut round__ = None;
                let mut block_id__ = None;
                let mut signatures__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Round => {
                            if round__.is_some() {
                                return Err(serde::de::Error::duplicate_field("round"));
                            }
                            round__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BlockId => {
                            if block_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockId"));
                            }
                            block_id__ = map.next_value()?;
                        }
                        GeneratedField::Signatures => {
                            if signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signatures"));
                            }
                            signatures__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Commit {
                    height: height__.unwrap_or_default(),
                    round: round__.unwrap_or_default(),
                    block_id: block_id__,
                    signatures: signatures__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("tendermint.light.Commit", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CommitSig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.block_id_flag != 0 {
            len += 1;
        }
        if !self.validator_address.is_empty() {
            len += 1;
        }
        if self.timestamp.is_some() {
            len += 1;
        }
        if !self.signature.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tendermint.light.CommitSig", len)?;
        if self.block_id_flag != 0 {
            let v = BlockIdFlag::from_i32(self.block_id_flag)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.block_id_flag)))?;
            struct_ser.serialize_field("block_id_flag", &v)?;
        }
        if !self.validator_address.is_empty() {
            struct_ser.serialize_field("validator_address", pbjson::private::base64::encode(&self.validator_address).as_str())?;
        }
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", v)?;
        }
        if !self.signature.is_empty() {
            struct_ser.serialize_field("signature", pbjson::private::base64::encode(&self.signature).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CommitSig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "block_id_flag",
            "blockIdFlag",
            "validator_address",
            "validatorAddress",
            "timestamp",
            "signature",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BlockIdFlag,
            ValidatorAddress,
            Timestamp,
            Signature,
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
                            "blockIdFlag" | "block_id_flag" => Ok(GeneratedField::BlockIdFlag),
                            "validatorAddress" | "validator_address" => Ok(GeneratedField::ValidatorAddress),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "signature" => Ok(GeneratedField::Signature),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CommitSig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tendermint.light.CommitSig")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CommitSig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut block_id_flag__ = None;
                let mut validator_address__ = None;
                let mut timestamp__ = None;
                let mut signature__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BlockIdFlag => {
                            if block_id_flag__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockIdFlag"));
                            }
                            block_id_flag__ = Some(map.next_value::<BlockIdFlag>()? as i32);
                        }
                        GeneratedField::ValidatorAddress => {
                            if validator_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validatorAddress"));
                            }
                            validator_address__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = map.next_value()?;
                        }
                        GeneratedField::Signature => {
                            if signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signature"));
                            }
                            signature__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(CommitSig {
                    block_id_flag: block_id_flag__.unwrap_or_default(),
                    validator_address: validator_address__.unwrap_or_default(),
                    timestamp: timestamp__,
                    signature: signature__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("tendermint.light.CommitSig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Consensus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.block != 0 {
            len += 1;
        }
        if self.app != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tendermint.light.Consensus", len)?;
        if self.block != 0 {
            struct_ser.serialize_field("block", ToString::to_string(&self.block).as_str())?;
        }
        if self.app != 0 {
            struct_ser.serialize_field("app", ToString::to_string(&self.app).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Consensus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "block",
            "app",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Block,
            App,
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
                            "block" => Ok(GeneratedField::Block),
                            "app" => Ok(GeneratedField::App),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Consensus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tendermint.light.Consensus")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Consensus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut block__ = None;
                let mut app__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Block => {
                            if block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("block"));
                            }
                            block__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::App => {
                            if app__.is_some() {
                                return Err(serde::de::Error::duplicate_field("app"));
                            }
                            app__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Consensus {
                    block: block__.unwrap_or_default(),
                    app: app__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("tendermint.light.Consensus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ConsensusState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.timestamp.is_some() {
            len += 1;
        }
        if self.root.is_some() {
            len += 1;
        }
        if !self.next_validators_hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tendermint.light.ConsensusState", len)?;
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", v)?;
        }
        if let Some(v) = self.root.as_ref() {
            struct_ser.serialize_field("root", v)?;
        }
        if !self.next_validators_hash.is_empty() {
            struct_ser.serialize_field("next_validators_hash", pbjson::private::base64::encode(&self.next_validators_hash).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConsensusState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "timestamp",
            "root",
            "next_validators_hash",
            "nextValidatorsHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Timestamp,
            Root,
            NextValidatorsHash,
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
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "root" => Ok(GeneratedField::Root),
                            "nextValidatorsHash" | "next_validators_hash" => Ok(GeneratedField::NextValidatorsHash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConsensusState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tendermint.light.ConsensusState")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ConsensusState, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut timestamp__ = None;
                let mut root__ = None;
                let mut next_validators_hash__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = map.next_value()?;
                        }
                        GeneratedField::Root => {
                            if root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("root"));
                            }
                            root__ = map.next_value()?;
                        }
                        GeneratedField::NextValidatorsHash => {
                            if next_validators_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextValidatorsHash"));
                            }
                            next_validators_hash__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ConsensusState {
                    timestamp: timestamp__,
                    root: root__,
                    next_validators_hash: next_validators_hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("tendermint.light.ConsensusState", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Duration {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.seconds != 0 {
            len += 1;
        }
        if self.nanos != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tendermint.light.Duration", len)?;
        if self.seconds != 0 {
            struct_ser.serialize_field("seconds", ToString::to_string(&self.seconds).as_str())?;
        }
        if self.nanos != 0 {
            struct_ser.serialize_field("nanos", &self.nanos)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Duration {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "seconds",
            "nanos",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Seconds,
            Nanos,
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
                            "seconds" => Ok(GeneratedField::Seconds),
                            "nanos" => Ok(GeneratedField::Nanos),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Duration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tendermint.light.Duration")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Duration, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut seconds__ = None;
                let mut nanos__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Seconds => {
                            if seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("seconds"));
                            }
                            seconds__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Nanos => {
                            if nanos__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nanos"));
                            }
                            nanos__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Duration {
                    seconds: seconds__.unwrap_or_default(),
                    nanos: nanos__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("tendermint.light.Duration", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Fraction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.numerator != 0 {
            len += 1;
        }
        if self.denominator != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tendermint.light.Fraction", len)?;
        if self.numerator != 0 {
            struct_ser.serialize_field("numerator", ToString::to_string(&self.numerator).as_str())?;
        }
        if self.denominator != 0 {
            struct_ser.serialize_field("denominator", ToString::to_string(&self.denominator).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Fraction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "numerator",
            "denominator",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Numerator,
            Denominator,
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
                            "numerator" => Ok(GeneratedField::Numerator),
                            "denominator" => Ok(GeneratedField::Denominator),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Fraction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tendermint.light.Fraction")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Fraction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut numerator__ = None;
                let mut denominator__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Numerator => {
                            if numerator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numerator"));
                            }
                            numerator__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Denominator => {
                            if denominator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("denominator"));
                            }
                            denominator__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Fraction {
                    numerator: numerator__.unwrap_or_default(),
                    denominator: denominator__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("tendermint.light.Fraction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LightHeader {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.version.is_some() {
            len += 1;
        }
        if !self.chain_id.is_empty() {
            len += 1;
        }
        if self.height != 0 {
            len += 1;
        }
        if self.time.is_some() {
            len += 1;
        }
        if self.last_block_id.is_some() {
            len += 1;
        }
        if !self.last_commit_hash.is_empty() {
            len += 1;
        }
        if !self.data_hash.is_empty() {
            len += 1;
        }
        if !self.validators_hash.is_empty() {
            len += 1;
        }
        if !self.next_validators_hash.is_empty() {
            len += 1;
        }
        if !self.consensus_hash.is_empty() {
            len += 1;
        }
        if !self.app_hash.is_empty() {
            len += 1;
        }
        if !self.last_results_hash.is_empty() {
            len += 1;
        }
        if !self.evidence_hash.is_empty() {
            len += 1;
        }
        if !self.proposer_address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tendermint.light.LightHeader", len)?;
        if let Some(v) = self.version.as_ref() {
            struct_ser.serialize_field("version", v)?;
        }
        if !self.chain_id.is_empty() {
            struct_ser.serialize_field("chain_id", &self.chain_id)?;
        }
        if self.height != 0 {
            struct_ser.serialize_field("height", ToString::to_string(&self.height).as_str())?;
        }
        if let Some(v) = self.time.as_ref() {
            struct_ser.serialize_field("time", v)?;
        }
        if let Some(v) = self.last_block_id.as_ref() {
            struct_ser.serialize_field("last_block_id", v)?;
        }
        if !self.last_commit_hash.is_empty() {
            struct_ser.serialize_field("last_commit_hash", pbjson::private::base64::encode(&self.last_commit_hash).as_str())?;
        }
        if !self.data_hash.is_empty() {
            struct_ser.serialize_field("data_hash", pbjson::private::base64::encode(&self.data_hash).as_str())?;
        }
        if !self.validators_hash.is_empty() {
            struct_ser.serialize_field("validators_hash", pbjson::private::base64::encode(&self.validators_hash).as_str())?;
        }
        if !self.next_validators_hash.is_empty() {
            struct_ser.serialize_field("next_validators_hash", pbjson::private::base64::encode(&self.next_validators_hash).as_str())?;
        }
        if !self.consensus_hash.is_empty() {
            struct_ser.serialize_field("consensus_hash", pbjson::private::base64::encode(&self.consensus_hash).as_str())?;
        }
        if !self.app_hash.is_empty() {
            struct_ser.serialize_field("app_hash", pbjson::private::base64::encode(&self.app_hash).as_str())?;
        }
        if !self.last_results_hash.is_empty() {
            struct_ser.serialize_field("last_results_hash", pbjson::private::base64::encode(&self.last_results_hash).as_str())?;
        }
        if !self.evidence_hash.is_empty() {
            struct_ser.serialize_field("evidence_hash", pbjson::private::base64::encode(&self.evidence_hash).as_str())?;
        }
        if !self.proposer_address.is_empty() {
            struct_ser.serialize_field("proposer_address", pbjson::private::base64::encode(&self.proposer_address).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LightHeader {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "chain_id",
            "chainId",
            "height",
            "time",
            "last_block_id",
            "lastBlockId",
            "last_commit_hash",
            "lastCommitHash",
            "data_hash",
            "dataHash",
            "validators_hash",
            "validatorsHash",
            "next_validators_hash",
            "nextValidatorsHash",
            "consensus_hash",
            "consensusHash",
            "app_hash",
            "appHash",
            "last_results_hash",
            "lastResultsHash",
            "evidence_hash",
            "evidenceHash",
            "proposer_address",
            "proposerAddress",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            ChainId,
            Height,
            Time,
            LastBlockId,
            LastCommitHash,
            DataHash,
            ValidatorsHash,
            NextValidatorsHash,
            ConsensusHash,
            AppHash,
            LastResultsHash,
            EvidenceHash,
            ProposerAddress,
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
                            "version" => Ok(GeneratedField::Version),
                            "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
                            "height" => Ok(GeneratedField::Height),
                            "time" => Ok(GeneratedField::Time),
                            "lastBlockId" | "last_block_id" => Ok(GeneratedField::LastBlockId),
                            "lastCommitHash" | "last_commit_hash" => Ok(GeneratedField::LastCommitHash),
                            "dataHash" | "data_hash" => Ok(GeneratedField::DataHash),
                            "validatorsHash" | "validators_hash" => Ok(GeneratedField::ValidatorsHash),
                            "nextValidatorsHash" | "next_validators_hash" => Ok(GeneratedField::NextValidatorsHash),
                            "consensusHash" | "consensus_hash" => Ok(GeneratedField::ConsensusHash),
                            "appHash" | "app_hash" => Ok(GeneratedField::AppHash),
                            "lastResultsHash" | "last_results_hash" => Ok(GeneratedField::LastResultsHash),
                            "evidenceHash" | "evidence_hash" => Ok(GeneratedField::EvidenceHash),
                            "proposerAddress" | "proposer_address" => Ok(GeneratedField::ProposerAddress),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LightHeader;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tendermint.light.LightHeader")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<LightHeader, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut chain_id__ = None;
                let mut height__ = None;
                let mut time__ = None;
                let mut last_block_id__ = None;
                let mut last_commit_hash__ = None;
                let mut data_hash__ = None;
                let mut validators_hash__ = None;
                let mut next_validators_hash__ = None;
                let mut consensus_hash__ = None;
                let mut app_hash__ = None;
                let mut last_results_hash__ = None;
                let mut evidence_hash__ = None;
                let mut proposer_address__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = map.next_value()?;
                        }
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Time => {
                            if time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("time"));
                            }
                            time__ = map.next_value()?;
                        }
                        GeneratedField::LastBlockId => {
                            if last_block_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastBlockId"));
                            }
                            last_block_id__ = map.next_value()?;
                        }
                        GeneratedField::LastCommitHash => {
                            if last_commit_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastCommitHash"));
                            }
                            last_commit_hash__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DataHash => {
                            if data_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataHash"));
                            }
                            data_hash__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ValidatorsHash => {
                            if validators_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validatorsHash"));
                            }
                            validators_hash__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NextValidatorsHash => {
                            if next_validators_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextValidatorsHash"));
                            }
                            next_validators_hash__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ConsensusHash => {
                            if consensus_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consensusHash"));
                            }
                            consensus_hash__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AppHash => {
                            if app_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("appHash"));
                            }
                            app_hash__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::LastResultsHash => {
                            if last_results_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastResultsHash"));
                            }
                            last_results_hash__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::EvidenceHash => {
                            if evidence_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("evidenceHash"));
                            }
                            evidence_hash__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ProposerAddress => {
                            if proposer_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proposerAddress"));
                            }
                            proposer_address__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(LightHeader {
                    version: version__,
                    chain_id: chain_id__.unwrap_or_default(),
                    height: height__.unwrap_or_default(),
                    time: time__,
                    last_block_id: last_block_id__,
                    last_commit_hash: last_commit_hash__.unwrap_or_default(),
                    data_hash: data_hash__.unwrap_or_default(),
                    validators_hash: validators_hash__.unwrap_or_default(),
                    next_validators_hash: next_validators_hash__.unwrap_or_default(),
                    consensus_hash: consensus_hash__.unwrap_or_default(),
                    app_hash: app_hash__.unwrap_or_default(),
                    last_results_hash: last_results_hash__.unwrap_or_default(),
                    evidence_hash: evidence_hash__.unwrap_or_default(),
                    proposer_address: proposer_address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("tendermint.light.LightHeader", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MerkleRoot {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tendermint.light.MerkleRoot", len)?;
        if !self.hash.is_empty() {
            struct_ser.serialize_field("hash", pbjson::private::base64::encode(&self.hash).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MerkleRoot {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "hash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Hash,
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
                            "hash" => Ok(GeneratedField::Hash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MerkleRoot;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tendermint.light.MerkleRoot")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MerkleRoot, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut hash__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(MerkleRoot {
                    hash: hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("tendermint.light.MerkleRoot", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PartSetHeader {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.total != 0 {
            len += 1;
        }
        if !self.hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tendermint.light.PartSetHeader", len)?;
        if self.total != 0 {
            struct_ser.serialize_field("total", &self.total)?;
        }
        if !self.hash.is_empty() {
            struct_ser.serialize_field("hash", pbjson::private::base64::encode(&self.hash).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PartSetHeader {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "total",
            "hash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Total,
            Hash,
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
                            "total" => Ok(GeneratedField::Total),
                            "hash" => Ok(GeneratedField::Hash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PartSetHeader;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tendermint.light.PartSetHeader")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PartSetHeader, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut total__ = None;
                let mut hash__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Total => {
                            if total__.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            total__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(PartSetHeader {
                    total: total__.unwrap_or_default(),
                    hash: hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("tendermint.light.PartSetHeader", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PublicKey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.sum.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tendermint.light.PublicKey", len)?;
        if let Some(v) = self.sum.as_ref() {
            match v {
                public_key::Sum::Ed25519(v) => {
                    struct_ser.serialize_field("ed25519", pbjson::private::base64::encode(v).as_str())?;
                }
                public_key::Sum::Secp256k1(v) => {
                    struct_ser.serialize_field("secp256k1", pbjson::private::base64::encode(v).as_str())?;
                }
                public_key::Sum::Sr25519(v) => {
                    struct_ser.serialize_field("sr25519", pbjson::private::base64::encode(v).as_str())?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PublicKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ed25519",
            "secp256k1",
            "sr25519",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ed25519,
            Secp256k1,
            Sr25519,
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
                            "ed25519" => Ok(GeneratedField::Ed25519),
                            "secp256k1" => Ok(GeneratedField::Secp256k1),
                            "sr25519" => Ok(GeneratedField::Sr25519),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PublicKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tendermint.light.PublicKey")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PublicKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sum__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Ed25519 => {
                            if sum__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ed25519"));
                            }
                            sum__ = map.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| public_key::Sum::Ed25519(x.0));
                        }
                        GeneratedField::Secp256k1 => {
                            if sum__.is_some() {
                                return Err(serde::de::Error::duplicate_field("secp256k1"));
                            }
                            sum__ = map.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| public_key::Sum::Secp256k1(x.0));
                        }
                        GeneratedField::Sr25519 => {
                            if sum__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sr25519"));
                            }
                            sum__ = map.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| public_key::Sum::Sr25519(x.0));
                        }
                    }
                }
                Ok(PublicKey {
                    sum: sum__,
                })
            }
        }
        deserializer.deserialize_struct("tendermint.light.PublicKey", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SignedHeader {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.header.is_some() {
            len += 1;
        }
        if self.commit.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tendermint.light.SignedHeader", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if let Some(v) = self.commit.as_ref() {
            struct_ser.serialize_field("commit", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SignedHeader {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "header",
            "commit",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
            Commit,
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
                            "header" => Ok(GeneratedField::Header),
                            "commit" => Ok(GeneratedField::Commit),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SignedHeader;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tendermint.light.SignedHeader")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SignedHeader, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut commit__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Commit => {
                            if commit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commit"));
                            }
                            commit__ = map.next_value()?;
                        }
                    }
                }
                Ok(SignedHeader {
                    header: header__,
                    commit: commit__,
                })
            }
        }
        deserializer.deserialize_struct("tendermint.light.SignedHeader", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SignedMsgType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::SignedMsgTypeUnknown => "SIGNED_MSG_TYPE_UNKNOWN",
            Self::SignedMsgTypePrevote => "SIGNED_MSG_TYPE_PREVOTE",
            Self::SignedMsgTypePrecommit => "SIGNED_MSG_TYPE_PRECOMMIT",
            Self::SignedMsgTypeProposal => "SIGNED_MSG_TYPE_PROPOSAL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SignedMsgType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SIGNED_MSG_TYPE_UNKNOWN",
            "SIGNED_MSG_TYPE_PREVOTE",
            "SIGNED_MSG_TYPE_PRECOMMIT",
            "SIGNED_MSG_TYPE_PROPOSAL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SignedMsgType;

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
                    .and_then(SignedMsgType::from_i32)
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
                    .and_then(SignedMsgType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "SIGNED_MSG_TYPE_UNKNOWN" => Ok(SignedMsgType::SignedMsgTypeUnknown),
                    "SIGNED_MSG_TYPE_PREVOTE" => Ok(SignedMsgType::SignedMsgTypePrevote),
                    "SIGNED_MSG_TYPE_PRECOMMIT" => Ok(SignedMsgType::SignedMsgTypePrecommit),
                    "SIGNED_MSG_TYPE_PROPOSAL" => Ok(SignedMsgType::SignedMsgTypeProposal),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for SimpleValidator {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pub_key.is_some() {
            len += 1;
        }
        if self.voting_power != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tendermint.light.SimpleValidator", len)?;
        if let Some(v) = self.pub_key.as_ref() {
            struct_ser.serialize_field("pub_key", v)?;
        }
        if self.voting_power != 0 {
            struct_ser.serialize_field("voting_power", ToString::to_string(&self.voting_power).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SimpleValidator {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pub_key",
            "pubKey",
            "voting_power",
            "votingPower",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PubKey,
            VotingPower,
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
                            "pubKey" | "pub_key" => Ok(GeneratedField::PubKey),
                            "votingPower" | "voting_power" => Ok(GeneratedField::VotingPower),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SimpleValidator;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tendermint.light.SimpleValidator")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SimpleValidator, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pub_key__ = None;
                let mut voting_power__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PubKey => {
                            if pub_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pubKey"));
                            }
                            pub_key__ = map.next_value()?;
                        }
                        GeneratedField::VotingPower => {
                            if voting_power__.is_some() {
                                return Err(serde::de::Error::duplicate_field("votingPower"));
                            }
                            voting_power__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(SimpleValidator {
                    pub_key: pub_key__,
                    voting_power: voting_power__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("tendermint.light.SimpleValidator", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Timestamp {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.seconds != 0 {
            len += 1;
        }
        if self.nanos != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tendermint.light.Timestamp", len)?;
        if self.seconds != 0 {
            struct_ser.serialize_field("seconds", ToString::to_string(&self.seconds).as_str())?;
        }
        if self.nanos != 0 {
            struct_ser.serialize_field("nanos", &self.nanos)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Timestamp {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "seconds",
            "nanos",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Seconds,
            Nanos,
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
                            "seconds" => Ok(GeneratedField::Seconds),
                            "nanos" => Ok(GeneratedField::Nanos),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Timestamp;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tendermint.light.Timestamp")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Timestamp, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut seconds__ = None;
                let mut nanos__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Seconds => {
                            if seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("seconds"));
                            }
                            seconds__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Nanos => {
                            if nanos__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nanos"));
                            }
                            nanos__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Timestamp {
                    seconds: seconds__.unwrap_or_default(),
                    nanos: nanos__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("tendermint.light.Timestamp", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TmHeader {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.signed_header.is_some() {
            len += 1;
        }
        if self.validator_set.is_some() {
            len += 1;
        }
        if self.trusted_height != 0 {
            len += 1;
        }
        if self.trusted_validators.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tendermint.light.TmHeader", len)?;
        if let Some(v) = self.signed_header.as_ref() {
            struct_ser.serialize_field("signed_header", v)?;
        }
        if let Some(v) = self.validator_set.as_ref() {
            struct_ser.serialize_field("validator_set", v)?;
        }
        if self.trusted_height != 0 {
            struct_ser.serialize_field("trusted_height", ToString::to_string(&self.trusted_height).as_str())?;
        }
        if let Some(v) = self.trusted_validators.as_ref() {
            struct_ser.serialize_field("trusted_validators", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TmHeader {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "signed_header",
            "signedHeader",
            "validator_set",
            "validatorSet",
            "trusted_height",
            "trustedHeight",
            "trusted_validators",
            "trustedValidators",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SignedHeader,
            ValidatorSet,
            TrustedHeight,
            TrustedValidators,
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
                            "signedHeader" | "signed_header" => Ok(GeneratedField::SignedHeader),
                            "validatorSet" | "validator_set" => Ok(GeneratedField::ValidatorSet),
                            "trustedHeight" | "trusted_height" => Ok(GeneratedField::TrustedHeight),
                            "trustedValidators" | "trusted_validators" => Ok(GeneratedField::TrustedValidators),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TmHeader;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tendermint.light.TmHeader")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TmHeader, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut signed_header__ = None;
                let mut validator_set__ = None;
                let mut trusted_height__ = None;
                let mut trusted_validators__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::SignedHeader => {
                            if signed_header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signedHeader"));
                            }
                            signed_header__ = map.next_value()?;
                        }
                        GeneratedField::ValidatorSet => {
                            if validator_set__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validatorSet"));
                            }
                            validator_set__ = map.next_value()?;
                        }
                        GeneratedField::TrustedHeight => {
                            if trusted_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trustedHeight"));
                            }
                            trusted_height__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TrustedValidators => {
                            if trusted_validators__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trustedValidators"));
                            }
                            trusted_validators__ = map.next_value()?;
                        }
                    }
                }
                Ok(TmHeader {
                    signed_header: signed_header__,
                    validator_set: validator_set__,
                    trusted_height: trusted_height__.unwrap_or_default(),
                    trusted_validators: trusted_validators__,
                })
            }
        }
        deserializer.deserialize_struct("tendermint.light.TmHeader", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Validator {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        if self.pub_key.is_some() {
            len += 1;
        }
        if self.voting_power != 0 {
            len += 1;
        }
        if self.proposer_priority != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tendermint.light.Validator", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", pbjson::private::base64::encode(&self.address).as_str())?;
        }
        if let Some(v) = self.pub_key.as_ref() {
            struct_ser.serialize_field("pub_key", v)?;
        }
        if self.voting_power != 0 {
            struct_ser.serialize_field("voting_power", ToString::to_string(&self.voting_power).as_str())?;
        }
        if self.proposer_priority != 0 {
            struct_ser.serialize_field("proposer_priority", ToString::to_string(&self.proposer_priority).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Validator {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
            "pub_key",
            "pubKey",
            "voting_power",
            "votingPower",
            "proposer_priority",
            "proposerPriority",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            PubKey,
            VotingPower,
            ProposerPriority,
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
                            "address" => Ok(GeneratedField::Address),
                            "pubKey" | "pub_key" => Ok(GeneratedField::PubKey),
                            "votingPower" | "voting_power" => Ok(GeneratedField::VotingPower),
                            "proposerPriority" | "proposer_priority" => Ok(GeneratedField::ProposerPriority),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Validator;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tendermint.light.Validator")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Validator, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut pub_key__ = None;
                let mut voting_power__ = None;
                let mut proposer_priority__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PubKey => {
                            if pub_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pubKey"));
                            }
                            pub_key__ = map.next_value()?;
                        }
                        GeneratedField::VotingPower => {
                            if voting_power__.is_some() {
                                return Err(serde::de::Error::duplicate_field("votingPower"));
                            }
                            voting_power__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ProposerPriority => {
                            if proposer_priority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proposerPriority"));
                            }
                            proposer_priority__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Validator {
                    address: address__.unwrap_or_default(),
                    pub_key: pub_key__,
                    voting_power: voting_power__.unwrap_or_default(),
                    proposer_priority: proposer_priority__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("tendermint.light.Validator", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ValidatorSet {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.validators.is_empty() {
            len += 1;
        }
        if self.proposer.is_some() {
            len += 1;
        }
        if self.total_voting_power != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tendermint.light.ValidatorSet", len)?;
        if !self.validators.is_empty() {
            struct_ser.serialize_field("validators", &self.validators)?;
        }
        if let Some(v) = self.proposer.as_ref() {
            struct_ser.serialize_field("proposer", v)?;
        }
        if self.total_voting_power != 0 {
            struct_ser.serialize_field("total_voting_power", ToString::to_string(&self.total_voting_power).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ValidatorSet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "validators",
            "proposer",
            "total_voting_power",
            "totalVotingPower",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Validators,
            Proposer,
            TotalVotingPower,
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
                            "validators" => Ok(GeneratedField::Validators),
                            "proposer" => Ok(GeneratedField::Proposer),
                            "totalVotingPower" | "total_voting_power" => Ok(GeneratedField::TotalVotingPower),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ValidatorSet;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tendermint.light.ValidatorSet")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ValidatorSet, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut validators__ = None;
                let mut proposer__ = None;
                let mut total_voting_power__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Validators => {
                            if validators__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validators"));
                            }
                            validators__ = Some(map.next_value()?);
                        }
                        GeneratedField::Proposer => {
                            if proposer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proposer"));
                            }
                            proposer__ = map.next_value()?;
                        }
                        GeneratedField::TotalVotingPower => {
                            if total_voting_power__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalVotingPower"));
                            }
                            total_voting_power__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ValidatorSet {
                    validators: validators__.unwrap_or_default(),
                    proposer: proposer__,
                    total_voting_power: total_voting_power__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("tendermint.light.ValidatorSet", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Vote {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type != 0 {
            len += 1;
        }
        if self.height != 0 {
            len += 1;
        }
        if self.round != 0 {
            len += 1;
        }
        if self.block_id.is_some() {
            len += 1;
        }
        if self.timestamp.is_some() {
            len += 1;
        }
        if !self.validator_address.is_empty() {
            len += 1;
        }
        if self.validator_index != 0 {
            len += 1;
        }
        if !self.signature.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tendermint.light.Vote", len)?;
        if self.r#type != 0 {
            let v = SignedMsgType::from_i32(self.r#type)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if self.height != 0 {
            struct_ser.serialize_field("height", ToString::to_string(&self.height).as_str())?;
        }
        if self.round != 0 {
            struct_ser.serialize_field("round", &self.round)?;
        }
        if let Some(v) = self.block_id.as_ref() {
            struct_ser.serialize_field("block_id", v)?;
        }
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", v)?;
        }
        if !self.validator_address.is_empty() {
            struct_ser.serialize_field("validator_address", pbjson::private::base64::encode(&self.validator_address).as_str())?;
        }
        if self.validator_index != 0 {
            struct_ser.serialize_field("validator_index", &self.validator_index)?;
        }
        if !self.signature.is_empty() {
            struct_ser.serialize_field("signature", pbjson::private::base64::encode(&self.signature).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Vote {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "height",
            "round",
            "block_id",
            "blockId",
            "timestamp",
            "validator_address",
            "validatorAddress",
            "validator_index",
            "validatorIndex",
            "signature",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            Height,
            Round,
            BlockId,
            Timestamp,
            ValidatorAddress,
            ValidatorIndex,
            Signature,
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
                            "type" => Ok(GeneratedField::Type),
                            "height" => Ok(GeneratedField::Height),
                            "round" => Ok(GeneratedField::Round),
                            "blockId" | "block_id" => Ok(GeneratedField::BlockId),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "validatorAddress" | "validator_address" => Ok(GeneratedField::ValidatorAddress),
                            "validatorIndex" | "validator_index" => Ok(GeneratedField::ValidatorIndex),
                            "signature" => Ok(GeneratedField::Signature),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Vote;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tendermint.light.Vote")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Vote, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut height__ = None;
                let mut round__ = None;
                let mut block_id__ = None;
                let mut timestamp__ = None;
                let mut validator_address__ = None;
                let mut validator_index__ = None;
                let mut signature__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map.next_value::<SignedMsgType>()? as i32);
                        }
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Round => {
                            if round__.is_some() {
                                return Err(serde::de::Error::duplicate_field("round"));
                            }
                            round__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BlockId => {
                            if block_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockId"));
                            }
                            block_id__ = map.next_value()?;
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = map.next_value()?;
                        }
                        GeneratedField::ValidatorAddress => {
                            if validator_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validatorAddress"));
                            }
                            validator_address__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ValidatorIndex => {
                            if validator_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validatorIndex"));
                            }
                            validator_index__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Signature => {
                            if signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signature"));
                            }
                            signature__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Vote {
                    r#type: r#type__.unwrap_or_default(),
                    height: height__.unwrap_or_default(),
                    round: round__.unwrap_or_default(),
                    block_id: block_id__,
                    timestamp: timestamp__,
                    validator_address: validator_address__.unwrap_or_default(),
                    validator_index: validator_index__.unwrap_or_default(),
                    signature: signature__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("tendermint.light.Vote", FIELDS, GeneratedVisitor)
    }
}
