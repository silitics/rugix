/* GENERATED WITH SIDEX. DO NOT MODIFY! */

pub mod foreign {
    #![doc = "Foreign types.\n"]
    #[allow(unused)]
    use :: serde as __serde;
    #[allow(unused)]
    use :: sidex_serde as __sidex_serde;
    #[doc = "Number of bytes.\n"]
    pub type NumBytes = ::byte_calc::NumBytes;
}
pub mod images {
    #![doc = "Image configuration.\n"]
    #[allow(unused)]
    use :: serde as __serde;
    #[allow(unused)]
    use :: sidex_serde as __sidex_serde;
    #[doc = "Timestamp.\n"]
    pub type Timestamp = ::jiff::Timestamp;
    #[doc = "Image configuration.\n"]
    #[derive(Clone, Debug)]
    pub struct ImageConfig {
        #[doc = "Layer the image is based on.\n"]
        pub layer: ::std::string::String,
        #[doc = "Architecture of the image.\n"]
        pub architecture: super::systems::Architecture,
        #[doc = "Rugix Bakery target.\n"]
        pub target: ::std::option::Option<super::systems::Target>,
        #[doc = "Size of the image.\n"]
        pub size: ::std::option::Option<super::foreign::NumBytes>,
        #[doc = "Layout of the image.\n"]
        pub layout: ::std::option::Option<ImageLayout>,
    }
    impl ImageConfig {
        #[doc = "Creates a new [`ImageConfig`]."]
        pub fn new(
            layer: ::std::string::String,
            architecture: super::systems::Architecture,
        ) -> Self {
            Self {
                layer,
                architecture,
                target: ::std::default::Default::default(),
                size: ::std::default::Default::default(),
                layout: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `layer`."]
        pub fn set_layer(&mut self, layer: ::std::string::String) -> &mut Self {
            self.layer = layer;
            self
        }
        #[doc = "Sets the value of `layer`."]
        pub fn with_layer(mut self, layer: ::std::string::String) -> Self {
            self.layer = layer;
            self
        }
        #[doc = "Sets the value of `architecture`."]
        pub fn set_architecture(
            &mut self,
            architecture: super::systems::Architecture,
        ) -> &mut Self {
            self.architecture = architecture;
            self
        }
        #[doc = "Sets the value of `architecture`."]
        pub fn with_architecture(mut self, architecture: super::systems::Architecture) -> Self {
            self.architecture = architecture;
            self
        }
        #[doc = "Sets the value of `target`."]
        pub fn set_target(
            &mut self,
            target: ::std::option::Option<super::systems::Target>,
        ) -> &mut Self {
            self.target = target;
            self
        }
        #[doc = "Sets the value of `target`."]
        pub fn with_target(
            mut self,
            target: ::std::option::Option<super::systems::Target>,
        ) -> Self {
            self.target = target;
            self
        }
        #[doc = "Sets the value of `size`."]
        pub fn set_size(
            &mut self,
            size: ::std::option::Option<super::foreign::NumBytes>,
        ) -> &mut Self {
            self.size = size;
            self
        }
        #[doc = "Sets the value of `size`."]
        pub fn with_size(mut self, size: ::std::option::Option<super::foreign::NumBytes>) -> Self {
            self.size = size;
            self
        }
        #[doc = "Sets the value of `layout`."]
        pub fn set_layout(&mut self, layout: ::std::option::Option<ImageLayout>) -> &mut Self {
            self.layout = layout;
            self
        }
        #[doc = "Sets the value of `layout`."]
        pub fn with_layout(mut self, layout: ::std::option::Option<ImageLayout>) -> Self {
            self.layout = layout;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for ImageConfig {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "ImageConfig", 5usize)?;
            __record.serialize_field("layer", &self.layer)?;
            __record.serialize_field("architecture", &self.architecture)?;
            __record
                .serialize_optional_field("target", ::core::option::Option::as_ref(&self.target))?;
            __record
                .serialize_optional_field("size", ::core::option::Option::as_ref(&self.size))?;
            __record
                .serialize_optional_field("layout", ::core::option::Option::as_ref(&self.layout))?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for ImageConfig {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = ImageConfig;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record ImageConfig")
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
                                __serde::de::Error::invalid_length(0usize, &"record with 5 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<
                        super::systems::Architecture,
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
                        ::std::option::Option<super::systems::Target>,
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
                        ::std::option::Option<super::foreign::NumBytes>,
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
                        ::std::option::Option<ImageLayout>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(4usize, &"record with 5 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(ImageConfig {
                        layer: __field0,
                        architecture: __field1,
                        target: __field2,
                        size: __field3,
                        layout: __field4,
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
                        &["layer", "architecture", "target", "size", "layout"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS : & 'static str = "an identifier in [\"layer\", \"architecture\", \"target\", \"size\", \"layout\"]" ;
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
                                "layer" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                "architecture" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                "target" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                                "size" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                                "layout" => ::core::result::Result::Ok(__Identifier::__Identifier4),
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
                                b"layer" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                b"architecture" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                b"target" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier2)
                                }
                                b"size" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                                b"layout" => {
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
                    let mut __field0: ::core::option::Option<::std::string::String> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<super::systems::Architecture> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<
                        ::std::option::Option<super::systems::Target>,
                    > = ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<
                        ::std::option::Option<super::foreign::NumBytes>,
                    > = ::core::option::Option::None;
                    let mut __field4: ::core::option::Option<::std::option::Option<ImageLayout>> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "layer",
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
                                            "architecture",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        super::systems::Architecture,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "target",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<super::systems::Target>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier3 => {
                                if ::core::option::Option::is_some(&__field3) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("size"),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<super::foreign::NumBytes>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier4 => {
                                if ::core::option::Option::is_some(&__field4) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "layout",
                                        ),
                                    );
                                }
                                __field4 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<ImageLayout>,
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
                                <__A::Error as __serde::de::Error>::missing_field("layer"),
                            );
                        }
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("architecture"),
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
                    let __field4 = match __field4 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(ImageConfig {
                        layer: __field0,
                        architecture: __field1,
                        target: __field2,
                        size: __field3,
                        layout: __field4,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] =
                &["layer", "architecture", "target", "size", "layout"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "ImageConfig",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "Layout of an image.\n"]
    #[derive(Clone, Debug)]
    pub struct ImageLayout {
        #[doc = "Type of the partition table.\n"]
        pub ty: ::std::option::Option<PartitionTableType>,
        #[doc = "Image partitions.\n"]
        pub partitions: ::std::option::Option<::std::vec::Vec<ImagePartition>>,
    }
    impl ImageLayout {
        #[doc = "Creates a new [`ImageLayout`]."]
        pub fn new() -> Self {
            Self {
                ty: ::std::default::Default::default(),
                partitions: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `ty`."]
        pub fn set_ty(&mut self, ty: ::std::option::Option<PartitionTableType>) -> &mut Self {
            self.ty = ty;
            self
        }
        #[doc = "Sets the value of `ty`."]
        pub fn with_ty(mut self, ty: ::std::option::Option<PartitionTableType>) -> Self {
            self.ty = ty;
            self
        }
        #[doc = "Sets the value of `partitions`."]
        pub fn set_partitions(
            &mut self,
            partitions: ::std::option::Option<::std::vec::Vec<ImagePartition>>,
        ) -> &mut Self {
            self.partitions = partitions;
            self
        }
        #[doc = "Sets the value of `partitions`."]
        pub fn with_partitions(
            mut self,
            partitions: ::std::option::Option<::std::vec::Vec<ImagePartition>>,
        ) -> Self {
            self.partitions = partitions;
            self
        }
    }
    impl ::std::default::Default for ImageLayout {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for ImageLayout {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "ImageLayout", 2usize)?;
            __record.serialize_optional_field("type", ::core::option::Option::as_ref(&self.ty))?;
            __record.serialize_optional_field(
                "partitions",
                ::core::option::Option::as_ref(&self.partitions),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for ImageLayout {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = ImageLayout;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record ImageLayout")
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
                        ::std::option::Option<PartitionTableType>,
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
                        ::std::option::Option<::std::vec::Vec<ImagePartition>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 2 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(ImageLayout {
                        ty: __field0,
                        partitions: __field1,
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
                    const __IDENTIFIERS: &'static [&'static str] = &["type", "partitions"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"type\", \"partitions\"]";
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
                                "type" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                "partitions" => {
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
                                b"type" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                b"partitions" => {
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
                    let mut __field0: ::core::option::Option<
                        ::std::option::Option<PartitionTableType>,
                    > = ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<
                        ::std::option::Option<::std::vec::Vec<ImagePartition>>,
                    > = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("type"),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<PartitionTableType>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "partitions",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<::std::vec::Vec<ImagePartition>>,
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
                    ::core::result::Result::Ok(ImageLayout {
                        ty: __field0,
                        partitions: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["type", "partitions"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "ImageLayout",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "Partition table type.\n"]
    #[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
    pub enum PartitionTableType {
        #[doc = "MBR partition.\n"]
        Mbr,
        #[doc = "GPT partition.\n"]
        Gpt,
    }
    #[automatically_derived]
    impl __serde::Serialize for PartitionTableType {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let __serializer =
                __sidex_serde::ser::VariantSerializer::new(__serializer, "PartitionTableType");
            match self {
                Self::Mbr => __serializer.serialize_tag("mbr", 0u32),
                Self::Gpt => __serializer.serialize_tag("gpt", 1u32),
            }
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for PartitionTableType {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &["mbr", "gpt"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"mbr\", \"gpt\"]";
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
                        "mbr" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "gpt" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
            const __VARIANTS: &'static [&'static str] = &["mbr", "gpt"];
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = PartitionTableType;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "enum PartitionTableType")
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
                            ::core::result::Result::Ok(PartitionTableType::Mbr)
                        }
                        __Identifier::__Identifier1 => {
                            ::core::result::Result::Ok(PartitionTableType::Gpt)
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
                            ::core::result::Result::Ok(PartitionTableType::Mbr)
                        }
                        (__Identifier::__Identifier1, __variant) => {
                            __serde::de::VariantAccess::unit_variant(__variant)?;
                            ::core::result::Result::Ok(PartitionTableType::Gpt)
                        }
                    }
                }
            }
            __serde::Deserializer::deserialize_enum(
                __deserializer,
                "PartitionTableType",
                __VARIANTS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "Partition of an image.\n"]
    #[derive(Clone, Debug)]
    pub struct ImagePartition {
        #[doc = "Size of the partition.\n"]
        pub size: ::std::option::Option<super::foreign::NumBytes>,
        #[doc = "Filesystem of the partition.\n"]
        pub filesystem: ::std::option::Option<Filesystem>,
        #[doc = "Root directory to copy into the filesystem.\n"]
        pub root: ::std::option::Option<::std::string::String>,
        #[doc = "Type of the partition (GUID or MBR hex value).\n"]
        pub ty: ::std::option::Option<PartitionType>,
    }
    impl ImagePartition {
        #[doc = "Creates a new [`ImagePartition`]."]
        pub fn new() -> Self {
            Self {
                size: ::std::default::Default::default(),
                filesystem: ::std::default::Default::default(),
                root: ::std::default::Default::default(),
                ty: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `size`."]
        pub fn set_size(
            &mut self,
            size: ::std::option::Option<super::foreign::NumBytes>,
        ) -> &mut Self {
            self.size = size;
            self
        }
        #[doc = "Sets the value of `size`."]
        pub fn with_size(mut self, size: ::std::option::Option<super::foreign::NumBytes>) -> Self {
            self.size = size;
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
        #[doc = "Sets the value of `root`."]
        pub fn set_root(
            &mut self,
            root: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.root = root;
            self
        }
        #[doc = "Sets the value of `root`."]
        pub fn with_root(mut self, root: ::std::option::Option<::std::string::String>) -> Self {
            self.root = root;
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
    }
    impl ::std::default::Default for ImagePartition {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for ImagePartition {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "ImagePartition", 4usize)?;
            __record
                .serialize_optional_field("size", ::core::option::Option::as_ref(&self.size))?;
            __record.serialize_optional_field(
                "filesystem",
                ::core::option::Option::as_ref(&self.filesystem),
            )?;
            __record
                .serialize_optional_field("root", ::core::option::Option::as_ref(&self.root))?;
            __record.serialize_optional_field("type", ::core::option::Option::as_ref(&self.ty))?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for ImagePartition {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = ImagePartition;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record ImagePartition")
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
                        ::std::option::Option<super::foreign::NumBytes>,
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
                        ::std::option::Option<Filesystem>,
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
                        ::std::option::Option<PartitionType>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(3usize, &"record with 4 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(ImagePartition {
                        size: __field0,
                        filesystem: __field1,
                        root: __field2,
                        ty: __field3,
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
                        &["size", "filesystem", "root", "type"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"size\", \"filesystem\", \"root\", \"type\"]";
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
                                "size" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                "filesystem" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                "root" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                                "type" => ::core::result::Result::Ok(__Identifier::__Identifier3),
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
                                b"size" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                b"filesystem" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                b"root" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                                b"type" => ::core::result::Result::Ok(__Identifier::__Identifier3),
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
                        ::std::option::Option<super::foreign::NumBytes>,
                    > = ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::std::option::Option<Filesystem>> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<
                        ::std::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<::std::option::Option<PartitionType>> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("size"),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<super::foreign::NumBytes>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "filesystem",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<Filesystem>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("root"),
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
                                        <__A::Error as __serde::de::Error>::duplicate_field("type"),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<PartitionType>,
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
                    ::core::result::Result::Ok(ImagePartition {
                        size: __field0,
                        filesystem: __field1,
                        root: __field2,
                        ty: __field3,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["size", "filesystem", "root", "type"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "ImagePartition",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "Filesystem.\n"]
    #[derive(Clone, Debug)]
    pub enum Filesystem {
        #[doc = "EXT4\n"]
        Ext4(Ext4Options),
        #[doc = "Fat32\n"]
        Fat32,
        #[doc = "Squashfs\n"]
        Squashfs(SquashfsOptions),
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
                Self::Fat32 => __serializer.serialize_internal_tag("type", "fat32", 1u32),
                Self::Squashfs(__value) => {
                    __serializer.serialize_internally_tagged("type", "squashfs", 2u32, __value)
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
            const __IDENTIFIERS: &'static [&'static str] = &["ext4", "fat32", "squashfs"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"ext4\", \"fat32\", \"squashfs\"]";
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
                        "ext4" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "fat32" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "squashfs" => ::core::result::Result::Ok(__Identifier::__Identifier2),
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
                        b"fat32" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"squashfs" => ::core::result::Result::Ok(__Identifier::__Identifier2),
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
            const __VARIANTS: &'static [&'static str] = &["ext4", "fat32", "squashfs"];
            if __serde::Deserializer::is_human_readable(&__deserializer) {
                let __tagged = __sidex_serde::de::tagged::deserialize_tagged_variant::<
                    __Identifier,
                    __D,
                >(__deserializer, "type")?;
                match __tagged.tag {
                    __Identifier::__Identifier0 => ::core::result::Result::Ok(Filesystem::Ext4(
                        __tagged.deserialize_internally_tagged::<Ext4Options, __D::Error>()?,
                    )),
                    __Identifier::__Identifier1 => ::core::result::Result::Ok(Filesystem::Fat32),
                    __Identifier::__Identifier2 => {
                        ::core::result::Result::Ok(Filesystem::Squashfs(
                            __tagged
                                .deserialize_internally_tagged::<SquashfsOptions, __D::Error>()?,
                        ))
                    }
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
                            __Identifier::__Identifier1 => {
                                ::core::result::Result::Ok(Filesystem::Fat32)
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
                                    Ext4Options,
                                >(__variant)?;
                                ::core::result::Result::Ok(Filesystem::Ext4(__value))
                            }
                            (__Identifier::__Identifier1, __variant) => {
                                __serde::de::VariantAccess::unit_variant(__variant)?;
                                ::core::result::Result::Ok(Filesystem::Fat32)
                            }
                            (__Identifier::__Identifier2, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    SquashfsOptions,
                                >(__variant)?;
                                ::core::result::Result::Ok(Filesystem::Squashfs(__value))
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
    pub struct Ext4Options {
        #[doc = ""]
        pub additional_options: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[doc = ""]
        pub clamp_mtime: ::std::option::Option<Timestamp>,
    }
    impl Ext4Options {
        #[doc = "Creates a new [`Ext4Options`]."]
        pub fn new() -> Self {
            Self {
                additional_options: ::std::default::Default::default(),
                clamp_mtime: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `additional_options`."]
        pub fn set_additional_options(
            &mut self,
            additional_options: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ) -> &mut Self {
            self.additional_options = additional_options;
            self
        }
        #[doc = "Sets the value of `additional_options`."]
        pub fn with_additional_options(
            mut self,
            additional_options: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.additional_options = additional_options;
            self
        }
        #[doc = "Sets the value of `clamp_mtime`."]
        pub fn set_clamp_mtime(
            &mut self,
            clamp_mtime: ::std::option::Option<Timestamp>,
        ) -> &mut Self {
            self.clamp_mtime = clamp_mtime;
            self
        }
        #[doc = "Sets the value of `clamp_mtime`."]
        pub fn with_clamp_mtime(mut self, clamp_mtime: ::std::option::Option<Timestamp>) -> Self {
            self.clamp_mtime = clamp_mtime;
            self
        }
    }
    impl ::std::default::Default for Ext4Options {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for Ext4Options {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "Ext4Options", 2usize)?;
            __record.serialize_optional_field(
                "additional-options",
                ::core::option::Option::as_ref(&self.additional_options),
            )?;
            __record.serialize_optional_field(
                "clamp-mtime",
                ::core::option::Option::as_ref(&self.clamp_mtime),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Ext4Options {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = Ext4Options;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record Ext4Options")
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
                        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
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
                        ::std::option::Option<Timestamp>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 2 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(Ext4Options {
                        additional_options: __field0,
                        clamp_mtime: __field1,
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
                        &["additional-options", "clamp-mtime"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"additional-options\", \"clamp-mtime\"]";
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
                                "additional-options" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                "clamp-mtime" => {
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
                                b"additional-options" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                b"clamp-mtime" => {
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
                    let mut __field0: ::core::option::Option<
                        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
                    > = ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::std::option::Option<Timestamp>> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "additional-options",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<
                                            ::std::vec::Vec<::std::string::String>,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "clamp-mtime",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<Timestamp>,
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
                    ::core::result::Result::Ok(Ext4Options {
                        additional_options: __field0,
                        clamp_mtime: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["additional-options", "clamp-mtime"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "Ext4Options",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = ""]
    #[derive(Clone, Debug)]
    pub struct SquashfsOptions {
        #[doc = ""]
        pub no_compression: ::std::option::Option<bool>,
    }
    impl SquashfsOptions {
        #[doc = "Creates a new [`SquashfsOptions`]."]
        pub fn new() -> Self {
            Self {
                no_compression: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `no_compression`."]
        pub fn set_no_compression(
            &mut self,
            no_compression: ::std::option::Option<bool>,
        ) -> &mut Self {
            self.no_compression = no_compression;
            self
        }
        #[doc = "Sets the value of `no_compression`."]
        pub fn with_no_compression(mut self, no_compression: ::std::option::Option<bool>) -> Self {
            self.no_compression = no_compression;
            self
        }
    }
    impl ::std::default::Default for SquashfsOptions {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for SquashfsOptions {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "SquashfsOptions", 1usize)?;
            __record.serialize_optional_field(
                "no-compression",
                ::core::option::Option::as_ref(&self.no_compression),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for SquashfsOptions {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = SquashfsOptions;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record SquashfsOptions")
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
                                __serde::de::Error::invalid_length(0usize, &"record with 1 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(SquashfsOptions {
                        no_compression: __field0,
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
                    const __IDENTIFIERS: &'static [&'static str] = &["no-compression"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"no-compression\"]";
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
                                "no-compression" => {
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
                                b"no-compression" => {
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
                    let mut __field0: ::core::option::Option<::std::option::Option<bool>> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "no-compression",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
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
                    ::core::result::Result::Ok(SquashfsOptions {
                        no_compression: __field0,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["no-compression"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "SquashfsOptions",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "Partition type.\n"]
    pub type PartitionType = ::rugix_common::disk::PartitionType;
}
pub mod layers {
    #![doc = "Layer configuration.\n"]
    #[allow(unused)]
    use :: serde as __serde;
    #[allow(unused)]
    use :: sidex_serde as __sidex_serde;
    #[doc = "Layer configuration.\n"]
    #[derive(Clone, Debug)]
    pub struct LayerConfig {
        #[doc = "Human-friendly name of the layer.\n"]
        pub name: ::std::option::Option<::std::string::String>,
        #[doc = "Description of the layer.\n"]
        pub description: ::std::option::Option<::std::string::String>,
        #[doc = "URL for importing the layer.\n"]
        pub url: ::std::option::Option<::std::string::String>,
        #[doc = "Parent layer.\n"]
        pub parent: ::std::option::Option<::std::string::String>,
        #[doc = "Indicates whether the layer is a root layer.\n"]
        pub root: ::std::option::Option<bool>,
        #[doc = "Recipes to build the layer with.\n"]
        pub recipes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[doc = "Recipes to specifically exclude.\n"]
        pub exclude: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[doc = "Recipe parameters.\n"]
        pub parameters: ::std::option::Option<
            ::std::collections::HashMap<
                ::std::string::String,
                ::std::collections::HashMap<::std::string::String, super::recipes::ParameterValue>,
            >,
        >,
    }
    impl LayerConfig {
        #[doc = "Creates a new [`LayerConfig`]."]
        pub fn new() -> Self {
            Self {
                name: ::std::default::Default::default(),
                description: ::std::default::Default::default(),
                url: ::std::default::Default::default(),
                parent: ::std::default::Default::default(),
                root: ::std::default::Default::default(),
                recipes: ::std::default::Default::default(),
                exclude: ::std::default::Default::default(),
                parameters: ::std::default::Default::default(),
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
        #[doc = "Sets the value of `description`."]
        pub fn set_description(
            &mut self,
            description: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn with_description(
            mut self,
            description: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `url`."]
        pub fn set_url(&mut self, url: ::std::option::Option<::std::string::String>) -> &mut Self {
            self.url = url;
            self
        }
        #[doc = "Sets the value of `url`."]
        pub fn with_url(mut self, url: ::std::option::Option<::std::string::String>) -> Self {
            self.url = url;
            self
        }
        #[doc = "Sets the value of `parent`."]
        pub fn set_parent(
            &mut self,
            parent: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.parent = parent;
            self
        }
        #[doc = "Sets the value of `parent`."]
        pub fn with_parent(mut self, parent: ::std::option::Option<::std::string::String>) -> Self {
            self.parent = parent;
            self
        }
        #[doc = "Sets the value of `root`."]
        pub fn set_root(&mut self, root: ::std::option::Option<bool>) -> &mut Self {
            self.root = root;
            self
        }
        #[doc = "Sets the value of `root`."]
        pub fn with_root(mut self, root: ::std::option::Option<bool>) -> Self {
            self.root = root;
            self
        }
        #[doc = "Sets the value of `recipes`."]
        pub fn set_recipes(
            &mut self,
            recipes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ) -> &mut Self {
            self.recipes = recipes;
            self
        }
        #[doc = "Sets the value of `recipes`."]
        pub fn with_recipes(
            mut self,
            recipes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.recipes = recipes;
            self
        }
        #[doc = "Sets the value of `exclude`."]
        pub fn set_exclude(
            &mut self,
            exclude: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ) -> &mut Self {
            self.exclude = exclude;
            self
        }
        #[doc = "Sets the value of `exclude`."]
        pub fn with_exclude(
            mut self,
            exclude: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.exclude = exclude;
            self
        }
        #[doc = "Sets the value of `parameters`."]
        pub fn set_parameters(
            &mut self,
            parameters: ::std::option::Option<
                ::std::collections::HashMap<
                    ::std::string::String,
                    ::std::collections::HashMap<
                        ::std::string::String,
                        super::recipes::ParameterValue,
                    >,
                >,
            >,
        ) -> &mut Self {
            self.parameters = parameters;
            self
        }
        #[doc = "Sets the value of `parameters`."]
        pub fn with_parameters(
            mut self,
            parameters: ::std::option::Option<
                ::std::collections::HashMap<
                    ::std::string::String,
                    ::std::collections::HashMap<
                        ::std::string::String,
                        super::recipes::ParameterValue,
                    >,
                >,
            >,
        ) -> Self {
            self.parameters = parameters;
            self
        }
    }
    impl ::std::default::Default for LayerConfig {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for LayerConfig {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "LayerConfig", 8usize)?;
            __record
                .serialize_optional_field("name", ::core::option::Option::as_ref(&self.name))?;
            __record.serialize_optional_field(
                "description",
                ::core::option::Option::as_ref(&self.description),
            )?;
            __record.serialize_optional_field("url", ::core::option::Option::as_ref(&self.url))?;
            __record
                .serialize_optional_field("parent", ::core::option::Option::as_ref(&self.parent))?;
            __record
                .serialize_optional_field("root", ::core::option::Option::as_ref(&self.root))?;
            __record.serialize_optional_field(
                "recipes",
                ::core::option::Option::as_ref(&self.recipes),
            )?;
            __record.serialize_optional_field(
                "exclude",
                ::core::option::Option::as_ref(&self.exclude),
            )?;
            __record.serialize_optional_field(
                "parameters",
                ::core::option::Option::as_ref(&self.parameters),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for LayerConfig {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = LayerConfig;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record LayerConfig")
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
                                __serde::de::Error::invalid_length(0usize, &"record with 8 fields"),
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
                                __serde::de::Error::invalid_length(1usize, &"record with 8 fields"),
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
                                __serde::de::Error::invalid_length(2usize, &"record with 8 fields"),
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
                                __serde::de::Error::invalid_length(3usize, &"record with 8 fields"),
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
                                __serde::de::Error::invalid_length(4usize, &"record with 8 fields"),
                            );
                        }
                    };
                    let __field5 = match __serde::de::SeqAccess::next_element::<
                        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(5usize, &"record with 8 fields"),
                            );
                        }
                    };
                    let __field6 = match __serde::de::SeqAccess::next_element::<
                        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(6usize, &"record with 8 fields"),
                            );
                        }
                    };
                    let __field7 = match __serde::de::SeqAccess::next_element::<
                        ::std::option::Option<
                            ::std::collections::HashMap<
                                ::std::string::String,
                                ::std::collections::HashMap<
                                    ::std::string::String,
                                    super::recipes::ParameterValue,
                                >,
                            >,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(7usize, &"record with 8 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(LayerConfig {
                        name: __field0,
                        description: __field1,
                        url: __field2,
                        parent: __field3,
                        root: __field4,
                        recipes: __field5,
                        exclude: __field6,
                        parameters: __field7,
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
                        "name",
                        "description",
                        "url",
                        "parent",
                        "root",
                        "recipes",
                        "exclude",
                        "parameters",
                    ];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS : & 'static str = "an identifier in [\"name\", \"description\", \"url\", \"parent\", \"root\", \"recipes\", \"exclude\", \"parameters\"]" ;
                    #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
                    #[doc(hidden)]
                    enum __Identifier {
                        __Identifier0,
                        __Identifier1,
                        __Identifier2,
                        __Identifier3,
                        __Identifier4,
                        __Identifier5,
                        __Identifier6,
                        __Identifier7,
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
                                5u64 => ::core::result::Result::Ok(__Identifier::__Identifier5),
                                6u64 => ::core::result::Result::Ok(__Identifier::__Identifier6),
                                7u64 => ::core::result::Result::Ok(__Identifier::__Identifier7),
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
                                "description" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                "url" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                                "parent" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                                "root" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                                "recipes" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier5)
                                }
                                "exclude" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier6)
                                }
                                "parameters" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier7)
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
                                b"description" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                b"url" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                                b"parent" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier3)
                                }
                                b"root" => ::core::result::Result::Ok(__Identifier::__Identifier4),
                                b"recipes" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier5)
                                }
                                b"exclude" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier6)
                                }
                                b"parameters" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier7)
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
                    let mut __field1: ::core::option::Option<
                        ::std::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<
                        ::std::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<
                        ::std::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field4: ::core::option::Option<::std::option::Option<bool>> =
                        ::core::option::Option::None;
                    let mut __field5: ::core::option::Option<
                        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
                    > = ::core::option::Option::None;
                    let mut __field6: ::core::option::Option<
                        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
                    > = ::core::option::Option::None;
                    let mut __field7: ::core::option::Option<
                        ::std::option::Option<
                            ::std::collections::HashMap<
                                ::std::string::String,
                                ::std::collections::HashMap<
                                    ::std::string::String,
                                    super::recipes::ParameterValue,
                                >,
                            >,
                        >,
                    > = ::core::option::Option::None;
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
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "description",
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
                                        <__A::Error as __serde::de::Error>::duplicate_field("url"),
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
                                            "parent",
                                        ),
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
                                        <__A::Error as __serde::de::Error>::duplicate_field("root"),
                                    );
                                }
                                __field4 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<bool>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier5 => {
                                if ::core::option::Option::is_some(&__field5) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "recipes",
                                        ),
                                    );
                                }
                                __field5 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<
                                            ::std::vec::Vec<::std::string::String>,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier6 => {
                                if ::core::option::Option::is_some(&__field6) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "exclude",
                                        ),
                                    );
                                }
                                __field6 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<
                                            ::std::vec::Vec<::std::string::String>,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier7 => {
                                if ::core::option::Option::is_some(&__field7) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "parameters",
                                        ),
                                    );
                                }
                                __field7 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<
                                            ::std::collections::HashMap<
                                                ::std::string::String,
                                                ::std::collections::HashMap<
                                                    ::std::string::String,
                                                    super::recipes::ParameterValue,
                                                >,
                                            >,
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
                    let __field5 = match __field5 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field6 = match __field6 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field7 = match __field7 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(LayerConfig {
                        name: __field0,
                        description: __field1,
                        url: __field2,
                        parent: __field3,
                        root: __field4,
                        recipes: __field5,
                        exclude: __field6,
                        parameters: __field7,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &[
                "name",
                "description",
                "url",
                "parent",
                "root",
                "recipes",
                "exclude",
                "parameters",
            ];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "LayerConfig",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
}
pub mod projects {
    #![doc = "Project configuration.\n"]
    #[allow(unused)]
    use :: serde as __serde;
    #[allow(unused)]
    use :: sidex_serde as __sidex_serde;
    #[doc = "Project configuration.\n"]
    #[derive(Clone, Debug)]
    pub struct ProjectConfig {
        #[doc = "Repositories imported into the project.\n"]
        pub repositories: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, super::repositories::SourceConfig>,
        >,
        #[doc = "System declarations.\n"]
        pub systems: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, super::systems::SystemConfig>,
        >,
    }
    impl ProjectConfig {
        #[doc = "Creates a new [`ProjectConfig`]."]
        pub fn new() -> Self {
            Self {
                repositories: ::std::default::Default::default(),
                systems: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `repositories`."]
        pub fn set_repositories(
            &mut self,
            repositories: ::std::option::Option<
                ::std::collections::HashMap<
                    ::std::string::String,
                    super::repositories::SourceConfig,
                >,
            >,
        ) -> &mut Self {
            self.repositories = repositories;
            self
        }
        #[doc = "Sets the value of `repositories`."]
        pub fn with_repositories(
            mut self,
            repositories: ::std::option::Option<
                ::std::collections::HashMap<
                    ::std::string::String,
                    super::repositories::SourceConfig,
                >,
            >,
        ) -> Self {
            self.repositories = repositories;
            self
        }
        #[doc = "Sets the value of `systems`."]
        pub fn set_systems(
            &mut self,
            systems: ::std::option::Option<
                ::std::collections::HashMap<::std::string::String, super::systems::SystemConfig>,
            >,
        ) -> &mut Self {
            self.systems = systems;
            self
        }
        #[doc = "Sets the value of `systems`."]
        pub fn with_systems(
            mut self,
            systems: ::std::option::Option<
                ::std::collections::HashMap<::std::string::String, super::systems::SystemConfig>,
            >,
        ) -> Self {
            self.systems = systems;
            self
        }
    }
    impl ::std::default::Default for ProjectConfig {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for ProjectConfig {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "ProjectConfig", 2usize)?;
            __record.serialize_optional_field(
                "repositories",
                ::core::option::Option::as_ref(&self.repositories),
            )?;
            __record.serialize_optional_field(
                "systems",
                ::core::option::Option::as_ref(&self.systems),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for ProjectConfig {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = ProjectConfig;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record ProjectConfig")
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
                        ::std::option::Option<
                            ::std::collections::HashMap<
                                ::std::string::String,
                                super::repositories::SourceConfig,
                            >,
                        >,
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
                        ::std::option::Option<
                            ::std::collections::HashMap<
                                ::std::string::String,
                                super::systems::SystemConfig,
                            >,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 2 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(ProjectConfig {
                        repositories: __field0,
                        systems: __field1,
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
                    const __IDENTIFIERS: &'static [&'static str] = &["repositories", "systems"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"repositories\", \"systems\"]";
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
                                "repositories" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                "systems" => {
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
                                b"repositories" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                b"systems" => {
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
                    let mut __field0: ::core::option::Option<
                        ::std::option::Option<
                            ::std::collections::HashMap<
                                ::std::string::String,
                                super::repositories::SourceConfig,
                            >,
                        >,
                    > = ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<
                        ::std::option::Option<
                            ::std::collections::HashMap<
                                ::std::string::String,
                                super::systems::SystemConfig,
                            >,
                        >,
                    > = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "repositories",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<
                                            ::std::collections::HashMap<
                                                ::std::string::String,
                                                super::repositories::SourceConfig,
                                            >,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "systems",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<
                                            ::std::collections::HashMap<
                                                ::std::string::String,
                                                super::systems::SystemConfig,
                                            >,
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
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(ProjectConfig {
                        repositories: __field0,
                        systems: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["repositories", "systems"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "ProjectConfig",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
}
pub mod recipes {
    #![doc = "Recipe configuration.\n"]
    #[allow(unused)]
    use :: serde as __serde;
    #[allow(unused)]
    use :: sidex_serde as __sidex_serde;
    #[doc = "Recipe configuration.\n"]
    #[derive(Clone, Debug)]
    pub struct RecipeConfig {
        #[doc = "Description of the recipe.\n"]
        pub description: ::std::option::Option<::std::string::String>,
        #[doc = "Priority of the recipe.\n"]
        pub priority: ::std::option::Option<i64>,
        #[doc = "Dependencies of the recipe.\n"]
        pub dependencies: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[doc = "Parameter definitions of the recipe.\n"]
        pub parameters:
            ::std::option::Option<::std::collections::HashMap<::std::string::String, ParameterDef>>,
    }
    impl RecipeConfig {
        #[doc = "Creates a new [`RecipeConfig`]."]
        pub fn new() -> Self {
            Self {
                description: ::std::default::Default::default(),
                priority: ::std::default::Default::default(),
                dependencies: ::std::default::Default::default(),
                parameters: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `description`."]
        pub fn set_description(
            &mut self,
            description: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn with_description(
            mut self,
            description: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `priority`."]
        pub fn set_priority(&mut self, priority: ::std::option::Option<i64>) -> &mut Self {
            self.priority = priority;
            self
        }
        #[doc = "Sets the value of `priority`."]
        pub fn with_priority(mut self, priority: ::std::option::Option<i64>) -> Self {
            self.priority = priority;
            self
        }
        #[doc = "Sets the value of `dependencies`."]
        pub fn set_dependencies(
            &mut self,
            dependencies: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ) -> &mut Self {
            self.dependencies = dependencies;
            self
        }
        #[doc = "Sets the value of `dependencies`."]
        pub fn with_dependencies(
            mut self,
            dependencies: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.dependencies = dependencies;
            self
        }
        #[doc = "Sets the value of `parameters`."]
        pub fn set_parameters(
            &mut self,
            parameters: ::std::option::Option<
                ::std::collections::HashMap<::std::string::String, ParameterDef>,
            >,
        ) -> &mut Self {
            self.parameters = parameters;
            self
        }
        #[doc = "Sets the value of `parameters`."]
        pub fn with_parameters(
            mut self,
            parameters: ::std::option::Option<
                ::std::collections::HashMap<::std::string::String, ParameterDef>,
            >,
        ) -> Self {
            self.parameters = parameters;
            self
        }
    }
    impl ::std::default::Default for RecipeConfig {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for RecipeConfig {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "RecipeConfig", 4usize)?;
            __record.serialize_optional_field(
                "description",
                ::core::option::Option::as_ref(&self.description),
            )?;
            __record.serialize_optional_field(
                "priority",
                ::core::option::Option::as_ref(&self.priority),
            )?;
            __record.serialize_optional_field(
                "dependencies",
                ::core::option::Option::as_ref(&self.dependencies),
            )?;
            __record.serialize_optional_field(
                "parameters",
                ::core::option::Option::as_ref(&self.parameters),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for RecipeConfig {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = RecipeConfig;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record RecipeConfig")
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
                        ::std::option::Option<i64>,
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
                        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
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
                        ::std::option::Option<
                            ::std::collections::HashMap<::std::string::String, ParameterDef>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(3usize, &"record with 4 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(RecipeConfig {
                        description: __field0,
                        priority: __field1,
                        dependencies: __field2,
                        parameters: __field3,
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
                        &["description", "priority", "dependencies", "parameters"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS : & 'static str = "an identifier in [\"description\", \"priority\", \"dependencies\", \"parameters\"]" ;
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
                                "description" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                "priority" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                "dependencies" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier2)
                                }
                                "parameters" => {
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
                                b"description" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                b"priority" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                b"dependencies" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier2)
                                }
                                b"parameters" => {
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
                    let mut __field1: ::core::option::Option<::std::option::Option<i64>> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<
                        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
                    > = ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<
                        ::std::option::Option<
                            ::std::collections::HashMap<::std::string::String, ParameterDef>,
                        >,
                    > = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "description",
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
                                            "priority",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<::std::option::Option<i64>>(
                                        &mut __map,
                                    )?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "dependencies",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<
                                            ::std::vec::Vec<::std::string::String>,
                                        >,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier3 => {
                                if ::core::option::Option::is_some(&__field3) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "parameters",
                                        ),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<
                                            ::std::collections::HashMap<
                                                ::std::string::String,
                                                ParameterDef,
                                            >,
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
                    ::core::result::Result::Ok(RecipeConfig {
                        description: __field0,
                        priority: __field1,
                        dependencies: __field2,
                        parameters: __field3,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] =
                &["description", "priority", "dependencies", "parameters"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "RecipeConfig",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "Recipe parameter definition.\n"]
    #[derive(Clone, Debug)]
    pub struct ParameterDef {
        #[doc = "Optional default value of the parameter.\n"]
        pub default: ::std::option::Option<ParameterValue>,
    }
    impl ParameterDef {
        #[doc = "Creates a new [`ParameterDef`]."]
        pub fn new() -> Self {
            Self {
                default: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `default`."]
        pub fn set_default(&mut self, default: ::std::option::Option<ParameterValue>) -> &mut Self {
            self.default = default;
            self
        }
        #[doc = "Sets the value of `default`."]
        pub fn with_default(mut self, default: ::std::option::Option<ParameterValue>) -> Self {
            self.default = default;
            self
        }
    }
    impl ::std::default::Default for ParameterDef {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for ParameterDef {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "ParameterDef", 1usize)?;
            __record.serialize_optional_field(
                "default",
                ::core::option::Option::as_ref(&self.default),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for ParameterDef {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = ParameterDef;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record ParameterDef")
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
                        ::std::option::Option<ParameterValue>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 1 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(ParameterDef { default: __field0 })
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
                    const __IDENTIFIERS: &'static [&'static str] = &["default"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"default\"]";
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
                                "default" => {
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
                                b"default" => {
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
                        ::std::option::Option<ParameterValue>,
                    > = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "default",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<ParameterValue>,
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
                    ::core::result::Result::Ok(ParameterDef { default: __field0 })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["default"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "ParameterDef",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "Value of a parameter.\n"]
    #[derive(Clone, Debug)]
    pub enum ParameterValue {
        #[doc = "String.\n"]
        String(::std::string::String),
        #[doc = "Boolean.\n"]
        Boolean(bool),
        #[doc = "Integer.\n"]
        Integer(i64),
        #[doc = "Float.\n"]
        Float(f64),
    }
    #[automatically_derived]
    impl __serde::Serialize for ParameterValue {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let __serializer =
                __sidex_serde::ser::VariantSerializer::new(__serializer, "ParameterValue");
            match self {
                Self::String(__value) => {
                    __serializer.serialize_implicitly_tagged("String", 0u32, __value)
                }
                Self::Boolean(__value) => {
                    __serializer.serialize_implicitly_tagged("Boolean", 1u32, __value)
                }
                Self::Integer(__value) => {
                    __serializer.serialize_implicitly_tagged("Integer", 2u32, __value)
                }
                Self::Float(__value) => {
                    __serializer.serialize_implicitly_tagged("Float", 3u32, __value)
                }
            }
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for ParameterValue {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] =
                &["String", "Boolean", "Integer", "Float"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"String\", \"Boolean\", \"Integer\", \"Float\"]";
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
                        "String" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "Boolean" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "Integer" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "Float" => ::core::result::Result::Ok(__Identifier::__Identifier3),
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
                        b"String" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"Boolean" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"Integer" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        b"Float" => ::core::result::Result::Ok(__Identifier::__Identifier3),
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
            const __VARIANTS: &'static [&'static str] = &["String", "Boolean", "Integer", "Float"];
            if __serde::Deserializer::is_human_readable(&__deserializer) {
                let __content =
                    __sidex_serde::de::content::deserialize_into_content(__deserializer)?;
                match __sidex_serde::de::content::deserialize_content_ref::<
                    ::std::string::String,
                    __D::Error,
                >(&__content)
                {
                    Ok(__value) => return Ok(ParameterValue::String(__value)),
                    Err(_) => {}
                };
                match __sidex_serde::de::content::deserialize_content_ref::<bool, __D::Error>(
                    &__content,
                ) {
                    Ok(__value) => return Ok(ParameterValue::Boolean(__value)),
                    Err(_) => {}
                };
                match __sidex_serde::de::content::deserialize_content_ref::<i64, __D::Error>(
                    &__content,
                ) {
                    Ok(__value) => return Ok(ParameterValue::Integer(__value)),
                    Err(_) => {}
                };
                match __sidex_serde::de::content::deserialize_content_ref::<f64, __D::Error>(
                    &__content,
                ) {
                    Ok(__value) => return Ok(ParameterValue::Float(__value)),
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
                    type Value = ParameterValue;
                    fn expecting(
                        &self,
                        __formatter: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(__formatter, "enum ParameterValue")
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
                                    ::std::string::String,
                                >(__variant)?;
                                ::core::result::Result::Ok(ParameterValue::String(__value))
                            }
                            (__Identifier::__Identifier1, __variant) => {
                                let __value =
                                    __serde::de::VariantAccess::newtype_variant::<bool>(__variant)?;
                                ::core::result::Result::Ok(ParameterValue::Boolean(__value))
                            }
                            (__Identifier::__Identifier2, __variant) => {
                                let __value =
                                    __serde::de::VariantAccess::newtype_variant::<i64>(__variant)?;
                                ::core::result::Result::Ok(ParameterValue::Integer(__value))
                            }
                            (__Identifier::__Identifier3, __variant) => {
                                let __value =
                                    __serde::de::VariantAccess::newtype_variant::<f64>(__variant)?;
                                ::core::result::Result::Ok(ParameterValue::Float(__value))
                            }
                        }
                    }
                }
                __serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "ParameterValue",
                    __VARIANTS,
                    __Visitor {
                        __phantom_vars: ::core::marker::PhantomData,
                    },
                )
            }
        }
    }
}
pub mod repositories {
    #![doc = "Repository configuration.\n"]
    #[allow(unused)]
    use :: serde as __serde;
    #[allow(unused)]
    use :: sidex_serde as __sidex_serde;
    #[doc = "Repository configuration.\n"]
    #[derive(Clone, Debug)]
    pub struct RepositoryConfig {
        #[doc = "Human-friendly name of the repository.\n"]
        pub name: ::std::option::Option<::std::string::String>,
        #[doc = "Description of the repository.\n"]
        pub description: ::std::option::Option<::std::string::String>,
        #[doc = "Repositories used by this repository.\n"]
        pub repositories:
            ::std::option::Option<::std::collections::HashMap<::std::string::String, SourceConfig>>,
    }
    impl RepositoryConfig {
        #[doc = "Creates a new [`RepositoryConfig`]."]
        pub fn new() -> Self {
            Self {
                name: ::std::default::Default::default(),
                description: ::std::default::Default::default(),
                repositories: ::std::default::Default::default(),
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
        #[doc = "Sets the value of `description`."]
        pub fn set_description(
            &mut self,
            description: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn with_description(
            mut self,
            description: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `repositories`."]
        pub fn set_repositories(
            &mut self,
            repositories: ::std::option::Option<
                ::std::collections::HashMap<::std::string::String, SourceConfig>,
            >,
        ) -> &mut Self {
            self.repositories = repositories;
            self
        }
        #[doc = "Sets the value of `repositories`."]
        pub fn with_repositories(
            mut self,
            repositories: ::std::option::Option<
                ::std::collections::HashMap<::std::string::String, SourceConfig>,
            >,
        ) -> Self {
            self.repositories = repositories;
            self
        }
    }
    impl ::std::default::Default for RepositoryConfig {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for RepositoryConfig {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record = __sidex_serde::ser::RecordSerializer::new(
                __serializer,
                "RepositoryConfig",
                3usize,
            )?;
            __record
                .serialize_optional_field("name", ::core::option::Option::as_ref(&self.name))?;
            __record.serialize_optional_field(
                "description",
                ::core::option::Option::as_ref(&self.description),
            )?;
            __record.serialize_optional_field(
                "repositories",
                ::core::option::Option::as_ref(&self.repositories),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for RepositoryConfig {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = RepositoryConfig;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record RepositoryConfig")
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
                        ::std::option::Option<::std::string::String>,
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
                        ::std::option::Option<
                            ::std::collections::HashMap<::std::string::String, SourceConfig>,
                        >,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(2usize, &"record with 3 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(RepositoryConfig {
                        name: __field0,
                        description: __field1,
                        repositories: __field2,
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
                        &["name", "description", "repositories"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"name\", \"description\", \"repositories\"]";
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
                                "name" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                "description" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                "repositories" => {
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
                                b"name" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                b"description" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                b"repositories" => {
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
                    let mut __field1: ::core::option::Option<
                        ::std::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<
                        ::std::option::Option<
                            ::std::collections::HashMap<::std::string::String, SourceConfig>,
                        >,
                    > = ::core::option::Option::None;
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
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "description",
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
                                            "repositories",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<
                                            ::std::collections::HashMap<
                                                ::std::string::String,
                                                SourceConfig,
                                            >,
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
                    ::core::result::Result::Ok(RepositoryConfig {
                        name: __field0,
                        description: __field1,
                        repositories: __field2,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["name", "description", "repositories"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "RepositoryConfig",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "Repository source.\n"]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum SourceConfig {
        #[doc = "Repository is an external Git repository.\n"]
        Git(GitSourceConfig),
        #[doc = "Repository is a path in the project directory.\n"]
        Path(PathSourceConfig),
    }
    #[automatically_derived]
    impl __serde::Serialize for SourceConfig {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let __serializer =
                __sidex_serde::ser::VariantSerializer::new(__serializer, "SourceConfig");
            match self {
                Self::Git(__value) => {
                    __serializer.serialize_implicitly_tagged("Git", 0u32, __value)
                }
                Self::Path(__value) => {
                    __serializer.serialize_implicitly_tagged("Path", 1u32, __value)
                }
            }
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for SourceConfig {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &["Git", "Path"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"Git\", \"Path\"]";
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
                        "Git" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "Path" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
                        b"Git" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"Path" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
            const __VARIANTS: &'static [&'static str] = &["Git", "Path"];
            if __serde::Deserializer::is_human_readable(&__deserializer) {
                let __content =
                    __sidex_serde::de::content::deserialize_into_content(__deserializer)?;
                match __sidex_serde::de::content::deserialize_content_ref::<
                    GitSourceConfig,
                    __D::Error,
                >(&__content)
                {
                    Ok(__value) => return Ok(SourceConfig::Git(__value)),
                    Err(_) => {}
                };
                match __sidex_serde::de::content::deserialize_content_ref::<
                    PathSourceConfig,
                    __D::Error,
                >(&__content)
                {
                    Ok(__value) => return Ok(SourceConfig::Path(__value)),
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
                    type Value = SourceConfig;
                    fn expecting(
                        &self,
                        __formatter: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(__formatter, "enum SourceConfig")
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
                                    GitSourceConfig,
                                >(__variant)?;
                                ::core::result::Result::Ok(SourceConfig::Git(__value))
                            }
                            (__Identifier::__Identifier1, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    PathSourceConfig,
                                >(__variant)?;
                                ::core::result::Result::Ok(SourceConfig::Path(__value))
                            }
                        }
                    }
                }
                __serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "SourceConfig",
                    __VARIANTS,
                    __Visitor {
                        __phantom_vars: ::core::marker::PhantomData,
                    },
                )
            }
        }
    }
    #[doc = "Git repository source.\n"]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct GitSourceConfig {
        #[doc = "URL of the Git repository.\n"]
        pub url: ::std::string::String,
        #[doc = "Specific tag of the Git repository.\n"]
        pub tag: ::std::option::Option<::std::string::String>,
        #[doc = "Specific branch of the Git repository.\n"]
        pub branch: ::std::option::Option<::std::string::String>,
        #[doc = "Specific revision of the Git repository.\n"]
        pub rev: ::std::option::Option<::std::string::String>,
        #[doc = "Subdirectory in which the repository is located.\n"]
        pub dir: ::std::option::Option<::std::string::String>,
    }
    impl GitSourceConfig {
        #[doc = "Creates a new [`GitSourceConfig`]."]
        pub fn new(url: ::std::string::String) -> Self {
            Self {
                url,
                tag: ::std::default::Default::default(),
                branch: ::std::default::Default::default(),
                rev: ::std::default::Default::default(),
                dir: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `url`."]
        pub fn set_url(&mut self, url: ::std::string::String) -> &mut Self {
            self.url = url;
            self
        }
        #[doc = "Sets the value of `url`."]
        pub fn with_url(mut self, url: ::std::string::String) -> Self {
            self.url = url;
            self
        }
        #[doc = "Sets the value of `tag`."]
        pub fn set_tag(&mut self, tag: ::std::option::Option<::std::string::String>) -> &mut Self {
            self.tag = tag;
            self
        }
        #[doc = "Sets the value of `tag`."]
        pub fn with_tag(mut self, tag: ::std::option::Option<::std::string::String>) -> Self {
            self.tag = tag;
            self
        }
        #[doc = "Sets the value of `branch`."]
        pub fn set_branch(
            &mut self,
            branch: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.branch = branch;
            self
        }
        #[doc = "Sets the value of `branch`."]
        pub fn with_branch(mut self, branch: ::std::option::Option<::std::string::String>) -> Self {
            self.branch = branch;
            self
        }
        #[doc = "Sets the value of `rev`."]
        pub fn set_rev(&mut self, rev: ::std::option::Option<::std::string::String>) -> &mut Self {
            self.rev = rev;
            self
        }
        #[doc = "Sets the value of `rev`."]
        pub fn with_rev(mut self, rev: ::std::option::Option<::std::string::String>) -> Self {
            self.rev = rev;
            self
        }
        #[doc = "Sets the value of `dir`."]
        pub fn set_dir(&mut self, dir: ::std::option::Option<::std::string::String>) -> &mut Self {
            self.dir = dir;
            self
        }
        #[doc = "Sets the value of `dir`."]
        pub fn with_dir(mut self, dir: ::std::option::Option<::std::string::String>) -> Self {
            self.dir = dir;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for GitSourceConfig {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "GitSourceConfig", 5usize)?;
            __record.serialize_field("git", &self.url)?;
            __record.serialize_optional_field("tag", ::core::option::Option::as_ref(&self.tag))?;
            __record
                .serialize_optional_field("branch", ::core::option::Option::as_ref(&self.branch))?;
            __record.serialize_optional_field("rev", ::core::option::Option::as_ref(&self.rev))?;
            __record.serialize_optional_field("dir", ::core::option::Option::as_ref(&self.dir))?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for GitSourceConfig {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = GitSourceConfig;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record GitSourceConfig")
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
                        ::std::option::Option<::std::string::String>,
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
                        ::std::option::Option<::std::string::String>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(4usize, &"record with 5 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(GitSourceConfig {
                        url: __field0,
                        tag: __field1,
                        branch: __field2,
                        rev: __field3,
                        dir: __field4,
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
                        &["git", "tag", "branch", "rev", "dir"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"git\", \"tag\", \"branch\", \"rev\", \"dir\"]";
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
                                "git" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                "tag" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                                "branch" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                                "rev" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                                "dir" => ::core::result::Result::Ok(__Identifier::__Identifier4),
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
                                b"git" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                b"tag" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                                b"branch" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier2)
                                }
                                b"rev" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                                b"dir" => ::core::result::Result::Ok(__Identifier::__Identifier4),
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
                        ::std::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field4: ::core::option::Option<
                        ::std::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("git"),
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
                                        <__A::Error as __serde::de::Error>::duplicate_field("tag"),
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
                                            "branch",
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
                                        <__A::Error as __serde::de::Error>::duplicate_field("rev"),
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
                                        <__A::Error as __serde::de::Error>::duplicate_field("dir"),
                                    );
                                }
                                __field4 = ::core::option::Option::Some(
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
                                <__A::Error as __serde::de::Error>::missing_field("git"),
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
                    let __field4 = match __field4 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(GitSourceConfig {
                        url: __field0,
                        tag: __field1,
                        branch: __field2,
                        rev: __field3,
                        dir: __field4,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["git", "tag", "branch", "rev", "dir"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "GitSourceConfig",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "Local repository source.\n"]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct PathSourceConfig {
        #[doc = "Path of the repository relative to the project directory.\n"]
        pub path: ::std::string::String,
    }
    impl PathSourceConfig {
        #[doc = "Creates a new [`PathSourceConfig`]."]
        pub fn new(path: ::std::string::String) -> Self {
            Self { path }
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
    }
    #[automatically_derived]
    impl __serde::Serialize for PathSourceConfig {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record = __sidex_serde::ser::RecordSerializer::new(
                __serializer,
                "PathSourceConfig",
                1usize,
            )?;
            __record.serialize_field("path", &self.path)?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for PathSourceConfig {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = PathSourceConfig;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record PathSourceConfig")
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
                    ::core::result::Result::Ok(PathSourceConfig { path: __field0 })
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
                    const __IDENTIFIERS: &'static [&'static str] = &["path"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"path\"]";
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
                                "path" => ::core::result::Result::Ok(__Identifier::__Identifier0),
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
                                        <__A::Error as __serde::de::Error>::duplicate_field("path"),
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
                                <__A::Error as __serde::de::Error>::missing_field("path"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(PathSourceConfig { path: __field0 })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["path"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "PathSourceConfig",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
}
pub mod systems {
    #![doc = "System configuration.\n"]
    #[allow(unused)]
    use :: serde as __serde;
    #[allow(unused)]
    use :: sidex_serde as __sidex_serde;
    #[doc = ""]
    #[derive(Clone, Debug)]
    pub struct SystemConfig {
        #[doc = "Layer the image is based on.\n"]
        pub layer: ::std::string::String,
        #[doc = "Architecture of the image.\n"]
        pub architecture: Architecture,
        #[doc = "Rugix Bakery target.\n"]
        pub target: ::std::option::Option<Target>,
        #[doc = "System image configuration.\n"]
        pub image: ::std::option::Option<SystemImageConfig>,
        #[doc = "Additional options.\n"]
        pub options: ::std::option::Option<SystemOptions>,
    }
    impl SystemConfig {
        #[doc = "Creates a new [`SystemConfig`]."]
        pub fn new(layer: ::std::string::String, architecture: Architecture) -> Self {
            Self {
                layer,
                architecture,
                target: ::std::default::Default::default(),
                image: ::std::default::Default::default(),
                options: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `layer`."]
        pub fn set_layer(&mut self, layer: ::std::string::String) -> &mut Self {
            self.layer = layer;
            self
        }
        #[doc = "Sets the value of `layer`."]
        pub fn with_layer(mut self, layer: ::std::string::String) -> Self {
            self.layer = layer;
            self
        }
        #[doc = "Sets the value of `architecture`."]
        pub fn set_architecture(&mut self, architecture: Architecture) -> &mut Self {
            self.architecture = architecture;
            self
        }
        #[doc = "Sets the value of `architecture`."]
        pub fn with_architecture(mut self, architecture: Architecture) -> Self {
            self.architecture = architecture;
            self
        }
        #[doc = "Sets the value of `target`."]
        pub fn set_target(&mut self, target: ::std::option::Option<Target>) -> &mut Self {
            self.target = target;
            self
        }
        #[doc = "Sets the value of `target`."]
        pub fn with_target(mut self, target: ::std::option::Option<Target>) -> Self {
            self.target = target;
            self
        }
        #[doc = "Sets the value of `image`."]
        pub fn set_image(&mut self, image: ::std::option::Option<SystemImageConfig>) -> &mut Self {
            self.image = image;
            self
        }
        #[doc = "Sets the value of `image`."]
        pub fn with_image(mut self, image: ::std::option::Option<SystemImageConfig>) -> Self {
            self.image = image;
            self
        }
        #[doc = "Sets the value of `options`."]
        pub fn set_options(&mut self, options: ::std::option::Option<SystemOptions>) -> &mut Self {
            self.options = options;
            self
        }
        #[doc = "Sets the value of `options`."]
        pub fn with_options(mut self, options: ::std::option::Option<SystemOptions>) -> Self {
            self.options = options;
            self
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
            __record.serialize_field("layer", &self.layer)?;
            __record.serialize_field("architecture", &self.architecture)?;
            __record
                .serialize_optional_field("target", ::core::option::Option::as_ref(&self.target))?;
            __record
                .serialize_optional_field("image", ::core::option::Option::as_ref(&self.image))?;
            __record.serialize_optional_field(
                "options",
                ::core::option::Option::as_ref(&self.options),
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
                        ::std::string::String,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 5 fields"),
                            );
                        }
                    };
                    let __field1 =
                        match __serde::de::SeqAccess::next_element::<Architecture>(&mut __seq)? {
                            ::core::option::Option::Some(__value) => __value,
                            ::core::option::Option::None => {
                                return ::core::result::Result::Err(
                                    __serde::de::Error::invalid_length(
                                        1usize,
                                        &"record with 5 fields",
                                    ),
                                );
                            }
                        };
                    let __field2 = match __serde::de::SeqAccess::next_element::<
                        ::std::option::Option<Target>,
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
                        ::std::option::Option<SystemImageConfig>,
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
                        ::std::option::Option<SystemOptions>,
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
                        layer: __field0,
                        architecture: __field1,
                        target: __field2,
                        image: __field3,
                        options: __field4,
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
                        &["layer", "architecture", "target", "image", "options"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS : & 'static str = "an identifier in [\"layer\", \"architecture\", \"target\", \"image\", \"options\"]" ;
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
                                "layer" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                "architecture" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                "target" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                                "image" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                                "options" => {
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
                                b"layer" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                b"architecture" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                b"target" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier2)
                                }
                                b"image" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                                b"options" => {
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
                    let mut __field0: ::core::option::Option<::std::string::String> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<Architecture> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<::std::option::Option<Target>> =
                        ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<
                        ::std::option::Option<SystemImageConfig>,
                    > = ::core::option::Option::None;
                    let mut __field4: ::core::option::Option<::std::option::Option<SystemOptions>> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "layer",
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
                                            "architecture",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<Architecture>(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "target",
                                        ),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<Target>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier3 => {
                                if ::core::option::Option::is_some(&__field3) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "image",
                                        ),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<SystemImageConfig>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier4 => {
                                if ::core::option::Option::is_some(&__field4) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "options",
                                        ),
                                    );
                                }
                                __field4 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<SystemOptions>,
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
                                <__A::Error as __serde::de::Error>::missing_field("layer"),
                            );
                        }
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("architecture"),
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
                    let __field4 = match __field4 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(SystemConfig {
                        layer: __field0,
                        architecture: __field1,
                        target: __field2,
                        image: __field3,
                        options: __field4,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] =
                &["layer", "architecture", "target", "image", "options"];
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
    #[doc = ""]
    #[derive(Clone, Debug)]
    pub struct SystemOptions {
        #[doc = "Use squashfs for system filesystems by default.\n"]
        pub use_squashfs: ::std::option::Option<super::images::SquashfsOptions>,
    }
    impl SystemOptions {
        #[doc = "Creates a new [`SystemOptions`]."]
        pub fn new() -> Self {
            Self {
                use_squashfs: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `use_squashfs`."]
        pub fn set_use_squashfs(
            &mut self,
            use_squashfs: ::std::option::Option<super::images::SquashfsOptions>,
        ) -> &mut Self {
            self.use_squashfs = use_squashfs;
            self
        }
        #[doc = "Sets the value of `use_squashfs`."]
        pub fn with_use_squashfs(
            mut self,
            use_squashfs: ::std::option::Option<super::images::SquashfsOptions>,
        ) -> Self {
            self.use_squashfs = use_squashfs;
            self
        }
    }
    impl ::std::default::Default for SystemOptions {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for SystemOptions {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "SystemOptions", 1usize)?;
            __record.serialize_optional_field(
                "use-squashfs",
                ::core::option::Option::as_ref(&self.use_squashfs),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for SystemOptions {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = SystemOptions;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record SystemOptions")
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
                        ::std::option::Option<super::images::SquashfsOptions>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(0usize, &"record with 1 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(SystemOptions {
                        use_squashfs: __field0,
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
                    const __IDENTIFIERS: &'static [&'static str] = &["use-squashfs"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"use-squashfs\"]";
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
                                "use-squashfs" => {
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
                                b"use-squashfs" => {
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
                        ::std::option::Option<super::images::SquashfsOptions>,
                    > = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "use-squashfs",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<super::images::SquashfsOptions>,
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
                    ::core::result::Result::Ok(SystemOptions {
                        use_squashfs: __field0,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["use-squashfs"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "SystemOptions",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "Architecture.\n"]
    #[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
    pub enum Architecture {
        #[doc = "64-bit x86.\n"]
        Amd64,
        #[doc = "64-bit ARMv8.\n"]
        Arm64,
        #[doc = "32-bit ARMv7.\n"]
        Armv7,
        #[doc = "32-bit ARMv6 (Hard-Float).\n"]
        Armhf,
        #[doc = "32-bit ARMv6.\n"]
        Arm,
    }
    #[automatically_derived]
    impl __serde::Serialize for Architecture {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let __serializer =
                __sidex_serde::ser::VariantSerializer::new(__serializer, "Architecture");
            match self {
                Self::Amd64 => __serializer.serialize_tag("amd64", 0u32),
                Self::Arm64 => __serializer.serialize_tag("arm64", 1u32),
                Self::Armv7 => __serializer.serialize_tag("armv7", 2u32),
                Self::Armhf => __serializer.serialize_tag("armhf", 3u32),
                Self::Arm => __serializer.serialize_tag("arm", 4u32),
            }
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Architecture {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] =
                &["amd64", "arm64", "armv7", "armhf", "arm"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str =
                "an identifier in [\"amd64\", \"arm64\", \"armv7\", \"armhf\", \"arm\"]";
            #[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
            #[doc(hidden)]
            enum __Identifier {
                __Identifier0,
                __Identifier1,
                __Identifier2,
                __Identifier3,
                __Identifier4,
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
                        "amd64" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "arm64" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "armv7" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "armhf" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        "arm" => ::core::result::Result::Ok(__Identifier::__Identifier4),
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
                        b"amd64" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"arm64" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"armv7" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        b"armhf" => ::core::result::Result::Ok(__Identifier::__Identifier3),
                        b"arm" => ::core::result::Result::Ok(__Identifier::__Identifier4),
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
                &["amd64", "arm64", "armv7", "armhf", "arm"];
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = Architecture;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "enum Architecture")
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
                            ::core::result::Result::Ok(Architecture::Amd64)
                        }
                        __Identifier::__Identifier1 => {
                            ::core::result::Result::Ok(Architecture::Arm64)
                        }
                        __Identifier::__Identifier2 => {
                            ::core::result::Result::Ok(Architecture::Armv7)
                        }
                        __Identifier::__Identifier3 => {
                            ::core::result::Result::Ok(Architecture::Armhf)
                        }
                        __Identifier::__Identifier4 => {
                            ::core::result::Result::Ok(Architecture::Arm)
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
                            ::core::result::Result::Ok(Architecture::Amd64)
                        }
                        (__Identifier::__Identifier1, __variant) => {
                            __serde::de::VariantAccess::unit_variant(__variant)?;
                            ::core::result::Result::Ok(Architecture::Arm64)
                        }
                        (__Identifier::__Identifier2, __variant) => {
                            __serde::de::VariantAccess::unit_variant(__variant)?;
                            ::core::result::Result::Ok(Architecture::Armv7)
                        }
                        (__Identifier::__Identifier3, __variant) => {
                            __serde::de::VariantAccess::unit_variant(__variant)?;
                            ::core::result::Result::Ok(Architecture::Armhf)
                        }
                        (__Identifier::__Identifier4, __variant) => {
                            __serde::de::VariantAccess::unit_variant(__variant)?;
                            ::core::result::Result::Ok(Architecture::Arm)
                        }
                    }
                }
            }
            __serde::Deserializer::deserialize_enum(
                __deserializer,
                "Architecture",
                __VARIANTS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "Target.\n"]
    #[derive(Clone, Debug)]
    pub enum Target {
        #[doc = "Generic target for EFI-compatible systems.\n"]
        GenericGrubEfi,
        #[doc = "Raspberry Pi-specific target using the `tryboot` mechanism.\n"]
        RpiTryboot,
        #[doc = "Raspberry Pi-specific target using U-Boot.\n"]
        RpiUboot,
        #[doc = "Target for unsupported devices.\n"]
        Unknown,
    }
    #[automatically_derived]
    impl __serde::Serialize for Target {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let __serializer = __sidex_serde::ser::VariantSerializer::new(__serializer, "Target");
            match self {
                Self::GenericGrubEfi => __serializer.serialize_tag("generic-grub-efi", 0u32),
                Self::RpiTryboot => __serializer.serialize_tag("rpi-tryboot", 1u32),
                Self::RpiUboot => __serializer.serialize_tag("rpi-uboot", 2u32),
                Self::Unknown => __serializer.serialize_tag("unknown", 3u32),
            }
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for Target {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] =
                &["generic-grub-efi", "rpi-tryboot", "rpi-uboot", "unknown"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS : & 'static str = "an identifier in [\"generic-grub-efi\", \"rpi-tryboot\", \"rpi-uboot\", \"unknown\"]" ;
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
                        "generic-grub-efi" => {
                            ::core::result::Result::Ok(__Identifier::__Identifier0)
                        }
                        "rpi-tryboot" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        "rpi-uboot" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        "unknown" => ::core::result::Result::Ok(__Identifier::__Identifier3),
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
                        b"generic-grub-efi" => {
                            ::core::result::Result::Ok(__Identifier::__Identifier0)
                        }
                        b"rpi-tryboot" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                        b"rpi-uboot" => ::core::result::Result::Ok(__Identifier::__Identifier2),
                        b"unknown" => ::core::result::Result::Ok(__Identifier::__Identifier3),
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
                &["generic-grub-efi", "rpi-tryboot", "rpi-uboot", "unknown"];
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = Target;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "enum Target")
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
                            ::core::result::Result::Ok(Target::GenericGrubEfi)
                        }
                        __Identifier::__Identifier1 => {
                            ::core::result::Result::Ok(Target::RpiTryboot)
                        }
                        __Identifier::__Identifier2 => ::core::result::Result::Ok(Target::RpiUboot),
                        __Identifier::__Identifier3 => ::core::result::Result::Ok(Target::Unknown),
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
                            ::core::result::Result::Ok(Target::GenericGrubEfi)
                        }
                        (__Identifier::__Identifier1, __variant) => {
                            __serde::de::VariantAccess::unit_variant(__variant)?;
                            ::core::result::Result::Ok(Target::RpiTryboot)
                        }
                        (__Identifier::__Identifier2, __variant) => {
                            __serde::de::VariantAccess::unit_variant(__variant)?;
                            ::core::result::Result::Ok(Target::RpiUboot)
                        }
                        (__Identifier::__Identifier3, __variant) => {
                            __serde::de::VariantAccess::unit_variant(__variant)?;
                            ::core::result::Result::Ok(Target::Unknown)
                        }
                    }
                }
            }
            __serde::Deserializer::deserialize_enum(
                __deserializer,
                "Target",
                __VARIANTS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = ""]
    #[derive(Clone, Debug)]
    pub struct SystemImageConfig {
        #[doc = "Size of the image.\n"]
        pub size: ::std::option::Option<super::foreign::NumBytes>,
        #[doc = "Layout of the image.\n"]
        pub layout: ::std::option::Option<super::images::ImageLayout>,
    }
    impl SystemImageConfig {
        #[doc = "Creates a new [`SystemImageConfig`]."]
        pub fn new() -> Self {
            Self {
                size: ::std::default::Default::default(),
                layout: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `size`."]
        pub fn set_size(
            &mut self,
            size: ::std::option::Option<super::foreign::NumBytes>,
        ) -> &mut Self {
            self.size = size;
            self
        }
        #[doc = "Sets the value of `size`."]
        pub fn with_size(mut self, size: ::std::option::Option<super::foreign::NumBytes>) -> Self {
            self.size = size;
            self
        }
        #[doc = "Sets the value of `layout`."]
        pub fn set_layout(
            &mut self,
            layout: ::std::option::Option<super::images::ImageLayout>,
        ) -> &mut Self {
            self.layout = layout;
            self
        }
        #[doc = "Sets the value of `layout`."]
        pub fn with_layout(
            mut self,
            layout: ::std::option::Option<super::images::ImageLayout>,
        ) -> Self {
            self.layout = layout;
            self
        }
    }
    impl ::std::default::Default for SystemImageConfig {
        fn default() -> Self {
            Self::new()
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for SystemImageConfig {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record = __sidex_serde::ser::RecordSerializer::new(
                __serializer,
                "SystemImageConfig",
                2usize,
            )?;
            __record
                .serialize_optional_field("size", ::core::option::Option::as_ref(&self.size))?;
            __record
                .serialize_optional_field("layout", ::core::option::Option::as_ref(&self.layout))?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for SystemImageConfig {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = SystemImageConfig;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record SystemImageConfig")
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
                        ::std::option::Option<super::foreign::NumBytes>,
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
                        ::std::option::Option<super::images::ImageLayout>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 2 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(SystemImageConfig {
                        size: __field0,
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
                    const __IDENTIFIERS: &'static [&'static str] = &["size", "layout"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"size\", \"layout\"]";
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
                                "size" => ::core::result::Result::Ok(__Identifier::__Identifier0),
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
                                b"size" => ::core::result::Result::Ok(__Identifier::__Identifier0),
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
                    let mut __field0: ::core::option::Option<
                        ::std::option::Option<super::foreign::NumBytes>,
                    > = ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<
                        ::std::option::Option<super::images::ImageLayout>,
                    > = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("size"),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<super::foreign::NumBytes>,
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
                                        ::std::option::Option<super::images::ImageLayout>,
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
                    ::core::result::Result::Ok(SystemImageConfig {
                        size: __field0,
                        layout: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["size", "layout"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "SystemImageConfig",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
}
pub mod tests {
    #![doc = "Test configuration.\n"]
    #[allow(unused)]
    use :: serde as __serde;
    #[allow(unused)]
    use :: sidex_serde as __sidex_serde;
    #[doc = "Test configuration.\n"]
    #[derive(Clone, Debug)]
    pub struct TestConfig {
        #[doc = "Systems to run the test on.\n"]
        pub systems: ::std::vec::Vec<SystemConfig>,
        #[doc = "Steps of the test.\n"]
        pub steps: ::std::vec::Vec<TestStep>,
    }
    impl TestConfig {
        #[doc = "Creates a new [`TestConfig`]."]
        pub fn new(
            systems: ::std::vec::Vec<SystemConfig>,
            steps: ::std::vec::Vec<TestStep>,
        ) -> Self {
            Self { systems, steps }
        }
        #[doc = "Sets the value of `systems`."]
        pub fn set_systems(&mut self, systems: ::std::vec::Vec<SystemConfig>) -> &mut Self {
            self.systems = systems;
            self
        }
        #[doc = "Sets the value of `systems`."]
        pub fn with_systems(mut self, systems: ::std::vec::Vec<SystemConfig>) -> Self {
            self.systems = systems;
            self
        }
        #[doc = "Sets the value of `steps`."]
        pub fn set_steps(&mut self, steps: ::std::vec::Vec<TestStep>) -> &mut Self {
            self.steps = steps;
            self
        }
        #[doc = "Sets the value of `steps`."]
        pub fn with_steps(mut self, steps: ::std::vec::Vec<TestStep>) -> Self {
            self.steps = steps;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for TestConfig {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "TestConfig", 2usize)?;
            __record.serialize_field("systems", &self.systems)?;
            __record.serialize_field("steps", &self.steps)?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for TestConfig {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = TestConfig;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record TestConfig")
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
                        ::std::vec::Vec<SystemConfig>,
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
                        ::std::vec::Vec<TestStep>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 2 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(TestConfig {
                        systems: __field0,
                        steps: __field1,
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
                    const __IDENTIFIERS: &'static [&'static str] = &["systems", "steps"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"systems\", \"steps\"]";
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
                                "systems" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                "steps" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
                                b"systems" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                b"steps" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
                    let mut __field0: ::core::option::Option<::std::vec::Vec<SystemConfig>> =
                        ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::std::vec::Vec<TestStep>> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "systems",
                                        ),
                                    );
                                }
                                __field0 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::vec::Vec<SystemConfig>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier1 => {
                                if ::core::option::Option::is_some(&__field1) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "steps",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<::std::vec::Vec<TestStep>>(
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
                                <__A::Error as __serde::de::Error>::missing_field("systems"),
                            );
                        }
                    };
                    let __field1 = match __field1 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("steps"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(TestConfig {
                        systems: __field0,
                        steps: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["systems", "steps"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "TestConfig",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "System configuration for testing.\n"]
    #[derive(Clone, Debug)]
    pub struct SystemConfig {
        #[doc = "Image to use.\n"]
        pub system: ::std::string::String,
        #[doc = "Size of the disk.\n"]
        pub disk_size: ::std::option::Option<super::foreign::NumBytes>,
        #[doc = "SSH configuration.\n"]
        pub ssh: ::std::option::Option<SshConfig>,
    }
    impl SystemConfig {
        #[doc = "Creates a new [`SystemConfig`]."]
        pub fn new(system: ::std::string::String) -> Self {
            Self {
                system,
                disk_size: ::std::default::Default::default(),
                ssh: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `system`."]
        pub fn set_system(&mut self, system: ::std::string::String) -> &mut Self {
            self.system = system;
            self
        }
        #[doc = "Sets the value of `system`."]
        pub fn with_system(mut self, system: ::std::string::String) -> Self {
            self.system = system;
            self
        }
        #[doc = "Sets the value of `disk_size`."]
        pub fn set_disk_size(
            &mut self,
            disk_size: ::std::option::Option<super::foreign::NumBytes>,
        ) -> &mut Self {
            self.disk_size = disk_size;
            self
        }
        #[doc = "Sets the value of `disk_size`."]
        pub fn with_disk_size(
            mut self,
            disk_size: ::std::option::Option<super::foreign::NumBytes>,
        ) -> Self {
            self.disk_size = disk_size;
            self
        }
        #[doc = "Sets the value of `ssh`."]
        pub fn set_ssh(&mut self, ssh: ::std::option::Option<SshConfig>) -> &mut Self {
            self.ssh = ssh;
            self
        }
        #[doc = "Sets the value of `ssh`."]
        pub fn with_ssh(mut self, ssh: ::std::option::Option<SshConfig>) -> Self {
            self.ssh = ssh;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for SystemConfig {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "SystemConfig", 3usize)?;
            __record.serialize_field("system", &self.system)?;
            __record.serialize_optional_field(
                "disk-size",
                ::core::option::Option::as_ref(&self.disk_size),
            )?;
            __record.serialize_optional_field("ssh", ::core::option::Option::as_ref(&self.ssh))?;
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
                        ::std::string::String,
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
                        ::std::option::Option<super::foreign::NumBytes>,
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
                        ::std::option::Option<SshConfig>,
                    >(&mut __seq)?
                    {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(2usize, &"record with 3 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(SystemConfig {
                        system: __field0,
                        disk_size: __field1,
                        ssh: __field2,
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
                    const __IDENTIFIERS: &'static [&'static str] = &["system", "disk-size", "ssh"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"system\", \"disk-size\", \"ssh\"]";
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
                                "system" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                                "disk-size" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                "ssh" => ::core::result::Result::Ok(__Identifier::__Identifier2),
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
                                b"system" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                b"disk-size" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                b"ssh" => ::core::result::Result::Ok(__Identifier::__Identifier2),
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
                        ::std::option::Option<super::foreign::NumBytes>,
                    > = ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<::std::option::Option<SshConfig>> =
                        ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "system",
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
                                            "disk-size",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<super::foreign::NumBytes>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier2 => {
                                if ::core::option::Option::is_some(&__field2) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field("ssh"),
                                    );
                                }
                                __field2 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<SshConfig>,
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
                                <__A::Error as __serde::de::Error>::missing_field("system"),
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
                    ::core::result::Result::Ok(SystemConfig {
                        system: __field0,
                        disk_size: __field1,
                        ssh: __field2,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["system", "disk-size", "ssh"];
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
    #[doc = "SSH configuration.\n"]
    #[derive(Clone, Debug)]
    pub struct SshConfig {
        #[doc = "Path to the private key.\n"]
        pub private_key: ::std::string::String,
    }
    impl SshConfig {
        #[doc = "Creates a new [`SshConfig`]."]
        pub fn new(private_key: ::std::string::String) -> Self {
            Self { private_key }
        }
        #[doc = "Sets the value of `private_key`."]
        pub fn set_private_key(&mut self, private_key: ::std::string::String) -> &mut Self {
            self.private_key = private_key;
            self
        }
        #[doc = "Sets the value of `private_key`."]
        pub fn with_private_key(mut self, private_key: ::std::string::String) -> Self {
            self.private_key = private_key;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for SshConfig {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "SshConfig", 1usize)?;
            __record.serialize_field("private-key", &self.private_key)?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for SshConfig {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = SshConfig;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record SshConfig")
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
                    ::core::result::Result::Ok(SshConfig {
                        private_key: __field0,
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
                    const __IDENTIFIERS: &'static [&'static str] = &["private-key"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"private-key\"]";
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
                                "private-key" => {
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
                                b"private-key" => {
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
                                            "private-key",
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
                                <__A::Error as __serde::de::Error>::missing_field("private-key"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(SshConfig {
                        private_key: __field0,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["private-key"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "SshConfig",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "Test step.\n"]
    #[derive(Clone, Debug)]
    pub enum TestStep {
        #[doc = "Run a script on the SUT.\n"]
        Run(RunStep),
        #[doc = "Wait for a given amount of time.\n"]
        Wait(WaitStep),
    }
    #[automatically_derived]
    impl __serde::Serialize for TestStep {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let __serializer = __sidex_serde::ser::VariantSerializer::new(__serializer, "TestStep");
            match self {
                Self::Run(__value) => {
                    __serializer.serialize_internally_tagged("action", "run", 0u32, __value)
                }
                Self::Wait(__value) => {
                    __serializer.serialize_internally_tagged("action", "wait", 1u32, __value)
                }
            }
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for TestStep {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            const __IDENTIFIERS: &'static [&'static str] = &["run", "wait"];
            #[doc(hidden)]
            const __EXPECTING_IDENTIFIERS: &'static str = "an identifier in [\"run\", \"wait\"]";
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
                        "run" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        "wait" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
                        b"run" => ::core::result::Result::Ok(__Identifier::__Identifier0),
                        b"wait" => ::core::result::Result::Ok(__Identifier::__Identifier1),
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
            const __VARIANTS: &'static [&'static str] = &["run", "wait"];
            if __serde::Deserializer::is_human_readable(&__deserializer) {
                let __tagged = __sidex_serde::de::tagged::deserialize_tagged_variant::<
                    __Identifier,
                    __D,
                >(__deserializer, "action")?;
                match __tagged.tag {
                    __Identifier::__Identifier0 => ::core::result::Result::Ok(TestStep::Run(
                        __tagged.deserialize_internally_tagged::<RunStep, __D::Error>()?,
                    )),
                    __Identifier::__Identifier1 => ::core::result::Result::Ok(TestStep::Wait(
                        __tagged.deserialize_internally_tagged::<WaitStep, __D::Error>()?,
                    )),
                }
            } else {
                #[doc(hidden)]
                struct __Visitor {
                    __phantom_vars: ::core::marker::PhantomData<fn(&())>,
                }
                impl<'de> __serde::de::Visitor<'de> for __Visitor {
                    type Value = TestStep;
                    fn expecting(
                        &self,
                        __formatter: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(__formatter, "enum TestStep")
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
                                let __value = __serde::de::VariantAccess::newtype_variant::<RunStep>(
                                    __variant,
                                )?;
                                ::core::result::Result::Ok(TestStep::Run(__value))
                            }
                            (__Identifier::__Identifier1, __variant) => {
                                let __value = __serde::de::VariantAccess::newtype_variant::<
                                    WaitStep,
                                >(__variant)?;
                                ::core::result::Result::Ok(TestStep::Wait(__value))
                            }
                        }
                    }
                }
                __serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "TestStep",
                    __VARIANTS,
                    __Visitor {
                        __phantom_vars: ::core::marker::PhantomData,
                    },
                )
            }
        }
    }
    #[doc = "Run step.\n"]
    #[derive(Clone, Debug)]
    pub struct RunStep {
        #[doc = "Description of the step.\n"]
        pub description: ::std::option::Option<::std::string::String>,
        #[doc = "Script to run.\n"]
        pub script: ::std::string::String,
        #[doc = "File to provide on the standard input to the script.\n"]
        pub stdin_file: ::std::option::Option<::std::string::String>,
        #[doc = "Do no treat SSH disconnects as failures.\n"]
        pub may_disconnect: ::std::option::Option<bool>,
        #[doc = "Do not treat non-zero exit code as failures.\n"]
        pub may_fail: ::std::option::Option<bool>,
    }
    impl RunStep {
        #[doc = "Creates a new [`RunStep`]."]
        pub fn new(script: ::std::string::String) -> Self {
            Self {
                script,
                description: ::std::default::Default::default(),
                stdin_file: ::std::default::Default::default(),
                may_disconnect: ::std::default::Default::default(),
                may_fail: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `description`."]
        pub fn set_description(
            &mut self,
            description: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn with_description(
            mut self,
            description: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `script`."]
        pub fn set_script(&mut self, script: ::std::string::String) -> &mut Self {
            self.script = script;
            self
        }
        #[doc = "Sets the value of `script`."]
        pub fn with_script(mut self, script: ::std::string::String) -> Self {
            self.script = script;
            self
        }
        #[doc = "Sets the value of `stdin_file`."]
        pub fn set_stdin_file(
            &mut self,
            stdin_file: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.stdin_file = stdin_file;
            self
        }
        #[doc = "Sets the value of `stdin_file`."]
        pub fn with_stdin_file(
            mut self,
            stdin_file: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.stdin_file = stdin_file;
            self
        }
        #[doc = "Sets the value of `may_disconnect`."]
        pub fn set_may_disconnect(
            &mut self,
            may_disconnect: ::std::option::Option<bool>,
        ) -> &mut Self {
            self.may_disconnect = may_disconnect;
            self
        }
        #[doc = "Sets the value of `may_disconnect`."]
        pub fn with_may_disconnect(mut self, may_disconnect: ::std::option::Option<bool>) -> Self {
            self.may_disconnect = may_disconnect;
            self
        }
        #[doc = "Sets the value of `may_fail`."]
        pub fn set_may_fail(&mut self, may_fail: ::std::option::Option<bool>) -> &mut Self {
            self.may_fail = may_fail;
            self
        }
        #[doc = "Sets the value of `may_fail`."]
        pub fn with_may_fail(mut self, may_fail: ::std::option::Option<bool>) -> Self {
            self.may_fail = may_fail;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for RunStep {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "RunStep", 5usize)?;
            __record.serialize_optional_field(
                "description",
                ::core::option::Option::as_ref(&self.description),
            )?;
            __record.serialize_field("script", &self.script)?;
            __record.serialize_optional_field(
                "stdin-file",
                ::core::option::Option::as_ref(&self.stdin_file),
            )?;
            __record.serialize_optional_field(
                "may-disconnect",
                ::core::option::Option::as_ref(&self.may_disconnect),
            )?;
            __record.serialize_optional_field(
                "may-fail",
                ::core::option::Option::as_ref(&self.may_fail),
            )?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for RunStep {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = RunStep;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record RunStep")
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
                                __serde::de::Error::invalid_length(0usize, &"record with 5 fields"),
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
                                __serde::de::Error::invalid_length(1usize, &"record with 5 fields"),
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
                                __serde::de::Error::invalid_length(2usize, &"record with 5 fields"),
                            );
                        }
                    };
                    let __field3 = match __serde::de::SeqAccess::next_element::<
                        ::std::option::Option<bool>,
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
                    ::core::result::Result::Ok(RunStep {
                        description: __field0,
                        script: __field1,
                        stdin_file: __field2,
                        may_disconnect: __field3,
                        may_fail: __field4,
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
                        "description",
                        "script",
                        "stdin-file",
                        "may-disconnect",
                        "may-fail",
                    ];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS : & 'static str = "an identifier in [\"description\", \"script\", \"stdin-file\", \"may-disconnect\", \"may-fail\"]" ;
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
                                "description" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                "script" => ::core::result::Result::Ok(__Identifier::__Identifier1),
                                "stdin-file" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier2)
                                }
                                "may-disconnect" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier3)
                                }
                                "may-fail" => {
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
                                b"description" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                b"script" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier1)
                                }
                                b"stdin-file" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier2)
                                }
                                b"may-disconnect" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier3)
                                }
                                b"may-fail" => {
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
                        ::std::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<::std::string::String> =
                        ::core::option::Option::None;
                    let mut __field2: ::core::option::Option<
                        ::std::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field3: ::core::option::Option<::std::option::Option<bool>> =
                        ::core::option::Option::None;
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
                                            "description",
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
                                            "script",
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
                                            "stdin-file",
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
                                            "may-disconnect",
                                        ),
                                    );
                                }
                                __field3 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<
                                        ::std::option::Option<bool>,
                                    >(&mut __map)?,
                                );
                            }
                            __Identifier::__Identifier4 => {
                                if ::core::option::Option::is_some(&__field4) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "may-fail",
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
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("script"),
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
                    let __field4 = match __field4 {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => ::core::option::Option::None,
                    };
                    ::core::result::Result::Ok(RunStep {
                        description: __field0,
                        script: __field1,
                        stdin_file: __field2,
                        may_disconnect: __field3,
                        may_fail: __field4,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &[
                "description",
                "script",
                "stdin-file",
                "may-disconnect",
                "may-fail",
            ];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "RunStep",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
    #[doc = "Wait step.\n"]
    #[derive(Clone, Debug)]
    pub struct WaitStep {
        #[doc = "Description of the step.\n"]
        pub description: ::std::option::Option<::std::string::String>,
        #[doc = "Time to wait in seconds.\n"]
        pub duration: f64,
    }
    impl WaitStep {
        #[doc = "Creates a new [`WaitStep`]."]
        pub fn new(duration: f64) -> Self {
            Self {
                duration,
                description: ::std::default::Default::default(),
            }
        }
        #[doc = "Sets the value of `description`."]
        pub fn set_description(
            &mut self,
            description: ::std::option::Option<::std::string::String>,
        ) -> &mut Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `description`."]
        pub fn with_description(
            mut self,
            description: ::std::option::Option<::std::string::String>,
        ) -> Self {
            self.description = description;
            self
        }
        #[doc = "Sets the value of `duration`."]
        pub fn set_duration(&mut self, duration: f64) -> &mut Self {
            self.duration = duration;
            self
        }
        #[doc = "Sets the value of `duration`."]
        pub fn with_duration(mut self, duration: f64) -> Self {
            self.duration = duration;
            self
        }
    }
    #[automatically_derived]
    impl __serde::Serialize for WaitStep {
        fn serialize<__S: __serde::Serializer>(
            &self,
            __serializer: __S,
        ) -> ::std::result::Result<__S::Ok, __S::Error> {
            let mut __record =
                __sidex_serde::ser::RecordSerializer::new(__serializer, "WaitStep", 2usize)?;
            __record.serialize_optional_field(
                "description",
                ::core::option::Option::as_ref(&self.description),
            )?;
            __record.serialize_field("duration", &self.duration)?;
            __record.end()
        }
    }
    #[automatically_derived]
    impl<'de> __serde::Deserialize<'de> for WaitStep {
        fn deserialize<__D: __serde::Deserializer<'de>>(
            __deserializer: __D,
        ) -> ::std::result::Result<Self, __D::Error> {
            #[doc(hidden)]
            struct __Visitor {
                __phantom_vars: ::core::marker::PhantomData<fn(&())>,
            }
            impl<'de> __serde::de::Visitor<'de> for __Visitor {
                type Value = WaitStep;
                fn expecting(
                    &self,
                    __formatter: &mut ::core::fmt::Formatter,
                ) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(__formatter, "record WaitStep")
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
                                __serde::de::Error::invalid_length(0usize, &"record with 2 fields"),
                            );
                        }
                    };
                    let __field1 = match __serde::de::SeqAccess::next_element::<f64>(&mut __seq)? {
                        ::core::option::Option::Some(__value) => __value,
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                __serde::de::Error::invalid_length(1usize, &"record with 2 fields"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(WaitStep {
                        description: __field0,
                        duration: __field1,
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
                    const __IDENTIFIERS: &'static [&'static str] = &["description", "duration"];
                    #[doc(hidden)]
                    const __EXPECTING_IDENTIFIERS: &'static str =
                        "an identifier in [\"description\", \"duration\"]";
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
                                "description" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                "duration" => {
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
                                b"description" => {
                                    ::core::result::Result::Ok(__Identifier::__Identifier0)
                                }
                                b"duration" => {
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
                    let mut __field0: ::core::option::Option<
                        ::std::option::Option<::std::string::String>,
                    > = ::core::option::Option::None;
                    let mut __field1: ::core::option::Option<f64> = ::core::option::Option::None;
                    while let ::core::option::Option::Some(__key) =
                        __serde::de::MapAccess::next_key::<__Identifier>(&mut __map)?
                    {
                        match __key {
                            __Identifier::__Identifier0 => {
                                if ::core::option::Option::is_some(&__field0) {
                                    return ::core::result::Result::Err(
                                        <__A::Error as __serde::de::Error>::duplicate_field(
                                            "description",
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
                                            "duration",
                                        ),
                                    );
                                }
                                __field1 = ::core::option::Option::Some(
                                    __serde::de::MapAccess::next_value::<f64>(&mut __map)?,
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
                        ::core::option::Option::None => {
                            return ::core::result::Result::Err(
                                <__A::Error as __serde::de::Error>::missing_field("duration"),
                            );
                        }
                    };
                    ::core::result::Result::Ok(WaitStep {
                        description: __field0,
                        duration: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const __FIELDS: &'static [&'static str] = &["description", "duration"];
            __serde::Deserializer::deserialize_struct(
                __deserializer,
                "WaitStep",
                __FIELDS,
                __Visitor {
                    __phantom_vars: ::core::marker::PhantomData,
                },
            )
        }
    }
}
