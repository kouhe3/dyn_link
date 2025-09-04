use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, ItemForeignMod, spanned::Spanned};
#[proc_macro_attribute]
pub fn dyn_link(attr: TokenStream, item: TokenStream) -> TokenStream {
    let meta = syn::parse_macro_input!(attr as syn::Meta);
    let libname = if let syn::Meta::Path(v) = meta {
        if let Some(name) = v.get_ident() {
            name.to_string()
        } else {
            panic!("libname is not a ident")
        }
    } else {
        panic!("libname is not a path")
    };
    let extern_block = syn::parse_macro_input!(item as ItemForeignMod);
    let mut result = Vec::new();
    for f in extern_block.items {
        match f {
            syn::ForeignItem::Fn(f) => {
                let sig = f.sig;
                let fn_name = &sig.ident;
                let fn_name_str = fn_name.to_string();
                let inputs = &sig.inputs;
                let output = &sig.output;
                let arg_names = inputs
                    .iter()
                    .filter_map(|arg| match arg {
                        syn::FnArg::Typed(pat) => {
                            if let syn::Pat::Ident(ident) = &*pat.pat {
                                Some(ident.ident.clone())
                            } else {
                                None
                            }
                        }
                        syn::FnArg::Receiver(_) => Some(Ident::new("self", arg.span())),
                    })
                    .collect::<Vec<Ident>>();
                let call_args = &*arg_names;
                let expanded = quote! {
                    #sig {
                        unsafe {
                            use libloading::{Library, Symbol};
                            let lib = Library::new(#libname).expect("Failed to load library");
                            let symbol: Symbol<unsafe extern "C" fn (#inputs) #output> = lib.get(#fn_name_str.as_bytes())
                                .expect("Failed to load symbol");
                            symbol(#(#call_args),*)
                        }
                    }
                };
                result.push(expanded);
            }
            _ => {}
        }
    }
    quote! {
        #(#result)*
    }
    .into()
}
