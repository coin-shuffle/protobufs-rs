// @generated
impl serde::Serialize for ConnectShuffleRoomRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.public_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("coin_shuffle.v1.ConnectShuffleRoomRequest", len)?;
        if let Some(v) = self.public_key.as_ref() {
            struct_ser.serialize_field("publicKey", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConnectShuffleRoomRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "public_key",
            "publicKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PublicKey,
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
                            "publicKey" | "public_key" => Ok(GeneratedField::PublicKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConnectShuffleRoomRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct coin_shuffle.v1.ConnectShuffleRoomRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ConnectShuffleRoomRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut public_key__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PublicKey => {
                            if public_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicKey"));
                            }
                            public_key__ = map.next_value()?;
                        }
                    }
                }
                Ok(ConnectShuffleRoomRequest {
                    public_key: public_key__,
                })
            }
        }
        deserializer.deserialize_struct("coin_shuffle.v1.ConnectShuffleRoomRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DecodedOutput {
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
        let mut struct_ser = serializer.serialize_struct("coin_shuffle.v1.DecodedOutput", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", pbjson::private::base64::encode(&self.address).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DecodedOutput {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DecodedOutput;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct coin_shuffle.v1.DecodedOutput")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DecodedOutput, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
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
                    }
                }
                Ok(DecodedOutput {
                    address: address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("coin_shuffle.v1.DecodedOutput", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EncodedOutputs {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.outputs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("coin_shuffle.v1.EncodedOutputs", len)?;
        if !self.outputs.is_empty() {
            struct_ser.serialize_field("outputs", &self.outputs.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EncodedOutputs {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "outputs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Outputs,
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
                            "outputs" => Ok(GeneratedField::Outputs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EncodedOutputs;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct coin_shuffle.v1.EncodedOutputs")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EncodedOutputs, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut outputs__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Outputs => {
                            if outputs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outputs"));
                            }
                            outputs__ = 
                                Some(map.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(EncodedOutputs {
                    outputs: outputs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("coin_shuffle.v1.EncodedOutputs", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IsReadyForShuffleRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("coin_shuffle.v1.IsReadyForShuffleRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IsReadyForShuffleRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IsReadyForShuffleRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct coin_shuffle.v1.IsReadyForShuffleRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IsReadyForShuffleRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(IsReadyForShuffleRequest {
                })
            }
        }
        deserializer.deserialize_struct("coin_shuffle.v1.IsReadyForShuffleRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IsReadyForShuffleResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.ready {
            len += 1;
        }
        if !self.room_access_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("coin_shuffle.v1.IsReadyForShuffleResponse", len)?;
        if self.ready {
            struct_ser.serialize_field("ready", &self.ready)?;
        }
        if !self.room_access_token.is_empty() {
            struct_ser.serialize_field("roomAccessToken", &self.room_access_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IsReadyForShuffleResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ready",
            "room_access_token",
            "roomAccessToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ready,
            RoomAccessToken,
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
                            "ready" => Ok(GeneratedField::Ready),
                            "roomAccessToken" | "room_access_token" => Ok(GeneratedField::RoomAccessToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IsReadyForShuffleResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct coin_shuffle.v1.IsReadyForShuffleResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IsReadyForShuffleResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ready__ = None;
                let mut room_access_token__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Ready => {
                            if ready__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ready"));
                            }
                            ready__ = Some(map.next_value()?);
                        }
                        GeneratedField::RoomAccessToken => {
                            if room_access_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roomAccessToken"));
                            }
                            room_access_token__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(IsReadyForShuffleResponse {
                    ready: ready__.unwrap_or_default(),
                    room_access_token: room_access_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("coin_shuffle.v1.IsReadyForShuffleResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for JoinShuffleRoomRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.utxo_id.is_empty() {
            len += 1;
        }
        if self.timestamp != 0 {
            len += 1;
        }
        if !self.signature.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("coin_shuffle.v1.JoinShuffleRoomRequest", len)?;
        if !self.utxo_id.is_empty() {
            struct_ser.serialize_field("utxoId", pbjson::private::base64::encode(&self.utxo_id).as_str())?;
        }
        if self.timestamp != 0 {
            struct_ser.serialize_field("timestamp", ToString::to_string(&self.timestamp).as_str())?;
        }
        if !self.signature.is_empty() {
            struct_ser.serialize_field("signature", pbjson::private::base64::encode(&self.signature).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for JoinShuffleRoomRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "utxo_id",
            "utxoId",
            "timestamp",
            "signature",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UtxoId,
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
                            "utxoId" | "utxo_id" => Ok(GeneratedField::UtxoId),
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
            type Value = JoinShuffleRoomRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct coin_shuffle.v1.JoinShuffleRoomRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<JoinShuffleRoomRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut utxo_id__ = None;
                let mut timestamp__ = None;
                let mut signature__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::UtxoId => {
                            if utxo_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("utxoId"));
                            }
                            utxo_id__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = 
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
                Ok(JoinShuffleRoomRequest {
                    utxo_id: utxo_id__.unwrap_or_default(),
                    timestamp: timestamp__.unwrap_or_default(),
                    signature: signature__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("coin_shuffle.v1.JoinShuffleRoomRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for JoinShuffleRoomResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.room_access_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("coin_shuffle.v1.JoinShuffleRoomResponse", len)?;
        if !self.room_access_token.is_empty() {
            struct_ser.serialize_field("roomAccessToken", &self.room_access_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for JoinShuffleRoomResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "room_access_token",
            "roomAccessToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RoomAccessToken,
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
                            "roomAccessToken" | "room_access_token" => Ok(GeneratedField::RoomAccessToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = JoinShuffleRoomResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct coin_shuffle.v1.JoinShuffleRoomResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<JoinShuffleRoomResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut room_access_token__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RoomAccessToken => {
                            if room_access_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("roomAccessToken"));
                            }
                            room_access_token__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(JoinShuffleRoomResponse {
                    room_access_token: room_access_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("coin_shuffle.v1.JoinShuffleRoomResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RsaPublicKey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.modulus.is_empty() {
            len += 1;
        }
        if !self.exponent.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("coin_shuffle.v1.RsaPublicKey", len)?;
        if !self.modulus.is_empty() {
            struct_ser.serialize_field("modulus", pbjson::private::base64::encode(&self.modulus).as_str())?;
        }
        if !self.exponent.is_empty() {
            struct_ser.serialize_field("exponent", pbjson::private::base64::encode(&self.exponent).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RsaPublicKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "modulus",
            "exponent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Modulus,
            Exponent,
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
                            "modulus" => Ok(GeneratedField::Modulus),
                            "exponent" => Ok(GeneratedField::Exponent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RsaPublicKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct coin_shuffle.v1.RsaPublicKey")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RsaPublicKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut modulus__ = None;
                let mut exponent__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Modulus => {
                            if modulus__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modulus"));
                            }
                            modulus__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Exponent => {
                            if exponent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exponent"));
                            }
                            exponent__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(RsaPublicKey {
                    modulus: modulus__.unwrap_or_default(),
                    exponent: exponent__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("coin_shuffle.v1.RsaPublicKey", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ShuffleError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.error.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("coin_shuffle.v1.ShuffleError", len)?;
        if !self.error.is_empty() {
            struct_ser.serialize_field("error", &self.error)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ShuffleError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "error",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Error,
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
                            "error" => Ok(GeneratedField::Error),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ShuffleError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct coin_shuffle.v1.ShuffleError")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ShuffleError, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut error__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ShuffleError {
                    error: error__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("coin_shuffle.v1.ShuffleError", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for shuffle_error::Code {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "CODE_UNSPECIFIED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for shuffle_error::Code {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CODE_UNSPECIFIED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = shuffle_error::Code;

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
                    .and_then(shuffle_error::Code::from_i32)
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
                    .and_then(shuffle_error::Code::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "CODE_UNSPECIFIED" => Ok(shuffle_error::Code::Unspecified),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ShuffleEvent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.body.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("coin_shuffle.v1.ShuffleEvent", len)?;
        if let Some(v) = self.body.as_ref() {
            match v {
                shuffle_event::Body::Error(v) => {
                    struct_ser.serialize_field("error", v)?;
                }
                shuffle_event::Body::ShuffleInfo(v) => {
                    struct_ser.serialize_field("shuffleInfo", v)?;
                }
                shuffle_event::Body::EncodedOutputs(v) => {
                    struct_ser.serialize_field("encodedOutputs", v)?;
                }
                shuffle_event::Body::TxSigningOutputs(v) => {
                    struct_ser.serialize_field("txSigningOutputs", v)?;
                }
                shuffle_event::Body::ShuffleTxHash(v) => {
                    struct_ser.serialize_field("shuffleTxHash", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ShuffleEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "error",
            "shuffle_info",
            "shuffleInfo",
            "encoded_outputs",
            "encodedOutputs",
            "tx_signing_outputs",
            "txSigningOutputs",
            "shuffle_tx_hash",
            "shuffleTxHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Error,
            ShuffleInfo,
            EncodedOutputs,
            TxSigningOutputs,
            ShuffleTxHash,
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
                            "error" => Ok(GeneratedField::Error),
                            "shuffleInfo" | "shuffle_info" => Ok(GeneratedField::ShuffleInfo),
                            "encodedOutputs" | "encoded_outputs" => Ok(GeneratedField::EncodedOutputs),
                            "txSigningOutputs" | "tx_signing_outputs" => Ok(GeneratedField::TxSigningOutputs),
                            "shuffleTxHash" | "shuffle_tx_hash" => Ok(GeneratedField::ShuffleTxHash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ShuffleEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct coin_shuffle.v1.ShuffleEvent")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ShuffleEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut body__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Error => {
                            if body__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            body__ = map.next_value::<::std::option::Option<_>>()?.map(shuffle_event::Body::Error)
;
                        }
                        GeneratedField::ShuffleInfo => {
                            if body__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shuffleInfo"));
                            }
                            body__ = map.next_value::<::std::option::Option<_>>()?.map(shuffle_event::Body::ShuffleInfo)
;
                        }
                        GeneratedField::EncodedOutputs => {
                            if body__.is_some() {
                                return Err(serde::de::Error::duplicate_field("encodedOutputs"));
                            }
                            body__ = map.next_value::<::std::option::Option<_>>()?.map(shuffle_event::Body::EncodedOutputs)
;
                        }
                        GeneratedField::TxSigningOutputs => {
                            if body__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txSigningOutputs"));
                            }
                            body__ = map.next_value::<::std::option::Option<_>>()?.map(shuffle_event::Body::TxSigningOutputs)
;
                        }
                        GeneratedField::ShuffleTxHash => {
                            if body__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shuffleTxHash"));
                            }
                            body__ = map.next_value::<::std::option::Option<_>>()?.map(shuffle_event::Body::ShuffleTxHash)
;
                        }
                    }
                }
                Ok(ShuffleEvent {
                    body: body__,
                })
            }
        }
        deserializer.deserialize_struct("coin_shuffle.v1.ShuffleEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ShuffleInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.public_keys_list.is_empty() {
            len += 1;
        }
        if !self.shuffle_access_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("coin_shuffle.v1.ShuffleInfo", len)?;
        if !self.public_keys_list.is_empty() {
            struct_ser.serialize_field("publicKeysList", &self.public_keys_list)?;
        }
        if !self.shuffle_access_token.is_empty() {
            struct_ser.serialize_field("shuffleAccessToken", &self.shuffle_access_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ShuffleInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "public_keys_list",
            "publicKeysList",
            "shuffle_access_token",
            "shuffleAccessToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PublicKeysList,
            ShuffleAccessToken,
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
                            "publicKeysList" | "public_keys_list" => Ok(GeneratedField::PublicKeysList),
                            "shuffleAccessToken" | "shuffle_access_token" => Ok(GeneratedField::ShuffleAccessToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ShuffleInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct coin_shuffle.v1.ShuffleInfo")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ShuffleInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut public_keys_list__ = None;
                let mut shuffle_access_token__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PublicKeysList => {
                            if public_keys_list__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicKeysList"));
                            }
                            public_keys_list__ = Some(map.next_value()?);
                        }
                        GeneratedField::ShuffleAccessToken => {
                            if shuffle_access_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shuffleAccessToken"));
                            }
                            shuffle_access_token__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ShuffleInfo {
                    public_keys_list: public_keys_list__.unwrap_or_default(),
                    shuffle_access_token: shuffle_access_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("coin_shuffle.v1.ShuffleInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ShuffleRoundRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.encoded_outputs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("coin_shuffle.v1.ShuffleRoundRequest", len)?;
        if !self.encoded_outputs.is_empty() {
            struct_ser.serialize_field("encodedOutputs", &self.encoded_outputs.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ShuffleRoundRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "encoded_outputs",
            "encodedOutputs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EncodedOutputs,
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
                            "encodedOutputs" | "encoded_outputs" => Ok(GeneratedField::EncodedOutputs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ShuffleRoundRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct coin_shuffle.v1.ShuffleRoundRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ShuffleRoundRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut encoded_outputs__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::EncodedOutputs => {
                            if encoded_outputs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("encodedOutputs"));
                            }
                            encoded_outputs__ = 
                                Some(map.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(ShuffleRoundRequest {
                    encoded_outputs: encoded_outputs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("coin_shuffle.v1.ShuffleRoundRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ShuffleRoundResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("coin_shuffle.v1.ShuffleRoundResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ShuffleRoundResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ShuffleRoundResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct coin_shuffle.v1.ShuffleRoundResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ShuffleRoundResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ShuffleRoundResponse {
                })
            }
        }
        deserializer.deserialize_struct("coin_shuffle.v1.ShuffleRoundResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ShuffleTxHash {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tx_hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("coin_shuffle.v1.ShuffleTxHash", len)?;
        if !self.tx_hash.is_empty() {
            struct_ser.serialize_field("txHash", pbjson::private::base64::encode(&self.tx_hash).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ShuffleTxHash {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tx_hash",
            "txHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TxHash,
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
                            "txHash" | "tx_hash" => Ok(GeneratedField::TxHash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ShuffleTxHash;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct coin_shuffle.v1.ShuffleTxHash")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ShuffleTxHash, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tx_hash__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TxHash => {
                            if tx_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txHash"));
                            }
                            tx_hash__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ShuffleTxHash {
                    tx_hash: tx_hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("coin_shuffle.v1.ShuffleTxHash", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SignShuffleTxRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.signature.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("coin_shuffle.v1.SignShuffleTxRequest", len)?;
        if !self.signature.is_empty() {
            struct_ser.serialize_field("signature", pbjson::private::base64::encode(&self.signature).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SignShuffleTxRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "signature",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = SignShuffleTxRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct coin_shuffle.v1.SignShuffleTxRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SignShuffleTxRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut signature__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
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
                Ok(SignShuffleTxRequest {
                    signature: signature__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("coin_shuffle.v1.SignShuffleTxRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SignShuffleTxResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("coin_shuffle.v1.SignShuffleTxResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SignShuffleTxResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SignShuffleTxResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct coin_shuffle.v1.SignShuffleTxResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SignShuffleTxResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(SignShuffleTxResponse {
                })
            }
        }
        deserializer.deserialize_struct("coin_shuffle.v1.SignShuffleTxResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TxSigningOutputs {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.outputs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("coin_shuffle.v1.TxSigningOutputs", len)?;
        if !self.outputs.is_empty() {
            struct_ser.serialize_field("outputs", &self.outputs.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TxSigningOutputs {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "outputs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Outputs,
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
                            "outputs" => Ok(GeneratedField::Outputs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TxSigningOutputs;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct coin_shuffle.v1.TxSigningOutputs")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TxSigningOutputs, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut outputs__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Outputs => {
                            if outputs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outputs"));
                            }
                            outputs__ = 
                                Some(map.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(TxSigningOutputs {
                    outputs: outputs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("coin_shuffle.v1.TxSigningOutputs", FIELDS, GeneratedVisitor)
    }
}
