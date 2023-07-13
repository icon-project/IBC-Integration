// @generated
impl serde::Serialize for BtpHeader {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.main_height != 0 {
            len += 1;
        }
        if self.round != 0 {
            len += 1;
        }
        if !self.next_proof_context_hash.is_empty() {
            len += 1;
        }
        if !self.network_section_to_root.is_empty() {
            len += 1;
        }
        if self.network_id != 0 {
            len += 1;
        }
        if self.update_number != 0 {
            len += 1;
        }
        if !self.prev_network_section_hash.is_empty() {
            len += 1;
        }
        if self.message_count != 0 {
            len += 1;
        }
        if !self.message_root.is_empty() {
            len += 1;
        }
        if !self.next_validators.is_empty() {
            len += 1;
        }
        if !self.current_validators.is_empty() {
            len += 1;
        }
        if self.trusted_height != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("icon.types.v1.BTPHeader", len)?;
        if self.main_height != 0 {
            struct_ser.serialize_field("main_height", ToString::to_string(&self.main_height).as_str())?;
        }
        if self.round != 0 {
            struct_ser.serialize_field("round", &self.round)?;
        }
        if !self.next_proof_context_hash.is_empty() {
            struct_ser.serialize_field("next_proof_context_hash", pbjson::private::base64::encode(&self.next_proof_context_hash).as_str())?;
        }
        if !self.network_section_to_root.is_empty() {
            struct_ser.serialize_field("network_section_to_root", &self.network_section_to_root)?;
        }
        if self.network_id != 0 {
            struct_ser.serialize_field("network_id", ToString::to_string(&self.network_id).as_str())?;
        }
        if self.update_number != 0 {
            struct_ser.serialize_field("update_number", ToString::to_string(&self.update_number).as_str())?;
        }
        if !self.prev_network_section_hash.is_empty() {
            struct_ser.serialize_field("prev_network_section_hash", pbjson::private::base64::encode(&self.prev_network_section_hash).as_str())?;
        }
        if self.message_count != 0 {
            struct_ser.serialize_field("message_count", ToString::to_string(&self.message_count).as_str())?;
        }
        if !self.message_root.is_empty() {
            struct_ser.serialize_field("message_root", pbjson::private::base64::encode(&self.message_root).as_str())?;
        }
        if !self.next_validators.is_empty() {
            struct_ser.serialize_field("nextValidators", &self.next_validators.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        if !self.current_validators.is_empty() {
            struct_ser.serialize_field("currentValidators", &self.current_validators.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        if self.trusted_height != 0 {
            struct_ser.serialize_field("trusted_height", ToString::to_string(&self.trusted_height).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BtpHeader {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "main_height",
            "mainHeight",
            "round",
            "next_proof_context_hash",
            "nextProofContextHash",
            "network_section_to_root",
            "networkSectionToRoot",
            "network_id",
            "networkId",
            "update_number",
            "updateNumber",
            "prev_network_section_hash",
            "prevNetworkSectionHash",
            "message_count",
            "messageCount",
            "message_root",
            "messageRoot",
            "nextValidators",
            "currentValidators",
            "trusted_height",
            "trustedHeight",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MainHeight,
            Round,
            NextProofContextHash,
            NetworkSectionToRoot,
            NetworkId,
            UpdateNumber,
            PrevNetworkSectionHash,
            MessageCount,
            MessageRoot,
            NextValidators,
            CurrentValidators,
            TrustedHeight,
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
                            "mainHeight" | "main_height" => Ok(GeneratedField::MainHeight),
                            "round" => Ok(GeneratedField::Round),
                            "nextProofContextHash" | "next_proof_context_hash" => Ok(GeneratedField::NextProofContextHash),
                            "networkSectionToRoot" | "network_section_to_root" => Ok(GeneratedField::NetworkSectionToRoot),
                            "networkId" | "network_id" => Ok(GeneratedField::NetworkId),
                            "updateNumber" | "update_number" => Ok(GeneratedField::UpdateNumber),
                            "prevNetworkSectionHash" | "prev_network_section_hash" => Ok(GeneratedField::PrevNetworkSectionHash),
                            "messageCount" | "message_count" => Ok(GeneratedField::MessageCount),
                            "messageRoot" | "message_root" => Ok(GeneratedField::MessageRoot),
                            "nextValidators" => Ok(GeneratedField::NextValidators),
                            "currentValidators" => Ok(GeneratedField::CurrentValidators),
                            "trustedHeight" | "trusted_height" => Ok(GeneratedField::TrustedHeight),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BtpHeader;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct icon.types.v1.BTPHeader")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BtpHeader, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut main_height__ = None;
                let mut round__ = None;
                let mut next_proof_context_hash__ = None;
                let mut network_section_to_root__ = None;
                let mut network_id__ = None;
                let mut update_number__ = None;
                let mut prev_network_section_hash__ = None;
                let mut message_count__ = None;
                let mut message_root__ = None;
                let mut next_validators__ = None;
                let mut current_validators__ = None;
                let mut trusted_height__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MainHeight => {
                            if main_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mainHeight"));
                            }
                            main_height__ = 
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
                        GeneratedField::NextProofContextHash => {
                            if next_proof_context_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextProofContextHash"));
                            }
                            next_proof_context_hash__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NetworkSectionToRoot => {
                            if network_section_to_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("networkSectionToRoot"));
                            }
                            network_section_to_root__ = Some(map.next_value()?);
                        }
                        GeneratedField::NetworkId => {
                            if network_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("networkId"));
                            }
                            network_id__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::UpdateNumber => {
                            if update_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateNumber"));
                            }
                            update_number__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PrevNetworkSectionHash => {
                            if prev_network_section_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prevNetworkSectionHash"));
                            }
                            prev_network_section_hash__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MessageCount => {
                            if message_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messageCount"));
                            }
                            message_count__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MessageRoot => {
                            if message_root__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messageRoot"));
                            }
                            message_root__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NextValidators => {
                            if next_validators__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextValidators"));
                            }
                            next_validators__ = 
                                Some(map.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::CurrentValidators => {
                            if current_validators__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currentValidators"));
                            }
                            current_validators__ = 
                                Some(map.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::TrustedHeight => {
                            if trusted_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trustedHeight"));
                            }
                            trusted_height__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(BtpHeader {
                    main_height: main_height__.unwrap_or_default(),
                    round: round__.unwrap_or_default(),
                    next_proof_context_hash: next_proof_context_hash__.unwrap_or_default(),
                    network_section_to_root: network_section_to_root__.unwrap_or_default(),
                    network_id: network_id__.unwrap_or_default(),
                    update_number: update_number__.unwrap_or_default(),
                    prev_network_section_hash: prev_network_section_hash__.unwrap_or_default(),
                    message_count: message_count__.unwrap_or_default(),
                    message_root: message_root__.unwrap_or_default(),
                    next_validators: next_validators__.unwrap_or_default(),
                    current_validators: current_validators__.unwrap_or_default(),
                    trusted_height: trusted_height__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("icon.types.v1.BTPHeader", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for MerkleNode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.dir != 0 {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("icon.types.v1.MerkleNode", len)?;
        if self.dir != 0 {
            struct_ser.serialize_field("Dir", &self.dir)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", pbjson::private::base64::encode(&self.value).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MerkleNode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "Dir",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Dir,
            Value,
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
                            "Dir" => Ok(GeneratedField::Dir),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MerkleNode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct icon.types.v1.MerkleNode")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MerkleNode, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut dir__ = None;
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Dir => {
                            if dir__.is_some() {
                                return Err(serde::de::Error::duplicate_field("Dir"));
                            }
                            dir__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(MerkleNode {
                    dir: dir__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("icon.types.v1.MerkleNode", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MerkleProofs {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.proofs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("icon.types.v1.MerkleProofs", len)?;
        if !self.proofs.is_empty() {
            struct_ser.serialize_field("proofs", &self.proofs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MerkleProofs {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "proofs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Proofs,
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
                            "proofs" => Ok(GeneratedField::Proofs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MerkleProofs;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct icon.types.v1.MerkleProofs")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MerkleProofs, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut proofs__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Proofs => {
                            if proofs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proofs"));
                            }
                            proofs__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MerkleProofs {
                    proofs: proofs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("icon.types.v1.MerkleProofs", FIELDS, GeneratedVisitor)
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
        if !self.signatures.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("icon.types.v1.SignedHeader", len)?;
        if let Some(v) = self.header.as_ref() {
            struct_ser.serialize_field("header", v)?;
        }
        if !self.signatures.is_empty() {
            struct_ser.serialize_field("signatures", &self.signatures.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
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
            "signatures",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Header,
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
                            "header" => Ok(GeneratedField::Header),
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
            type Value = SignedHeader;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct icon.types.v1.SignedHeader")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SignedHeader, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut header__ = None;
                let mut signatures__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Header => {
                            if header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            header__ = map.next_value()?;
                        }
                        GeneratedField::Signatures => {
                            if signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signatures"));
                            }
                            signatures__ = 
                                Some(map.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(SignedHeader {
                    header: header__,
                    signatures: signatures__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("icon.types.v1.SignedHeader", FIELDS, GeneratedVisitor)
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
