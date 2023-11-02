use std::collections::HashMap;

use proc_macro::{TokenStream, TokenTree, Ident, Group};
use convert_case::{Case, Casing};

fn replace_idents(token: &mut TokenTree, to_replace: &HashMap<&'static str, Ident>) {
    match token {
        TokenTree::Group(g) => {
            let mut stream = g.stream().into_iter().collect::<Vec<_>>();
            for element in &mut stream {
                replace_idents(element, to_replace);
            }
            *g = Group::new(g.delimiter(), stream.into_iter().collect());
        }
        TokenTree::Ident(ident) => {
            if let Some(new_ident) = to_replace.get(ident.to_string().as_str()) {
                *ident = new_ident.clone();
            }
        }
        TokenTree::Punct(_) => (),
        TokenTree::Literal(_) => (),
    }
}

#[proc_macro_attribute]
pub fn inherit(attr: TokenStream, item: TokenStream) -> TokenStream {
    // List inherited items
    let mut inherited = Vec::new();
    let mut attrs = attr.into_iter();
    loop {
        match attrs.next() {
            Some(TokenTree::Ident(ident)) => inherited.push(ident),
            Some(other) => panic!("unexpected token: {:?}", other),
            None => break,
        }

        match attrs.next() {
            Some(TokenTree::Punct(punct)) => {
                if punct.as_char() != ',' {
                    panic!("unexpected punct: {:?}", punct);
                }
            },
            Some(other) => panic!("unexpected token: {:?}", other),
            None => break,
        }
    }
    let parent = inherited.remove(0);

    // Get struct name
    let mut items = item.clone().into_iter();
    match items.next() {
        Some(TokenTree::Ident(ident)) if ident.to_string() == "pub" => (),
        Some(other) => panic!("expected struct to be public, found {:?}", other),
        None => panic!("expected public struct, found nothing"),
    }
    match items.next() {
        Some(TokenTree::Ident(ident)) if ident.to_string() == "struct" => (),
        Some(other) => panic!("expected struct, found {:?}", other),
        None => panic!("expected struct, found nothing"),
    }
    let struct_name = match items.next() {
        Some(TokenTree::Ident(ident)) => ident,
        Some(other) => panic!("expected struct name, found {:?}", other),
        None => panic!("expected struct name, found nothing"),
    };

    let mut codes = Vec::new();

    // Generate code for parent
    let code: TokenStream = r#"
        impl ParentDescendant for This {
            fn get_parent(&self) -> &Parent { &self.parent }
            fn get_parent_mut(&mut self) -> &mut Parent { &mut self.parent }
        }
    "#.parse().unwrap();
    let mut to_replace = HashMap::new();
    to_replace.insert("ParentDescendant", Ident::new(&format!("{}Descendant", parent), parent.span()));
    to_replace.insert("Parent", parent.clone());
    to_replace.insert("This", struct_name.clone());
    to_replace.insert("get_parent", Ident::new(&format!("get_{}", parent.to_string().to_case(Case::Snake)), parent.span()));
    to_replace.insert("get_parent_mut", Ident::new(&format!("get_{}_mut", parent.to_string().to_case(Case::Snake)), parent.span()));
    to_replace.insert("parent", Ident::new(&parent.to_string().to_case(Case::Snake), parent.span()));
    let mut code = code.clone().into_iter().collect::<Vec<_>>();
    for element in &mut code {
        replace_idents(element, &to_replace);
    }
    let code: TokenStream = code.into_iter().collect();
    println!("{}", code);
    codes.push(code);

    // Generate code for higher inheritance levels
    let code: TokenStream = r#"
        impl InheritedDescendant for This {
            fn get_inherited(&self) -> &Inherited { self.parent.get_inherited() }
            fn get_inherited_mut(&mut self) -> &mut Inherited { self.parent.get_inherited_mut() }
        }
    "#.parse().unwrap();
    for inherited in inherited {
        let mut to_replace = HashMap::new();
        to_replace.insert("InheritedDescendant", Ident::new(&format!("{}Descendant", inherited), inherited.span()));
        to_replace.insert("Inherited", inherited.clone());
        to_replace.insert("This", struct_name.clone());
        to_replace.insert("get_inherited", Ident::new(&format!("get_{}", inherited.to_string().to_case(Case::Snake)), inherited.span()));
        to_replace.insert("get_inherited_mut", Ident::new(&format!("get_{}_mut", inherited.to_string().to_case(Case::Snake)), inherited.span()));
        to_replace.insert("parent", Ident::new(&parent.to_string().to_case(Case::Snake), parent.span()));

        let mut code = code.clone().into_iter().collect::<Vec<_>>();
        for element in &mut code {
            replace_idents(element, &to_replace);
        }
        let code: TokenStream = code.into_iter().collect();
        println!("{}", code);
        codes.push(code);
    }

    // Generate final code
    let mut final_code = item;
    for code in codes {
        final_code.extend(code);
    }
    final_code
}
