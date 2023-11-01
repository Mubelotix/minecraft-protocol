#[macro_use]
extern crate quote;
extern crate proc_macro;

use std::collections::HashMap;

use proc_macro::TokenStream;
use syn::{parse_macro_input, Token, Ident, parse};

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
