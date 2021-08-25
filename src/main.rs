#![allow(unused_must_use)]
#![allow(unused_variables)]
pub mod preloaded;
use std::fmt::format;
use std::fs::File;
// use std::io::prelude::*;
use deno_core::op_sync;
use deno_core::JsRuntime;
use std::env;


use std::io::Read;


fn read(filename: &str) -> std::string::String {
    // let filename = "src/main.rs";
    // Open the file in read-only mode.
    match File::open(filename) {
        // The file is open (no error).
        Ok(mut file) => {
            let mut content = String::new();

            // Read all the file content into a variable (ignoring the result of the operation).
            file.read_to_string(&mut content).unwrap();

            return content

            // The file is automatically closed when is goes out of scope.
        },
        // Error handling.
        Err(error) => {
            println!("Error opening file {}: {}", filename, error);
            return String::from("No");
        },
    }
}

fn main() {
  // Initialize a runtime instance
  let args: Vec<String> = env::args().collect();
  let mut runtime = JsRuntime::new(Default::default());

  // Register an op for summing a number array.
  runtime.register_op(
    "print",
    // The op-layer automatically deserializes inputs
    // and serializes the returned Result & value
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
  /*
  let contents = fs::read_to_string(filename)
        // .expect("Something went wrong reading the file");
    Ok(contents)
  */
  runtime.sync_ops_cache();

  // Now we see how to invoke the op we just defined. The runtime automatically
  // contains a Deno.core object with several functions for interacting with it.
  // You can find its definition in core.js.
  let command = &args[1];
  let mut filename = String::new();
  if command == "run" {
    filename = format!("{}", args[2]);
  }
  filename.to_owned();
  let contents = read(&filename);
  let pre = preloaded::preloaded::getPreloaded();
  let code = format!("{}{}", pre, contents);
  // println!("{}", code);
  runtime.execute_script("<usage>", &code);
  // let mut contents = String::new();
  // file.read_to_string(&mut contents);
  
  /* 
  runtime
    .execute_script(
      "<usage>",
      r#"

function println(text) {
  Deno.core.opSync('print', text);
}

require.cache = Object.create(null); 

function require(name) {   
    console.log(`Evaluating file ${name}`)
    if (!(name in require.cache)) {
        // console.log(`${name} is not in cache; reading from disk`)
        let code = Deno.core.opSync('readFile', "./index.js")    
        let module = {exports: {}};
        require.cache[name] = module; 
        let wrapper = Function("require, exports, module", code);     
        wrapper(require, module.exports, module);
    }
    // console.log(`${name} is in cache. Returning it...`)
    return require.cache[name].exports;
}

let thing = require('./index.js')
thing()

"#,
    )
    .unwrap();
    */
}