use std::collections::HashMap;

use proc_macro::{TokenStream, TokenTree, Ident, Group};
use convert_case::{Case, Casing};
use proc_macro_error::*;

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
        Some(TokenTree::Punct(punct)) if punct.as_char() == '#' => {
            items.next();
            match items.next() {
                Some(TokenTree::Ident(ident)) if ident.to_string() == "pub" => (),
                Some(other) => panic!("expected struct to be public, found {:?}", other),
                None => panic!("expected public struct, found nothing"),
            }
        }
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
        codes.push(code);
    }

    // Generate final code
    let mut final_code = item;
    for code in codes {
        final_code.extend(code);
    }
    final_code
}

#[proc_macro_attribute]
pub fn inheritable(_attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("{:?}", item);

    // Get struct name
    let mut items = item.clone().into_iter();
    match items.next() {
        Some(TokenTree::Ident(ident)) if ident.to_string() == "pub" => (),
        Some(TokenTree::Punct(punct)) if punct.as_char() == '#' => {
            items.next();
            match items.next() {
                Some(TokenTree::Ident(ident)) if ident.to_string() == "pub" => (),
                Some(other) => panic!("expected struct to be public, found {:?}", other),
                None => panic!("expected public struct, found nothing"),
            }
        }
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

    // Generate implementation
    let code: TokenStream = r#"
        pub trait ThisDescendant {
            fn get_this(&self) -> &This;
            fn get_this_mut(&mut self) -> &mut This;
        }
        
        impl ThisDescendant for This {
            fn get_this(&self) -> &This { self }
            fn get_this_mut(&mut self) -> &mut This { self }
        }
    "#.parse().unwrap();
    let mut to_replace = HashMap::new();
    to_replace.insert("This", struct_name.clone());
    to_replace.insert("ThisDescendant", Ident::new(&format!("{}Descendant", struct_name), struct_name.span()));
    to_replace.insert("get_this", Ident::new(&format!("get_{}", struct_name.to_string().to_case(Case::Snake)), struct_name.span()));
    to_replace.insert("get_this_mut", Ident::new(&format!("get_{}_mut", struct_name.to_string().to_case(Case::Snake)), struct_name.span()));
    let mut code = code.clone().into_iter().collect::<Vec<_>>();
    for element in &mut code {
        replace_idents(element, &to_replace);
    }
    let code: TokenStream = code.into_iter().collect();

    // Assemble final code
    let mut final_code = item;
    final_code.extend(code);
    final_code
}

#[allow(non_snake_case)]
#[proc_macro_attribute]
#[proc_macro_error]
pub fn MinecraftEntity(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut inherited = Vec::new();
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
            "parents" => {
                let Some(token_tree) = attrs.next() else { abort!(ident.span(), "expected group after parents") };
                let TokenTree::Group(group) = token_tree else { abort!(token_tree.span(), "expected group") };
                let mut group_attrs = group.stream().into_iter().peekable();
                while let Some(ident) = group_attrs.next() {
                    let TokenTree::Ident(ident) = ident else { abort!(ident.span(), "expected ident") };
                    inherited.push(ident);
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
                    if params.delimiter() != proc_macro::Delimiter::Parenthesis {
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
    let mut hierarchy = inherited.clone();
    hierarchy.insert(0, struct_name.clone());

    let mut to_replace = HashMap::new();
    to_replace.insert("This", struct_name.clone());
    to_replace.insert("ThisDescendant", Ident::new(&format!("{}Descendant", struct_name), struct_name.span()));
    to_replace.insert("ThisMethods", Ident::new(&format!("{}Methods", struct_name), struct_name.span()));
    to_replace.insert("ThisExt", Ident::new(&format!("{}Ext", struct_name), struct_name.span()));
    to_replace.insert("get_this", Ident::new(&format!("get_{}", struct_name.to_string().to_case(Case::Snake)), struct_name.span()));
    to_replace.insert("get_this_mut", Ident::new(&format!("get_{}_mut", struct_name.to_string().to_case(Case::Snake)), struct_name.span()));

    if !inherited.is_empty() {
        // Generate code for parent
        let parent = inherited.remove(0);
        let code: TokenStream = r#"
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

        // Generate code for higher inheritance levels
        let code: TokenStream = r#"
            impl InheritedDescendant for This {
                fn get_inherited(&self) -> &Inherited { self.parent.get_inherited() }
                fn get_inherited_mut(&mut self) -> &mut Inherited { self.parent.get_inherited_mut() }
            }
        "#.parse().unwrap();
        for inherited in inherited {
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
    }

    if inheritable {
        // Generate descendant trait
        let code: TokenStream = r#"
            pub trait ThisDescendant {
                fn get_this(&self) -> &This;
                fn get_this_mut(&mut self) -> &mut This;
            }
            
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
        trait ThisExt: Sized + Into<Handler<This>> {
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
    for ascendant in hierarchy.iter().peekable() {
        let code: TokenStream = r#"
            const ASCENDANT_METHODS_FOR_THIS: &AscendantMethods = &AscendantMethods {

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
                0 => r#"method: |s| Box::pin(s.assume_other::<Ascendant>().method()),"#,
                1 => r#"method: |s, arg1| Box::pin(s.assume_other::<Ascendant>().method(arg1)),"#,
                2 => r#"method: |s, arg1, arg2| Box::pin(s.assume_other::<Ascendant>().method(arg1, arg2)),"#,
                3 => r#"method: |s, arg1, arg2, arg3| Box::pin(s.assume_other::<Ascendant>().method(arg1, arg2, arg3)),"#,
                4 => r#"method: |s, arg1, arg2, arg3, arg4| Box::pin(s.assume_other::<Ascendant>().method(arg1, arg2, arg3, arg4)),"#,
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
        codes.push(code);
    }



    // Generate final code
    let mut final_code = item;
    for code in codes {
        final_code.extend(code);
    }
    final_code
}
