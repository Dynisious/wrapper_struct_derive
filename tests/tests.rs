//! Author --- daniel.bechaz@gmail.com  
//! Last Moddified --- 2019-03-12

#[macro_use]
extern crate wrapper_struct_derive;


#[cfg(test,)]
mod tests {
  #[derive(DerefMut, AsMut, BorrowMut, From,)]
  struct Foo(u32,);
  #[derive(DerefMut, AsMut, BorrowMut,)]
  struct Bar {
    #[deref]
    #[as_ref]
    #[borrow]
    a: u32,
    _b: i32,
  }

  #[test]
  fn test_deref() {
    let mut foo = Foo(0,);

    *foo = 1;

    let mut bar = Bar { a: 0, _b: 0, };

    *bar = 1;
  }
  #[test]
  fn test_borrow() {
    use std::borrow::BorrowMut;

    let mut foo = Foo(0,);
    let foo: &mut u32 = foo.borrow_mut();

    *foo = 1;

    let mut bar = Bar { a: 0, _b: 0, };
    let bar: &mut u32 = bar.borrow_mut();

    *bar = 1;
  }
  #[test]
  fn test_as_ref() {
    use std::convert::AsMut;

    let mut foo = Foo(0,);
    let foo: &mut u32 = foo.as_mut();

    *foo = 1;

    let mut bar = Bar { a: 0, _b: 0, };
    let bar: &mut u32 = bar.as_mut();

    *bar = 1;
  }
  #[test]
  fn test_from() {
    Foo::from(0,);
  }
}
