use cap_async_std::{ambient_authority, fs};
use tide_naive_static_files::{serve_static_files, StaticRootDir};

struct AppState {
    static_root_dir: fs::Dir,
}

impl StaticRootDir for AppState {
    fn root_dir(&self) -> &fs::Dir {
        &self.static_root_dir
    }
}

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let mut app = tide::with_state(AppState {
        // Obtain the directory using "ambient" permissions. All other
        // operations on `Dir` are sandboxed to stay within this directory.
        static_root_dir: fs::Dir::open_ambient_dir(
            "./examples/static-example-files/",
            ambient_authority(),
        )
        .await?,
    });

    app.at("static/*path")
        .get(|req| async { serve_static_files(req).await.unwrap() });

    app.listen("127.0.0.1:8000").await
}
