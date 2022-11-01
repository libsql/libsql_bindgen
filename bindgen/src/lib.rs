use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use quote::ToTokens;
use std::borrow::Borrow;
use std::borrow::BorrowMut;
use syn::{parse, parse_str, FnArg, ItemFn, Pat, Stmt};

fn gen_libsql_type(ty: &syn::Type) -> Box<syn::Type> {
    match ty.to_owned().into_token_stream().to_string().as_str() {
        "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "bool" | "char" => {
            Box::new(syn::Type::from(parse_str::<syn::TypePath>("i64").unwrap()))
        }
        "f32" | "f64" => Box::new(syn::Type::from(parse_str::<syn::TypePath>("f64").unwrap())),
        _ => Box::new(syn::Type::from(parse_str::<syn::TypePath>("i32").unwrap())),
    }
}

fn gen_from_expr(ty: &syn::Type, id: &str) -> String {
    let type_str = ty.to_owned().into_token_stream().to_string();
    match type_str.as_str() {
        "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "bool" | "char" => {
            format!("{} as i64", id)
        }
        "f32" | "f64" => format!("{} as f64", id),
        _ => format!("<{}>::from_libsql_type({})", type_str, id),
    }
}

fn gen_into_expr(return_type_str: &str, expr: syn::Expr) -> syn::Expr {
    let expr_str = expr.into_token_stream().to_string();
    parse_str::<syn::Expr>(&match return_type_str {
        "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "bool" | "char" => {
            format!("{} as i64", expr_str)
        }
        "f32" | "f64" => format!("{} as f64", expr_str),
        _ => format!("{}.into_libsql_type()", expr_str),
    })
    .unwrap()
}

/* libSQL bindgen transforms a native Rust function into a form compilable to Wasm
** and immediately usable as a libSQL user-defined function.
** It follows the following ABI rules for libSQL types (INTEGER, FLOAT, STRING, BLOB, NULL):
**  1. integers are passed as i64
**  2. floats are passed as f64
**  3. Strings, blobs and nulls are pointers,
**     with the first byte indicating the type - SQLITE_TEXT, SQLITE_BLOB or SQLITE_NULL
**     and then:
**       - strings are encoded as null-terminated strings
**       - blobs are encoded as [4 bytes of size information][data]
**       - nulls are encoded as nothing, because the type byte already indicates it's null
*/    
#[proc_macro_attribute]
pub fn libsql_bindgen(_attrs: TokenStream, item: TokenStream) -> TokenStream {
    let input = match parse::<ItemFn>(item) {
        Ok(i) => i,
        Err(_) => {
            return TokenStream::from(
                syn::Error::new(
                    Span::call_site(),
                    "libsql_bindgen operates on function definitions only",
                )
                .to_compile_error(),
            )
        }
    };

    let mut native_sig = input.sig.clone();
    let mut generated_sig = input.sig.clone();
    native_sig.ident = syn::Ident::new(
        format!("__libsql_native_{}", generated_sig.ident).as_str(),
        Span::call_site(),
    );
    // Translate parameter types
    for raw_param in &mut generated_sig.inputs {
        if let &mut FnArg::Typed(ref mut param) = raw_param {
            param.ty = gen_libsql_type(&param.ty);
            if let &mut Pat::Ident(ref mut id) = param.pat.borrow_mut() {
                id.mutability = Option::None;
            }
        }
    }
    // Translate the return type
    let mut return_type_str = String::new();
    if let &syn::ReturnType::Type(_, ref ty) = &generated_sig.output {
        return_type_str = ty.to_owned().into_token_stream().to_string();
        generated_sig.output =
            syn::ReturnType::Type(syn::token::RArrow::default(), gen_libsql_type(ty));
    }

    // Copy the native function body
    let native_block = &input.block;
    let mut generated_block = syn::Block {
        brace_token: input.block.brace_token,
        stmts: Vec::<Stmt>::new(),
    };

    // Generate the exported function body
    let mut raw_ret_expr =
        parse_str::<syn::Expr>(&format!("{}()", &native_sig.ident.to_string())).unwrap();
    if let &mut syn::Expr::Call(ref mut call) = &mut raw_ret_expr {
        for raw_param in &native_sig.inputs {
            if let &FnArg::Typed(ref param) = raw_param {
                if let &Pat::Ident(ref id) = param.pat.borrow() {
                    let from_expr =
                        parse_str::<syn::Expr>(&gen_from_expr(&param.ty, &id.ident.to_string()))
                            .unwrap();
                    call.args.push(from_expr);
                }
            }
        }
    }
    let ret_expr = gen_into_expr(&return_type_str, raw_ret_expr);
    generated_block.stmts.push(Stmt::Expr(ret_expr));

    let ts = TokenStream::from(quote! {
        extern crate libsql_wasm_abi;
        use crate::libsql_wasm_abi::*;

        #native_sig
        #native_block

        #[no_mangle]
        pub #generated_sig
        #generated_block
    });
    println!("Generated binding:\n{}", ts);
    ts
}
