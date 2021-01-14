use cap_async_std::{ambient_authority, fs};
use tide_naive_static_files::StaticFilesEndpoint;

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let mut app = tide::new();
    app.at("/static").strip_prefix().get(StaticFilesEndpoint {
        // Obtain the directory using "ambient" permissions. All other
        // operations on `Dir` are sandboxed to stay within this directory.
        root: fs::Dir::open_ambient_dir("./examples/static-example-files/", ambient_authority())
            .await?,
    });

    app.listen("127.0.0.1:8000").await
}
