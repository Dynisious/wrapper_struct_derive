//! Author --- daniel.bechaz@gmail.com  
//! Last Moddified --- 2019-03-12

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span,};
use quote::quote;
use syn::Type;

pub fn impl_borrow(ast: &syn::DeriveInput,) -> (TokenStream, Ident, Type, syn::ExprField,) {
  use syn::{Data, Fields,};

  let name = ast.ident.clone();
  //Find the borrow field of the struct.
  let mfield = match &ast.data {
    Data::Struct(data) => 'field: {
      //The attribute tag to get.
      static DEREF: &str = "borrow";

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
      
      //Search for the `borrow` attribute.
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
    _ => panic!("`Borrow` macro can only be used on structs",),
  };
  //The `self` for the field expression.
  let base = Box::new(syn::Expr::Verbatim(
    syn::ExprVerbatim { tts: quote! { self }.into(), }
  ));
  //Unpack the field and type.
  let (field, borrow,) = match mfield {
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
    None => panic!("The borrow field to use for borrow could not be identified; if there is more than one field add the #[borrow] attribute too the field to use.",),
  };
  //Expand the definition for Borrow.
  let expanded = quote! {
    impl std::borrow::Borrow<#borrow> for #name {
      #[inline]
      fn borrow(&self,) -> &#borrow { &#field }
    }
  };

  (expanded.into(), name, borrow, field,)
}

pub fn impl_borrow_mut(ast: &syn::DeriveInput,) -> TokenStream {
  use std::iter::Extend;

  let (mut expanded, name, borrow, field,) = impl_borrow(ast,);
  let borrow_mut = quote! {
    impl std::borrow::BorrowMut<#borrow> for #name {
      #[inline]
      fn borrow_mut(&mut self,) -> &mut #borrow { &mut #field }
    }
  };

  expanded.extend(TokenStream::from(borrow_mut,),);
  expanded
}
