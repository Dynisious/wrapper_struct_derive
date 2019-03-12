//! Provides procedural macros for deriving common traits on wrapper structs.
//! 
//! * Deref
//! * DerefMut
//! * Borrow
//! * BorrowMut
//! * AsRef
//! * AsMut
//! * From
//! 
//! Applying any of the `Mut` variants of a trait automatically applies the shared variant.
//! 
//! # Example
//! 
//! ```
//! # #![feature(custom_attribute,)] #[macro_use] extern crate wrapper_struct_derive; fn main() {
//! 
//! #[derive(DerefMut)]
//! struct Wrapper1(u32);
//! 
//! let mut val = Wrapper1(0);
//! 
//! *val = 1;
//! 
//! #[derive(DerefMut)]
//! struct Wrapper2 {
//!   #[deref]
//!   a: u32,
//!   b: i32
//! }
//! 
//! let mut val = Wrapper2 { a: 0, b: 0 };
//! 
//! *val = 1;
//! # }
//! ```
//! 
//! Author --- daniel.bechaz@gmail.com  
//! Last Moddified --- 2019-03-12

#![feature(label_break_value,)]

extern crate proc_macro;
#[macro_use]
extern crate syn;

mod deref;
mod borrow;
mod as_ref;
mod from;

use proc_macro::TokenStream;

#[proc_macro_derive(Deref, attributes(deref,),)]
pub fn deref_derive(input: TokenStream,) -> TokenStream {
  let ast = match syn::parse(input,) {
    Ok(ast) => ast,
    Err(e) => panic!("Could not parse `input`: {:?}", e,),
  };
  
  deref::impl_deref(&ast,).0
}

#[proc_macro_derive(DerefMut, attributes(deref,),)]
pub fn deref_mut_derive(input: TokenStream,) -> TokenStream {
  let ast = match syn::parse(input,) {
    Ok(ast) => ast,
    Err(e) => panic!("Could not parse `input`: {:?}", e,),
  };

  deref::impl_deref_mut(&ast,)
}

#[proc_macro_derive(Borrow, attributes(borrow,),)]
pub fn borrow_derive(input: TokenStream,) -> TokenStream {
  let ast = match syn::parse(input,) {
    Ok(ast) => ast,
    Err(e) => panic!("Could not parse `input`: {:?}", e,),
  };
  
  borrow::impl_borrow(&ast,).0
}

#[proc_macro_derive(BorrowMut, attributes(borrow,),)]
pub fn borrow_mut_derive(input: TokenStream,) -> TokenStream {
  let ast = match syn::parse(input,) {
    Ok(ast) => ast,
    Err(e) => panic!("Could not parse `input`: {:?}", e,),
  };

  borrow::impl_borrow_mut(&ast,)
}

#[proc_macro_derive(AsRef, attributes(as_ref,),)]
pub fn as_ref_derive(input: TokenStream,) -> TokenStream {
  let ast = match syn::parse(input,) {
    Ok(ast) => ast,
    Err(e) => panic!("Could not parse `input`: {:?}", e,),
  };
  
  as_ref::impl_as_ref(&ast,).0
}

#[proc_macro_derive(AsMut, attributes(as_ref,),)]
pub fn as_mut_derive(input: TokenStream,) -> TokenStream {
  let ast = match syn::parse(input,) {
    Ok(ast) => ast,
    Err(e) => panic!("Could not parse `input`: {:?}", e,),
  };

  as_ref::impl_as_mut(&ast,)
}

#[proc_macro_derive(From, attributes(from,),)]
pub fn from_derive(input: TokenStream,) -> TokenStream {
  let ast = match syn::parse(input,) {
    Ok(ast) => ast,
    Err(e) => panic!("Could not parse `input`: {:?}", e,),
  };

  from::impl_from(&ast,)
}
