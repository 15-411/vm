extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Ident};
use quote::quote;


#[proc_macro_derive(DebugFromDisplay)]
pub fn debug_from_display(input: TokenStream) -> TokenStream {
  let mut input = parse_macro_input!(input as DeriveInput);
  let name = &input.ident;
  let generics = &mut input.generics;

  let typ: Vec<Ident> = generics.type_params().map(|t| t.ident.clone()).collect();
  let (impl_generics, ty_generics, _) = generics.split_for_impl();
  
  let where_clause = if typ.len() == 0 {
    None
  } else {
    let first = typ.get(0).clone().unwrap();
    Some(quote!(where #first: ::std::fmt::Debug + ::std::fmt::Display))
  };

  let result = quote! {
    impl #impl_generics ::std::fmt::Debug for #name #ty_generics #where_clause {
      fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(self, f)
      }
    }
  };
  
  TokenStream::from(result)
}


#[proc_macro_derive(DisplayFromDebug)]
pub fn display_from_debug(input: TokenStream) -> TokenStream {
  let mut input = parse_macro_input!(input as DeriveInput);
  let name = &input.ident;
  let generics = &mut input.generics;

  let typ: Vec<Ident> = generics.type_params().map(|t| t.ident.clone()).collect();
  let (impl_generics, ty_generics, _) = generics.split_for_impl();
  
  let where_clause = if typ.len() == 0 {
    None
  } else {
    let first = typ.get(0).clone().unwrap();
    Some(quote!(where #first: ::std::fmt::Display + ::std::fmt::Debug))
  };

  let result = quote! {
    impl #impl_generics ::std::fmt::Display for #name #ty_generics #where_clause {
      fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        ::std::fmt::Debug::fmt(self, f)
      }
    }
  };
  
  TokenStream::from(result)
}
