
pub struct Bst<T> {
  left : std::option::Option<std::boxed::Box<Bst<T>>>,
  right : std::option::Option<std::boxed::Box<Bst<T>>>,
  payload : std::option::Option<std::boxed::Box<T>>
}

impl<T> std::default::Default for Bst<T> {
  fn default() -> Bst<T> {
    Bst::<T> {
      left : None,
      right : None,
      payload: None
    }
  }
}

