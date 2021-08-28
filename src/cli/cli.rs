use std::{process};
extern crate colored; // not needed in Rust 2018
use colored::*;

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[derive(Debug)]
pub struct parsed {
  pub action: String,
  pub filename: String,
  pub allow_console: bool
}

fn error(reason: &str, message: &str) -> () {
  println!("[{}]: {}", reason.red().bold(), message);
  process::exit(0);
}
#[allow(unreachable_code)]
pub fn parse(mut args: Vec<String>) -> parsed {
  let mut action:String;
  args.remove(0);
  if args.get(0) == Some(&String::from("run")) {
    if args.get(1) != None {
      let filename = &args[1];
      let mut allow_console = false;
      if args.iter().any(|i| i =="--allow-console") {
        allow_console = true;
      }
      let options: parsed = parsed {
        action: String::from("run"),
        filename: filename.to_string(),
        allow_console
      };
      return options;
    }

    else {
      error("Filename not found", "The filename must be specified to run");
      let options: parsed = parsed { action: String::from(""), filename: String::from(""), allow_console: false };
      return options
    }
  }
  if args.get(0) == None { 
    println!(r#"
      Help command!
    "#);
    process::exit(0);
    let options: parsed = parsed { action: String::from(""), filename: String::from(""), allow_console: false };
    return options;
  }
  else {
    match args.get(0) {
        Some(x) => {
          error("Bad arg", &format!("Arg {} does not exist", x)); 
        },
        None => (),
    }
    let options: parsed = parsed { action: String::from(""), filename: String::from(""), allow_console: false };
    return options;
  }
}