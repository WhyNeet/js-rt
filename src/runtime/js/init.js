((globalThis) => {
  globalThis.rt = {
    ...globalThis.rt,
    core: Deno.core
  }
})(globalThis);