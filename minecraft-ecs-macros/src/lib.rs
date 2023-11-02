#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate lazy_static;

use std::collections::HashMap;

use proc_macro::{TokenStream, TokenTree};
use syn::{parse_macro_input, Token, Ident, parse, DeriveInput};

lazy_static::lazy_static! {
    static ref UNIQUE_ID: String = {
        use std::time::{SystemTime, UNIX_EPOCH};
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        (time.as_secs() * 1000).to_string()
    };
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
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        component.to_string().hash(&mut hasher);
        let u = hasher.finish();

        let tags = tags.iter().map(|t| {
            let tag = Ident::new(&format!("{}", t), t.span());
            quote! {
                Tag::#tag
            }
        });
        quote! {
            #u => vec![#(#tags),*],
        }
    });
    let map_tag_components = items.tags;
    let map_tag_components = map_tag_components.iter().map(|(tag, components)| {
        let tag = Ident::new(&format!("{}", tag), tag.span());
        let components = components.iter().map(|c| {
            use std::hash::{Hash, Hasher};
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            c.to_string().hash(&mut hasher);
            let u = hasher.finish();
    
            quote! {
                ComponentId(#u)
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
            pub fn get_components(&self) -> Vec<ComponentId> {
                match self {
                    #(#map_tag_components)*
                }
            }

            /// we get all tags from a component
            pub fn get_tags_from_component(component: ComponentId) -> Vec<Tag> {
                match component.0 {
                    #(#match_patterns)*
                    _ => vec![],
                }
            }

            /// we get all tags that have the components
            pub fn get_tags_from_components(components: HashSet<ComponentId>) -> HashSet<Tag> {
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

#[proc_macro_attribute]
pub fn is_component(attr: TokenStream, input_stream: TokenStream) -> TokenStream {
    let input = input_stream.clone();
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Check if attr contains an unique id
    let mut id = None;
    let mut attr = attr.into_iter();
    while let Some(token) = attr.next() {
        if let TokenTree::Ident(ident) = token {
            if ident.to_string() == "id" {
                if let Some(TokenTree::Punct(punct)) = attr.next() {
                    if punct.as_char() == '=' {
                        if let Some(TokenTree::Literal(literal)) = attr.next() {
                            id = Some(literal.to_string());
                        }
                    }
                }
            }
        }
    }

    let id = if let Some(id) = id {
        id.parse::<u64>().unwrap()
    } else {
        // Hash the name
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        name.to_string().hash(&mut hasher);
        hasher.finish()
    };

    let gen = quote! {    
        #input
        use std::any::Any;
        use minecraft_ecs::component::ComponentId;

        impl Component for #name {
            
            fn get_component_id(&self) -> ComponentId { 
                ComponentId(#id)
            }
            fn as_any(&self) -> &dyn Any {
                self
            }
        }
    };
    gen.into()
}
