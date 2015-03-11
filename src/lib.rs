
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

impl<T> Bst<T> {
  pub fn insert(&mut self, t: T) {
    self.payload = Some(Box::new(t));
  }

  pub fn get_payload<'a>(&'a self) -> Result<&'a T, u8> {
    match self.payload {
      None => Err(22),
      Some(ref p) => Ok(&*p)
    }
  }

  pub fn contains(&self, t: T) -> bool {
    true
  }
}

