/* GENERATED WITH SIDEX. DO NOT MODIFY! */

#![allow(warnings)]

pub mod bootstrapping {
    #![doc = "Bootstrapping configuration.\n"]
    #[allow(unused)]
    use :: serde as __serde;
    #[allow(unused)]
    use :: sidex_serde as __sidex_serde;
    #[doc = ""]
    pub type NumBytes = byte_calc::NumBytes;
    #[doc = ""]
    pub type PartitionType = rugix_common::disk::PartitionType;
    #[doc = "Bootstrapping configuration.\n"]
    #[derive(Clone, Debug)]
    pub struct BootstrappingConfig {
        #[doc = "Disable bootstrapping altogether.\n"]
        pub disabled: ::std::option::Option<bool>,
        #[doc = "System layout configuration to use for bootstrapping.\n"]
        pub layout: ::std::option::Option<SystemLayoutConfig>,
    }
    impl BootstrappingConfig {
        #[doc = "Creates a new [`BootstrappingConfig`]."]
        pub fn new() -> Self {
            Self {
                disabled: ::std::default::Default::default(),
                layout: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `disabled`."]
        pub fn set_disabled(&mut self, disabled: ::std::option::Option<bool>) -> &mut Self {
            self.disabled = disabled;
            self
        }
        #[doc = "Sets the value of `disabled`."]
        pub fn with_disabled(mut self, disabled: ::std::option::Option<bool>) -> Self {
            self.disabled = disabled;
            self
        }
        #[doc = "Sets the value of `layout`."]
        pub fn set_layout(
            &mut self,
            layout: ::std::option::Option<SystemLayoutConfig>,
        ) -> &mut Self {
            self.layout = layout;
            self
        }
        #[doc = "Sets the value of `layout`."]
        pub fn with_layout(mut self, layout: ::std::option::Option<SystemLayoutConfig>) -> Self {
            self.layout = layout;
            self
        }
    }
    impl ::std::default::Default for BootstrappingConfig {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for BootstrappingConfig {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record = __sidex_serde::ser::RecordSerializer::new(
                __serializer,
                "BootstrappingConfig",
                2usize,
            )?;
            __record.serialize_optional_field(
                "disabled",
                ::core::option::Option::as_ref(&self.disabled),
            )?;
            __record
                .serialize_optional_field("layout", ::core::option::Option::as_ref(&self.layout))?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for BootstrappingConfig {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = BootstrappingConfig;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record BootstrappingConfig")
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
                        ::std::option::Option<bool>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 2 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::std::option::Option<SystemLayoutConfig>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 2 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(BootstrappingConfig {
                        disabled: __field0,
                        layout: __field1,
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
                    const __IDENTIFIERS: &'static [&'static str] = &["disabled", "layout"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"disabled\", \"layout\"]";
                    #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
                    #[doc(hidden)]
                    enum __Identifier {
                        __Identifier0,
                        __Identifier1,
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
                                "disabled" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                "layout" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
                                b"disabled" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                b"layout" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
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
                    let mut __field0: ::core::option::Option<::std::option::Option<bool>> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<
                        ::std::option::Option<SystemLayoutConfig>,
                    > = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "disabled",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<bool>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "layout",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<SystemLayoutConfig>,
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
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(BootstrappingConfig {
                        disabled: __field0,
                        layout: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["disabled", "layout"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "BootstrappingConfig",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "System layout configuration.\n"]
    #[derive(Clone, Debug)]
    pub enum SystemLayoutConfig {
        #[doc = "MBR partition layout.\n"]
        Mbr(PartitionLayoutConfig),
        #[doc = "GPT partition layout.\n"]
        Gpt(PartitionLayoutConfig),
        #[doc = "Default partition layout.\n"]
        Default(DefaultLayoutConfig),
        #[doc = "No partition layout.\n"]
        None,
    }
    #[automatically_derived]
    impl __serde::Serialize for SystemLayoutConfig {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let __serializer =
                __sidex_serde::ser::VariantSerializer::new(__serializer, "SystemLayoutConfig");
            match self {
                Self::Mbr(__value) => {
                    __serializer.serialize_internally_tagged("type", "mbr", 0u32, __value)
                }
                Self::Gpt(__value) => {
                    __serializer.serialize_internally_tagged("type", "gpt", 1u32, __value)
                }
                Self::Default(__value) => {
                    __serializer.serialize_internally_tagged("type", "default", 2u32, __value)
                }
                Self::None => __serializer.serialize_internal_tag("type", "none", 3u32),
            }
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for SystemLayoutConfig {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &["mbr", "gpt", "default", "none"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"mbr\", \"gpt\", \"default\", \"none\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            #[doc(hidden)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Identifier2,
                __Identifier3,
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
                        2u64 => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        3u64 => ::core::result::Result::Ok(__Identifier::__Identifier3),
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
                        "mbr" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "gpt" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "default" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "none" => ::core::result::Result::Ok(__Identifier::__Identifier3),
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
                        b"mbr" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"gpt" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"default" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        b"none" => ::core::result::Result::Ok(__Identifier::__Identifier3),
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
            const __VARIANTS: &'static [&'static str] = &["mbr", "gpt", "default", "none"];
            if __serde::Deserializer::is_human_readable(&__deserializer) {
                let __tagged = __sidex_serde::de::tagged::deserialize_tagged_variant::<
                    __Identifier,
                    __D,
                >(__deserializer, "type")?;
                match __tagged.tag {
                    __Identifier::__Identifier0 => {
                        ::core::result::Result::Ok(SystemLayoutConfig::Mbr(
                            __tagged
                                .deserialize_internally_tagged::<PartitionLayoutConfig, __D::Error>(
                                )?,
                        ))
                    }
                    __Identifier::__Identifier1 => {
                        ::core::result::Result::Ok(SystemLayoutConfig::Gpt(
                            __tagged
                                .deserialize_internally_tagged::<PartitionLayoutConfig, __D::Error>(
                                )?,
                        ))
                    }
                    __Identifier::__Identifier2 => {
                        ::core::result::Result::Ok(SystemLayoutConfig::Default(
                            __tagged
                                .deserialize_internally_tagged::<DefaultLayoutConfig, __D::Error>(
                                )?,
                        ))
                    }
                    __Identifier::__Identifier3 => {
                        ::core::result::Result::Ok(SystemLayoutConfig::None)
                    }
                }
            } else {
                #[doc(hidden)]
                struct __Visitor {
                    __phantom_vars: ::core::marker::PhantomData<fn(&())>,
                }
                impl<'de> __serde::de::Visitor<'de> for __Visitor {
                    type Value = SystemLayoutConfig;
                    fn expecting(
                        &self,
                        __formatter: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(__formatter, "enum SystemLayoutConfig")
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
                            __Identifier::__Identifier3 => {
                                ::core::result::Result::Ok(SystemLayoutConfig::None)
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
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    PartitionLayoutConfig,
                                >(__variant)?;
                                ::core::result::Result::Ok(SystemLayoutConfig::Mbr(__value))
                            }
                            (__Identifier::__Identifier1, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    PartitionLayoutConfig,
                                >(__variant)?;
                                ::core::result::Result::Ok(SystemLayoutConfig::Gpt(__value))
                            }
                            (__Identifier::__Identifier2, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    DefaultLayoutConfig,
                                >(__variant)?;
                                ::core::result::Result::Ok(SystemLayoutConfig::Default(__value))
                            }
                            (__Identifier::__Identifier3, __variant) => {
                                __serde::de::VariantAccess::unit_variant(__variant)?;
                                ::core::result::Result::Ok(SystemLayoutConfig::None)
                            }
                        }
                    }
                }
                __serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "SystemLayoutConfig",
                    __VARIANTS,
                    __Visitor {
                        __phantom_vars: ::core::marker::PhantomData,
                    },
                )
            }
        }
    }
    #[doc = "Partition layout configuration.\n"]
    #[derive(Clone, Debug)]
    pub struct PartitionLayoutConfig {
        #[doc = "Partitions of the layout.\n"]
        pub partitions: ::std::vec::Vec<LayoutPartitionConfig>,
    }
    impl PartitionLayoutConfig {
        #[doc = "Creates a new [`PartitionLayoutConfig`]."]
        pub fn new(partitions: ::std::vec::Vec<LayoutPartitionConfig>) -> Self {
            Self { partitions }
        }
        #[doc = "Sets the value of `partitions`."]
        pub fn set_partitions(
            &mut self,
            partitions: ::std::vec::Vec<LayoutPartitionConfig>,
        ) -> &mut Self {
            self.partitions = partitions;
            self
        }
        #[doc = "Sets the value of `partitions`."]
        pub fn with_partitions(
            mut self,
            partitions: ::std::vec::Vec<LayoutPartitionConfig>,
        ) -> Self {
            self.partitions = partitions;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for PartitionLayoutConfig {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record = __sidex_serde::ser::RecordSerializer::new(
                __serializer,
                "PartitionLayoutConfig",
                1usize,
            )?;
            __record.serialize_field("partitions", &self.partitions)?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for PartitionLayoutConfig {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = PartitionLayoutConfig;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record PartitionLayoutConfig")
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
                        ::std::vec::Vec<LayoutPartitionConfig>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 1 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(PartitionLayoutConfig {
                        partitions: __field0,
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
                    const __IDENTIFIERS: &'static [&'static str] = &["partitions"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"partitions\"]";
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
                                "partitions" => {
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
                                b"partitions" => {
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
                        ::std::vec::Vec<LayoutPartitionConfig>,
                    > = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "partitions",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::vec::Vec<LayoutPartitionConfig>,
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
                                <__A::Error as __serde::de::Error>::missing_field("partitions"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(PartitionLayoutConfig {
                        partitions: __field0,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["partitions"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "PartitionLayoutConfig",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "Configuration of a partition of a layout.\n"]
    #[derive(Clone, Debug)]
    pub struct LayoutPartitionConfig {
        #[doc = "Optional name of the partition.\n"]
        pub name: ::std::option::Option<::std::string::String>,
        #[doc = "Size of the partition.\n"]
        pub size: ::std::option::Option<NumBytes>,
        #[doc = "Type of the partition (one byte hex or GUID).\n"]
        pub ty: ::std::option::Option<PartitionType>,
        #[doc = "Filesystem of the partition.\n"]
        pub filesystem: ::std::option::Option<Filesystem>,
    }
    impl LayoutPartitionConfig {
        #[doc = "Creates a new [`LayoutPartitionConfig`]."]
        pub fn new() -> Self {
            Self {
                name: ::std::default::Default::default(),
                size: ::std::default::Default::default(),
                ty: ::std::default::Default::default(),
                filesystem: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `name`."]
        pub fn set_name(
            &mut self,
            name: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.name = name;
            self
        }
        #[doc = "Sets the value of `name`."]
        pub fn with_name(mut self, name: ::std::option::Option<::std::string::String>) -> Self {
            self.name = name;
            self
        }
        #[doc = "Sets the value of `size`."]
        pub fn set_size(&mut self, size: ::std::option::Option<NumBytes>) -> &mut Self {
            self.size = size;
            self
        }
        #[doc = "Sets the value of `size`."]
        pub fn with_size(mut self, size: ::std::option::Option<NumBytes>) -> Self {
            self.size = size;
            self
        }
        #[doc = "Sets the value of `ty`."]
        pub fn set_ty(&mut self, ty: ::std::option::Option<PartitionType>) -> &mut Self {
            self.ty = ty;
            self
        }
        #[doc = "Sets the value of `ty`."]
        pub fn with_ty(mut self, ty: ::std::option::Option<PartitionType>) -> Self {
            self.ty = ty;
            self
        }
        #[doc = "Sets the value of `filesystem`."]
        pub fn set_filesystem(
            &mut self,
            filesystem: ::std::option::Option<Filesystem>,
        ) -> &mut Self {
            self.filesystem = filesystem;
            self
        }
        #[doc = "Sets the value of `filesystem`."]
        pub fn with_filesystem(mut self, filesystem: ::std::option::Option<Filesystem>) -> Self {
            self.filesystem = filesystem;
            self
        }
    }
    impl ::std::default::Default for LayoutPartitionConfig {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for LayoutPartitionConfig {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record = __sidex_serde::ser::RecordSerializer::new(
                __serializer,
                "LayoutPartitionConfig",
                4usize,
            )?;
            __record
                .serialize_optional_field("name", ::core::option::Option::as_ref(&self.name))?;
            __record
                .serialize_optional_field("size", ::core::option::Option::as_ref(&self.size))?;
            __record.serialize_optional_field("type", ::core::option::Option::as_ref(&self.ty))?;
            __record.serialize_optional_field(
                "filesystem",
                ::core::option::Option::as_ref(&self.filesystem),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for LayoutPartitionConfig {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = LayoutPartitionConfig;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record LayoutPartitionConfig")
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
                        ::std::option::Option<::std::string::String>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 4 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::std::option::Option<NumBytes>,
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
                        ::std::option::Option<PartitionType>,
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
                        ::std::option::Option<Filesystem>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(3usize, &"record with 4 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(LayoutPartitionConfig {
                        name: __field0,
                        size: __field1,
                        ty: __field2,
                        filesystem: __field3,
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
                        &["name", "size", "type", "filesystem"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"name\", \"size\", \"type\", \"filesystem\"]";
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
                                "name" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                "size" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                                "type" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                                "filesystem" => {
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
                                b"name" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                b"size" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                                b"type" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                                b"filesystem" => {
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
                    let mut __field0: ::core::option::Option<
                        ::std::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::std::option::Option<NumBytes>> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<::std::option::Option<PartitionType>> =
                        ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<::std::option::Option<Filesystem>> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("name"),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<::std::string::String>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("size"),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<NumBytes>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("type"),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<PartitionType>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier3 => {
                                if ::core::option::Option::is_some(&__field3) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "filesystem",
                                        ),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<Filesystem>,
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
                        ::core::option::Option::None => ::core::option::Option::None,
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
                    ::core::result::Result::Ok(LayoutPartitionConfig {
                        name: __field0,
                        size: __field1,
                        ty: __field2,
                        filesystem: __field3,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["name", "size", "type", "filesystem"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "LayoutPartitionConfig",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = ""]
    #[derive(Clone, Debug)]
    pub enum Filesystem {
        #[doc = ""]
        Ext4(Ext4Filesystem),
    }
    #[automatically_derived]
    impl __serde::Serialize for Filesystem {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let __serializer =
                __sidex_serde::ser::VariantSerializer::new(__serializer, "Filesystem");
            match self {
                Self::Ext4(__value) => {
                    __serializer.serialize_internally_tagged("type", "ext4", 0u32, __value)
                }
            }
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Filesystem {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &["ext4"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"ext4\"]";
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
                        "ext4" => ::core::result::Result::Ok(__Identifier::__Identifier0),
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
                        b"ext4" => ::core::result::Result::Ok(__Identifier::__Identifier0),
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
            const __VARIANTS: &'static [&'static str] = &["ext4"];
            if __serde::Deserializer::is_human_readable(&__deserializer) {
                let __tagged = __sidex_serde::de::tagged::deserialize_tagged_variant::<
                    __Identifier,
                    __D,
                >(__deserializer, "type")?;
                match __tagged.tag {
                    __Identifier::__Identifier0 => ::core::result::Result::Ok(Filesystem::Ext4(
                        __tagged.deserialize_internally_tagged::<Ext4Filesystem, __D::Error>()?,
                    )),
                }
            } else {
                #[doc(hidden)]
                struct __Visitor {
                    __phantom_vars: ::core::marker::PhantomData<fn(&())>,
                }
                impl<'de> __serde::de::Visitor<'de> for __Visitor {
                    type Value = Filesystem;
                    fn expecting(
                        &self,
                        __formatter: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(__formatter, "enum Filesystem")
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
                                    Ext4Filesystem,
                                >(__variant)?;
                                ::core::result::Result::Ok(Filesystem::Ext4(__value))
                            }
                        }
                    }
                }
                __serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "Filesystem",
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
    pub struct Ext4Filesystem {
        #[doc = ""]
        pub label: ::std::option::Option<::std::string::String>,
    }
    impl Ext4Filesystem {
        #[doc = "Creates a new [`Ext4Filesystem`]."]
        pub fn new() -> Self {
            Self {
                label: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `label`."]
        pub fn set_label(
            &mut self,
            label: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.label = label;
            self
        }
        #[doc = "Sets the value of `label`."]
        pub fn with_label(mut self, label: ::std::option::Option<::std::string::String>) -> Self {
            self.label = label;
            self
        }
    }
    impl ::std::default::Default for Ext4Filesystem {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for Ext4Filesystem {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "Ext4Filesystem", 1usize)?;
            __record
                .serialize_optional_field("label", ::core::option::Option::as_ref(&self.label))?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Ext4Filesystem {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = Ext4Filesystem;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record Ext4Filesystem")
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
                        ::std::option::Option<::std::string::String>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 1 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Ext4Filesystem { label: __field0 })
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
                    const __IDENTIFIERS: &'static [&'static str] = &["label"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"label\"]";
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
                                "label" => ::core::result::Result::Ok(__Identifier::__Identifier0),
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
                                b"label" => ::core::result::Result::Ok(__Identifier::__Identifier0),
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
                        ::std::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "label",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<::std::string::String>,
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
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(Ext4Filesystem { label: __field0 })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["label"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "Ext4Filesystem",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "Default layout configuration.\n"]
    #[derive(Clone, Debug)]
    pub struct DefaultLayoutConfig {
        #[doc = "Size of the system partitions.\n"]
        pub system_size: NumBytes,
    }
    impl DefaultLayoutConfig {
        #[doc = "Creates a new [`DefaultLayoutConfig`]."]
        pub fn new(system_size: NumBytes) -> Self {
            Self { system_size }
        }
        #[doc = "Sets the value of `system_size`."]
        pub fn set_system_size(&mut self, system_size: NumBytes) -> &mut Self {
            self.system_size = system_size;
            self
        }
        #[doc = "Sets the value of `system_size`."]
        pub fn with_system_size(mut self, system_size: NumBytes) -> Self {
            self.system_size = system_size;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for DefaultLayoutConfig {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record = __sidex_serde::ser::RecordSerializer::new(
                __serializer,
                "DefaultLayoutConfig",
                1usize,
            )?;
            __record.serialize_field("system-size", &self.system_size)?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for DefaultLayoutConfig {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = DefaultLayoutConfig;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record DefaultLayoutConfig")
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
                        match __serde::de::SeqAccess::next_element::<NumBytes>(&mut __seq)? {
                            ::core::option::Option::Some(__value) => __value,
                            ::core::option::Option::None => {
                                return ::core::result::Result::Err(
                                    __serde::de::Error::invalid_length(
                                        0usize,
                                        &"record with 1 fields",
                                    ),
                                );
                            }
                        };
                    ::core::result::Result::Ok(DefaultLayoutConfig {
                        system_size: __field0,
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
                    const __IDENTIFIERS: &'static [&'static str] = &["system-size"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"system-size\"]";
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
                                "system-size" => {
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
                                b"system-size" => {
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
                    let mut __field0: ::core::option::Option<NumBytes> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "system-size",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<NumBytes>(&mut __map)?,
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
                                <__A::Error as __serde::de::Error>::missing_field("system-size"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(DefaultLayoutConfig {
                        system_size: __field0,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["system-size"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "DefaultLayoutConfig",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
}
pub mod output {
    #![doc = ""]
    #[allow(unused)]
    use :: serde as __serde;
    #[allow(unused)]
    use :: sidex_serde as __sidex_serde;
    #[doc = ""]
    #[derive(Clone, Debug)]
    pub struct SystemStateOutput {
        #[doc = ""]
        pub slots: indexmap::IndexMap<::std::string::String, SlotStateOutput>,
        #[doc = ""]
        pub boot: ::std::option::Option<BootStateOutput>,
    }
    impl SystemStateOutput {
        #[doc = "Creates a new [`SystemStateOutput`]."]
        pub fn new(slots: indexmap::IndexMap<::std::string::String, SlotStateOutput>) -> Self {
            Self {
                slots,
                boot: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `slots`."]
        pub fn set_slots(
            &mut self,
            slots: indexmap::IndexMap<::std::string::String, SlotStateOutput>,
        ) -> &mut Self {
            self.slots = slots;
            self
        }
        #[doc = "Sets the value of `slots`."]
        pub fn with_slots(
            mut self,
            slots: indexmap::IndexMap<::std::string::String, SlotStateOutput>,
        ) -> Self {
            self.slots = slots;
            self
        }
        #[doc = "Sets the value of `boot`."]
        pub fn set_boot(&mut self, boot: ::std::option::Option<BootStateOutput>) -> &mut Self {
            self.boot = boot;
            self
        }
        #[doc = "Sets the value of `boot`."]
        pub fn with_boot(mut self, boot: ::std::option::Option<BootStateOutput>) -> Self {
            self.boot = boot;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for SystemStateOutput {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record = __sidex_serde::ser::RecordSerializer::new(
                __serializer,
                "SystemStateOutput",
                2usize,
            )?;
            __record.serialize_field("slots", &self.slots)?;
            __record
                .serialize_optional_field("boot", ::core::option::Option::as_ref(&self.boot))?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for SystemStateOutput {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = SystemStateOutput;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record SystemStateOutput")
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
                        indexmap::IndexMap<::std::string::String, SlotStateOutput>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 2 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::std::option::Option<BootStateOutput>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 2 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(SystemStateOutput {
                        slots: __field0,
                        boot: __field1,
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
                    const __IDENTIFIERS: &'static [&'static str] = &["slots", "boot"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"slots\", \"boot\"]";
                    #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
                    #[doc(hidden)]
                    enum __Identifier {
                        __Identifier0,
                        __Identifier1,
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
                                "slots" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                "boot" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
                                b"slots" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                b"boot" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
                        indexmap::IndexMap<::std::string::String, SlotStateOutput>,
                    > = ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<
                        ::std::option::Option<BootStateOutput>,
                    > = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "slots",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        indexmap::IndexMap<::std::string::String, SlotStateOutput>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("boot"),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<BootStateOutput>,
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
                                <__A::Error as __serde::de::Error>::missing_field("slots"),
                            );
                        }
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(SystemStateOutput {
                        slots: __field0,
                        boot: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["slots", "boot"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "SystemStateOutput",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = ""]
    #[derive(Clone, Debug)]
    pub struct SlotStateOutput {
        #[doc = ""]
        pub active: ::std::option::Option<bool>,
        #[doc = ""]
        pub hashes:
            ::std::option::Option<indexmap::IndexMap<::std::string::String, ::std::string::String>>,
        #[doc = ""]
        pub size: ::std::option::Option<u64>,
        #[doc = ""]
        pub updated_at: ::std::option::Option<::std::string::String>,
    }
    impl SlotStateOutput {
        #[doc = "Creates a new [`SlotStateOutput`]."]
        pub fn new() -> Self {
            Self {
                active: ::std::default::Default::default(),
                hashes: ::std::default::Default::default(),
                size: ::std::default::Default::default(),
                updated_at: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `active`."]
        pub fn set_active(&mut self, active: ::std::option::Option<bool>) -> &mut Self {
            self.active = active;
            self
        }
        #[doc = "Sets the value of `active`."]
        pub fn with_active(mut self, active: ::std::option::Option<bool>) -> Self {
            self.active = active;
            self
        }
        #[doc = "Sets the value of `hashes`."]
        pub fn set_hashes(
            &mut self,
            hashes: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, ::std::string::String>,
            >,
        ) -> &mut Self {
            self.hashes = hashes;
            self
        }
        #[doc = "Sets the value of `hashes`."]
        pub fn with_hashes(
            mut self,
            hashes: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, ::std::string::String>,
            >,
        ) -> Self {
            self.hashes = hashes;
            self
        }
        #[doc = "Sets the value of `size`."]
        pub fn set_size(&mut self, size: ::std::option::Option<u64>) -> &mut Self {
            self.size = size;
            self
        }
        #[doc = "Sets the value of `size`."]
        pub fn with_size(mut self, size: ::std::option::Option<u64>) -> Self {
            self.size = size;
            self
        }
        #[doc = "Sets the value of `updated_at`."]
        pub fn set_updated_at(
            &mut self,
            updated_at: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.updated_at = updated_at;
            self
        }
        #[doc = "Sets the value of `updated_at`."]
        pub fn with_updated_at(
            mut self,
            updated_at: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.updated_at = updated_at;
            self
        }
    }
    impl ::std::default::Default for SlotStateOutput {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for SlotStateOutput {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "SlotStateOutput", 4usize)?;
            __record
                .serialize_optional_field("active", ::core::option::Option::as_ref(&self.active))?;
            __record
                .serialize_optional_field("hashes", ::core::option::Option::as_ref(&self.hashes))?;
            __record
                .serialize_optional_field("size", ::core::option::Option::as_ref(&self.size))?;
            __record.serialize_optional_field(
                "updatedAt",
                ::core::option::Option::as_ref(&self.updated_at),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for SlotStateOutput {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = SlotStateOutput;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record SlotStateOutput")
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
                        ::std::option::Option<bool>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 4 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::std::option::Option<
                            indexmap::IndexMap<::std::string::String, ::std::string::String>,
                        >,
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
                        ::std::option::Option<u64>,
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
                        ::std::option::Option<::std::string::String>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(3usize, &"record with 4 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(SlotStateOutput {
                        active: __field0,
                        hashes: __field1,
                        size: __field2,
                        updated_at: __field3,
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
                        &["active", "hashes", "size", "updatedAt"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"active\", \"hashes\", \"size\", \"updatedAt\"]";
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
                                "active" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                "hashes" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                                "size" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                                "updatedAt" => {
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
                                b"active" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                b"hashes" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                b"size" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                                b"updatedAt" => {
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
                    let mut __field0: ::core::option::Option<::std::option::Option<bool>> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<
                        ::std::option::Option<
                            indexmap::IndexMap<::std::string::String, ::std::string::String>,
                        >,
                    > = ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<::std::option::Option<u64>> =
                        ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<
                        ::std::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "active",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<bool>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "hashes",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<
                                            indexmap::IndexMap<
                                                ::std::string::String,
                                                ::std::string::String,
                                            >,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("size"),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<::std::option::Option<u64>>(
                                        &mut __map,
                                    )?,
                                );
                            }
                            __Identifier::__Identifier3 => {
                                if ::core::option::Option::is_some(&__field3) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "updatedAt",
                                        ),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<::std::string::String>,
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
                        ::core::option::Option::None => ::core::option::Option::None,
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
                    ::core::result::Result::Ok(SlotStateOutput {
                        active: __field0,
                        hashes: __field1,
                        size: __field2,
                        updated_at: __field3,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["active", "hashes", "size", "updatedAt"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "SlotStateOutput",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = ""]
    #[derive(Clone, Debug)]
    pub struct BootStateOutput {
        #[doc = ""]
        pub boot_flow: ::std::string::String,
        #[doc = ""]
        pub active_group: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub default_group: ::std::option::Option<::std::string::String>,
        #[doc = ""]
        pub groups: indexmap::IndexMap<::std::string::String, BootGroupStateOutput>,
    }
    impl BootStateOutput {
        #[doc = "Creates a new [`BootStateOutput`]."]
        pub fn new(
            boot_flow: ::std::string::String,
            groups: indexmap::IndexMap<::std::string::String, BootGroupStateOutput>,
        ) -> Self {
            Self {
                boot_flow,
                groups,
                active_group: ::std::default::Default::default(),
                default_group: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `boot_flow`."]
        pub fn set_boot_flow(&mut self, boot_flow: ::std::string::String) -> &mut Self {
            self.boot_flow = boot_flow;
            self
        }
        #[doc = "Sets the value of `boot_flow`."]
        pub fn with_boot_flow(mut self, boot_flow: ::std::string::String) -> Self {
            self.boot_flow = boot_flow;
            self
        }
        #[doc = "Sets the value of `active_group`."]
        pub fn set_active_group(
            &mut self,
            active_group: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.active_group = active_group;
            self
        }
        #[doc = "Sets the value of `active_group`."]
        pub fn with_active_group(
            mut self,
            active_group: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.active_group = active_group;
            self
        }
        #[doc = "Sets the value of `default_group`."]
        pub fn set_default_group(
            &mut self,
            default_group: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.default_group = default_group;
            self
        }
        #[doc = "Sets the value of `default_group`."]
        pub fn with_default_group(
            mut self,
            default_group: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.default_group = default_group;
            self
        }
        #[doc = "Sets the value of `groups`."]
        pub fn set_groups(
            &mut self,
            groups: indexmap::IndexMap<::std::string::String, BootGroupStateOutput>,
        ) -> &mut Self {
            self.groups = groups;
            self
        }
        #[doc = "Sets the value of `groups`."]
        pub fn with_groups(
            mut self,
            groups: indexmap::IndexMap<::std::string::String, BootGroupStateOutput>,
        ) -> Self {
            self.groups = groups;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for BootStateOutput {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "BootStateOutput", 4usize)?;
            __record.serialize_field("bootFlow", &self.boot_flow)?;
            __record.serialize_optional_field(
                "activeGroup",
                ::core::option::Option::as_ref(&self.active_group),
            )?;
            __record.serialize_optional_field(
                "defaultGroup",
                ::core::option::Option::as_ref(&self.default_group),
            )?;
            __record.serialize_field("groups", &self.groups)?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for BootStateOutput {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = BootStateOutput;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record BootStateOutput")
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
                                __serde::de::Error::invalid_length(0usize, &"record with 4 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::std::option::Option<::std::string::String>,
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
                        ::std::option::Option<::std::string::String>,
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
                        indexmap::IndexMap<::std::string::String, BootGroupStateOutput>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(3usize, &"record with 4 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(BootStateOutput {
                        boot_flow: __field0,
                        active_group: __field1,
                        default_group: __field2,
                        groups: __field3,
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
                        &["bootFlow", "activeGroup", "defaultGroup", "groups"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS : & 'static str = "an identifier in [\"bootFlow\", \"activeGroup\", \"defaultGroup\", \"groups\"]" ;
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
                                "bootFlow" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                "activeGroup" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                "defaultGroup" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier2)
                                }
                                "groups" => ::core::result::Result::Ok(__Identifier::__Identifier3),
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
                                b"bootFlow" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                b"activeGroup" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                b"defaultGroup" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier2)
                                }
                                b"groups" => {
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
                    let mut __field0: ::core::option::Option<::std::string::String> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<
                        ::std::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<
                        ::std::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<
                        indexmap::IndexMap<::std::string::String, BootGroupStateOutput>,
                    > = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "bootFlow",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<::std::string::String>(
                                        &mut __map,
                                    )?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "activeGroup",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<::std::string::String>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "defaultGroup",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<::std::string::String>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier3 => {
                                if ::core::option::Option::is_some(&__field3) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "groups",
                                        ),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        indexmap::IndexMap<
                                            ::std::string::String,
                                            BootGroupStateOutput,
                                        >,
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
                                <__A::Error as __serde::de::Error>::missing_field("bootFlow"),
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
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("groups"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(BootStateOutput {
                        boot_flow: __field0,
                        active_group: __field1,
                        default_group: __field2,
                        groups: __field3,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] =
                &["bootFlow", "activeGroup", "defaultGroup", "groups"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "BootStateOutput",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = ""]
    #[derive(Clone, Debug)]
    pub struct BootGroupStateOutput {}
    impl BootGroupStateOutput {
        #[doc = "Creates a new [`BootGroupStateOutput`]."]
        pub fn new() -> Self {
            Self {}
        }
    }
    impl ::std::default::Default for BootGroupStateOutput {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for BootGroupStateOutput {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record = __sidex_serde::ser::RecordSerializer::new(
                __serializer,
                "BootGroupStateOutput",
                0usize,
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for BootGroupStateOutput {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = BootGroupStateOutput;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record BootGroupStateOutput")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> ::core::result::Result<Self::Value, __A::Error>
                where
                    __A: __serde::de::SeqAccess<'de>,
                {
                    ::core::result::Result::Ok(BootGroupStateOutput {})
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
                    const __IDENTIFIERS: &'static [&'static str] = &[];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in []";
                    #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
                    #[doc(hidden)]
                    enum __Identifier {
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
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            _ => {
                                __serde::de::MapAccess::next_value::<__serde::de::IgnoredAny>(
                                    &mut __map,
                                )?;
                            }
                        }
                    }
                    ::core::result::Result::Ok(BootGroupStateOutput {})
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &[];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "BootGroupStateOutput",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
}
pub mod state {
    #![doc = "State management configuration.\n"]
    #[allow(unused)]
    use :: serde as __serde;
    #[allow(unused)]
    use :: sidex_serde as __sidex_serde;
    #[doc = "State management configuration.\n"]
    #[derive(Clone, Debug)]
    pub struct StateConfig {
        #[doc = "Configuration of the root overlay.\n"]
        pub overlay: ::std::option::Option<OverlayConfig>,
        #[doc = "Files and directories to persist.\n"]
        pub persist: ::std::option::Option<::std::vec::Vec<PersistConfig>>,
    }
    impl StateConfig {
        #[doc = "Creates a new [`StateConfig`]."]
        pub fn new() -> Self {
            Self {
                overlay: ::std::default::Default::default(),
                persist: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `overlay`."]
        pub fn set_overlay(&mut self, overlay: ::std::option::Option<OverlayConfig>) -> &mut Self {
            self.overlay = overlay;
            self
        }
        #[doc = "Sets the value of `overlay`."]
        pub fn with_overlay(mut self, overlay: ::std::option::Option<OverlayConfig>) -> Self {
            self.overlay = overlay;
            self
        }
        #[doc = "Sets the value of `persist`."]
        pub fn set_persist(
            &mut self,
            persist: ::std::option::Option<::std::vec::Vec<PersistConfig>>,
        ) -> &mut Self {
            self.persist = persist;
            self
        }
        #[doc = "Sets the value of `persist`."]
        pub fn with_persist(
            mut self,
            persist: ::std::option::Option<::std::vec::Vec<PersistConfig>>,
        ) -> Self {
            self.persist = persist;
            self
        }
    }
    impl ::std::default::Default for StateConfig {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for StateConfig {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "StateConfig", 2usize)?;
            __record.serialize_optional_field(
                "overlay",
                ::core::option::Option::as_ref(&self.overlay),
            )?;
            __record.serialize_optional_field(
                "persist",
                ::core::option::Option::as_ref(&self.persist),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for StateConfig {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = StateConfig;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record StateConfig")
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
                        ::std::option::Option<OverlayConfig>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 2 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::std::option::Option<::std::vec::Vec<PersistConfig>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 2 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(StateConfig {
                        overlay: __field0,
                        persist: __field1,
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
                    const __IDENTIFIERS: &'static [&'static str] = &["overlay", "persist"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"overlay\", \"persist\"]";
                    #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
                    #[doc(hidden)]
                    enum __Identifier {
                        __Identifier0,
                        __Identifier1,
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
                                "overlay" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                "persist" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
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
                                b"overlay" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                b"persist" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
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
                    let mut __field0: ::core::option::Option<::std::option::Option<OverlayConfig>> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<
                        ::std::option::Option<::std::vec::Vec<PersistConfig>>,
                    > = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "overlay",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<OverlayConfig>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "persist",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<::std::vec::Vec<PersistConfig>>,
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
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(StateConfig {
                        overlay: __field0,
                        persist: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["overlay", "persist"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "StateConfig",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "Configuration of the root overlay.\n"]
    #[derive(Clone, Debug)]
    pub enum OverlayConfig {
        #[doc = "Put the overlay on the data partition and persist it across boots.\n"]
        Persist,
        #[doc = "Put the overlay on the data partition and discard it on each boot.\n"]
        Discard,
        #[doc = "Put the overlay in a temporary, in-memory filesystem.\n"]
        InMemory,
        #[doc = "Disable the overlay.\n"]
        Disabled,
    }
    #[automatically_derived]
    impl __serde::Serialize for OverlayConfig {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let __serializer =
                __sidex_serde::ser::VariantSerializer::new(__serializer, "OverlayConfig");
            match self {
                Self::Persist => __serializer.serialize_tag("persist", 0u32),
                Self::Discard => __serializer.serialize_tag("discard", 1u32),
                Self::InMemory => __serializer.serialize_tag("in-memory", 2u32),
                Self::Disabled => __serializer.serialize_tag("disabled", 3u32),
            }
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for OverlayConfig {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] =
                &["persist", "discard", "in-memory", "disabled"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"persist\", \"discard\", \"in-memory\", \"disabled\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            #[doc(hidden)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Identifier2,
                __Identifier3,
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
                        2u64 => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        3u64 => ::core::result::Result::Ok(__Identifier::__Identifier3),
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
                        "persist" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "discard" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "in-memory" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "disabled" => ::core::result::Result::Ok(__Identifier::__Identifier3),
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
                        b"persist" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"discard" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"in-memory" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        b"disabled" => ::core::result::Result::Ok(__Identifier::__Identifier3),
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
            const __VARIANTS: &'static [&'static str] =
                &["persist", "discard", "in-memory", "disabled"];
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = OverlayConfig;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "enum OverlayConfig")
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
                            ::core::result::Result::Ok(OverlayConfig::Persist)
                        }
                        __Identifier::__Identifier1 => {
                            ::core::result::Result::Ok(OverlayConfig::Discard)
                        }
                        __Identifier::__Identifier2 => {
                            ::core::result::Result::Ok(OverlayConfig::InMemory)
                        }
                        __Identifier::__Identifier3 => {
                            ::core::result::Result::Ok(OverlayConfig::Disabled)
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
                            ::core::result::Result::Ok(OverlayConfig::Persist)
                        }
                        (__Identifier::__Identifier1, __variant) => {
                            __serde::de::VariantAccess::unit_variant(__variant)?;
                            ::core::result::Result::Ok(OverlayConfig::Discard)
                        }
                        (__Identifier::__Identifier2, __variant) => {
                            __serde::de::VariantAccess::unit_variant(__variant)?;
                            ::core::result::Result::Ok(OverlayConfig::InMemory)
                        }
                        (__Identifier::__Identifier3, __variant) => {
                            __serde::de::VariantAccess::unit_variant(__variant)?;
                            ::core::result::Result::Ok(OverlayConfig::Disabled)
                        }
                    }
                }
            }
            __serde::Deserializer::deserialize_enum(
                __deserializer,
                "OverlayConfig",
                __VARIANTS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "Configuration to persist a file or directory.\n"]
    #[derive(Clone, Debug)]
    pub enum PersistConfig {
        #[doc = "Configuration to persist a file.\n"]
        File(PersistFileConfig),
        #[doc = "Configuration to persist a directory.\n"]
        Directory(PersistDirectoryConfig),
    }
    #[automatically_derived]
    impl __serde::Serialize for PersistConfig {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let __serializer =
                __sidex_serde::ser::VariantSerializer::new(__serializer, "PersistConfig");
            match self {
                Self::File(__value) => {
                    __serializer.serialize_implicitly_tagged("File", 0u32, __value)
                }
                Self::Directory(__value) => {
                    __serializer.serialize_implicitly_tagged("Directory", 1u32, __value)
                }
            }
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for PersistConfig {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &["File", "Directory"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"File\", \"Directory\"]";
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
                        "File" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "Directory" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
                        b"File" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"Directory" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
            const __VARIANTS: &'static [&'static str] = &["File", "Directory"];
            if __serde::Deserializer::is_human_readable(&__deserializer) {
                let __content =
                    __sidex_serde::de::content::deserialize_into_content(__deserializer)?;
                match __sidex_serde::de::content::deserialize_content_ref::<
                    PersistFileConfig,
                    __D::Error,
                >(&__content)
                {
                    Ok(__value) => return Ok(PersistConfig::File(__value)),
                    Err(_) => {}
                };
                match __sidex_serde::de::content::deserialize_content_ref::<
                    PersistDirectoryConfig,
                    __D::Error,
                >(&__content)
                {
                    Ok(__value) => return Ok(PersistConfig::Directory(__value)),
                    Err(_) => {}
                };
                Err(<__D::Error as __serde::de::Error>::custom(
                    "no matching variant found",
                ))
            } else {
                #[doc(hidden)]
                struct __Visitor {
                    __phantom_vars: ::core::marker::PhantomData<fn(&())>,
                }
                impl<'de> __serde::de::Visitor<'de> for __Visitor {
                    type Value = PersistConfig;
                    fn expecting(
                        &self,
                        __formatter: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(__formatter, "enum PersistConfig")
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
                                    PersistFileConfig,
                                >(__variant)?;
                                ::core::result::Result::Ok(PersistConfig::File(__value))
                            }
                            (__Identifier::__Identifier1, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    PersistDirectoryConfig,
                                >(__variant)?;
                                ::core::result::Result::Ok(PersistConfig::Directory(__value))
                            }
                        }
                    }
                }
                __serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "PersistConfig",
                    __VARIANTS,
                    __Visitor {
                        __phantom_vars: ::core::marker::PhantomData,
                    },
                )
            }
        }
    }
    #[doc = "Configuration to persist a file.\n"]
    #[derive(Clone, Debug)]
    pub struct PersistFileConfig {
        #[doc = "Path of the file to persist.\n"]
        pub file: ::std::string::String,
        #[doc = "Default contents to initialize the file with if it does not exist.\n"]
        pub default: ::std::option::Option<::std::string::String>,
    }
    impl PersistFileConfig {
        #[doc = "Creates a new [`PersistFileConfig`]."]
        pub fn new(file: ::std::string::String) -> Self {
            Self {
                file,
                default: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `file`."]
        pub fn set_file(&mut self, file: ::std::string::String) -> &mut Self {
            self.file = file;
            self
        }
        #[doc = "Sets the value of `file`."]
        pub fn with_file(mut self, file: ::std::string::String) -> Self {
            self.file = file;
            self
        }
        #[doc = "Sets the value of `default`."]
        pub fn set_default(
            &mut self,
            default: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.default = default;
            self
        }
        #[doc = "Sets the value of `default`."]
        pub fn with_default(
            mut self,
            default: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.default = default;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for PersistFileConfig {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record = __sidex_serde::ser::RecordSerializer::new(
                __serializer,
                "PersistFileConfig",
                2usize,
            )?;
            __record.serialize_field("file", &self.file)?;
            __record.serialize_optional_field(
                "default",
                ::core::option::Option::as_ref(&self.default),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for PersistFileConfig {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = PersistFileConfig;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record PersistFileConfig")
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
                                __serde::de::Error::invalid_length(0usize, &"record with 2 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::std::option::Option<::std::string::String>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 2 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(PersistFileConfig {
                        file: __field0,
                        default: __field1,
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
                    const __IDENTIFIERS: &'static [&'static str] = &["file", "default"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"file\", \"default\"]";
                    #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
                    #[doc(hidden)]
                    enum __Identifier {
                        __Identifier0,
                        __Identifier1,
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
                                "file" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                "default" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
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
                                b"file" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                b"default" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
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
                    let mut __field0: ::core::option::Option<::std::string::String> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<
                        ::std::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("file"),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<::std::string::String>(
                                        &mut __map,
                                    )?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "default",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<::std::string::String>,
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
                                <__A::Error as __serde::de::Error>::missing_field("file"),
                            );
                        }
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(PersistFileConfig {
                        file: __field0,
                        default: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["file", "default"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "PersistFileConfig",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "Configuration to persist a directory.\n"]
    #[derive(Clone, Debug)]
    pub struct PersistDirectoryConfig {
        #[doc = "Path of the directory to persist.\n"]
        pub directory: ::std::string::String,
    }
    impl PersistDirectoryConfig {
        #[doc = "Creates a new [`PersistDirectoryConfig`]."]
        pub fn new(directory: ::std::string::String) -> Self {
            Self { directory }
        }
        #[doc = "Sets the value of `directory`."]
        pub fn set_directory(&mut self, directory: ::std::string::String) -> &mut Self {
            self.directory = directory;
            self
        }
        #[doc = "Sets the value of `directory`."]
        pub fn with_directory(mut self, directory: ::std::string::String) -> Self {
            self.directory = directory;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for PersistDirectoryConfig {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record = __sidex_serde::ser::RecordSerializer::new(
                __serializer,
                "PersistDirectoryConfig",
                1usize,
            )?;
            __record.serialize_field("directory", &self.directory)?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for PersistDirectoryConfig {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = PersistDirectoryConfig;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record PersistDirectoryConfig")
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
                    ::core::result::Result::Ok(PersistDirectoryConfig {
                        directory: __field0,
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
                    const __IDENTIFIERS: &'static [&'static str] = &["directory"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"directory\"]";
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
                                "directory" => {
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
                                b"directory" => {
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
                    let mut __field0: ::core::option::Option<::std::string::String> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "directory",
                                        ),
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
                                <__A::Error as __serde::de::Error>::missing_field("directory"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(PersistDirectoryConfig {
                        directory: __field0,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["directory"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "PersistDirectoryConfig",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
}
pub mod system {
    #![doc = ""]
    #[allow(unused)]
    use :: serde as __serde;
    #[allow(unused)]
    use :: sidex_serde as __sidex_serde;
    #[doc = "System configuration.\n"]
    #[derive(Clone, Debug)]
    pub struct SystemConfig {
        #[doc = "Config partition configuration.\n"]
        pub config_partition: ::std::option::Option<PartitionConfig>,
        #[doc = "Data partition configuration.\n"]
        pub data_partition: ::std::option::Option<PartitionConfig>,
        #[doc = "System slots.\n"]
        pub slots: ::std::option::Option<indexmap::IndexMap<::std::string::String, SlotConfig>>,
        #[doc = "System boot groups.\n"]
        pub boot_groups:
            ::std::option::Option<indexmap::IndexMap<::std::string::String, BootGroupConfig>>,
        #[doc = "Boot flow configuration.\n"]
        pub boot_flow: ::std::option::Option<BootFlowConfig>,
    }
    impl SystemConfig {
        #[doc = "Creates a new [`SystemConfig`]."]
        pub fn new() -> Self {
            Self {
                config_partition: ::std::default::Default::default(),
                data_partition: ::std::default::Default::default(),
                slots: ::std::default::Default::default(),
                boot_groups: ::std::default::Default::default(),
                boot_flow: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `config_partition`."]
        pub fn set_config_partition(
            &mut self,
            config_partition: ::std::option::Option<PartitionConfig>,
        ) -> &mut Self {
            self.config_partition = config_partition;
            self
        }
        #[doc = "Sets the value of `config_partition`."]
        pub fn with_config_partition(
            mut self,
            config_partition: ::std::option::Option<PartitionConfig>,
        ) -> Self {
            self.config_partition = config_partition;
            self
        }
        #[doc = "Sets the value of `data_partition`."]
        pub fn set_data_partition(
            &mut self,
            data_partition: ::std::option::Option<PartitionConfig>,
        ) -> &mut Self {
            self.data_partition = data_partition;
            self
        }
        #[doc = "Sets the value of `data_partition`."]
        pub fn with_data_partition(
            mut self,
            data_partition: ::std::option::Option<PartitionConfig>,
        ) -> Self {
            self.data_partition = data_partition;
            self
        }
        #[doc = "Sets the value of `slots`."]
        pub fn set_slots(
            &mut self,
            slots: ::std::option::Option<indexmap::IndexMap<::std::string::String, SlotConfig>>,
        ) -> &mut Self {
            self.slots = slots;
            self
        }
        #[doc = "Sets the value of `slots`."]
        pub fn with_slots(
            mut self,
            slots: ::std::option::Option<indexmap::IndexMap<::std::string::String, SlotConfig>>,
        ) -> Self {
            self.slots = slots;
            self
        }
        #[doc = "Sets the value of `boot_groups`."]
        pub fn set_boot_groups(
            &mut self,
            boot_groups: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, BootGroupConfig>,
            >,
        ) -> &mut Self {
            self.boot_groups = boot_groups;
            self
        }
        #[doc = "Sets the value of `boot_groups`."]
        pub fn with_boot_groups(
            mut self,
            boot_groups: ::std::option::Option<
                indexmap::IndexMap<::std::string::String, BootGroupConfig>,
            >,
        ) -> Self {
            self.boot_groups = boot_groups;
            self
        }
        #[doc = "Sets the value of `boot_flow`."]
        pub fn set_boot_flow(
            &mut self,
            boot_flow: ::std::option::Option<BootFlowConfig>,
        ) -> &mut Self {
            self.boot_flow = boot_flow;
            self
        }
        #[doc = "Sets the value of `boot_flow`."]
        pub fn with_boot_flow(mut self, boot_flow: ::std::option::Option<BootFlowConfig>) -> Self {
            self.boot_flow = boot_flow;
            self
        }
    }
    impl ::std::default::Default for SystemConfig {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for SystemConfig {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "SystemConfig", 5usize)?;
            __record.serialize_optional_field(
                "config-partition",
                ::core::option::Option::as_ref(&self.config_partition),
            )?;
            __record.serialize_optional_field(
                "data-partition",
                ::core::option::Option::as_ref(&self.data_partition),
            )?;
            __record
                .serialize_optional_field("slots", ::core::option::Option::as_ref(&self.slots))?;
            __record.serialize_optional_field(
                "boot-groups",
                ::core::option::Option::as_ref(&self.boot_groups),
            )?;
            __record.serialize_optional_field(
                "boot-flow",
                ::core::option::Option::as_ref(&self.boot_flow),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for SystemConfig {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = SystemConfig;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record SystemConfig")
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
                        ::std::option::Option<PartitionConfig>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 5 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::std::option::Option<PartitionConfig>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 5 fields"),
                            );
                        }
                    };
                    let __field2 = match __serde::de::SeqAccess::next_element::<
                        ::std::option::Option<
                            indexmap::IndexMap<::std::string::String, SlotConfig>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(2usize, &"record with 5 fields"),
                            );
                        }
                    };
                    let __field3 = match __serde::de::SeqAccess::next_element::<
                        ::std::option::Option<
                            indexmap::IndexMap<::std::string::String, BootGroupConfig>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(3usize, &"record with 5 fields"),
                            );
                        }
                    };
                    let __field4 = match __serde::de::SeqAccess::next_element::<
                        ::std::option::Option<BootFlowConfig>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(4usize, &"record with 5 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(SystemConfig {
                        config_partition: __field0,
                        data_partition: __field1,
                        slots: __field2,
                        boot_groups: __field3,
                        boot_flow: __field4,
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
                    const __IDENTIFIERS: &'static [&'static str] = &[
                        "config-partition",
                        "data-partition",
                        "slots",
                        "boot-groups",
                        "boot-flow",
                    ];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS : & 'static str = "an identifier in [\"config-partition\", \"data-partition\", \"slots\", \"boot-groups\", \"boot-flow\"]" ;
                    #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
                    #[doc(hidden)]
                    enum __Identifier {
                        __Identifier0,
                        __Identifier1,
                        __Identifier2,
                        __Identifier3,
                        __Identifier4,
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
                                4u64 => ::core::result::Result::Ok(__Identifier::__Identifier4),
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
                                "config-partition" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                "data-partition" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                "slots" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                                "boot-groups" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier3)
                                }
                                "boot-flow" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier4)
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
                                b"config-partition" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                b"data-partition" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                b"slots" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                                b"boot-groups" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier3)
                                }
                                b"boot-flow" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier4)
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
                        ::std::option::Option<PartitionConfig>,
                    > = ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<
                        ::std::option::Option<PartitionConfig>,
                    > = ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<
                        ::std::option::Option<
                            indexmap::IndexMap<::std::string::String, SlotConfig>,
                        >,
                    > = ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<
                        ::std::option::Option<
                            indexmap::IndexMap<::std::string::String, BootGroupConfig>,
                        >,
                    > = ::core::option::Option::None;
                    let mut __field4: ::core::option::Option<
                        ::std::option::Option<BootFlowConfig>,
                    > = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "config-partition",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<PartitionConfig>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "data-partition",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<PartitionConfig>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "slots",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<
                                            indexmap::IndexMap<::std::string::String, SlotConfig>,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier3 => {
                                if ::core::option::Option::is_some(&__field3) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "boot-groups",
                                        ),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<
                                            indexmap::IndexMap<
                                                ::std::string::String,
                                                BootGroupConfig,
                                            >,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier4 => {
                                if ::core::option::Option::is_some(&__field4) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "boot-flow",
                                        ),
                                    );
                                }
                                __field4 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<BootFlowConfig>,
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
                        ::core::option::Option::None => ::core::option::Option::None,
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
                    let __field4 = match __field4 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(SystemConfig {
                        config_partition: __field0,
                        data_partition: __field1,
                        slots: __field2,
                        boot_groups: __field3,
                        boot_flow: __field4,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &[
                "config-partition",
                "data-partition",
                "slots",
                "boot-groups",
                "boot-flow",
            ];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "SystemConfig",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "Partition configuration.\n"]
    #[derive(Clone, Debug)]
    pub struct PartitionConfig {
        #[doc = ""]
        pub disabled: ::std::option::Option<bool>,
        #[doc = "Path to the partition block device.\n"]
        pub device: ::std::option::Option<::std::string::String>,
        #[doc = "Partition number of the root device.\n"]
        pub partition: ::std::option::Option<u32>,
        #[doc = "Path where the partition is or should be mounted.\n"]
        pub path: ::std::option::Option<::std::string::String>,
        #[doc = "Indicates whether the partition is write-protected.\n"]
        pub protected: ::std::option::Option<bool>,
    }
    impl PartitionConfig {
        #[doc = "Creates a new [`PartitionConfig`]."]
        pub fn new() -> Self {
            Self {
                disabled: ::std::default::Default::default(),
                device: ::std::default::Default::default(),
                partition: ::std::default::Default::default(),
                path: ::std::default::Default::default(),
                protected: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `disabled`."]
        pub fn set_disabled(&mut self, disabled: ::std::option::Option<bool>) -> &mut Self {
            self.disabled = disabled;
            self
        }
        #[doc = "Sets the value of `disabled`."]
        pub fn with_disabled(mut self, disabled: ::std::option::Option<bool>) -> Self {
            self.disabled = disabled;
            self
        }
        #[doc = "Sets the value of `device`."]
        pub fn set_device(
            &mut self,
            device: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.device = device;
            self
        }
        #[doc = "Sets the value of `device`."]
        pub fn with_device(mut self, device: ::std::option::Option<::std::string::String>) -> Self {
            self.device = device;
            self
        }
        #[doc = "Sets the value of `partition`."]
        pub fn set_partition(&mut self, partition: ::std::option::Option<u32>) -> &mut Self {
            self.partition = partition;
            self
        }
        #[doc = "Sets the value of `partition`."]
        pub fn with_partition(mut self, partition: ::std::option::Option<u32>) -> Self {
            self.partition = partition;
            self
        }
        #[doc = "Sets the value of `path`."]
        pub fn set_path(
            &mut self,
            path: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.path = path;
            self
        }
        #[doc = "Sets the value of `path`."]
        pub fn with_path(mut self, path: ::std::option::Option<::std::string::String>) -> Self {
            self.path = path;
            self
        }
        #[doc = "Sets the value of `protected`."]
        pub fn set_protected(&mut self, protected: ::std::option::Option<bool>) -> &mut Self {
            self.protected = protected;
            self
        }
        #[doc = "Sets the value of `protected`."]
        pub fn with_protected(mut self, protected: ::std::option::Option<bool>) -> Self {
            self.protected = protected;
            self
        }
    }
    impl ::std::default::Default for PartitionConfig {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for PartitionConfig {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "PartitionConfig", 5usize)?;
            __record.serialize_optional_field(
                "disabled",
                ::core::option::Option::as_ref(&self.disabled),
            )?;
            __record
                .serialize_optional_field("device", ::core::option::Option::as_ref(&self.device))?;
            __record.serialize_optional_field(
                "partition",
                ::core::option::Option::as_ref(&self.partition),
            )?;
            __record
                .serialize_optional_field("path", ::core::option::Option::as_ref(&self.path))?;
            __record.serialize_optional_field(
                "protected",
                ::core::option::Option::as_ref(&self.protected),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for PartitionConfig {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = PartitionConfig;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record PartitionConfig")
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
                        ::std::option::Option<bool>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 5 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::std::option::Option<::std::string::String>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 5 fields"),
                            );
                        }
                    };
                    let __field2 = match __serde::de::SeqAccess::next_element::<
                        ::std::option::Option<u32>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(2usize, &"record with 5 fields"),
                            );
                        }
                    };
                    let __field3 = match __serde::de::SeqAccess::next_element::<
                        ::std::option::Option<::std::string::String>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(3usize, &"record with 5 fields"),
                            );
                        }
                    };
                    let __field4 = match __serde::de::SeqAccess::next_element::<
                        ::std::option::Option<bool>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(4usize, &"record with 5 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(PartitionConfig {
                        disabled: __field0,
                        device: __field1,
                        partition: __field2,
                        path: __field3,
                        protected: __field4,
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
                        &["disabled", "device", "partition", "path", "protected"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS : & 'static str = "an identifier in [\"disabled\", \"device\", \"partition\", \"path\", \"protected\"]" ;
                    #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
                    #[doc(hidden)]
                    enum __Identifier {
                        __Identifier0,
                        __Identifier1,
                        __Identifier2,
                        __Identifier3,
                        __Identifier4,
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
                                4u64 => ::core::result::Result::Ok(__Identifier::__Identifier4),
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
                                "disabled" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                "device" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                                "partition" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier2)
                                }
                                "path" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                                "protected" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier4)
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
                                b"disabled" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                b"device" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                b"partition" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier2)
                                }
                                b"path" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                                b"protected" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier4)
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
                    let mut __field0: ::core::option::Option<::std::option::Option<bool>> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<
                        ::std::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<::std::option::Option<u32>> =
                        ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<
                        ::std::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field4: ::core::option::Option<::std::option::Option<bool>> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "disabled",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<bool>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "device",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<::std::string::String>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "partition",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<::std::option::Option<u32>>(
                                        &mut __map,
                                    )?,
                                );
                            }
                            __Identifier::__Identifier3 => {
                                if ::core::option::Option::is_some(&__field3) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("path"),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<::std::string::String>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier4 => {
                                if ::core::option::Option::is_some(&__field4) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "protected",
                                        ),
                                    );
                                }
                                __field4 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<bool>,
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
                        ::core::option::Option::None => ::core::option::Option::None,
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
                    let __field4 = match __field4 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(PartitionConfig {
                        disabled: __field0,
                        device: __field1,
                        partition: __field2,
                        path: __field3,
                        protected: __field4,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] =
                &["disabled", "device", "partition", "path", "protected"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "PartitionConfig",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "System slot configuration.\n"]
    #[derive(Clone, Debug)]
    pub enum SlotConfig {
        #[doc = "Block device slot.\n"]
        Block(BlockSlotConfig),
        #[doc = "File slot.\n"]
        File(FileSlotConfig),
        #[doc = "Custom slot.\n"]
        Custom(CustomSlotConfig),
    }
    #[automatically_derived]
    impl __serde::Serialize for SlotConfig {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let __serializer =
                __sidex_serde::ser::VariantSerializer::new(__serializer, "SlotConfig");
            match self {
                Self::Block(__value) => {
                    __serializer.serialize_internally_tagged("type", "block", 0u32, __value)
                }
                Self::File(__value) => {
                    __serializer.serialize_internally_tagged("type", "file", 1u32, __value)
                }
                Self::Custom(__value) => {
                    __serializer.serialize_internally_tagged("type", "custom", 2u32, __value)
                }
            }
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for SlotConfig {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &["block", "file", "custom"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"block\", \"file\", \"custom\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            #[doc(hidden)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Identifier2,
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
                        2u64 => ::core::result::Result::Ok(__Identifier::__Identifier2),
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
                        "block" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "file" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "custom" => ::core::result::Result::Ok(__Identifier::__Identifier2),
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
                        b"block" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"file" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"custom" => ::core::result::Result::Ok(__Identifier::__Identifier2),
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
            const __VARIANTS: &'static [&'static str] = &["block", "file", "custom"];
            if __serde::Deserializer::is_human_readable(&__deserializer) {
                let __tagged = __sidex_serde::de::tagged::deserialize_tagged_variant::<
                    __Identifier,
                    __D,
                >(__deserializer, "type")?;
                match __tagged.tag {
                    __Identifier::__Identifier0 => ::core::result::Result::Ok(SlotConfig::Block(
                        __tagged.deserialize_internally_tagged::<BlockSlotConfig, __D::Error>()?,
                    )),
                    __Identifier::__Identifier1 => ::core::result::Result::Ok(SlotConfig::File(
                        __tagged.deserialize_internally_tagged::<FileSlotConfig, __D::Error>()?,
                    )),
                    __Identifier::__Identifier2 => ::core::result::Result::Ok(SlotConfig::Custom(
                        __tagged.deserialize_internally_tagged::<CustomSlotConfig, __D::Error>()?,
                    )),
                }
            } else {
                #[doc(hidden)]
                struct __Visitor {
                    __phantom_vars: ::core::marker::PhantomData<fn(&())>,
                }
                impl<'de> __serde::de::Visitor<'de> for __Visitor {
                    type Value = SlotConfig;
                    fn expecting(
                        &self,
                        __formatter: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(__formatter, "enum SlotConfig")
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
                                    BlockSlotConfig,
                                >(__variant)?;
                                ::core::result::Result::Ok(SlotConfig::Block(__value))
                            }
                            (__Identifier::__Identifier1, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    FileSlotConfig,
                                >(__variant)?;
                                ::core::result::Result::Ok(SlotConfig::File(__value))
                            }
                            (__Identifier::__Identifier2, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    CustomSlotConfig,
                                >(__variant)?;
                                ::core::result::Result::Ok(SlotConfig::Custom(__value))
                            }
                        }
                    }
                }
                __serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "SlotConfig",
                    __VARIANTS,
                    __Visitor {
                        __phantom_vars: ::core::marker::PhantomData,
                    },
                )
            }
        }
    }
    #[doc = "Block device slot configuration.\n"]
    #[derive(Clone, Debug)]
    pub struct BlockSlotConfig {
        #[doc = "Path to the block device.\n"]
        pub device: ::std::option::Option<::std::string::String>,
        #[doc = "Partition number of the block device.\n"]
        pub partition: ::std::option::Option<u32>,
        #[doc = ""]
        pub immutable: ::std::option::Option<bool>,
    }
    impl BlockSlotConfig {
        #[doc = "Creates a new [`BlockSlotConfig`]."]
        pub fn new() -> Self {
            Self {
                device: ::std::default::Default::default(),
                partition: ::std::default::Default::default(),
                immutable: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `device`."]
        pub fn set_device(
            &mut self,
            device: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.device = device;
            self
        }
        #[doc = "Sets the value of `device`."]
        pub fn with_device(mut self, device: ::std::option::Option<::std::string::String>) -> Self {
            self.device = device;
            self
        }
        #[doc = "Sets the value of `partition`."]
        pub fn set_partition(&mut self, partition: ::std::option::Option<u32>) -> &mut Self {
            self.partition = partition;
            self
        }
        #[doc = "Sets the value of `partition`."]
        pub fn with_partition(mut self, partition: ::std::option::Option<u32>) -> Self {
            self.partition = partition;
            self
        }
        #[doc = "Sets the value of `immutable`."]
        pub fn set_immutable(&mut self, immutable: ::std::option::Option<bool>) -> &mut Self {
            self.immutable = immutable;
            self
        }
        #[doc = "Sets the value of `immutable`."]
        pub fn with_immutable(mut self, immutable: ::std::option::Option<bool>) -> Self {
            self.immutable = immutable;
            self
        }
    }
    impl ::std::default::Default for BlockSlotConfig {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for BlockSlotConfig {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "BlockSlotConfig", 3usize)?;
            __record
                .serialize_optional_field("device", ::core::option::Option::as_ref(&self.device))?;
            __record.serialize_optional_field(
                "partition",
                ::core::option::Option::as_ref(&self.partition),
            )?;
            __record.serialize_optional_field(
                "immutable",
                ::core::option::Option::as_ref(&self.immutable),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for BlockSlotConfig {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = BlockSlotConfig;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record BlockSlotConfig")
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
                        ::std::option::Option<::std::string::String>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 3 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::std::option::Option<u32>,
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
                        ::std::option::Option<bool>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(2usize, &"record with 3 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(BlockSlotConfig {
                        device: __field0,
                        partition: __field1,
                        immutable: __field2,
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
                        &["device", "partition", "immutable"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"device\", \"partition\", \"immutable\"]";
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
                                "device" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                "partition" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                "immutable" => {
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
                                b"device" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                b"partition" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                b"immutable" => {
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
                    let mut __field0: ::core::option::Option<
                        ::std::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::std::option::Option<u32>> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<::std::option::Option<bool>> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "device",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<::std::string::String>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "partition",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<::std::option::Option<u32>>(
                                        &mut __map,
                                    )?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "immutable",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<bool>,
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
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field2 = match __field2 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(BlockSlotConfig {
                        device: __field0,
                        partition: __field1,
                        immutable: __field2,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["device", "partition", "immutable"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "BlockSlotConfig",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "File slot configuration.\n"]
    #[derive(Clone, Debug)]
    pub struct FileSlotConfig {
        #[doc = ""]
        pub path: ::std::string::String,
        #[doc = ""]
        pub immutable: ::std::option::Option<bool>,
    }
    impl FileSlotConfig {
        #[doc = "Creates a new [`FileSlotConfig`]."]
        pub fn new(path: ::std::string::String) -> Self {
            Self {
                path,
                immutable: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `path`."]
        pub fn set_path(&mut self, path: ::std::string::String) -> &mut Self {
            self.path = path;
            self
        }
        #[doc = "Sets the value of `path`."]
        pub fn with_path(mut self, path: ::std::string::String) -> Self {
            self.path = path;
            self
        }
        #[doc = "Sets the value of `immutable`."]
        pub fn set_immutable(&mut self, immutable: ::std::option::Option<bool>) -> &mut Self {
            self.immutable = immutable;
            self
        }
        #[doc = "Sets the value of `immutable`."]
        pub fn with_immutable(mut self, immutable: ::std::option::Option<bool>) -> Self {
            self.immutable = immutable;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for FileSlotConfig {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "FileSlotConfig", 2usize)?;
            __record.serialize_field("path", &self.path)?;
            __record.serialize_optional_field(
                "immutable",
                ::core::option::Option::as_ref(&self.immutable),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for FileSlotConfig {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = FileSlotConfig;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record FileSlotConfig")
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
                                __serde::de::Error::invalid_length(0usize, &"record with 2 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        ::std::option::Option<bool>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 2 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(FileSlotConfig {
                        path: __field0,
                        immutable: __field1,
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
                    const __IDENTIFIERS: &'static [&'static str] = &["path", "immutable"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"path\", \"immutable\"]";
                    #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
                    #[doc(hidden)]
                    enum __Identifier {
                        __Identifier0,
                        __Identifier1,
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
                                "path" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                "immutable" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
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
                                b"path" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                b"immutable" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
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
                    let mut __field0: ::core::option::Option<::std::string::String> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::std::option::Option<bool>> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("path"),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<::std::string::String>(
                                        &mut __map,
                                    )?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "immutable",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<bool>,
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
                                <__A::Error as __serde::de::Error>::missing_field("path"),
                            );
                        }
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(FileSlotConfig {
                        path: __field0,
                        immutable: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["path", "immutable"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "FileSlotConfig",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "Custom slot configuration.\n"]
    #[derive(Clone, Debug)]
    pub struct CustomSlotConfig {
        #[doc = ""]
        pub handler: ::std::vec::Vec<::std::string::String>,
    }
    impl CustomSlotConfig {
        #[doc = "Creates a new [`CustomSlotConfig`]."]
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
    impl __serde::Serialize for CustomSlotConfig {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record = __sidex_serde::ser::RecordSerializer::new(
                __serializer,
                "CustomSlotConfig",
                1usize,
            )?;
            __record.serialize_field("handler", &self.handler)?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for CustomSlotConfig {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = CustomSlotConfig;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record CustomSlotConfig")
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
                    ::core::result::Result::Ok(CustomSlotConfig { handler: __field0 })
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
                    ::core::result::Result::Ok(CustomSlotConfig { handler: __field0 })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["handler"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "CustomSlotConfig",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "Boot group configuration.\n"]
    #[derive(Clone, Debug)]
    pub struct BootGroupConfig {
        #[doc = "Slot aliases of the boot group.\n"]
        pub slots: indexmap::IndexMap<::std::string::String, ::std::string::String>,
    }
    impl BootGroupConfig {
        #[doc = "Creates a new [`BootGroupConfig`]."]
        pub fn new(
            slots: indexmap::IndexMap<::std::string::String, ::std::string::String>,
        ) -> Self {
            Self { slots }
        }
        #[doc = "Sets the value of `slots`."]
        pub fn set_slots(
            &mut self,
            slots: indexmap::IndexMap<::std::string::String, ::std::string::String>,
        ) -> &mut Self {
            self.slots = slots;
            self
        }
        #[doc = "Sets the value of `slots`."]
        pub fn with_slots(
            mut self,
            slots: indexmap::IndexMap<::std::string::String, ::std::string::String>,
        ) -> Self {
            self.slots = slots;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for BootGroupConfig {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "BootGroupConfig", 1usize)?;
            __record.serialize_field("slots", &self.slots)?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for BootGroupConfig {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = BootGroupConfig;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record BootGroupConfig")
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
                        indexmap::IndexMap<::std::string::String, ::std::string::String>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 1 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(BootGroupConfig { slots: __field0 })
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
                    const __IDENTIFIERS: &'static [&'static str] = &["slots"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"slots\"]";
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
                                "slots" => ::core::result::Result::Ok(__Identifier::__Identifier0),
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
                                b"slots" => ::core::result::Result::Ok(__Identifier::__Identifier0),
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
                        indexmap::IndexMap<::std::string::String, ::std::string::String>,
                    > = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "slots",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        indexmap::IndexMap<
                                            ::std::string::String,
                                            ::std::string::String,
                                        >,
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
                                <__A::Error as __serde::de::Error>::missing_field("slots"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(BootGroupConfig { slots: __field0 })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["slots"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "BootGroupConfig",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "Boot flow configuration\n"]
    #[derive(Clone, Debug)]
    pub enum BootFlowConfig {
        #[doc = "Tryboot boot flow.\n"]
        Tryboot,
        #[doc = "U-Boot boot flow.\n"]
        UBoot,
        #[doc = "Grub (EFI) boot flow.\n"]
        GrubEfi,
        #[doc = "Mender Grub.\n"]
        MenderGrub,
        #[doc = "Mender Grub.\n"]
        MenderUboot,
        #[doc = "Custom boot flow.\n"]
        Custom(CustomBootFlowConfig),
    }
    #[automatically_derived]
    impl __serde::Serialize for BootFlowConfig {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let __serializer =
                __sidex_serde::ser::VariantSerializer::new(__serializer, "BootFlowConfig");
            match self {
                Self::Tryboot => __serializer.serialize_internal_tag("type", "tryboot", 0u32),
                Self::UBoot => __serializer.serialize_internal_tag("type", "u-boot", 1u32),
                Self::GrubEfi => __serializer.serialize_internal_tag("type", "grub-efi", 2u32),
                Self::MenderGrub => {
                    __serializer.serialize_internal_tag("type", "mender-grub", 3u32)
                }
                Self::MenderUboot => {
                    __serializer.serialize_internal_tag("type", "mender-uboot", 4u32)
                }
                Self::Custom(__value) => {
                    __serializer.serialize_internally_tagged("type", "custom", 5u32, __value)
                }
            }
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for BootFlowConfig {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &[
                "tryboot",
                "u-boot",
                "grub-efi",
                "mender-grub",
                "mender-uboot",
                "custom",
            ];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS : & 'static str = "an identifier in [\"tryboot\", \"u-boot\", \"grub-efi\", \"mender-grub\", \"mender-uboot\", \"custom\"]" ;
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            #[doc(hidden)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Identifier2,
                __Identifier3,
                __Identifier4,
                __Identifier5,
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
                        2u64 => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        3u64 => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        4u64 => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        5u64 => ::core::result::Result::Ok(__Identifier::__Identifier5),
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
                        "tryboot" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "u-boot" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "grub-efi" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "mender-grub" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        "mender-uboot" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        "custom" => ::core::result::Result::Ok(__Identifier::__Identifier5),
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
                        b"tryboot" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"u-boot" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"grub-efi" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        b"mender-grub" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        b"mender-uboot" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                        b"custom" => ::core::result::Result::Ok(__Identifier::__Identifier5),
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
            const __VARIANTS: &'static [&'static str] = &[
                "tryboot",
                "u-boot",
                "grub-efi",
                "mender-grub",
                "mender-uboot",
                "custom",
            ];
            if __serde::Deserializer::is_human_readable(&__deserializer) {
                let __tagged = __sidex_serde::de::tagged::deserialize_tagged_variant::<
                    __Identifier,
                    __D,
                >(__deserializer, "type")?;
                match __tagged.tag {
                    __Identifier::__Identifier0 => {
                        ::core::result::Result::Ok(BootFlowConfig::Tryboot)
                    }
                    __Identifier::__Identifier1 => {
                        ::core::result::Result::Ok(BootFlowConfig::UBoot)
                    }
                    __Identifier::__Identifier2 => {
                        ::core::result::Result::Ok(BootFlowConfig::GrubEfi)
                    }
                    __Identifier::__Identifier3 => {
                        ::core::result::Result::Ok(BootFlowConfig::MenderGrub)
                    }
                    __Identifier::__Identifier4 => {
                        ::core::result::Result::Ok(BootFlowConfig::MenderUboot)
                    }
                    __Identifier::__Identifier5 => {
                        ::core::result::Result::Ok(BootFlowConfig::Custom(
                            __tagged
                                .deserialize_internally_tagged::<CustomBootFlowConfig, __D::Error>(
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
                    type Value = BootFlowConfig;
                    fn expecting(
                        &self,
                        __formatter: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(__formatter, "enum BootFlowConfig")
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
                            __Identifier::__Identifier0 => {
                                ::core::result::Result::Ok(BootFlowConfig::Tryboot)
                            }
                            __Identifier::__Identifier1 => {
                                ::core::result::Result::Ok(BootFlowConfig::UBoot)
                            }
                            __Identifier::__Identifier2 => {
                                ::core::result::Result::Ok(BootFlowConfig::GrubEfi)
                            }
                            __Identifier::__Identifier3 => {
                                ::core::result::Result::Ok(BootFlowConfig::MenderGrub)
                            }
                            __Identifier::__Identifier4 => {
                                ::core::result::Result::Ok(BootFlowConfig::MenderUboot)
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
                                ::core::result::Result::Ok(BootFlowConfig::Tryboot)
                            }
                            (__Identifier::__Identifier1, __variant) => {
                                __serde::de::VariantAccess::unit_variant(__variant)?;
                                ::core::result::Result::Ok(BootFlowConfig::UBoot)
                            }
                            (__Identifier::__Identifier2, __variant) => {
                                __serde::de::VariantAccess::unit_variant(__variant)?;
                                ::core::result::Result::Ok(BootFlowConfig::GrubEfi)
                            }
                            (__Identifier::__Identifier3, __variant) => {
                                __serde::de::VariantAccess::unit_variant(__variant)?;
                                ::core::result::Result::Ok(BootFlowConfig::MenderGrub)
                            }
                            (__Identifier::__Identifier4, __variant) => {
                                __serde::de::VariantAccess::unit_variant(__variant)?;
                                ::core::result::Result::Ok(BootFlowConfig::MenderUboot)
                            }
                            (__Identifier::__Identifier5, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    CustomBootFlowConfig,
                                >(__variant)?;
                                ::core::result::Result::Ok(BootFlowConfig::Custom(__value))
                            }
                        }
                    }
                }
                __serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "BootFlowConfig",
                    __VARIANTS,
                    __Visitor {
                        __phantom_vars: ::core::marker::PhantomData,
                    },
                )
            }
        }
    }
    #[doc = "Custom boot flow configuration.\n"]
    #[derive(Clone, Debug)]
    pub struct CustomBootFlowConfig {
        #[doc = "Path to the script implementing the boot flow.\n"]
        pub controller: ::std::string::String,
    }
    impl CustomBootFlowConfig {
        #[doc = "Creates a new [`CustomBootFlowConfig`]."]
        pub fn new(controller: ::std::string::String) -> Self {
            Self { controller }
        }
        #[doc = "Sets the value of `controller`."]
        pub fn set_controller(&mut self, controller: ::std::string::String) -> &mut Self {
            self.controller = controller;
            self
        }
        #[doc = "Sets the value of `controller`."]
        pub fn with_controller(mut self, controller: ::std::string::String) -> Self {
            self.controller = controller;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for CustomBootFlowConfig {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record = __sidex_serde::ser::RecordSerializer::new(
                __serializer,
                "CustomBootFlowConfig",
                1usize,
            )?;
            __record.serialize_field("controller", &self.controller)?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for CustomBootFlowConfig {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = CustomBootFlowConfig;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record CustomBootFlowConfig")
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
                    ::core::result::Result::Ok(CustomBootFlowConfig {
                        controller: __field0,
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
                    const __IDENTIFIERS: &'static [&'static str] = &["controller"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"controller\"]";
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
                                "controller" => {
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
                                b"controller" => {
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
                    let mut __field0: ::core::option::Option<::std::string::String> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "controller",
                                        ),
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
                                <__A::Error as __serde::de::Error>::missing_field("controller"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(CustomBootFlowConfig {
                        controller: __field0,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["controller"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "CustomBootFlowConfig",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
}
