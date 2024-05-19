globalThis.rt = {
    ...(globalThis.rt ?? {}),
    fs: {
        readFile: Deno.core.ops.read_file,
        writeFile: Deno.core.ops.write_file,
        removeFile: Deno.core.ops.remove_file
    }
}