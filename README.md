# zydis-rs to WebAssembly

Demo of using [zydis-rs](https://github.com/zyantific/zydis-rs) in a Rust project that targets WebAssembly.
Naively, this is a little tricky to do, because although Rust supports WebAssembly, the underlying Zydis library is built from C.
So, we use a C/C++ cross compiler that can emit and link WebAssembly objects.

Specifically:
This is a Rust library that exposes a single function `disassemble(Vec<u8>) -> String`.
We use [wasm-pack](https://github.com/rustwasm/wasm-pack) to invoke the compiler to build the library for `wasm32`.
Notably, we override `CC` and `CXX` to use [zig](https://ziglang.org/) rather than clang, so that we build the underyling zydis C library for wasm (the original idea is described [here](https://actually.fyi/posts/zig-makes-rust-cross-compilation-just-work/) and [here](https://dev.to/kristoff/zig-makes-go-cross-compilation-just-work-29ho)).
Next, [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) creates Javascript wrappers that converts to and from Rust types.
Finally, a simple HTML page loads the .wasm file and wrappers and invokes the `disassemble` routine.

## prerequisites

  - [wasm-pack](https://rustwasm.github.io/wasm-pack/)
  - [zig](https://ziglang.org/download/), v0.8.1 is known to work

## build

First, update the path to `zig` in `zcc`:

```sh
#!/bin/sh

/home/user/.bin/zig-linux-x86_64-0.8.1/zig cc -target wasm32-wasi $@
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ here
```

Then invoke `wasm-pack`:

```
CC=(readlink -f zcc) CXX=(readlink -f zcc) wasm-pack build --target web
```

And load the application in your browser: http://localhost:8000/

You should see the output:

```
0x007FFFFFFF400000 push rcx
0x007FFFFFFF400001 lea eax, [rbp-0x01]
0x007FFFFFFF400004 push rax
0x007FFFFFFF400005 push [rbp+0x0C]
0x007FFFFFFF400008 push [rbp+0x08]
0x007FFFFFFF40000B call [0x008000007588A5B1]
0x007FFFFFFF400011 test eax, eax
0x007FFFFFFF400013 js 0x007FFFFFFF42DB15
```

That replicates the example on the [Zydis repository](https://github.com/zyantific/zydis/blob/46cd3e0/README.md#decoder-example).
