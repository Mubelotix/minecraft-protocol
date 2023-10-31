extern crate proc_macro;
use proc_macro::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, punctuated::Punctuated, Data, DeriveInput, Expr, Fields, Lit, LitInt, ExprUnary, UnOp, token::Sub
};

#[proc_macro_derive(MinecraftPacketPart, attributes(discriminant, value))]
pub fn minecraft_packet_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let (lifetime_impl, lifetime_struct, lifetime) =
        match input.generics.lifetimes().collect::<Vec<&_>>() {
            lifetimes if lifetimes.is_empty() => (quote! {<'a>}, None, None),
            lifetimes if lifetimes.len() == 1 => {
                let lifetime = lifetimes[0].lifetime.clone();
                (
                    quote! {<#lifetime>},
                    Some(quote! {<#lifetime>}),
                    Some(quote! {#lifetime}),
                )
            }
            _ => return quote!(compile_error!("Too many lifetimes");).into(),
        };

    let name = input.ident;

    match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => {
                let fields = fields.named.into_iter().map(|field| field.ident.unwrap());
                let fields2 = fields.clone();
                let fields3 = fields.clone();

                quote! {
                    #[automatically_derived]
                    impl#lifetime_impl MinecraftPacketPart#lifetime_impl for #name#lifetime_struct {
                        fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
                            #(self.#fields.serialize_minecraft_packet_part(output)?;)*
                            Ok(())
                        }
                        
                        fn deserialize_minecraft_packet_part(input: &#lifetime [u8]) -> Result<(Self, &#lifetime [u8]), &'static str> {
                            #(let (#fields2, input) = MinecraftPacketPart::deserialize_minecraft_packet_part(input)?;)*
                            Ok((#name {
                                #(#fields3,)*
                            }, input))
                        }
                    }
                }
            }
            Fields::Unnamed(_) => todo!("unnamed fields"),
            Fields::Unit => panic!("how did you put a variant in a struct??"),
        }.into(),
        Data::Enum(variants) => {
            let variants = variants.variants;
            let mut tag_type_string = "u8".to_string();
            let mut varint = false;
            for attr in input.attrs {
                if let Some(path) = attr.path.segments.first() {
                    match path.ident.to_string().as_str() {
                        "discriminant" => {
                            tag_type_string = attr.tokens.to_string();
                            if tag_type_string.starts_with('(') && tag_type_string.ends_with(')') {
                                tag_type_string.remove(tag_type_string.len() - 1);
                                tag_type_string.remove(0);
                            }
                            if tag_type_string == "VarInt" {
                                tag_type_string = "i32".to_string();
                                varint = true;
                            }
                        },
                        "value" => return quote!(compile_error!("Not the right place for value attribute");).into(),
                        _ => (),
                    }
                }
            }
            let tag_type_ident = format_ident!("{}", tag_type_string);
            let unmatched_message = format!(
                "The {} ID is outside the definition range.",
                name.to_string()
            );

            // Process variants one by one
            let mut serialization_arms = Vec::new();
            let mut deserialization_arms = Vec::new();
            let mut next_discriminant = 0;
            for variant in variants {
                // Collect variant data
                let mut discriminant = next_discriminant;
                for attr in variant.attrs {
                    if let Some(path) = attr.path.segments.first() {
                        match path.ident.to_string().as_str() {
                            "discriminant" => return quote!(compile_error!("Not the right place for discriminant attribute");).into(),
                            "value" => {
                                let mut discriminant_string = attr.tokens.to_string();
                                if discriminant_string.starts_with(' ') {
                                    discriminant_string.remove(0);
                                }
                                if discriminant_string.starts_with('=') {
                                    discriminant_string.remove(0);
                                } else {
                                    return quote!(compile_error!("Invalid value attribute");).into();
                                }
                                if discriminant_string.starts_with(' ') {
                                    discriminant_string.remove(0);
                                }
                                discriminant = discriminant_string.parse().unwrap();
                            },
                            _ => (),
                        }
                    }
                }
                let discriminant_lit = Lit::Int(LitInt::new(
                    &format!("{}{}", discriminant, tag_type_string),
                    Span::call_site().into(),
                ));
                next_discriminant = discriminant + 1;
                let variant_name = variant.ident;
                let fields = variant.fields;
                let fields = match fields {
                    Fields::Named(fields) => fields.named,
                    Fields::Unit => Punctuated::new(),
                    _ => return quote!(compile_error!("All fields must be named");).into(),
                };

                // Build a serialization arm
                let field_names = fields.iter().map(|field| field.ident.as_ref().unwrap());
                let field_names2 = fields.iter().map(|field| field.ident.as_ref().unwrap());
                let serialization_arm = match varint {
                    true => quote! {
                        #name::#variant_name{#(#field_names2, )*} => {
                            VarInt(#discriminant_lit).serialize_minecraft_packet_part(output)?;
                            #(#field_names.serialize_minecraft_packet_part(output)?;)*
                        },
                    },
                    false => quote! {
                        #name::#variant_name{#(#field_names2, )*} => {
                            #discriminant_lit.serialize_minecraft_packet_part(output)?;
                            #(#field_names.serialize_minecraft_packet_part(output)?;)*
                        },
                    }
                };
                serialization_arms.push(serialization_arm);

                // Build a deserialization arm
                let field_names = fields.iter().map(|field| field.ident.as_ref().unwrap());
                let field_names2 = fields.iter().map(|field| field.ident.as_ref().unwrap());
                let field_types = fields.iter().map(|field| &field.ty);
                let deserialization_arm = quote! {
                    #discriminant_lit => {
                        #(let (#field_names, input) = <#field_types>::deserialize_minecraft_packet_part(input)?;)*
                        Ok((#name::#variant_name {
                            #(#field_names2, )*
                        }, input))
                    },
                };
                deserialization_arms.push(deserialization_arm);
            }

            // Gather serialization arms
            let serialization_implementation = quote! {
                match self {
                    #(#serialization_arms)*
                }
                Ok(())
            };

            // Gather deserialization arms
            let deserialization_implementation = match varint {
                true => quote! {
                    let (id, input) = VarInt::deserialize_minecraft_packet_part(input)?;
                    match id.0 {
                        #(#deserialization_arms)*
                        _ => Err(#unmatched_message),
                    }
                },
                false => quote! {
                    let (id, input) = #tag_type_ident::deserialize_minecraft_packet_part(input)?;
                    match id {
                        #(#deserialization_arms)*
                        _ => Err(#unmatched_message),
                    }
                }
            };

            // Derive MinecraftPacketPart
            {quote! {
                #[automatically_derived]
                impl#lifetime_impl MinecraftPacketPart#lifetime_impl for #name#lifetime_struct {
                    fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
                        #serialization_implementation
                    }

                    fn deserialize_minecraft_packet_part(input: &#lifetime [u8]) -> Result<(Self, &#lifetime [u8]), &'static str> {
                        #deserialization_implementation
                    }
                }
            }}.into()
        },
        _ => quote!(compile_error!("Unsupported data structure");).into(),
    }
}

#[proc_macro_attribute]
pub fn minecraft_enum(attr: TokenStream, input: TokenStream) -> TokenStream {
    // Collect data
    let argument_type = attr.to_string();
    let representation_type = match argument_type.as_str() {
        "VarInt" => "i32".to_string(),
        "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" => argument_type.clone(),
        _ => return quote!(compile_error!("Unsupported tag type");).into(),
    };
    let representation_ident = format_ident!("{}", representation_type);
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();
    let data = match input.data.clone() {
        Data::Enum(data) => data,
        _ => return quote!(compile_error!("Unsupported data structure");).into(),
    };
    let unmatched_message = format!(
        "The {} ID is outside the definition range.",
        name.to_string()
    );

    // Analyse enum variants
    let mut variant_name = Vec::new();
    let mut variant_value = Vec::new();
    let mut next_discriminant = 0;
    for variant in data.variants {
        let discriminant = if let Some((_, Expr::Lit(d))) = variant.discriminant {
            if let Lit::Int(d) = d.lit {
                d.base10_parse::<i64>().unwrap()
            } else {
                panic!("The expression of variant discriminant is not an integer literal")
            }
        } else if let Some((_, Expr::Unary(ExprUnary {op: UnOp::Neg(_), expr, ..}))) = variant.discriminant {
            if let Expr::Lit(expr) = *expr {
                if let Lit::Int(d) = expr.lit {
                    -d.base10_parse::<i64>().unwrap()
                } else {
                    panic!("The expression after negation of variant discriminant is not an integer literal")
                }
            } else {
                panic!("No expression after negation of variant discriminant")
            }
        } else {
            next_discriminant
        };
        next_discriminant = discriminant + 1;
        let discriminant = Lit::Int(LitInt::new(
            &format!("{}{}", discriminant, representation_type),
            Span::call_site().into(),
        ));
        variant_name.push(variant.ident);
        variant_value.push(discriminant);
    }

    // Construct the serialize_minecraft_packet_part method
    let append_implementation = match argument_type.as_str() {
        "u8" => quote! {
            output.push(self as u8);
            Ok(())
        },
        "VarInt" => quote! {
            VarInt(self as i32).serialize_minecraft_packet_part(output)
        },
        _ => quote! {
            (self as #representation_ident).serialize_minecraft_packet_part(output)
        },
    };

    // Construct the deserialize_minecraft_packet_part method
    let build_implementation = match argument_type.as_str() {
        "VarInt" => quote! {
            let (id, input) = VarInt::deserialize_minecraft_packet_part(input)?;
            let value = match id.0 {
                #(#variant_value => #name::#variant_name,)*
                _ => return Err(#unmatched_message),
            };
            Ok((value, input))
        },
        _ => quote! {
            let (id, input) = #representation_ident::deserialize_minecraft_packet_part(input)?;
            let value = match id {
                #(#variant_value => #name::#variant_name,)*
                _ => return Err(#unmatched_message),
            };
            Ok((value, input))
        },
    };

    // Derive MinecraftPacketPart
    {quote! {
        #[repr(#representation_ident)]
        #input

        #[automatically_derived]
        impl<'a> MinecraftPacketPart<'a> for #name {
            fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
                #append_implementation
            }

            fn deserialize_minecraft_packet_part(input: &'a [u8]) -> Result<(Self, &'a [u8]), &'static str> {
                #build_implementation
            }
        }
    }}.into()
}
