extern crate rustree;

#[test]
fn it_works() {
  assert!(true);
}

#[test]
fn init_bst() {
  let bst: rustree::Bst<u8> = std::default::Default::default();

  bst.insert(19);
  let r : Result<&u8, u8> = bst.get_payload();
  assert_eq!(r, Ok(&19));
}

