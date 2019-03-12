//! Author --- daniel.bechaz@gmail.com  
//! Last Moddified --- 2019-03-12

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span,};
use quote::quote;

pub fn impl_deref(ast: &syn::DeriveInput,) -> (TokenStream, Ident, syn::ExprField,) {
  use syn::{Data, Fields,};

  let name = ast.ident.clone();
  //Find the deref field of the struct.
  let mfield = match &ast.data {
    Data::Struct(data) => 'field: {
      //The attribute tag to get.
      static DEREF: &str = "deref";

      //Get the fields from the struct data.
      let fields = match &data.fields {
        Fields::Named(fields) => &fields.named,
        Fields::Unnamed(fields) => &fields.unnamed,
        _ => break 'field None,
      };

      //If there is only one field then there is no need to choose which one.
      if fields.len() == 1 {
        break 'field fields.first()
          //Pair the field with its index in case this is a tuple struct.
          .map(|pair,| (0, pair.into_value().clone(),),)
      }
      
      //Search for the `deref` attribute.
      fields.iter().enumerate()
      .find(|(_, field,),| !field.attrs.is_empty()
        && field.attrs.iter().any(|attr,| {
          attr.path.segments.len() == 1
          && format!("{}", attr.path.segments[0].ident,) == DEREF
        },),
      )
      //Clone the field value.
      .map(|(index, field,),| (index as u32, field.clone(),),)
    },
    _ => panic!("`Deref` macro can only be used on structs",),
  };
  //The `self` for the field expression.
  let base = Box::new(syn::Expr::Verbatim(
    syn::ExprVerbatim { tts: quote! { self }.into(), }
  ));
  //Unpack the field and type.
  let (field, deref,) = match mfield {
    Some((index, field,)) => {
      let ident = match field.ident {
        //Named field.
        Some(ident) => syn::ExprField {
          base,
          attrs: Vec::new(),
          dot_token: Token![.]([Span::call_site()]),
          member: syn::Member::Named(ident),
        },
        //Unnamed field.
        None => syn::ExprField {
          base,
          attrs: Vec::new(),
          dot_token: Token![.]([Span::call_site()]),
          member: syn::Member::Unnamed(syn::Index {
            index,
            span: Span::call_site(),
          }),
        },
      };

      (ident, field.ty,)
    },
    None => panic!("The deref field to use for deref could not be identified; if there is more than one field add the #[deref] attribute too the field to use.",),
  };
  //Expand the definition for Deref.
  let expanded = quote! {
    impl std::ops::Deref for #name {
      type Target = #deref;

      #[inline]
      fn deref(&self,) -> &Self::Target { &#field }
    }
  };

  (expanded.into(), name, field,)
}

pub fn impl_deref_mut(ast: &syn::DeriveInput,) -> TokenStream {
  use std::iter::Extend;

  let (mut deref, name, field,) = impl_deref(ast,);
  let deref_mut = quote! {
    impl std::ops::DerefMut for #name {
      #[inline]
      fn deref_mut(&mut self,) -> &mut Self::Target { &mut #field }
    }
  };

  deref.extend(TokenStream::from(deref_mut,),);
  deref
}
