use js_rt::runtime;

fn main() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime
        .block_on(runtime::runner::run("./index.js"))
        .unwrap_or_else(|e| eprintln!("error: {e}"))
}
