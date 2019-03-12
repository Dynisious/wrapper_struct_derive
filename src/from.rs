//! Author --- daniel.bechaz@gmail.com  
//! Last Moddified --- 2019-03-12

use proc_macro::TokenStream;
use quote::quote;

pub fn impl_from(ast: &syn::DeriveInput,) -> TokenStream {
  use syn::{Data, Fields,};

  let name = &ast.ident;
  //Find the from field of the struct.
  let mfield = match &ast.data {
    Data::Struct(data) => 'field: {
      //The attribute tag to get.
      static DEREF: &str = "from";

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
          .map(|pair,| pair.into_value(),)
      }
      
      //Search for the `from` attribute.
      fields.iter()
      .find(|field,| !field.attrs.is_empty()
        && field.attrs.iter().any(|attr,| {
          attr.path.segments.len() == 1
          && format!("{}", attr.path.segments[0].ident,) == DEREF
        },),
      )
    },
    _ => panic!("`From` macro can only be used on structs",),
  };
  //Unpack the field and type.
  let (body, from,) = match mfield {
    Some(field) => {
      let body = match &field.ident {
        //Named field.
        Some(ident) => quote! {
          #name { #ident: from, }
        },
        //Unnamed field.
        None => quote! {
          #name(from,)
        },
      };

      (body, field.ty.clone(),)
    },
    None => panic!("The from field to use for from could not be identified; if there is more than one field add the #[from] attribute too the field to use.",),
  };
  //Expand the definition for From.
  let expanded = quote! {
    impl std::convert::From<#from> for #name {
      #[inline]
      fn from(from: #from,) -> #name { #body }
    }
  };

  expanded.into()
}
