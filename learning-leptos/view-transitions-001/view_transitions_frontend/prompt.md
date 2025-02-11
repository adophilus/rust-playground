Hi there, I'm trying to learn about view-transitions and rust (leptos). Are you familiar with both of them?


Cool, I'm using v0.6.15 of leptos and it's using v0.3.70 of web_sys under the hood. There's a function in leptos `leptos::document` which is a function that enables access tot he underlying `Document` object. Here's the official docs below:
```rust
/// Returns the Document.
/// This is cached as a thread-local variable, so calling document() multiple times requires only one call out to JavaScript.
pub fn document() -> Document
```

The `Document` here is actually a reference to `web_sys::Document`. In the docs for the `web_sys::Document` there's a function which I'm interested in calling -- `start_view_transition`. Here's the official docs for it:
```rust
/// The startViewTransition() method.
/// MDN Documentation
/// This API requires the following crate features to be activated: Document, ViewTransition
/// This API is unstable and requires --cfg=web_sys_unstable_apis to be activated, as described in the wasm-bindgen guide
pub fn start_view_transition(&self) -> Result<ViewTransition, JsValue>
```

In my `Cargo.toml` file I have this line in there (which I added because I wanted to call the `start_view_transition` function):
```toml
web-sys = { version = "0.3.70", features = ["Document", "ViewTransition"] }
```

But even after that, it didn't work so I thought that I'd add the cfg block (which I got from the `web_sys` docs):
```toml
[build]
rustflags = ["--cfg=web_sys_unstable_apis"]
```

But even after that, I still get this error:
```
error[E0599]: no method named `start_view_transition` found for struct `Document` in the current scope
  --> view_transitions_frontend/src/components/blog.rs:10:20
   |
10 |         document().start_view_transition();
   |                    ^^^^^^^^^^^^^^^^^^^^^ method not found in `Document`

warning: unused variable: `params`
  --> view_transitions_frontend/src/views/blog.rs:12:9
   |
12 |     let params = use_params::<BlogParams>();
   |         ^^^^^^ help: if this is intentional, prefix it with an underscore: `_params`
   |
   = note: `#[warn(unused_variables)]` on by default

For more information about this error, try `rustc --explain E0599`.
warning: `view_transitions_frontend` (bin "view_transitions_frontend") generated 2 warnings
error: could not compile `view_transitions_frontend` (bin "view_transitions_frontend") due to 1 previous error; 2 warnings emitted
2025-02-11T22:28:03.077759Z ERROR ❌ error
error from build pipeline

Caused by:
    0: HTML build pipeline failed (1 errors), showing first
    1: error from asset pipeline
    2: running cargo build
    3: error during cargo build execution
    4: cargo call to executable 'cargo' with args: '["build", "--target=wasm32-unknown-unknown", "--manifest-path", "/home/adophilus/.projects/pe
```


So here's what I want: I want to actually learn about this stuff so I'd like you to give me detailed explanations where necessary.
- I want to know what's going on with this error
- Why does adding `web_sys` v0.3.70 not have an effect? Or even adding the cfg block not have an effect on anything?
- Is there some special thing that happens when a crate re-exports structs from another crate? Is this even termed as a re-export (the `document()` function)?
