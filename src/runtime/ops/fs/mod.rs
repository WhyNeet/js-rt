use deno_core::{error::AnyError, extension, op2};

#[op2(async)]
#[string]
pub async fn read_file(#[string] path: String) -> Result<String, AnyError> {
    let contents = tokio::fs::read_to_string(path).await?;

    Ok(contents)
}

#[op2(async)]
pub async fn write_file(
    #[string] path: String,
    #[string] contents: String,
) -> Result<(), AnyError> {
    tokio::fs::write(path, contents).await?;

    Ok(())
}

#[op2(async)]
pub async fn remove_file(#[string] path: String) -> Result<(), AnyError> {
    tokio::fs::remove_file(path).await?;

    Ok(())
}

extension!(
    fs,
    ops = [read_file, write_file, remove_file],
    esm_entry_point = "ext:fs/fs.js",
    esm = [dir "src/runtime/ops/fs", "fs.js"],
    docs = "fs module"
);
