extern crate proc_macro;
use crate::proc_macro::TokenStream;
use proc_macro2 as pm2;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_derive(Foo)]
pub fn my_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    let output = impl_hello_macro(&ast);
    output.into()
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> pm2::TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Foo for #name {
        }
    };
    gen.into()
}


#[proc_macro_attribute]
pub fn log(attr: TokenStream, item: TokenStream) -> TokenStream {
    impl_log_macro(attr.into(), item.into()).into()
}

fn impl_log_macro(attr: pm2::TokenStream, item: pm2::TokenStream) -> pm2::TokenStream {
    // let attr = syn::parse_attr(attr);
    let item_fn: syn::ItemFn = syn::parse2(item).expect("Input is not a function");
    let name = item_fn.ident;
    let vis = item_fn.vis;
    let decl = item_fn.decl;
    let generics = decl.generics;
    let inputs = decl.inputs;
    inputs.iter().fold(vec![], |lst: Vec<syn::Ident>, input| match input {
        syn::FnArg::SelfValue(o) => lst,
        syn::FnArg::SelfRef(o) => lst,
        syn::FnArg::Captured(o) => lst,
        syn::FnArg::Inferred(pat) => lst,
        _ => lst,
    });
    let output = decl.output;
    let block = item_fn.block;
    let gen = quote! {
        #vis fn #name<#generics>(#inputs) -> #output {
            let a = #name();
            dbg!(a);
            return a;
            fn #name<#generics>(#inputs) -> #output {
                #block
            }
        }
    };
    gen.into()
}
