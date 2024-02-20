use heck::{ToShoutySnakeCase, ToSnakeCase};
use proc_macro::{self, TokenStream};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{
    parse_macro_input, Field, Fields, FieldsUnnamed, File, Ident, Item, ItemEnum, ItemStruct, Path,
    Type, TypePath,
};

/// Derives an enum `BlockId` whose variants are the same as the names of the blocks passed to this
/// macro. Also derives `Block::id`.
///
/// # Panics
///
/// Panics if the provided token stream consists of items other than structs.
#[allow(clippy::too_many_lines)]
#[proc_macro]
pub fn blocks(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as File);
    let items = input.items.iter().map(|item| match item {
        Item::Struct(item_struct) => item_struct,
        _ => panic!("expected a struct"),
    });
    let block_count = input.items.len();
    let max_id = u16::try_from(block_count - 1).unwrap();
    let ident1 = items.clone().map(|ItemStruct { ident, .. }| ident);
    let ident2 = ident1.clone();
    let screaming_ident = ident1
        .clone()
        .map(|ident| Ident::new(&ident.to_string().to_shouty_snake_case(), Span::call_site()));
    let property_names = items.clone().map(|ItemStruct { fields, .. }| {
        let property_name = fields.iter().map(property_name);

        quote! { #(#property_name,)* }
    });
    let property_definitions = items.clone().map(|ItemStruct { fields, .. }| {
        let property_name = fields.iter().map(property_name);
        let property_type1 = fields.iter().map(|field| &field.ty);
        let property_type2 = property_type1.clone();
        let offset = (0..fields.len()).map(|i| {
            let property_type = property_type2.clone().take(i);

            quote! { (16 #(+ <#property_type>::BITS)*) }
        });

        quote! {
            #(
                state::PropertyDefinition {
                    name: #property_name,
                    id: <#property_type1>::ID,
                    offset: #offset,
                },
            )*
        }
    });
    let numeric_id = 0..=max_id;
    let string_id1 = ident1
        .clone()
        .map(|ident| ident.to_string().to_snake_case());
    let string_id2 = string_id1.clone();

    quote! {
        /// Block IDs. This enum is automatically derived by the `blocks!` proc macro.
        #[derive(Clone, Copy, Debug)]
        #[repr(u16)]
        pub enum BlockId {
            #(#ident1 = #numeric_id,)*
        }

        impl BlockId {
            /// The number of block IDs.
            const COUNT: usize = #block_count;

            /// The maximum numeric ID.
            const MAX: u16 = #max_id;

            /// Converts the given resource location to a `BlockId`.
            #[must_use]
            pub fn from_location(s: &str) -> Option<Self> {
                // NOTE: This adds compilation overhead because we are involving another proc
                // macro, but in return this is currently 22x faster than using a `match`. Another
                // option would be to use `LazyLock<HashMap<_, _>>` which is very slightly slower.
                static LUT: ::phf::Map<&'static str, BlockId> = ::phf::phf_map! {
                    #(#string_id1 => BlockId::#ident2,)*
                };

                LUT.get(s).copied()
            }

            /// Converts the `BlockId` to its resource location representation.
            #[must_use]
            pub const fn to_location(self) -> &'static str {
                // NOTE: We declare a lookup table manually so that the compiler doesn't struggle
                // with exhaustiveness checks so much. This is what a `match` would desugar down to
                // anyway.
                const LUT: [&'static str; BlockId::COUNT] = [
                    #(#string_id2,)*
                ];

                LUT[self as usize]
            }

            /// Returns the properties of the block.
            #[must_use]
            pub const fn properties(self) -> &'static behavior::Properties {
                const LUT: [behavior::Properties; BlockId::COUNT] = [
                    #(behavior::#screaming_ident,)*
                ];

                &LUT[self as usize]
            }

            /// Returns the state definition of the block.
            #[must_use]
            pub const fn state_definition(self) -> &'static state::Definition {
                const LUT: [state::Definition; BlockId::COUNT] = [
                    #(
                        state::Definition {
                            property_names: &[#property_names],
                            properties: &[#property_definitions],
                        },
                    )*
                ];

                &LUT[self as usize]
            }
        }
    }
    .into()
}

fn property_name(field: &Field) -> String {
    let name = field
        .ident
        .as_ref()
        .expect("expected a named field")
        .to_string();

    if &name == "ty" {
        "type".to_owned()
    } else {
        name
    }
}

#[allow(clippy::too_many_lines)]
#[proc_macro]
pub fn properties(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as File);
    let mut types = TokenStream2::new();
    let mut item_idents = Vec::with_capacity(input.items.len());

    for item in &input.items {
        match item {
            Item::Enum(item) => {
                types.extend(quote! {
                    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
                    #item
                });
                types.extend(impl_enum_property(item));
                item_idents.push(&item.ident);
            }
            Item::Struct(item) => {
                let ident = &item.ident;
                let Fields::Unnamed(FieldsUnnamed { unnamed, .. }) = &item.fields else {
                    panic!("expected a struct with unnamed fields");
                };
                let Type::Path(TypePath { path, .. }) = &unnamed.first().unwrap().ty else {
                    panic!("expected a path");
                };

                types.extend(quote! {
                    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
                    #[repr(transparent)]
                    #item
                });

                if path.is_ident("bool") {
                    types.extend(impl_boolean_property(ident));
                } else {
                    types.extend(impl_integer_property(ident, path));
                }

                item_idents.push(ident);
            }
            _ => panic!("expected an enum or struct"),
        }
    }

    let ident1 = item_idents.iter();
    let ident2 = ident1.clone();
    let ident3 = ident1.clone();
    let ident4 = ident1.clone();
    let ident5 = ident1.clone();
    let ident6 = ident1.clone();
    let ident7 = ident1.clone();
    let ident8 = ident1.clone();
    let ident9 = ident1.clone();

    quote! {
        #types

        /// Block state properties. This enum is automatically derived by the `properties!` proc
        /// macro.
        #[derive(Clone, Copy, Debug)]
        pub enum Property {
            #(#ident1(#ident1),)*
        }

        impl Property {
            pub fn from_str(id: PropertyId, s: &str) -> ::anyhow::Result<Self> {
                match id {
                    #(
                        PropertyId::#ident2 => {
                            ::std::str::FromStr::from_str(s).map(Self::#ident2).map_err(Into::into)
                        }
                    )*
                }
            }

            #[must_use]
            pub fn from_bits(id: PropertyId, bits: u32) -> Option<Self> {
                match id {
                    #(PropertyId::#ident3 => #ident3::from_bits(bits).map(Self::#ident3),)*
                }
            }

            #[must_use]
            pub fn to_bits(self) -> u32 {
                match self {
                    #(Self::#ident4(property) => property.to_bits(),)*
                }
            }

            #[must_use]
            pub fn id(&self) -> PropertyId {
                match self {
                    #(Self::#ident5(_) => PropertyId::#ident5,)*
                }
            }
        }

        /// Block state property IDs. This enum is automatically derived by the `properties!` proc
        /// macro.
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum PropertyId {
            #(#ident6,)*
        }

        impl PropertyId {
            #[must_use]
            pub const fn ty(self) -> PropertyType {
                match self {
                    #(Self::#ident7 => #ident7::TYPE,)*
                }
            }

            /// Returns the number of bits needed to represent this property.
            #[must_use]
            pub const fn bits(self) -> u32 {
                match self {
                    #(Self::#ident8 => u32::BITS - (#ident8::MAX - #ident8::MIN).leading_zeros(),)*
                }
            }
        }

        impl<'de> ::serde::de::DeserializeSeed<'de> for PropertyId {
            type Value = Property;

            fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                match self {
                    #(
                        PropertyId::#ident9 => {
                            ::serde::de::Deserialize::deserialize(deserializer)
                                .map(Property::#ident9)
                        }
                    )*
                }
            }
        }
    }
    .into()
}

fn impl_boolean_property(ident: &Ident) -> TokenStream2 {
    quote! {
        impl #ident {
            pub const ID: PropertyId = PropertyId::#ident;

            pub const TYPE: PropertyType = PropertyType::Boolean;

            pub const MIN: u32 = 0;

            pub const MAX: u32 = 1;

            pub const BITS: u32 = 2;

            #[must_use]
            pub fn from_bits(bits: u32) -> Option<Self> {
                match bits {
                    0 => Some(Self(false)),
                    1 => Some(Self(true)),
                    _ => None,
                }
            }

            #[must_use]
            pub fn to_bits(self) -> u32 {
                match self {
                    Self(false) => 0,
                    Self(true) => 1,
                }
            }
        }

        impl<'de> ::serde::de::Deserialize<'de> for #ident {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                struct PropertyVisitor;

                impl<'de> ::serde::de::Visitor<'de> for PropertyVisitor {
                    type Value = #ident;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str("a string or a boolean")
                    }

                    fn visit_bool<E>(self, b: bool) -> Result<Self::Value, E>
                    where
                        E: ::serde::de::Error,
                    {
                        Ok(#ident(b))
                    }

                    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
                    where
                        E: ::serde::de::Error,
                    {
                        let b = match s {
                            "false" => false,
                            "true" => true,
                            _ => return Err(::serde::de::Error::custom(format_args!(
                                r#"invalid value: string {:?}, expected `"false"` or `"true"`"#,
                                s,
                            ))),
                        };

                        self.visit_bool(b)
                    }
                }

                deserializer.deserialize_any(PropertyVisitor)
            }
        }

        impl ::std::str::FromStr for #ident {
            type Err = ::anyhow::Error;

            fn from_str(s: &str) -> ::anyhow::Result<Self> {
                s.parse().map(Self).map_err(Into::into)
            }
        }
    }
}

fn impl_integer_property(ident: &Ident, path: &Path) -> TokenStream2 {
    quote! {
        impl #ident {
            pub const ID: PropertyId = PropertyId::#ident;

            pub const TYPE: PropertyType = PropertyType::Integer;

            pub const MIN: u32 = <#path>::MIN;

            pub const MAX: u32 = <#path>::MAX;

            pub const BITS: u32 =
                u32::BITS - (Self::MAX - Self::MIN).leading_zeros();

            #[must_use]
            pub fn from_bits(bits: u32) -> Option<Self> {
                Integer::new(bits + <#path>::MIN).map(Self).ok()
            }

            #[must_use]
            pub fn to_bits(self) -> u32 {
                // The value is stored in the range [0, MAX - MIN] instead of
                // [MIN, MAX] so that a block whose state is initialized to all
                // zeros is valid and the default state.
                self.0.0 - <#path>::MIN
            }
        }

        impl<'de> ::serde::de::Deserialize<'de> for #ident {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                struct PropertyVisitor;

                impl<'de> ::serde::de::Visitor<'de> for PropertyVisitor {
                    type Value = #ident;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str("a string or an integer")
                    }

                    fn visit_u64<E>(self, n: u64) -> Result<Self::Value, E>
                    where
                        E: ::serde::de::Error,
                    {
                        if n >= u64::from(#ident::MIN) && n <= u64::from(#ident::MAX) {
                            Ok(#ident(Integer(n as u32)))
                        } else {
                            Err(::serde::de::Error::custom(format_args!(
                                "invalid value: integer `{}`, expected an integer between `{}` \
                                and `{}`",
                                n, #ident::MIN, #ident::MAX,
                            )))
                        }
                    }

                    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
                    where
                        E: ::serde::de::Error,
                    {
                        let err = || {
                            ::serde::de::Error::custom(format_args!(
                                "invalid value: string {:?}, expected a string representing an \
                                integer between `{}` and `{}`",
                                s, #ident::MIN, #ident::MAX,
                            ))
                        };

                        let n = s.parse().map_err(|_| err())?;

                        self.visit_u64(n).map_err(|_: E| err())
                    }
                }

                deserializer.deserialize_any(PropertyVisitor)
            }
        }

        impl ::std::str::FromStr for #ident {
            type Err = ::anyhow::Error;

            fn from_str(s: &str) -> ::anyhow::Result<Self> {
                s.parse().map(Self).map_err(Into::into)
            }
        }
    }
}

fn impl_enum_property(item: &ItemEnum) -> TokenStream2 {
    let ident = &item.ident;
    let max_variant = u32::try_from(item.variants.len() - 1).unwrap();
    let numeric_variant = 0..=max_variant;
    let numeric_variant2 = numeric_variant.clone();
    let variant1 = item.variants.iter().map(|variant| &variant.ident);
    let variant2 = variant1.clone();
    let variant3 = variant1.clone();
    let variant4 = variant1.clone();
    let string_variant1 = variant1
        .clone()
        .map(|ident| ident.to_string().to_snake_case());
    let string_variant2 = string_variant1.clone();
    let string_variant3 = string_variant1.clone();

    quote! {
        impl #ident {
            pub const ID: PropertyId = PropertyId::#ident;

            pub const TYPE: PropertyType = PropertyType::Enum;

            pub const MIN: u32 = 0;

            pub const MAX: u32 = #max_variant;

            pub const BITS: u32 = u32::BITS - Self::MAX.leading_zeros();

            #[must_use]
            pub fn from_bits(bits: u32) -> Option<Self> {
                match bits {
                    #(#numeric_variant => Some(Self::#variant1),)*
                    _ => None,
                }
            }

            #[must_use]
            pub fn to_bits(self) -> u32 {
                match self {
                    #(Self::#variant2 => #numeric_variant2,)*
                }
            }
        }

        impl<'de> ::serde::de::Deserialize<'de> for #ident {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                struct PropertyVisitor;

                impl<'de> ::serde::de::Visitor<'de> for PropertyVisitor {
                    type Value = #ident;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str("a string")
                    }

                    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
                    where
                        E: ::serde::de::Error,
                    {
                        match s {
                            #(#string_variant1 => Ok(#ident::#variant3),)*
                            _ => Err(::serde::de::Error::unknown_variant(s, VARIANTS)),
                        }
                    }
                }

                const VARIANTS: &[&str] = &[ #(#string_variant2,)* ];

                deserializer.deserialize_str(PropertyVisitor)
            }
        }

        impl ::std::str::FromStr for #ident {
            type Err = ::anyhow::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #(#string_variant3 => Ok(#ident::#variant4),)*
                    _ => Err(::anyhow::Error::msg("Matching variant not found")),
                }
            }
        }
    }
}
