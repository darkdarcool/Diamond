
#[allow(non_snake_case)]
pub fn getPreloaded() -> &'static str {
  let code = r#"
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
  
  "#;
  return code;
}