use std::{env, rc::Rc};

use deno_core::{error::AnyError, PollEventLoopOptions};

use super::{init, ops::fs::fs};

pub async fn run(file_path: &str) -> Result<(), AnyError> {
    let main_module = deno_core::resolve_path(file_path, &env::current_dir()?)?;
    let mut runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        extensions: vec![fs::init_ops_and_esm()],
        ..Default::default()
    });

    init::init(&mut runtime)?;

    let module_id = runtime.load_main_es_module(&main_module).await?;
    let result = runtime.mod_evaluate(module_id);

    runtime
        .run_event_loop(PollEventLoopOptions::default())
        .await?;
    result.await
}
