use std::collections::HashMap;

use proc_macro2::{TokenStream, TokenTree, Ident, Group, Delimiter};
use convert_case::{Case, Casing};
use proc_macro_error::*;
use quote::quote;

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

#[allow(non_snake_case)]
#[proc_macro_attribute]
#[proc_macro_error]
pub fn MinecraftEntity(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let attr = TokenStream::from(attr.clone());
    let item = TokenStream::from(item.clone());

    let mut ancestors = Vec::new();
    let mut descendants = Vec::new();
    let mut wildcard_descendants = Vec::new();
    let mut inheritable = false;
    let mut defines = Vec::new();
    let mut codes = Vec::new();
        
    // Get struct name
    let mut items = item.clone().into_iter();
    match items.next() {
        Some(TokenTree::Ident(ident)) if ident.to_string() == "pub" => (),
        Some(TokenTree::Punct(punct)) if punct.as_char() == '#' => {
            items.next();
            match items.next() {
                Some(TokenTree::Ident(ident)) if ident.to_string() == "pub" => (),
                Some(other) => abort!(other.span(), "expected struct to be public"),
                None => panic!("expected public struct, found nothing"),
            }
        }
        Some(other) => abort!(other.span(), "expected struct to be public"),
        None => panic!("expected public struct, found nothing"),
    }
    match items.next() {
        Some(TokenTree::Ident(ident)) if ident.to_string() == "struct" => (),
        Some(other) => abort!(other.span(), "expected struct, found {:?}"),
        None => panic!("expected struct, found nothing"),
    }
    let struct_name = match items.next() {
        Some(TokenTree::Ident(ident)) => ident,
        Some(other) => abort!(other.span(), "expected struct name, found {:?}"),
        None => panic!("expected struct name, found nothing"),
    };

    // Parse attributes
    let mut attrs = attr.into_iter().peekable();
    while let Some(ident) = attrs.next() {
        let TokenTree::Ident(ident) = ident else { abort!(ident.span(), "expected ident") };
        match ident.to_string().as_str() {
            "ancestors" => {
                let Some(token_tree) = attrs.next() else { abort!(ident.span(), "expected group after parents") };
                let TokenTree::Group(group) = token_tree else { abort!(token_tree.span(), "expected group") };
                let mut group_attrs = group.stream().into_iter().peekable();
                while let Some(ident) = group_attrs.next() {
                    let TokenTree::Ident(ident) = ident else { abort!(ident.span(), "expected ident") };
                    ancestors.push(ident);
                    if matches!(group_attrs.peek(), Some(TokenTree::Punct(punct)) if punct.as_char() == ',') {
                        group_attrs.next();
                    }
                }
            }
            "descendants" => {
                let Some(token_tree) = attrs.next() else { abort!(ident.span(), "expected group after parents") };
                let TokenTree::Group(group) = token_tree else { abort!(token_tree.span(), "expected group") };
                let mut group_attrs = group.stream().into_iter().peekable();
                while let Some(ident) = group_attrs.next() {
                    let TokenTree::Ident(ident) = ident else { abort!(ident.span(), "expected ident") };
                    if matches!(group_attrs.peek(), Some(TokenTree::Punct(punct)) if punct.as_char() == '.') {
                        let dot = group_attrs.next().unwrap();
                        if !matches!(group_attrs.next(), Some(TokenTree::Punct(punct)) if punct.as_char() == '.') {
                            abort!(dot.span(), "this dot needs to come with two other dots");
                        }
                        if !matches!(group_attrs.next(), Some(TokenTree::Punct(punct)) if punct.as_char() == '.') {
                            abort!(dot.span(), "this dot needs to come with two other dots");
                        }
                        descendants.push(ident);
                    } else {
                        wildcard_descendants.push(ident);
                    }
                    if matches!(group_attrs.peek(), Some(TokenTree::Punct(punct)) if punct.as_char() == ',') {
                        group_attrs.next();
                    }
                }
            }
            "inheritable" => inheritable = true,
            "defines" => {
                let Some(token_tree) = attrs.next() else { abort!(ident.span(), "expected group after") };
                let TokenTree::Group(group) = token_tree else { abort!(token_tree.span(), "expected group") };
                let mut group_attrs = group.stream().into_iter().peekable();
                while group_attrs.peek().is_some() {
                    let TokenTree::Ident(mut method) = group_attrs.next().unwrap() else { abort!(ident.span(), "expected ident") };
                    let mut ty = None;
                    if matches!(group_attrs.peek(), Some(TokenTree::Punct(punct)) if punct.as_char() == '.') {
                        let point = group_attrs.next().unwrap();
                        let TokenTree::Ident(ident) = group_attrs.next().unwrap() else { abort!(point.span(), "expected method name") };
                        ty = Some(method);
                        method = ident;
                    }
                    let Some(group) = group_attrs.next() else { abort!(method.span(), "expected group after method name") };
                    let TokenTree::Group(params) = group else { abort!(group.span(), "expected group") };
                    if params.delimiter() != Delimiter::Parenthesis {
                        abort!(params.span(), "expected parenthesis");
                    }
                    let mut params = params.stream().into_iter().peekable();
                    if matches!(params.peek(), Some(TokenTree::Ident(ident)) if ident.to_string() == "self") {
                        params.next();
                        if matches!(params.peek(), Some(TokenTree::Punct(punct)) if punct.as_char() == ',') {
                            params.next();
                        }
                    } else {
                        abort!(params.peek().unwrap().span(), "expected self as first parameter");
                    }
                    let mut args: Vec<(Ident, TokenStream)> = Vec::new();
                    while params.peek().is_some() {
                        let TokenTree::Ident(name) = params.next().unwrap() else { abort!(params.peek().unwrap().span(), "expected ident") };
                        if !matches!(params.next(), Some(TokenTree::Punct(punct)) if punct.as_char() == ':') {
                            abort!(name.span(), "expected colon after name");
                        }
                        let mut ty = TokenStream::new();
                        while params.peek().is_some() && !matches!(params.peek(), Some(TokenTree::Punct(punct)) if punct.as_char() == ',') {
                            ty.extend(params.next());
                        }
                        params.next();
                        args.push((name, ty));
                    }
                    defines.push((ty.unwrap_or_else(|| struct_name.clone()), method, args));
                    if matches!(group_attrs.peek(), Some(TokenTree::Punct(punct)) if punct.as_char() == ';') {
                        group_attrs.next();
                    }
                }
            }
            other => abort!(ident.span(), "unrecognized identifier {}", other),
        }
        if matches!(attrs.peek(), Some(TokenTree::Punct(punct)) if punct.as_char() == ',') {
            attrs.next();
        }
    }
    let mut hierarchy = ancestors.clone();
    hierarchy.insert(0, struct_name.clone());

    let mut to_replace = HashMap::new();
    let this = struct_name.clone();
    let this_snake = Ident::new(&struct_name.to_string().to_case(Case::Snake), struct_name.span());
    to_replace.insert("This", this.clone());
    to_replace.insert("ThisDescendant", Ident::new(&format!("{}Descendant", struct_name), struct_name.span()));
    to_replace.insert("ThisMethods", Ident::new(&format!("{}Methods", struct_name), struct_name.span()));
    to_replace.insert("ThisExt", Ident::new(&format!("{}Ext", struct_name), struct_name.span()));
    to_replace.insert("get_this", Ident::new(&format!("get_{}", struct_name.to_string().to_case(Case::Snake)), struct_name.span()));
    to_replace.insert("get_this_mut", Ident::new(&format!("get_{}_mut", struct_name.to_string().to_case(Case::Snake)), struct_name.span()));

    if !ancestors.is_empty() {
        // Generate code for parent
        let parent = ancestors.remove(0);
        let code: TokenStream = r#"
            #[automatically_derived]
            impl ParentDescendant for This {
                fn get_parent(&self) -> &Parent { &self.parent }
                fn get_parent_mut(&mut self) -> &mut Parent { &mut self.parent }
            }
        "#.parse().unwrap();
        to_replace.insert("ParentDescendant", Ident::new(&format!("{}Descendant", parent), parent.span()));
        to_replace.insert("Parent", parent.clone());
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
    }

    // Generate code for higher inheritance levels
    let code: TokenStream = r#"
        #[automatically_derived]
        impl InheritedDescendant for This {
            fn get_inherited(&self) -> &Inherited { self.parent.get_inherited() }
            fn get_inherited_mut(&mut self) -> &mut Inherited { self.parent.get_inherited_mut() }
        }
    "#.parse().unwrap();
    for inherited in ancestors {
        to_replace.insert("InheritedDescendant", Ident::new(&format!("{}Descendant", inherited), inherited.span()));
        to_replace.insert("Inherited", inherited.clone());
        to_replace.insert("get_inherited", Ident::new(&format!("get_{}", inherited.to_string().to_case(Case::Snake)), inherited.span()));
        to_replace.insert("get_inherited_mut", Ident::new(&format!("get_{}_mut", inherited.to_string().to_case(Case::Snake)), inherited.span()));

        let mut code = code.clone().into_iter().collect::<Vec<_>>();
        for element in &mut code {
            replace_idents(element, &to_replace);
        }
        let code: TokenStream = code.into_iter().collect();
        codes.push(code);
    }

    if inheritable {
        // Generate descendant trait
        let code: TokenStream = r#"
            #[automatically_derived]
            pub trait ThisDescendant {
                fn get_this(&self) -> &This;
                fn get_this_mut(&mut self) -> &mut This;
            }
            
            #[automatically_derived]
            impl ThisDescendant for This {
                fn get_this(&self) -> &This { self }
                fn get_this_mut(&mut self) -> &mut This { self }
            }
        "#.parse().unwrap();
        let mut code = code.clone().into_iter().collect::<Vec<_>>();
        for element in &mut code {
            replace_idents(element, &to_replace);
        }
        let code: TokenStream = code.into_iter().collect();
        codes.push(code);
    }

    // Generate ext trait
    let code: TokenStream = r#"
        #[automatically_derived]
        pub trait ThisExt: Sized + Into<Handler<This>> {
            fn methods() -> &'static ThisMethods;
        }
    "#.parse().unwrap();
    let mut code = code.clone().into_iter().collect::<Vec<_>>();
    for element in &mut code {
        replace_idents(element, &to_replace);
    }
    let mut inner_codes = TokenStream::new();
    for (_, method, args) in defines.iter().filter(|(ty, _, _)| ty.to_string() == struct_name.to_string()) {
        let inner_code: TokenStream = match args.len() {
            0 => String::from(r#"
                fn method(self) -> Pin<Box<dyn Future<Output = ()>>> {{
                    (Self::methods().method)(self.into())
                }}
            "#),
            1 => format!(r#"
                fn method(self, arg1: {}) -> Pin<Box<dyn Future<Output = ()>>> {{
                    (Self::methods().method)(self.into(), arg1)
                }}
            "#, args[0].1),
            2 => format!(r#"
                fn method(self, arg1: {}, arg2: {}) -> Pin<Box<dyn Future<Output = ()>>> {{
                    (Self::methods().method)(self.into(), arg1, arg2)
                }}
            "#, args[0].1, args[1].1),
            3 => format!(r#"
                fn method(self, arg1: {}, arg2: {}, arg3: {}) -> Pin<Box<dyn Future<Output = ()>>> {{
                    (Self::methods().method)(self.into(), arg1, arg2, arg3)
                }}
            "#, args[0].1, args[1].1, args[2].1),
            4 => format!(r#"
                fn method(self, arg1: {}, arg2: {}, arg3: {}, arg4: {}) -> Pin<Box<dyn Future<Output = ()>>> {{
                    (Self::methods().method)(self.into(), arg1, arg2, arg3, arg4)
                }}
            "#, args[0].1, args[1].1, args[2].1, args[3].1),
            _ => abort!(method.span(), "too many arguments"),
        }.parse().unwrap();
        to_replace.insert("method", method.clone());
        let mut inner_code = inner_code.clone().into_iter().collect::<Vec<_>>();
        for element in &mut inner_code {
            replace_idents(element, &to_replace);
        }
        let inner_code: TokenStream = inner_code.into_iter().collect();
        inner_codes.extend(inner_code);
    }
    let TokenTree::Group(ref mut group) = code.last_mut().unwrap() else {unreachable!()};
    *group = Group::new(group.delimiter(), group.stream().into_iter().chain(inner_codes.into_iter()).collect());
    let code: TokenStream = code.into_iter().collect();
    codes.push(code);

    // Generate methods struct
    let code: TokenStream = r#"
        #[automatically_derived]
        pub struct ThisMethods {
            
        }
    "#.parse().unwrap();
    let mut code = code.clone().into_iter().collect::<Vec<_>>();
    for element in &mut code {
        replace_idents(element, &to_replace);
    }
    let mut inner_codes = TokenStream::new();
    for (_, method, args) in defines.iter().filter(|(ty, _, _)| ty.to_string() == struct_name.to_string()) {
        let inner_code: TokenStream = match args.len() {
            0 => String::from(r#"pub method: CallBack<Handler<This>>,"#),
            1 => format!(r#"pub method: CallBack1<Handler<This>, {}>,"#, args[0].1),
            2 => format!(r#"pub method: CallBack2<Handler<This>, {}, {}>,"#, args[0].1, args[1].1),
            3 => format!(r#"pub method: CallBack3<Handler<This>, {}, {}, {}>,"#, args[0].1, args[1].1, args[2].1),
            4 => format!(r#"pub method: CallBack4<Handler<This>, {}, {}, {}, {}>,"#, args[0].1, args[1].1, args[2].1, args[3].1),
            _ => abort!(method.span(), "too many arguments"),
        }.parse().unwrap();
        to_replace.insert("method", method.clone());
        let mut inner_code = inner_code.clone().into_iter().collect::<Vec<_>>();
        for element in &mut inner_code {
            replace_idents(element, &to_replace);
        }
        let inner_code: TokenStream = inner_code.into_iter().collect();
        inner_codes.extend(inner_code);
    }
    let TokenTree::Group(ref mut group) = code.last_mut().unwrap() else {unreachable!()};
    *group = Group::new(group.delimiter(), group.stream().into_iter().chain(inner_codes.into_iter()).collect());
    let code: TokenStream = code.into_iter().collect();
    codes.push(code);

    // Generate default for methods struct
    let mut mod_codes = Vec::new();
    for ascendant in hierarchy.iter().peekable() {
        let code: TokenStream = r#"
            pub const ASCENDANT_METHODS_FOR_THIS: &AscendantMethods = &AscendantMethods {

            };
        "#.parse().unwrap();
        to_replace.insert("ASCENDANT_METHODS_FOR_THIS", Ident::new(&format!("{}_METHODS_FOR_{}", ascendant.to_string().to_case(Case::ScreamingSnake), struct_name.to_string().to_case(Case::ScreamingSnake)), ascendant.span()));
        to_replace.insert("Ascendant", ascendant.clone());
        to_replace.insert("AscendantMethods", Ident::new(&format!("{}Methods", ascendant), ascendant.span()));
        let mut code = code.clone().into_iter().collect::<Vec<_>>();
        for element in &mut code {
            replace_idents(element, &to_replace);
        }    
        let mut inner_codes = TokenStream::new();
        for (_, method, args) in defines.iter().filter(|(ty, _, _)| ty.to_string() == ascendant.to_string()) {
            let inner_code: TokenStream = match args.len() {
                0 => r#"method: |s| Box::pin(s.assume_other::<This>().method()),"#,
                1 => r#"method: |s, arg1| Box::pin(s.assume_other::<This>().method(arg1)),"#,
                2 => r#"method: |s, arg1, arg2| Box::pin(s.assume_other::<This>().method(arg1, arg2)),"#,
                3 => r#"method: |s, arg1, arg2, arg3| Box::pin(s.assume_other::<This>().method(arg1, arg2, arg3)),"#,
                4 => r#"method: |s, arg1, arg2, arg3, arg4| Box::pin(s.assume_other::<This>().method(arg1, arg2, arg3, arg4)),"#,
                _ => abort!(method.span(), "too many arguments"),
            }.parse().unwrap();
            to_replace.insert("method", method.clone());
            for (i, (name, _)) in args.iter().enumerate() {
                to_replace.insert(["arg1", "arg2", "arg3", "arg4"][i], name.clone());
            }
            let mut inner_code = inner_code.clone().into_iter().collect::<Vec<_>>();
            for element in &mut inner_code {
                replace_idents(element, &to_replace);
            }
            let inner_code: TokenStream = inner_code.into_iter().collect();
            inner_codes.extend(inner_code);
        }
        let i = code.len() - 2;
        let TokenTree::Group(ref mut group) = code.get_mut(i).unwrap() else {unreachable!()};
        *group = Group::new(group.delimiter(), group.stream().into_iter().chain(inner_codes.into_iter()).collect());

        if ascendant.to_string() != struct_name.to_string() {
            let inner_code: TokenStream = r#"..*ASCENDANT_METHODS_FOR_PARENT"#.parse().unwrap();
            to_replace.insert("ASCENDANT_METHODS_FOR_PARENT", Ident::new(&format!("{}_METHODS_FOR_{}", ascendant.to_string().to_case(Case::ScreamingSnake), hierarchy[1].to_string().to_case(Case::ScreamingSnake)), ascendant.span()));
            let mut inner_code = inner_code.clone().into_iter().collect::<Vec<_>>();
            for element in &mut inner_code {
                replace_idents(element, &to_replace);
            }    
            let inner_code: TokenStream = inner_code.into_iter().collect();
            let TokenTree::Group(ref mut group) = code.get_mut(i).unwrap() else {unreachable!()};
            *group = Group::new(group.delimiter(), group.stream().into_iter().chain(inner_code.into_iter()).collect());
        }

        let code: TokenStream = code.into_iter().collect();
        mod_codes.push(code);
    }
    let mod_codes: TokenStream = mod_codes.into_iter().collect();
    let mod_code: TokenStream = format!(r#"
        #[allow(clippy::needless_update)]
        #[automatically_derived]
        pub mod {struct_name}_methods {{
            use super::{{ {} }};
        }}
    "#, hierarchy.iter().map(|i| format!("{i}, {i}Methods")).chain(hierarchy.iter().skip(1).map(|i| format!("{i}_methods::*"))).collect::<Vec<_>>().join(","),
    ).parse().unwrap();
    let mut mod_code = mod_code.into_iter().collect::<Vec<_>>();
    let TokenTree::Group(ref mut group) = mod_code.last_mut().unwrap() else {unreachable!()};
    *group = Group::new(group.delimiter(), group.stream().into_iter().chain(mod_codes.into_iter()).collect());
    codes.push(mod_code.into_iter().collect());

    // Implement ext traits
    for ascendant in hierarchy.iter().peekable() {
        let code: TokenStream = format!(r#"
            #[automatically_derived]
            impl AscendantExt for Handler<This> {{
                fn methods() -> &'static AscendantMethods {{
                    {struct_name}_methods::ASCENDANT_METHODS_FOR_THIS
                }}
            }}
        "#).parse().unwrap();
        to_replace.insert("ASCENDANT_METHODS_FOR_THIS", Ident::new(&format!("{}_METHODS_FOR_{}", ascendant.to_string().to_case(Case::ScreamingSnake), struct_name.to_string().to_case(Case::ScreamingSnake)), ascendant.span()));
        to_replace.insert("Ascendant", ascendant.clone());
        to_replace.insert("AscendantExt", Ident::new(&format!("{}Ext", ascendant), ascendant.span()));
        to_replace.insert("AscendantMethods", Ident::new(&format!("{}Methods", ascendant), ascendant.span()));
        let mut code = code.clone().into_iter().collect::<Vec<_>>();
        for element in &mut code {
            replace_idents(element, &to_replace);
        }
        let code: TokenStream = code.into_iter().collect();
        codes.push(code);
    }

    // Implement conversion traits
    for ascendant in hierarchy.iter().skip(1) {
        let code: TokenStream = r#"
            #[automatically_derived]
            impl From<Handler<This>> for Handler<Ascendant> {
                fn from(val: Handler<This>) -> Self {
                    val.assume_other()
                }
            }
        "#.parse().unwrap();
        to_replace.insert("Ascendant", ascendant.clone());
        let mut code = code.clone().into_iter().collect::<Vec<_>>();
        for element in &mut code {
            replace_idents(element, &to_replace);
        }
        let code: TokenStream = code.into_iter().collect();
        codes.push(code);
    }
    
    // Implement TryAsEntityRef
    let descendants_snake = descendants.iter().map(|i| Ident::new(i.to_string().to_case(Case::Snake).as_str(), i.span())).collect::<Vec<_>>();
    let wildcard_descendants_snake = wildcard_descendants.iter().map(|i| Ident::new(i.to_string().to_case(Case::Snake).as_str(), i.span())).collect::<Vec<_>>();
    let code = quote! {
        #[automatically_derived]
        impl TryAsEntityRef<#this> for AnyEntity {
            fn try_as_entity_ref(&self) -> Option<&#this> {
                match self {
                    AnyEntity::#this(#this_snake) => return Some(#this_snake),
                    #( AnyEntity::#descendants(#descendants_snake) => return Some(&#descendants_snake.#this_snake), )*
                    _ => (),
                }
                #(
                if let Some(#wildcard_descendants_snake) = <Self as TryAsEntityRef<#wildcard_descendants>>::try_as_entity_ref(self) {
                    return Some(&#wildcard_descendants_snake.#this_snake)
                }
                )*
                None
            }
        
            fn try_as_entity_mut(&mut self) -> Option<&mut #this> {
                match self {
                    AnyEntity::#this(#this_snake) => return Some(#this_snake),
                    #( AnyEntity::#descendants(#descendants_snake) => return Some(&mut #descendants_snake.#this_snake), )*
                    _ => (),
                }
                #(
                if <Self as TryAsEntityRef<#wildcard_descendants>>::try_as_entity_ref(self).is_some() {
                    return <Self as TryAsEntityRef<#wildcard_descendants>>::try_as_entity_mut(self).map(|#wildcard_descendants_snake| &mut #wildcard_descendants_snake.#this_snake)
                }
                )*
                None
            }
        }
    };
    codes.push(code);

    // Generate final code
    let mut final_code = item;
    for code in codes {
        final_code.extend(code);
    }
    proc_macro::TokenStream::from(final_code)
}
