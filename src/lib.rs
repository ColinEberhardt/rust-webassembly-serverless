extern crate cfg_if;
extern crate wasm_bindgen;
extern crate js_sys;

mod utils;

use js_sys::{Object, Reflect, Promise};
use wasm_bindgen::prelude::*;

pub fn response(status: u8, body: String) -> Object {
  let object = Object::new();
  Reflect::set(&object.as_ref(), &"statusCode".into(), &status.into()).unwrap();
  Reflect::set(&object.as_ref(), &"body".into(), &body.into()).unwrap();
  return object;
}

#[wasm_bindgen]
pub fn handler() -> Promise {
  let mut string = String::new();
  string.push_str("Hello, rust-lambda!");
  let res = response(200, string);
  return Promise::resolve(res.as_ref());
}
