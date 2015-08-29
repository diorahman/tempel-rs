#![deny(warnings)]
extern crate hyper;

use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;

use std::env;
use hyper::Client;
use hyper::header::Location;
use hyper::client::RedirectPolicy;

fn main() {
  let file_path = match env::args().nth(1) {
    Some(file_path) => file_path,
      None => {
        println!("Usage: tempel </path/to/file-to-be-nempel-ed>");
        return;
      }
  };

  let path = Path::new(&file_path);
  let mut file = match File::open(&path) {
    Err(why) => panic!("{}", Error::description(&why)),
    Ok(file) => file,
  };

  let mut s = String::new();
  match file.read_to_string(&mut s) {
    Err(why) => panic!("{}", Error::description(&why)),
    Ok(_) => print!(""),
  };

  let mut body = String::from("language=bash&content=");
  body = body + &s;
  let mut client = Client::new();
  client.set_redirect_policy(RedirectPolicy::FollowNone);
  let res = client.post("http://tempel.blankon.in")
    .body(body.as_bytes())
    .send().unwrap();
  println!("{}", res.headers.get::<Location>().unwrap());
  println!("<3");
}
