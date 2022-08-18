# Working with Skyline
Mod Ardain uses [Skyline](https://github.com/skyline-dev/skyline) to hook into the game's executable.

## Hooks
Skyline lets you redirect the program execution to user-provided callbacks.

To replace a function, you can use a function hook:
```rs
#[hook(replace = function_path)]
// or
#[hook(offset = 0xdeadbeef)]
// use the original function's signature
fn hook_name(param_1: u32, param_2: i32) -> f32 {
    if param_1 == 0 {
        2.0
    } else {
        // Optional: call the original function
        call_original!(param_1, param_2)
    }
}
```

If you'd like to hook into a specific instruction, you can use inline hooks. 
```rs
#[hook(offset = 0xcafebabe, inline)]
fn inline_hook(ctx: &mut InlineCtx) {
    // Manipulate registers...
    *ctx.registers[0].as_mut() = 0;
    // Inline assembly also works
    asm!("ret");
}
```

To register hooks, use
```rs
skyline::install_hooks!(hook_name, inline_hook /*, ... */);
```

## Patching memory
You can also replace memory directly.  
[Docs](https://ultimate-research.github.io/skyline-rs-template/doc/skyline/patching/index.html)

## Configurable offsets
It is advised to use Mod Ardain's hook utilities that automatically load offsets for the game version.

With an instance of a `FfiConfig` you can:
```rs
let hook = config.get_hook("input").unwrap();
hook.patch_inline(&platform_data, hook_callback);
```