/* GENERATED WITH SIDEX. DO NOT MODIFY! */

pub mod manifest {
    #![doc = ""]
    #[allow(unused)]
    use :: serde as __serde;
    #[allow(unused)]
    use :: sidex_serde as __sidex_serde;
    #[doc = ""]
    pub type HashAlgorithm = si_crypto_hashes::HashAlgorithm;
    #[doc = ""]
    pub type HashDigest = si_crypto_hashes::HashDigest;
    #[doc = ""]
    pub type ChunkerAlgorithm = rugix_chunker::ChunkerAlgorithm;
    #[doc = ""]
    pub type NumBytes = byte_calc::NumBytes;
    #[doc = ""]
    #[derive(Clone, Debug)]
    pub struct BundleManifest {
        #[doc = ""]
        pub update_type: UpdateType,
        #[doc = ""]
        pub hash_algorithm: ::std::option::Option<HashAlgorithm>,
        #[doc = ""]
        pub payloads: ::std::vec::Vec<Payload>,
    }
    impl BundleManifest {
        #[doc = "Creates a new [`BundleManifest`]."]
        pub fn new(update_type: UpdateType, payloads: ::std::vec::Vec<Payload>) -> Self {
            Self {
                update_type,
                payloads,
                hash_algorithm: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `update_type`."]
        pub fn set_update_type(&mut self, update_type: UpdateType) -> &mut Self {
            self.update_type = update_type;
            self
        }
        #[doc = "Sets the value of `update_type`."]
        pub fn with_update_type(mut self, update_type: UpdateType) -> Self {
            self.update_type = update_type;
            self
        }
        #[doc = "Sets the value of `hash_algorithm`."]
        pub fn set_hash_algorithm(
            &mut self,
            hash_algorithm: ::std::option::Option<HashAlgorithm>,
        ) -> &mut Self {
            self.hash_algorithm = hash_algorithm;
            self
        }
        #[doc = "Sets the value of `hash_algorithm`."]
        pub fn with_hash_algorithm(
            mut self,
            hash_algorithm: ::std::option::Option<HashAlgorithm>,
        ) -> Self {
            self.hash_algorithm = hash_algorithm;
            self
        }
        #[doc = "Sets the value of `payloads`."]
        pub fn set_payloads(&mut self, payloads: ::std::vec::Vec<Payload>) -> &mut Self {
            self.payloads = payloads;
            self
        }
        #[doc = "Sets the value of `payloads`."]
        pub fn with_payloads(mut self, payloads: ::std::vec::Vec<Payload>) -> Self {
            self.payloads = payloads;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for BundleManifest {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "BundleManifest", 3usize)?;
            __record.serialize_field("update-type", &self.update_type)?;
            __record.serialize_optional_field(
                "hash-algorithm",
                ::core::option::Option::as_ref(&self.hash_algorithm),
            )?;
            __record.serialize_field("payloads", &self.payloads)?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for BundleManifest {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = BundleManifest;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record BundleManifest")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::SeqAccess<'de>,
                {
                    let __field0 =
                        match __serde::de::SeqAccess::next_element::<UpdateType>(&mut __seq)? {
                            ::core::option::Option::Some(__value) => __value,
                            ::core::option::Option::None => {
                                return ::core::result::Result::Err(
                                    __serde::de::Error::invalid_length(
                                        0usize,
                                        &"record with 3 fields",
                                    ),
                                );
                            }
                        };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::std::option::Option<HashAlgorithm>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 3 fields"),
                            );
                        }
                    };
                    let __field2 = match __serde::de::SeqAccess::next_element::<
                        ::std::vec::Vec<Payload>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(2usize, &"record with 3 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(BundleManifest {
                        update_type: __field0,
                        hash_algorithm: __field1,
                        payloads: __field2,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::MapAccess<'de>,
                {
                    #[doc(hidden)]
                    const __IDENTIFIERS: &'static [&'static str] =
                        &["update-type", "hash-algorithm", "payloads"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"update-type\", \"hash-algorithm\", \"payloads\"]";
                    #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
                    #[doc(hidden)]
                    enum __Identifier {
                        __Identifier0,
                        __Identifier1,
                        __Identifier2,
                        __Unknown,
                    }
                    #[doc(hidden)]
                    struct __IdentifierVisitor;
                    impl<'de> __serde::de::Visitor<'de> for __IdentifierVisitor {
                        type Value = __Identifier;
                        fn expecting(
                            &self,
                            __formatter: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::write_str(__formatter, __EXPECTING_IDENTIFIERS)
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> ::core::result::Result<Self::Value, __E>
                        where
                            __E: __serde::de::Error,
                        {
                            match __value {
                                0u64 => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                1u64 => ::core::result::Result::Ok(__Identifier::__Identifier1),
                                2u64 => ::core::result::Result::Ok(__Identifier::__Identifier2),
                                _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> ::core::result::Result<Self::Value, __E>
                        where
                            __E: __serde::de::Error,
                        {
                            match __value {
                                "update-type" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                "hash-algorithm" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                "payloads" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier2)
                                }
                                _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> ::core::result::Result<Self::Value, __E>
                        where
                            __E: __serde::de::Error,
                        {
                            match __value {
                                b"update-type" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                b"hash-algorithm" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                b"payloads" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier2)
                                }
                                _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                            }
                        }
                    }
                    impl<'de> __serde::Deserialize<'de> for __Identifier {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> ::core::result::Result<Self, __D::Error>
                        where
                            __D: __serde::Deserializer<'de>,
                        {
                            __serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __IdentifierVisitor,
                            )
                        }
                    }
                    let mut __field0: ::core::option::Option<UpdateType> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::std::option::Option<HashAlgorithm>> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<::std::vec::Vec<Payload>> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "update-type",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<UpdateType>(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "hash-algorithm",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<HashAlgorithm>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "payloads",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<::std::vec::Vec<Payload>>(
                                        &mut __map,
                                    )?,
                                );
                            }
                            _ => {
                                __serde::de::MapAccess::next_value::<__serde::de::IgnoredAny>(
                                    &mut __map,
                                )?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("update-type"),
                            );
                        }
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field2 = match __field2 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("payloads"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(BundleManifest {
                        update_type: __field0,
                        hash_algorithm: __field1,
                        payloads: __field2,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] =
                &["update-type", "hash-algorithm", "payloads"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "BundleManifest",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = ""]
    #[derive(Clone, Debug)]
    pub enum UpdateType {
        #[doc = ""]
        Full,
        #[doc = ""]
        Incremental,
    }
    #[automatically_derived]
    impl __serde::Serialize for UpdateType {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let __serializer =
                __sidex_serde::ser::VariantSerializer::new(__serializer, "UpdateType");
            match self {
                Self::Full => __serializer.serialize_tag("full", 0u32),
                Self::Incremental => __serializer.serialize_tag("incremental", 1u32),
            }
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for UpdateType {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &["full", "incremental"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"full\", \"incremental\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            #[doc(hidden)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
            }
            #[doc(hidden)]
            struct __IdentifierVisitor;
            impl<'de> __serde::de::Visitor<'de> for __IdentifierVisitor {
                type Value = __Identifier;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, __EXPECTING_IDENTIFIERS)
                }
                fn visit_u64<__E>(self, __value: u64) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        0u64 => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        1u64 => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        __variant => {
                            ::core::result::Result::Err(__serde::de::Error::invalid_value(
                                __serde::de::Unexpected::Unsigned(__variant),
                                &__EXPECTING_IDENTIFIERS,
                            ))
                        }
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        "full" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "incremental" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        __variant => ::core::result::Result::Err(
                            __serde::de::Error::unknown_variant(__variant, __IDENTIFIERS),
                        ),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        b"full" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"incremental" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        __variant => {
                            ::core::result::Result::Err(__serde::de::Error::invalid_value(
                                __serde::de::Unexpected::Bytes(__variant),
                                &__EXPECTING_IDENTIFIERS,
                            ))
                        }
                    }
                }
            }
            impl<'de> __serde::Deserialize<'de> for __Identifier {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> ::core::result::Result<Self, __D::Error>
                where
                    __D: __serde::Deserializer<'de>,
                {
                    __serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __IdentifierVisitor,
                    )
                }
            }
            #[doc(hidden)]
            const __VARIANTS: &'static [&'static str] = &["full", "incremental"];
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = UpdateType;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "enum UpdateType")
                }
                #[inline]
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    let __identifier = __IdentifierVisitor.visit_str(__value)?;
                    #[allow(unreachable_patterns)]
                    match __identifier {
                        __Identifier::__Identifier0 => ::core::result::Result::Ok(UpdateType::Full),
                        __Identifier::__Identifier1 => {
                            ::core::result::Result::Ok(UpdateType::Incremental)
                        }
                        _ => Err(__E::invalid_value(
                            __serde::de::Unexpected::Str(__value),
                            &self,
                        )),
                    }
                }
                #[inline]
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::EnumAccess<'de>,
                {
                    match __serde::de::EnumAccess::variant::<__Identifier>(__data)? {
                        (__Identifier::__Identifier0, __variant) => {
                            __serde::de::VariantAccess::unit_variant(__variant)?;
                            ::core::result::Result::Ok(UpdateType::Full)
                        }
                        (__Identifier::__Identifier1, __variant) => {
                            __serde::de::VariantAccess::unit_variant(__variant)?;
                            ::core::result::Result::Ok(UpdateType::Incremental)
                        }
                    }
                }
            }
            __serde::Deserializer::deserialize_enum(
                __deserializer,
                "UpdateType",
                __VARIANTS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = ""]
    #[derive(Clone, Debug)]
    pub struct Payload {
        #[doc = "Payload configuration.\n"]
        pub delivery: DeliveryConfig,
        #[doc = "Filename of the payload file.\n"]
        pub filename: ::std::string::String,
        #[doc = "Block encoding.\n"]
        pub block_encoding: ::std::option::Option<BlockEncoding>,
        #[doc = "Payload file has been delta encoded.\n\nSpecifies the delta encoding used to produce the payload file.\n"]
        pub delta_encoding: ::std::option::Option<DeltaEncoding>,
    }
    impl Payload {
        #[doc = "Creates a new [`Payload`]."]
        pub fn new(delivery: DeliveryConfig, filename: ::std::string::String) -> Self {
            Self {
                delivery,
                filename,
                block_encoding: ::std::default::Default::default(),
                delta_encoding: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `delivery`."]
        pub fn set_delivery(&mut self, delivery: DeliveryConfig) -> &mut Self {
            self.delivery = delivery;
            self
        }
        #[doc = "Sets the value of `delivery`."]
        pub fn with_delivery(mut self, delivery: DeliveryConfig) -> Self {
            self.delivery = delivery;
            self
        }
        #[doc = "Sets the value of `filename`."]
        pub fn set_filename(&mut self, filename: ::std::string::String) -> &mut Self {
            self.filename = filename;
            self
        }
        #[doc = "Sets the value of `filename`."]
        pub fn with_filename(mut self, filename: ::std::string::String) -> Self {
            self.filename = filename;
            self
        }
        #[doc = "Sets the value of `block_encoding`."]
        pub fn set_block_encoding(
            &mut self,
            block_encoding: ::std::option::Option<BlockEncoding>,
        ) -> &mut Self {
            self.block_encoding = block_encoding;
            self
        }
        #[doc = "Sets the value of `block_encoding`."]
        pub fn with_block_encoding(
            mut self,
            block_encoding: ::std::option::Option<BlockEncoding>,
        ) -> Self {
            self.block_encoding = block_encoding;
            self
        }
        #[doc = "Sets the value of `delta_encoding`."]
        pub fn set_delta_encoding(
            &mut self,
            delta_encoding: ::std::option::Option<DeltaEncoding>,
        ) -> &mut Self {
            self.delta_encoding = delta_encoding;
            self
        }
        #[doc = "Sets the value of `delta_encoding`."]
        pub fn with_delta_encoding(
            mut self,
            delta_encoding: ::std::option::Option<DeltaEncoding>,
        ) -> Self {
            self.delta_encoding = delta_encoding;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for Payload {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "Payload", 4usize)?;
            __record.serialize_field("delivery", &self.delivery)?;
            __record.serialize_field("filename", &self.filename)?;
            __record.serialize_optional_field(
                "block-encoding",
                ::core::option::Option::as_ref(&self.block_encoding),
            )?;
            __record.serialize_optional_field(
                "delta-encoding",
                ::core::option::Option::as_ref(&self.delta_encoding),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Payload {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = Payload;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record Payload")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::SeqAccess<'de>,
                {
                    let __field0 =
                        match __serde::de::SeqAccess::next_element::<DeliveryConfig>(&mut __seq)? {
                            ::core::option::Option::Some(__value) => __value,
                            ::core::option::Option::None => {
                                return ::core::result::Result::Err(
                                    __serde::de::Error::invalid_length(
                                        0usize,
                                        &"record with 4 fields",
                                    ),
                                );
                            }
                        };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::std::string::String,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 4 fields"),
                            );
                        }
                    };
                    let __field2 = match __serde::de::SeqAccess::next_element::<
                        ::std::option::Option<BlockEncoding>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(2usize, &"record with 4 fields"),
                            );
                        }
                    };
                    let __field3 = match __serde::de::SeqAccess::next_element::<
                        ::std::option::Option<DeltaEncoding>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(3usize, &"record with 4 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Payload {
                        delivery: __field0,
                        filename: __field1,
                        block_encoding: __field2,
                        delta_encoding: __field3,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::MapAccess<'de>,
                {
                    #[doc(hidden)]
                    const __IDENTIFIERS: &'static [&'static str] =
                        &["delivery", "filename", "block-encoding", "delta-encoding"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS : & 'static str = "an identifier in [\"delivery\", \"filename\", \"block-encoding\", \"delta-encoding\"]" ;
                    #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
                    #[doc(hidden)]
                    enum __Identifier {
                        __Identifier0,
                        __Identifier1,
                        __Identifier2,
                        __Identifier3,
                        __Unknown,
                    }
                    #[doc(hidden)]
                    struct __IdentifierVisitor;
                    impl<'de> __serde::de::Visitor<'de> for __IdentifierVisitor {
                        type Value = __Identifier;
                        fn expecting(
                            &self,
                            __formatter: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::write_str(__formatter, __EXPECTING_IDENTIFIERS)
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> ::core::result::Result<Self::Value, __E>
                        where
                            __E: __serde::de::Error,
                        {
                            match __value {
                                0u64 => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                1u64 => ::core::result::Result::Ok(__Identifier::__Identifier1),
                                2u64 => ::core::result::Result::Ok(__Identifier::__Identifier2),
                                3u64 => ::core::result::Result::Ok(__Identifier::__Identifier3),
                                _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> ::core::result::Result<Self::Value, __E>
                        where
                            __E: __serde::de::Error,
                        {
                            match __value {
                                "delivery" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                "filename" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                "block-encoding" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier2)
                                }
                                "delta-encoding" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier3)
                                }
                                _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> ::core::result::Result<Self::Value, __E>
                        where
                            __E: __serde::de::Error,
                        {
                            match __value {
                                b"delivery" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                b"filename" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                b"block-encoding" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier2)
                                }
                                b"delta-encoding" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier3)
                                }
                                _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                            }
                        }
                    }
                    impl<'de> __serde::Deserialize<'de> for __Identifier {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> ::core::result::Result<Self, __D::Error>
                        where
                            __D: __serde::Deserializer<'de>,
                        {
                            __serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __IdentifierVisitor,
                            )
                        }
                    }
                    let mut __field0: ::core::option::Option<DeliveryConfig> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::std::string::String> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<::std::option::Option<BlockEncoding>> =
                        ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<::std::option::Option<DeltaEncoding>> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "delivery",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<DeliveryConfig>(
                                        &mut __map,
                                    )?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "filename",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<::std::string::String>(
                                        &mut __map,
                                    )?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "block-encoding",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<BlockEncoding>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier3 => {
                                if ::core::option::Option::is_some(&__field3) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "delta-encoding",
                                        ),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<DeltaEncoding>,
                                    >(&mut __map)?,
                                );
                            }
                            _ => {
                                __serde::de::MapAccess::next_value::<__serde::de::IgnoredAny>(
                                    &mut __map,
                                )?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("delivery"),
                            );
                        }
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("filename"),
                            );
                        }
                    };
                    let __field2 = match __field2 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field3 = match __field3 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(Payload {
                        delivery: __field0,
                        filename: __field1,
                        block_encoding: __field2,
                        delta_encoding: __field3,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] =
                &["delivery", "filename", "block-encoding", "delta-encoding"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "Payload",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = ""]
    #[derive(Clone, Debug)]
    pub struct DeltaEncoding {
        #[doc = "Inputs to the encoding.\n"]
        pub inputs: ::std::vec::Vec<DeltaEncodingInput>,
        #[doc = "Delta encoding format.\n"]
        pub format: DeltaEncodingFormat,
        #[doc = "Hash of the decoded data.\n"]
        pub original_hash: HashDigest,
    }
    impl DeltaEncoding {
        #[doc = "Creates a new [`DeltaEncoding`]."]
        pub fn new(
            inputs: ::std::vec::Vec<DeltaEncodingInput>,
            format: DeltaEncodingFormat,
            original_hash: HashDigest,
        ) -> Self {
            Self {
                inputs,
                format,
                original_hash,
            }
        }
        #[doc = "Sets the value of `inputs`."]
        pub fn set_inputs(&mut self, inputs: ::std::vec::Vec<DeltaEncodingInput>) -> &mut Self {
            self.inputs = inputs;
            self
        }
        #[doc = "Sets the value of `inputs`."]
        pub fn with_inputs(mut self, inputs: ::std::vec::Vec<DeltaEncodingInput>) -> Self {
            self.inputs = inputs;
            self
        }
        #[doc = "Sets the value of `format`."]
        pub fn set_format(&mut self, format: DeltaEncodingFormat) -> &mut Self {
            self.format = format;
            self
        }
        #[doc = "Sets the value of `format`."]
        pub fn with_format(mut self, format: DeltaEncodingFormat) -> Self {
            self.format = format;
            self
        }
        #[doc = "Sets the value of `original_hash`."]
        pub fn set_original_hash(&mut self, original_hash: HashDigest) -> &mut Self {
            self.original_hash = original_hash;
            self
        }
        #[doc = "Sets the value of `original_hash`."]
        pub fn with_original_hash(mut self, original_hash: HashDigest) -> Self {
            self.original_hash = original_hash;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for DeltaEncoding {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "DeltaEncoding", 3usize)?;
            __record.serialize_field("inputs", &self.inputs)?;
            __record.serialize_field("format", &self.format)?;
            __record.serialize_field("original-hash", &self.original_hash)?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for DeltaEncoding {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = DeltaEncoding;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record DeltaEncoding")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::SeqAccess<'de>,
                {
                    let __field0 = match __serde::de::SeqAccess::next_element::<
                        ::std::vec::Vec<DeltaEncodingInput>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 3 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<DeltaEncodingFormat>(
                        &mut __seq,
                    )? {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 3 fields"),
                            );
                        }
                    };
                    let __field2 =
                        match __serde::de::SeqAccess::next_element::<HashDigest>(&mut __seq)? {
                            ::core::option::Option::Some(__value) => __value,
                            ::core::option::Option::None => {
                                return ::core::result::Result::Err(
                                    __serde::de::Error::invalid_length(
                                        2usize,
                                        &"record with 3 fields",
                                    ),
                                );
                            }
                        };
                    ::core::result::Result::Ok(DeltaEncoding {
                        inputs: __field0,
                        format: __field1,
                        original_hash: __field2,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::MapAccess<'de>,
                {
                    #[doc(hidden)]
                    const __IDENTIFIERS: &'static [&'static str] =
                        &["inputs", "format", "original-hash"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"inputs\", \"format\", \"original-hash\"]";
                    #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
                    #[doc(hidden)]
                    enum __Identifier {
                        __Identifier0,
                        __Identifier1,
                        __Identifier2,
                        __Unknown,
                    }
                    #[doc(hidden)]
                    struct __IdentifierVisitor;
                    impl<'de> __serde::de::Visitor<'de> for __IdentifierVisitor {
                        type Value = __Identifier;
                        fn expecting(
                            &self,
                            __formatter: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::write_str(__formatter, __EXPECTING_IDENTIFIERS)
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> ::core::result::Result<Self::Value, __E>
                        where
                            __E: __serde::de::Error,
                        {
                            match __value {
                                0u64 => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                1u64 => ::core::result::Result::Ok(__Identifier::__Identifier1),
                                2u64 => ::core::result::Result::Ok(__Identifier::__Identifier2),
                                _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> ::core::result::Result<Self::Value, __E>
                        where
                            __E: __serde::de::Error,
                        {
                            match __value {
                                "inputs" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                "format" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                                "original-hash" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier2)
                                }
                                _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> ::core::result::Result<Self::Value, __E>
                        where
                            __E: __serde::de::Error,
                        {
                            match __value {
                                b"inputs" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                b"format" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                b"original-hash" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier2)
                                }
                                _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                            }
                        }
                    }
                    impl<'de> __serde::Deserialize<'de> for __Identifier {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> ::core::result::Result<Self, __D::Error>
                        where
                            __D: __serde::Deserializer<'de>,
                        {
                            __serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __IdentifierVisitor,
                            )
                        }
                    }
                    let mut __field0: ::core::option::Option<::std::vec::Vec<DeltaEncodingInput>> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<DeltaEncodingFormat> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<HashDigest> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "inputs",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::vec::Vec<DeltaEncodingInput>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "format",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<DeltaEncodingFormat>(
                                        &mut __map,
                                    )?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "original-hash",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<HashDigest>(&mut __map)?,
                                );
                            }
                            _ => {
                                __serde::de::MapAccess::next_value::<__serde::de::IgnoredAny>(
                                    &mut __map,
                                )?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("inputs"),
                            );
                        }
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("format"),
                            );
                        }
                    };
                    let __field2 = match __field2 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("original-hash"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(DeltaEncoding {
                        inputs: __field0,
                        format: __field1,
                        original_hash: __field2,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["inputs", "format", "original-hash"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "DeltaEncoding",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = ""]
    #[derive(Clone, Debug)]
    pub enum DeltaEncodingFormat {
        #[doc = "Format emitted by Xdelta.\n\nWhile Xdelta claims to use the VCDIFF format, the patches it produces are non-compliant.\n"]
        Xdelta,
    }
    #[automatically_derived]
    impl __serde::Serialize for DeltaEncodingFormat {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let __serializer =
                __sidex_serde::ser::VariantSerializer::new(__serializer, "DeltaEncodingFormat");
            match self {
                Self::Xdelta => __serializer.serialize_tag("xdelta", 0u32),
            }
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for DeltaEncodingFormat {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &["xdelta"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"xdelta\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            #[doc(hidden)]
            enum __Identifier {
                __Identifier0,
            }
            #[doc(hidden)]
            struct __IdentifierVisitor;
            impl<'de> __serde::de::Visitor<'de> for __IdentifierVisitor {
                type Value = __Identifier;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, __EXPECTING_IDENTIFIERS)
                }
                fn visit_u64<__E>(self, __value: u64) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        0u64 => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        __variant => {
                            ::core::result::Result::Err(__serde::de::Error::invalid_value(
                                __serde::de::Unexpected::Unsigned(__variant),
                                &__EXPECTING_IDENTIFIERS,
                            ))
                        }
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        "xdelta" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        __variant => ::core::result::Result::Err(
                            __serde::de::Error::unknown_variant(__variant, __IDENTIFIERS),
                        ),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        b"xdelta" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        __variant => {
                            ::core::result::Result::Err(__serde::de::Error::invalid_value(
                                __serde::de::Unexpected::Bytes(__variant),
                                &__EXPECTING_IDENTIFIERS,
                            ))
                        }
                    }
                }
            }
            impl<'de> __serde::Deserialize<'de> for __Identifier {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> ::core::result::Result<Self, __D::Error>
                where
                    __D: __serde::Deserializer<'de>,
                {
                    __serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __IdentifierVisitor,
                    )
                }
            }
            #[doc(hidden)]
            const __VARIANTS: &'static [&'static str] = &["xdelta"];
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = DeltaEncodingFormat;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "enum DeltaEncodingFormat")
                }
                #[inline]
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    let __identifier = __IdentifierVisitor.visit_str(__value)?;
                    #[allow(unreachable_patterns)]
                    match __identifier {
                        __Identifier::__Identifier0 => {
                            ::core::result::Result::Ok(DeltaEncodingFormat::Xdelta)
                        }
                        _ => Err(__E::invalid_value(
                            __serde::de::Unexpected::Str(__value),
                            &self,
                        )),
                    }
                }
                #[inline]
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::EnumAccess<'de>,
                {
                    match __serde::de::EnumAccess::variant::<__Identifier>(__data)? {
                        (__Identifier::__Identifier0, __variant) => {
                            __serde::de::VariantAccess::unit_variant(__variant)?;
                            ::core::result::Result::Ok(DeltaEncodingFormat::Xdelta)
                        }
                    }
                }
            }
            __serde::Deserializer::deserialize_enum(
                __deserializer,
                "DeltaEncodingFormat",
                __VARIANTS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = ""]
    #[derive(Clone, Debug)]
    pub struct DeltaEncodingInput {
        #[doc = "Hashes to identify the input.\n"]
        pub hashes: ::std::vec::Vec<HashDigest>,
    }
    impl DeltaEncodingInput {
        #[doc = "Creates a new [`DeltaEncodingInput`]."]
        pub fn new(hashes: ::std::vec::Vec<HashDigest>) -> Self {
            Self { hashes }
        }
        #[doc = "Sets the value of `hashes`."]
        pub fn set_hashes(&mut self, hashes: ::std::vec::Vec<HashDigest>) -> &mut Self {
            self.hashes = hashes;
            self
        }
        #[doc = "Sets the value of `hashes`."]
        pub fn with_hashes(mut self, hashes: ::std::vec::Vec<HashDigest>) -> Self {
            self.hashes = hashes;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for DeltaEncodingInput {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record = __sidex_serde::ser::RecordSerializer::new(
                __serializer,
                "DeltaEncodingInput",
                1usize,
            )?;
            __record.serialize_field("hashes", &self.hashes)?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for DeltaEncodingInput {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = DeltaEncodingInput;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record DeltaEncodingInput")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::SeqAccess<'de>,
                {
                    let __field0 = match __serde::de::SeqAccess::next_element::<
                        ::std::vec::Vec<HashDigest>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 1 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(DeltaEncodingInput { hashes: __field0 })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::MapAccess<'de>,
                {
                    #[doc(hidden)]
                    const __IDENTIFIERS: &'static [&'static str] = &["hashes"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"hashes\"]";
                    #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
                    #[doc(hidden)]
                    enum __Identifier {
                        __Identifier0,
                        __Unknown,
                    }
                    #[doc(hidden)]
                    struct __IdentifierVisitor;
                    impl<'de> __serde::de::Visitor<'de> for __IdentifierVisitor {
                        type Value = __Identifier;
                        fn expecting(
                            &self,
                            __formatter: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::write_str(__formatter, __EXPECTING_IDENTIFIERS)
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> ::core::result::Result<Self::Value, __E>
                        where
                            __E: __serde::de::Error,
                        {
                            match __value {
                                0u64 => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> ::core::result::Result<Self::Value, __E>
                        where
                            __E: __serde::de::Error,
                        {
                            match __value {
                                "hashes" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> ::core::result::Result<Self::Value, __E>
                        where
                            __E: __serde::de::Error,
                        {
                            match __value {
                                b"hashes" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                            }
                        }
                    }
                    impl<'de> __serde::Deserialize<'de> for __Identifier {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> ::core::result::Result<Self, __D::Error>
                        where
                            __D: __serde::Deserializer<'de>,
                        {
                            __serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __IdentifierVisitor,
                            )
                        }
                    }
                    let mut __field0: ::core::option::Option<::std::vec::Vec<HashDigest>> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "hashes",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::vec::Vec<HashDigest>,
                                    >(&mut __map)?,
                                );
                            }
                            _ => {
                                __serde::de::MapAccess::next_value::<__serde::de::IgnoredAny>(
                                    &mut __map,
                                )?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("hashes"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(DeltaEncodingInput { hashes: __field0 })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["hashes"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "DeltaEncodingInput",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = ""]
    #[derive(Clone, Debug)]
    pub enum DeliveryConfig {
        #[doc = ""]
        Slot(SlotDeliveryConfig),
        #[doc = ""]
        Execute(ExecuteDeliveryConfig),
    }
    #[automatically_derived]
    impl __serde::Serialize for DeliveryConfig {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let __serializer =
                __sidex_serde::ser::VariantSerializer::new(__serializer, "DeliveryConfig");
            match self {
                Self::Slot(__value) => {
                    __serializer.serialize_internally_tagged("type", "slot", 0u32, __value)
                }
                Self::Execute(__value) => {
                    __serializer.serialize_internally_tagged("type", "execute", 1u32, __value)
                }
            }
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for DeliveryConfig {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &["slot", "execute"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"slot\", \"execute\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            #[doc(hidden)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
            }
            #[doc(hidden)]
            struct __IdentifierVisitor;
            impl<'de> __serde::de::Visitor<'de> for __IdentifierVisitor {
                type Value = __Identifier;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, __EXPECTING_IDENTIFIERS)
                }
                fn visit_u64<__E>(self, __value: u64) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        0u64 => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        1u64 => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        __variant => {
                            ::core::result::Result::Err(__serde::de::Error::invalid_value(
                                __serde::de::Unexpected::Unsigned(__variant),
                                &__EXPECTING_IDENTIFIERS,
                            ))
                        }
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        "slot" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "execute" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        __variant => ::core::result::Result::Err(
                            __serde::de::Error::unknown_variant(__variant, __IDENTIFIERS),
                        ),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        b"slot" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"execute" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        __variant => {
                            ::core::result::Result::Err(__serde::de::Error::invalid_value(
                                __serde::de::Unexpected::Bytes(__variant),
                                &__EXPECTING_IDENTIFIERS,
                            ))
                        }
                    }
                }
            }
            impl<'de> __serde::Deserialize<'de> for __Identifier {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> ::core::result::Result<Self, __D::Error>
                where
                    __D: __serde::Deserializer<'de>,
                {
                    __serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __IdentifierVisitor,
                    )
                }
            }
            #[doc(hidden)]
            const __VARIANTS: &'static [&'static str] = &["slot", "execute"];
            if __serde::Deserializer::is_human_readable(&__deserializer) {
                let __tagged = __sidex_serde::de::tagged::deserialize_tagged_variant::<
                    __Identifier,
                    __D,
                >(__deserializer, "type")?;
                match __tagged.tag {
                    __Identifier::__Identifier0 => {
                        ::core::result::Result::Ok(DeliveryConfig::Slot(
                            __tagged
                                .deserialize_internally_tagged::<SlotDeliveryConfig, __D::Error>(
                                )?,
                        ))
                    }
                    __Identifier::__Identifier1 => {
                        ::core::result::Result::Ok(DeliveryConfig::Execute(
                            __tagged
                                .deserialize_internally_tagged::<ExecuteDeliveryConfig, __D::Error>(
                                )?,
                        ))
                    }
                }
            } else {
                #[doc(hidden)]
                struct __Visitor {
                    __phantom_vars: ::core::marker::PhantomData<fn(&())>,
                }
                impl<'de> __serde::de::Visitor<'de> for __Visitor {
                    type Value = DeliveryConfig;
                    fn expecting(
                        &self,
                        __formatter: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(__formatter, "enum DeliveryConfig")
                    }
                    #[inline]
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> ::core::result::Result<Self::Value, __E>
                    where
                        __E: __serde::de::Error,
                    {
                        let __identifier = __IdentifierVisitor.visit_str(__value)?;
                        #[allow(unreachable_patterns)]
                        match __identifier {
                            _ => Err(__E::invalid_value(
                                __serde::de::Unexpected::Str(__value),
                                &self,
                            )),
                        }
                    }
                    #[inline]
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> ::core::result::Result<Self::Value, __A::Error>
                    where
                        __A: __serde::de::EnumAccess<'de>,
                    {
                        match __serde::de::EnumAccess::variant::<__Identifier>(__data)? {
                            (__Identifier::__Identifier0, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    SlotDeliveryConfig,
                                >(__variant)?;
                                ::core::result::Result::Ok(DeliveryConfig::Slot(__value))
                            }
                            (__Identifier::__Identifier1, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    ExecuteDeliveryConfig,
                                >(__variant)?;
                                ::core::result::Result::Ok(DeliveryConfig::Execute(__value))
                            }
                        }
                    }
                }
                __serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "DeliveryConfig",
                    __VARIANTS,
                    __Visitor {
                        __phantom_vars: ::core::marker::PhantomData,
                    },
                )
            }
        }
    }
    #[doc = ""]
    #[derive(Clone, Debug)]
    pub struct SlotDeliveryConfig {
        #[doc = "Slot where the payload should be installed.\n"]
        pub slot: ::std::string::String,
    }
    impl SlotDeliveryConfig {
        #[doc = "Creates a new [`SlotDeliveryConfig`]."]
        pub fn new(slot: ::std::string::String) -> Self {
            Self { slot }
        }
        #[doc = "Sets the value of `slot`."]
        pub fn set_slot(&mut self, slot: ::std::string::String) -> &mut Self {
            self.slot = slot;
            self
        }
        #[doc = "Sets the value of `slot`."]
        pub fn with_slot(mut self, slot: ::std::string::String) -> Self {
            self.slot = slot;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for SlotDeliveryConfig {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record = __sidex_serde::ser::RecordSerializer::new(
                __serializer,
                "SlotDeliveryConfig",
                1usize,
            )?;
            __record.serialize_field("slot", &self.slot)?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for SlotDeliveryConfig {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = SlotDeliveryConfig;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record SlotDeliveryConfig")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::SeqAccess<'de>,
                {
                    let __field0 = match __serde::de::SeqAccess::next_element::<
                        ::std::string::String,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 1 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(SlotDeliveryConfig { slot: __field0 })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::MapAccess<'de>,
                {
                    #[doc(hidden)]
                    const __IDENTIFIERS: &'static [&'static str] = &["slot"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"slot\"]";
                    #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
                    #[doc(hidden)]
                    enum __Identifier {
                        __Identifier0,
                        __Unknown,
                    }
                    #[doc(hidden)]
                    struct __IdentifierVisitor;
                    impl<'de> __serde::de::Visitor<'de> for __IdentifierVisitor {
                        type Value = __Identifier;
                        fn expecting(
                            &self,
                            __formatter: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::write_str(__formatter, __EXPECTING_IDENTIFIERS)
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> ::core::result::Result<Self::Value, __E>
                        where
                            __E: __serde::de::Error,
                        {
                            match __value {
                                0u64 => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> ::core::result::Result<Self::Value, __E>
                        where
                            __E: __serde::de::Error,
                        {
                            match __value {
                                "slot" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> ::core::result::Result<Self::Value, __E>
                        where
                            __E: __serde::de::Error,
                        {
                            match __value {
                                b"slot" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                            }
                        }
                    }
                    impl<'de> __serde::Deserialize<'de> for __Identifier {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> ::core::result::Result<Self, __D::Error>
                        where
                            __D: __serde::Deserializer<'de>,
                        {
                            __serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __IdentifierVisitor,
                            )
                        }
                    }
                    let mut __field0: ::core::option::Option<::std::string::String> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("slot"),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<::std::string::String>(
                                        &mut __map,
                                    )?,
                                );
                            }
                            _ => {
                                __serde::de::MapAccess::next_value::<__serde::de::IgnoredAny>(
                                    &mut __map,
                                )?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("slot"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(SlotDeliveryConfig { slot: __field0 })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["slot"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "SlotDeliveryConfig",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = ""]
    #[derive(Clone, Debug)]
    pub struct ExecuteDeliveryConfig {
        #[doc = ""]
        pub handler: ::std::vec::Vec<::std::string::String>,
    }
    impl ExecuteDeliveryConfig {
        #[doc = "Creates a new [`ExecuteDeliveryConfig`]."]
        pub fn new(handler: ::std::vec::Vec<::std::string::String>) -> Self {
            Self { handler }
        }
        #[doc = "Sets the value of `handler`."]
        pub fn set_handler(
            &mut self,
            handler: ::std::vec::Vec<::std::string::String>,
        ) -> &mut Self {
            self.handler = handler;
            self
        }
        #[doc = "Sets the value of `handler`."]
        pub fn with_handler(mut self, handler: ::std::vec::Vec<::std::string::String>) -> Self {
            self.handler = handler;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for ExecuteDeliveryConfig {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record = __sidex_serde::ser::RecordSerializer::new(
                __serializer,
                "ExecuteDeliveryConfig",
                1usize,
            )?;
            __record.serialize_field("handler", &self.handler)?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for ExecuteDeliveryConfig {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = ExecuteDeliveryConfig;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record ExecuteDeliveryConfig")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::SeqAccess<'de>,
                {
                    let __field0 = match __serde::de::SeqAccess::next_element::<
                        ::std::vec::Vec<::std::string::String>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 1 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(ExecuteDeliveryConfig { handler: __field0 })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::MapAccess<'de>,
                {
                    #[doc(hidden)]
                    const __IDENTIFIERS: &'static [&'static str] = &["handler"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"handler\"]";
                    #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
                    #[doc(hidden)]
                    enum __Identifier {
                        __Identifier0,
                        __Unknown,
                    }
                    #[doc(hidden)]
                    struct __IdentifierVisitor;
                    impl<'de> __serde::de::Visitor<'de> for __IdentifierVisitor {
                        type Value = __Identifier;
                        fn expecting(
                            &self,
                            __formatter: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::write_str(__formatter, __EXPECTING_IDENTIFIERS)
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> ::core::result::Result<Self::Value, __E>
                        where
                            __E: __serde::de::Error,
                        {
                            match __value {
                                0u64 => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> ::core::result::Result<Self::Value, __E>
                        where
                            __E: __serde::de::Error,
                        {
                            match __value {
                                "handler" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> ::core::result::Result<Self::Value, __E>
                        where
                            __E: __serde::de::Error,
                        {
                            match __value {
                                b"handler" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                            }
                        }
                    }
                    impl<'de> __serde::Deserialize<'de> for __Identifier {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> ::core::result::Result<Self, __D::Error>
                        where
                            __D: __serde::Deserializer<'de>,
                        {
                            __serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __IdentifierVisitor,
                            )
                        }
                    }
                    let mut __field0: ::core::option::Option<
                        ::std::vec::Vec<::std::string::String>,
                    > = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "handler",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::vec::Vec<::std::string::String>,
                                    >(&mut __map)?,
                                );
                            }
                            _ => {
                                __serde::de::MapAccess::next_value::<__serde::de::IgnoredAny>(
                                    &mut __map,
                                )?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("handler"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(ExecuteDeliveryConfig { handler: __field0 })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["handler"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "ExecuteDeliveryConfig",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = ""]
    #[derive(Clone, Debug)]
    pub struct BlockEncoding {
        #[doc = ""]
        pub chunker: ChunkerAlgorithm,
        #[doc = "Indicates whether to add a block index for the payload.\n"]
        pub hash_algorithm: ::std::option::Option<HashAlgorithm>,
        #[doc = "Enable or disable block deduplication.\n"]
        pub deduplicate: ::std::option::Option<bool>,
        #[doc = ""]
        pub compression: ::std::option::Option<Compression>,
    }
    impl BlockEncoding {
        #[doc = "Creates a new [`BlockEncoding`]."]
        pub fn new(chunker: ChunkerAlgorithm) -> Self {
            Self {
                chunker,
                hash_algorithm: ::std::default::Default::default(),
                deduplicate: ::std::default::Default::default(),
                compression: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `chunker`."]
        pub fn set_chunker(&mut self, chunker: ChunkerAlgorithm) -> &mut Self {
            self.chunker = chunker;
            self
        }
        #[doc = "Sets the value of `chunker`."]
        pub fn with_chunker(mut self, chunker: ChunkerAlgorithm) -> Self {
            self.chunker = chunker;
            self
        }
        #[doc = "Sets the value of `hash_algorithm`."]
        pub fn set_hash_algorithm(
            &mut self,
            hash_algorithm: ::std::option::Option<HashAlgorithm>,
        ) -> &mut Self {
            self.hash_algorithm = hash_algorithm;
            self
        }
        #[doc = "Sets the value of `hash_algorithm`."]
        pub fn with_hash_algorithm(
            mut self,
            hash_algorithm: ::std::option::Option<HashAlgorithm>,
        ) -> Self {
            self.hash_algorithm = hash_algorithm;
            self
        }
        #[doc = "Sets the value of `deduplicate`."]
        pub fn set_deduplicate(&mut self, deduplicate: ::std::option::Option<bool>) -> &mut Self {
            self.deduplicate = deduplicate;
            self
        }
        #[doc = "Sets the value of `deduplicate`."]
        pub fn with_deduplicate(mut self, deduplicate: ::std::option::Option<bool>) -> Self {
            self.deduplicate = deduplicate;
            self
        }
        #[doc = "Sets the value of `compression`."]
        pub fn set_compression(
            &mut self,
            compression: ::std::option::Option<Compression>,
        ) -> &mut Self {
            self.compression = compression;
            self
        }
        #[doc = "Sets the value of `compression`."]
        pub fn with_compression(mut self, compression: ::std::option::Option<Compression>) -> Self {
            self.compression = compression;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for BlockEncoding {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "BlockEncoding", 4usize)?;
            __record.serialize_field("chunker", &self.chunker)?;
            __record.serialize_optional_field(
                "hash-algorithm",
                ::core::option::Option::as_ref(&self.hash_algorithm),
            )?;
            __record.serialize_optional_field(
                "deduplicate",
                ::core::option::Option::as_ref(&self.deduplicate),
            )?;
            __record.serialize_optional_field(
                "compression",
                ::core::option::Option::as_ref(&self.compression),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for BlockEncoding {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = BlockEncoding;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record BlockEncoding")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::SeqAccess<'de>,
                {
                    let __field0 =
                        match __serde::de::SeqAccess::next_element::<ChunkerAlgorithm>(&mut __seq)?
                        {
                            ::core::option::Option::Some(__value) => __value,
                            ::core::option::Option::None => {
                                return ::core::result::Result::Err(
                                    __serde::de::Error::invalid_length(
                                        0usize,
                                        &"record with 4 fields",
                                    ),
                                );
                            }
                        };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::std::option::Option<HashAlgorithm>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 4 fields"),
                            );
                        }
                    };
                    let __field2 = match __serde::de::SeqAccess::next_element::<
                        ::std::option::Option<bool>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(2usize, &"record with 4 fields"),
                            );
                        }
                    };
                    let __field3 = match __serde::de::SeqAccess::next_element::<
                        ::std::option::Option<Compression>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(3usize, &"record with 4 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(BlockEncoding {
                        chunker: __field0,
                        hash_algorithm: __field1,
                        deduplicate: __field2,
                        compression: __field3,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::MapAccess<'de>,
                {
                    #[doc(hidden)]
                    const __IDENTIFIERS: &'static [&'static str] =
                        &["chunker", "hash-algorithm", "deduplicate", "compression"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS : & 'static str = "an identifier in [\"chunker\", \"hash-algorithm\", \"deduplicate\", \"compression\"]" ;
                    #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
                    #[doc(hidden)]
                    enum __Identifier {
                        __Identifier0,
                        __Identifier1,
                        __Identifier2,
                        __Identifier3,
                        __Unknown,
                    }
                    #[doc(hidden)]
                    struct __IdentifierVisitor;
                    impl<'de> __serde::de::Visitor<'de> for __IdentifierVisitor {
                        type Value = __Identifier;
                        fn expecting(
                            &self,
                            __formatter: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::write_str(__formatter, __EXPECTING_IDENTIFIERS)
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> ::core::result::Result<Self::Value, __E>
                        where
                            __E: __serde::de::Error,
                        {
                            match __value {
                                0u64 => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                1u64 => ::core::result::Result::Ok(__Identifier::__Identifier1),
                                2u64 => ::core::result::Result::Ok(__Identifier::__Identifier2),
                                3u64 => ::core::result::Result::Ok(__Identifier::__Identifier3),
                                _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> ::core::result::Result<Self::Value, __E>
                        where
                            __E: __serde::de::Error,
                        {
                            match __value {
                                "chunker" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                "hash-algorithm" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                "deduplicate" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier2)
                                }
                                "compression" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier3)
                                }
                                _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> ::core::result::Result<Self::Value, __E>
                        where
                            __E: __serde::de::Error,
                        {
                            match __value {
                                b"chunker" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                b"hash-algorithm" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                b"deduplicate" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier2)
                                }
                                b"compression" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier3)
                                }
                                _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                            }
                        }
                    }
                    impl<'de> __serde::Deserialize<'de> for __Identifier {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> ::core::result::Result<Self, __D::Error>
                        where
                            __D: __serde::Deserializer<'de>,
                        {
                            __serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __IdentifierVisitor,
                            )
                        }
                    }
                    let mut __field0: ::core::option::Option<ChunkerAlgorithm> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::std::option::Option<HashAlgorithm>> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<::std::option::Option<bool>> =
                        ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<::std::option::Option<Compression>> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "chunker",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<ChunkerAlgorithm>(
                                        &mut __map,
                                    )?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "hash-algorithm",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<HashAlgorithm>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "deduplicate",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<bool>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier3 => {
                                if ::core::option::Option::is_some(&__field3) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "compression",
                                        ),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<Compression>,
                                    >(&mut __map)?,
                                );
                            }
                            _ => {
                                __serde::de::MapAccess::next_value::<__serde::de::IgnoredAny>(
                                    &mut __map,
                                )?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("chunker"),
                            );
                        }
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field2 = match __field2 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field3 = match __field3 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(BlockEncoding {
                        chunker: __field0,
                        hash_algorithm: __field1,
                        deduplicate: __field2,
                        compression: __field3,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] =
                &["chunker", "hash-algorithm", "deduplicate", "compression"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "BlockEncoding",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = ""]
    #[derive(Clone, Debug)]
    pub enum Compression {
        #[doc = ""]
        Xz(XzCompression),
    }
    #[automatically_derived]
    impl __serde::Serialize for Compression {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let __serializer =
                __sidex_serde::ser::VariantSerializer::new(__serializer, "Compression");
            match self {
                Self::Xz(__value) => {
                    __serializer.serialize_internally_tagged("type", "xz", 0u32, __value)
                }
            }
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Compression {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &["xz"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"xz\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            #[doc(hidden)]
            enum __Identifier {
                __Identifier0,
            }
            #[doc(hidden)]
            struct __IdentifierVisitor;
            impl<'de> __serde::de::Visitor<'de> for __IdentifierVisitor {
                type Value = __Identifier;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, __EXPECTING_IDENTIFIERS)
                }
                fn visit_u64<__E>(self, __value: u64) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        0u64 => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        __variant => {
                            ::core::result::Result::Err(__serde::de::Error::invalid_value(
                                __serde::de::Unexpected::Unsigned(__variant),
                                &__EXPECTING_IDENTIFIERS,
                            ))
                        }
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        "xz" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        __variant => ::core::result::Result::Err(
                            __serde::de::Error::unknown_variant(__variant, __IDENTIFIERS),
                        ),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> ::core::result::Result<Self::Value, __E>
                where
                    __E: __serde::de::Error,
                {
                    match __value {
                        b"xz" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        __variant => {
                            ::core::result::Result::Err(__serde::de::Error::invalid_value(
                                __serde::de::Unexpected::Bytes(__variant),
                                &__EXPECTING_IDENTIFIERS,
                            ))
                        }
                    }
                }
            }
            impl<'de> __serde::Deserialize<'de> for __Identifier {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> ::core::result::Result<Self, __D::Error>
                where
                    __D: __serde::Deserializer<'de>,
                {
                    __serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __IdentifierVisitor,
                    )
                }
            }
            #[doc(hidden)]
            const __VARIANTS: &'static [&'static str] = &["xz"];
            if __serde::Deserializer::is_human_readable(&__deserializer) {
                let __tagged = __sidex_serde::de::tagged::deserialize_tagged_variant::<
                    __Identifier,
                    __D,
                >(__deserializer, "type")?;
                match __tagged.tag {
                    __Identifier::__Identifier0 => ::core::result::Result::Ok(Compression::Xz(
                        __tagged.deserialize_internally_tagged::<XzCompression, __D::Error>()?,
                    )),
                }
            } else {
                #[doc(hidden)]
                struct __Visitor {
                    __phantom_vars: ::core::marker::PhantomData<fn(&())>,
                }
                impl<'de> __serde::de::Visitor<'de> for __Visitor {
                    type Value = Compression;
                    fn expecting(
                        &self,
                        __formatter: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(__formatter, "enum Compression")
                    }
                    #[inline]
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> ::core::result::Result<Self::Value, __E>
                    where
                        __E: __serde::de::Error,
                    {
                        let __identifier = __IdentifierVisitor.visit_str(__value)?;
                        #[allow(unreachable_patterns)]
                        match __identifier {
                            _ => Err(__E::invalid_value(
                                __serde::de::Unexpected::Str(__value),
                                &self,
                            )),
                        }
                    }
                    #[inline]
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> ::core::result::Result<Self::Value, __A::Error>
                    where
                        __A: __serde::de::EnumAccess<'de>,
                    {
                        match __serde::de::EnumAccess::variant::<__Identifier>(__data)? {
                            (__Identifier::__Identifier0, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    XzCompression,
                                >(__variant)?;
                                ::core::result::Result::Ok(Compression::Xz(__value))
                            }
                        }
                    }
                }
                __serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "Compression",
                    __VARIANTS,
                    __Visitor {
                        __phantom_vars: ::core::marker::PhantomData,
                    },
                )
            }
        }
    }
    #[doc = ""]
    #[derive(Clone, Debug)]
    pub struct XzCompression {
        #[doc = ""]
        pub level: ::std::option::Option<u8>,
    }
    impl XzCompression {
        #[doc = "Creates a new [`XzCompression`]."]
        pub fn new() -> Self {
            Self {
                level: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `level`."]
        pub fn set_level(&mut self, level: ::std::option::Option<u8>) -> &mut Self {
            self.level = level;
            self
        }
        #[doc = "Sets the value of `level`."]
        pub fn with_level(mut self, level: ::std::option::Option<u8>) -> Self {
            self.level = level;
            self
        }
    }
    impl ::std::default::Default for XzCompression {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for XzCompression {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "XzCompression", 1usize)?;
            __record
                .serialize_optional_field("level", ::core::option::Option::as_ref(&self.level))?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for XzCompression {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = XzCompression;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record XzCompression")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::SeqAccess<'de>,
                {
                    let __field0 = match __serde::de::SeqAccess::next_element::<
                        ::std::option::Option<u8>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 1 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(XzCompression { level: __field0 })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::MapAccess<'de>,
                {
                    #[doc(hidden)]
                    const __IDENTIFIERS: &'static [&'static str] = &["level"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"level\"]";
                    #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
                    #[doc(hidden)]
                    enum __Identifier {
                        __Identifier0,
                        __Unknown,
                    }
                    #[doc(hidden)]
                    struct __IdentifierVisitor;
                    impl<'de> __serde::de::Visitor<'de> for __IdentifierVisitor {
                        type Value = __Identifier;
                        fn expecting(
                            &self,
                            __formatter: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            ::core::fmt::Formatter::write_str(__formatter, __EXPECTING_IDENTIFIERS)
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> ::core::result::Result<Self::Value, __E>
                        where
                            __E: __serde::de::Error,
                        {
                            match __value {
                                0u64 => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> ::core::result::Result<Self::Value, __E>
                        where
                            __E: __serde::de::Error,
                        {
                            match __value {
                                "level" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> ::core::result::Result<Self::Value, __E>
                        where
                            __E: __serde::de::Error,
                        {
                            match __value {
                                b"level" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                _ => ::core::result::Result::Ok(__Identifier::__Unknown),
                            }
                        }
                    }
                    impl<'de> __serde::Deserialize<'de> for __Identifier {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> ::core::result::Result<Self, __D::Error>
                        where
                            __D: __serde::Deserializer<'de>,
                        {
                            __serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __IdentifierVisitor,
                            )
                        }
                    }
                    let mut __field0: ::core::option::Option<::std::option::Option<u8>> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "level",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<::std::option::Option<u8>>(
                                        &mut __map,
                                    )?,
                                );
                            }
                            _ => {
                                __serde::de::MapAccess::next_value::<__serde::de::IgnoredAny>(
                                    &mut __map,
                                )?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(XzCompression { level: __field0 })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["level"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "XzCompression",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
}
