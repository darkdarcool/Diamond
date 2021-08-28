#![allow(unused_must_use)]
#![allow(unused_variables)]
pub mod preloaded;
use std::fs::File;
use deno_core::op_sync;
use deno_core::JsRuntime;
use std::env;
use std::io::Read;
pub mod cli;
use cli::cli::parse;

fn read(filename: &str) -> std::string::String {
    match File::open(filename) {
        Ok(mut file) => {
            let mut content = String::new();

            file.read_to_string(&mut content).unwrap();

            return content

        },
        Err(error) => {
            println!("Error opening file {}: {}", filename, error);
            return String::from("No");
        },
    }
}

fn runtime(filename: &str, allowconsole: bool) {
  // let args: Vec<String> = env::args().collect();
  let mut runtime = JsRuntime::new(Default::default());
  
  runtime.register_op(
    "print",
    op_sync(|_state, text: String, _: ()| {
      // Sum inputs
      println!("{}", text);
      Ok(())
    }),
  );
  

  runtime.register_op(
    "readFile",
    // The op-layer automatically deserializes inputs
    // and serializes the returned Result & value
    op_sync(|_state, filename: String, _: ()| {
      let mut file = File::open(filename)?;
      let mut contents = String::new();
      file.read_to_string(&mut contents)?;
      Ok(contents)
    }),
  );
  
  runtime.sync_ops_cache();
  filename.to_owned();
  let contents = read(&filename);
  let pre = preloaded::preloaded::getPreloaded(allowconsole);
  let code = format!("try {{\n{}{}\n\n}}\ncatch(err) {{ println(`${{err}}`)}}", pre, contents);
  runtime.execute_script("<usage>", &code);
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let parsed = parse(args);
  if parsed.action == "run" {
    runtime(&parsed.filename, parsed.allow_console);
  }
  // runtime("./tests/require.js");
}