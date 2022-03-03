//
// init.rs
// motive
// 
// Author: wess (me@wess.io)
// Created: 02/11/2022
// 
// Copywrite (c) 2022 Wess.io
//

use std::{
  env,
  path::Path,
  fs::File,
  io::{
    self,
    Write,
    prelude::*,
  },
  collections::HashMap,
};

use colored::*;

use clap::{App};

const LUA_STUB:&str = r#"
-- Generated by Motive

function example_function()
  print("Hello world")
end

task example -- This is a task description
  example_function()

  exec("ls -la")
end

"#;

pub struct Init {}

impl Init {
  pub fn app() -> App<'static> {
    App::new("init")
    .about("Creates a new Motive Manifest file in current directory")
  }

  pub fn run() {
    let cwd = env::current_dir().unwrap();
    let file_path = format!("{}/manifest", cwd.to_str().unwrap());
    let path = Path::new(file_path.as_str());

    let mut file = match File::create(&path) {
      Ok(file) => file,
      Err(e) => {
        console_panic!("Unable to create Manifest file: {}", e);
      }
    };

    match file.write_all(LUA_STUB.as_bytes()) {
      Ok(_) => {
        console_success!("Created Manifest file: {}", file_path);
      }
      Err(e) => {
        console_error!("Unable to write to Manifest file: {}", e);
      }
    }

  }
}