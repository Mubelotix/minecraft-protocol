#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate lazy_static;

use std::{collections::{HashMap, HashSet}, fs};

use proc_macro::{TokenStream, Span};
use syn::{parse_macro_input, Token, Ident, parse, DeriveInput};

lazy_static::lazy_static! {
    static ref UNIQUE_ID: String = {
        use std::time::{SystemTime, UNIX_EPOCH};
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        (time.as_secs() * 1000).to_string()
    };
}

fn clear_file() {
    let path = &format!("component_names_{}.txt", *UNIQUE_ID);
    // Get all files in the directory 
    let files = fs::read_dir("target/tmp").unwrap();
    for file in files {
        let file = file.unwrap();
        let file_name = file.file_name().to_str().unwrap().to_string();
        if file_name.starts_with("component_names_") && file_name.ends_with(".txt")  && !file_name.contains(path) {
            fs::remove_file(file.path()).unwrap();
        }
    }
}

struct MultipleTags {
    tags: Vec<(Ident, Vec<Ident>)>,
    components: HashMap<Ident, Vec<Ident>>,
}

impl parse::Parse for MultipleTags {
    fn parse(input: syn::parse::ParseStream) -> parse::Result<Self> {
        let mut tags = Vec::new();
        let mut components_table: HashMap<Ident, Vec<Ident>> = HashMap::new();
        while !input.is_empty() {
            let tag_name: Ident = input.parse()?;
            let content;
            syn::braced!(content in input);

            let components = content
                .parse_terminated(Ident::parse, Token![,])?;
            for component in components.iter() {
                // We add the tag to the component
                let tags_of_component = components_table.get_mut(component);
                if let Some(tags_of_component) = tags_of_component {
                    tags_of_component.push(tag_name.clone());
                } else {
                    components_table.insert(component.clone(), vec![tag_name.clone()]);
                }
            }
            tags.push((tag_name, components.into_iter().collect()));
        }

        Ok(Self { tags, components: components_table })
    }
}



#[proc_macro]
pub fn tags(input: TokenStream) -> TokenStream {
    /*
        We have something like that:
        tags! {
            Player  {
                HealthComponent,
                PositionComponent,
            }
        }
    */
    let items = parse_macro_input!(input as MultipleTags);

    let tags = items.tags.iter().map(|t| t.0.clone());

    // We write an enum with all tags
    let mut output = quote! {
        use std::collections::HashSet;
        
        /// All tags in the game
        #[derive(Clone, PartialEq, Eq, Hash)]
        pub enum Tag {
            #(
                #tags,
            )*
        }

    };

    let tags_with_components = items.components;    
    let match_patterns = tags_with_components.iter().map(|(component, tags)| {
        let tags = tags.iter().map(|t| {
            let tag = Ident::new(&format!("{}", t), t.span());
            quote! {
                Tag::#tag
            }
        });
        let component = Ident::new(&format!("{}", component), component.span());
        quote! {
            Component::#component => vec![#(#tags),*],
        }
    });
    let map_tag_components = items.tags;
    let map_tag_components = map_tag_components.iter().map(|(tag, components)| {
        let tag = Ident::new(&format!("{}", tag), tag.span());
        let components = components.iter().map(|c| {
            let component = Ident::new(&format!("{}", c), c.span());
            quote! {
                Component::#component
            }
        });
        quote! {
            Tag::#tag => vec![#(#components),*],
        }
    });
    // We write a function to get all tags from components
    let get_tags_from_components = quote! {
        impl Tag {
            /// we get all components from a tag
            pub fn get_components(&self) -> Vec<Component> {
                match self {
                    #(#map_tag_components)*
                }
            }

            /// we get all tags from a component
            pub fn get_tags_from_component(component: Component) -> Vec<Tag> {
                match component {
                    #(#match_patterns)*
                    _ => vec![],
                }
            }

            /// we get all tags that have the components
            pub fn get_tags_from_components(components: HashSet<Component>) -> HashSet<Tag> {
                let mut tags = HashSet::new();
                for component in components.iter() {
                    for tag in Tag::get_tags_from_component(component.clone()) {
                        let components_of_tag = tag.get_components();
                        if components_of_tag.iter().all(|c| components.contains(c)) {
                            tags.insert(tag);
                        }
                    }
                }
                tags
            }
        }
    };
    

    output.extend(get_tags_from_components);
    
    output.into()
}


use quote::quote;

#[proc_macro_derive(Component)]
pub fn component_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Path to the accumulator file
    let path = &format!("target/tmp/component_names_{}.txt", *UNIQUE_ID);
    clear_file();
    // Read the existing component names, if any
    let mut components = if std::path::Path::new(&path).exists() {
        fs::read_to_string(path).unwrap_or_default().lines().map(String::from).collect::<HashSet<_>>()
    } else {
        HashSet::new()
    };

    let name_trimed = name.to_string().trim_end_matches("Component").to_string();

    // Add the new component
    if !components.contains(&name_trimed.to_string()) {
        
        components.insert(name_trimed.to_string());
        fs::write(path, components.iter().cloned().collect::<Vec<_>>().join("\n")).expect("Failed to write to component_names.txt");
    }

    let attach_fn_name = Ident::new(&format!("attach_{}", name_trimed.to_lowercase()), name.span());
    let field_name = Ident::new(&format!("{}_components", name_trimed.to_lowercase()), name.span());
    let name_trimed = Ident::new(&name_trimed, name.span());

    let gen = quote! {
        impl ComponentTrait for #name {
            fn get_component_enum(&self) -> Component {
                Component::#name_trimed
            }
        }

        impl Entities {
            async fn #attach_fn_name(&self, id: Eid, component: #name) -> Option<()> {
                let mut components = self.#field_name.write().await;
                components.insert(id, component);
                self.entities.write().await.get_mut(&id)?.insert_component(Component::#name_trimed);
                Some(())
            }
        }
    };
    gen.into()
}

#[proc_macro]
pub fn generate_component_enum(_: TokenStream) -> TokenStream {
    let path = &format!("target/tmp/component_names_{}.txt", *UNIQUE_ID);
    let components = if std::path::Path::new(&path).exists() {
        fs::read_to_string(path).unwrap_or_default().lines().map(|e| Ident::new(e, Span::call_site().into())).collect::<Vec<_>>()
    } else {
        Vec::new()
    };

    let gen = quote! {
        #[derive(Eq, Hash, PartialEq, Clone)]
        pub enum Component {
            #(#components,)*
        }
    };

    gen.into()
}

/// Insert the fields to the structure with [#[insert_components_fields]]
#[proc_macro_attribute]
pub fn insert_components_fields(_attr: TokenStream, input: TokenStream) -> TokenStream {
 
    let mut input_ast = syn::parse_macro_input!(input as syn::ItemStruct);
    let struct_name = &input_ast.ident;
    let path = &format!("target/tmp/component_names_{}.txt", *UNIQUE_ID);
    let components = if std::path::Path::new(&path).exists() {
        fs::read_to_string(path).unwrap_or_default().lines().map(|e| Ident::new(e, Span::call_site().into())).collect::<Vec<_>>()
    } else {
        Vec::new()
    };

    let mut fields = Vec::new();

    for component in &components {
        let field_ident = Ident::new(&format!("{}_components", component.to_string().to_lowercase()), component.span());
        let component_type_ident = Ident::new(&format!("{}Component", component), component.span());
        fields.push(field_ident.clone());

        let field = syn::Field {
            attrs: Vec::new(),
            vis: syn::Visibility::Public(syn::token::Pub { span: component.span() }),
            ident: Some(field_ident),
            colon_token: Some(syn::token::Colon { spans: [component.span()] }),
            ty: syn::parse_quote! {
                RwLock<HashMap<Eid, #component_type_ident>>
            },
            mutability: syn::FieldMutability::None,
        };
    
        match &mut input_ast.fields {
            syn::Fields::Named(fields_named) => {
                fields_named.named.push(field);
            }
            _ => {}
        }
    }

    let impl_ = quote! {
        impl #struct_name {
            /// Create a new entity manager
            pub fn new() -> Self {
                Self {
                    entities: RwLock::new(HashMap::new()),
                    entity_count: RwLock::new(0),
                    chunks: RwLock::new(HashMap::new()),
                    uuids: RwLock::new(HashMap::new()),
                    entities_by_tag: RwLock::new(HashMap::new()),
                    #(#fields: RwLock::new(HashMap::new()),)*
                }
            }
        }
    };

    let gen = quote! {
        #impl_

        #input_ast
    };
    
    gen.into()
}    