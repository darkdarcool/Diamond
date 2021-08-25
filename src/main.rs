#![allow(unused_must_use)]
#![allow(unused_variables)]
pub mod preloaded;
use std::fs::File;
use deno_core::op_sync;
use deno_core::JsRuntime;
use std::env;
use std::io::Read;

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

fn main() {
  let args: Vec<String> = env::args().collect();
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
  let command = &args[1];
  let mut filename = String::new();
  if command == "run" {
    filename = format!("{}", args[2]);
  }
  filename.to_owned();
  let contents = read(&filename);
  let pre = preloaded::preloaded::getPreloaded();
  let code = format!("try {{\n{}{}\n\n}}\ncatch(err) {{ println(`${{err}}`)}}", pre, contents);
  runtime.execute_script("<usage>", &code);
}