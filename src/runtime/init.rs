use deno_core::{error::AnyError, JsRuntime};

pub fn init(runtime: &mut JsRuntime) -> Result<(), AnyError> {
    runtime.execute_script("[rt:init.js]", include_str!("./js/init.js"))?;

    Ok(())
}
