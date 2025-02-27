# tide-naive-static-files, ported to cap-async-std

This is a port of [tide-naive-static-files] to [cap-async-std]. By using
[`cap_async_std::fs::Dir`], it refuses to follow symlinks which lead to paths
outside of the directory it's configured to serve from.

[tide-naive-static-files]: https://github.com/eignnx/tide-naive-static-files
[cap-async-std]: https://crates.io/crates/cap-async-std
[`cap_async_std::fs::Dir`]: https://docs.rs/cap-async-std/latest/cap_async_std/fs/struct.Dir.html

Original README from upstream repository follows...

A simple static file serving component for Rust's Tide web framework.

## Acknowledgements

This code is based heavily on [this archived example](https://github.com/http-rs/tide/blob/4aec5fe2bb6b8202f7ae48e416eeb37345cf029f/backup/examples/staticfile.rs).

This crate is not officially associated with the [`tide`](https://github.com/http-rs/tide) project, it's more of an interim solution while `tide` is still in a state of (relative) flux.

## Note on Version Numbers

Mistakes were made when initially selecting version numbers for this crate. In the Rust ecosystem, a 1.0.0 release generally means the crate is _fit for production._ **This crate makes no such claim.** It would be best to "divide by ten" when looking at the crate's version number (i.e. 2.0.1 should be thought of as 0.2.0.1).

## Example

To use the library:

1. Define the route to host your assets under
2. Stip the prefix so the routes match your files
3. Set up a `get` endpoint with the `StaticFilesEndpoint` making sure the `root` represents the path from where you run the server to the root of your assets

```rust
use async_std::task;
use tide_naive_static_files::StaticFilesEndpoint;

fn main() {
    let mut app = tide::new();

    app.at("/static") // 1.
       .strip_prefix() // 2
       .get(StaticFilesEndpoint {
            root: "./examples/".into(), // 3.
        });

    task::block_on(app.listen("127.0.0.1:8000")).unwrap();
}
```

## Contributors

- [eignnx](https://github.com/eignnx)
- Felipe Seré, [felipesere](https://github.com/felipesere)

If you're interested in contributing to the project, please see our [CONTRIBUTING.md](https://github.com/eignnx/tide-naive-static-files/blob/master/CONTRIBUTING.md) file!
