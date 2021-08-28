
#[allow(non_snake_case)]
pub fn getPreloaded(allowconsole: bool) -> &'static str {
  if allowconsole == false {
    let code = r#"
function println(text) {
  Deno.core.opSync('print', text);
}
  
require.cache = Object.create(null); 
  
function require(name) {   
  if (!(name in require.cache)) {
    let code = Deno.core.opSync('readFile', name)    
    let module = {exports: {}};
    require.cache[name] = module; 
    let wrapper = Function("require, exports, module", code);     
    wrapper(require, module.exports, module);
    }
    return require.cache[name].exports;
}
  
    "#;
    return code;
  }
  else if allowconsole == true {
    let code = r#"
      const console = {
        log: (text) => {
          Deno.core.opSync('print', text);
        }
      }
      function require(name) {   
        if (!(name in require.cache)) {
          let code = Deno.core.opSync('readFile', name)    
          let module = {exports: {}};
          require.cache[name] = module; 
          let wrapper = Function("require, exports, module", code);     
          wrapper(require, module.exports, module);
          }
          return require.cache[name].exports;
      }
    "#;
    return code;
  }
  else {
    return r#"
    // none! just for rust
    "#
  }
}

