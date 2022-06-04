use crate::Cli;

use rocket::response::content::RawHtml;
use rocket::State;

use std::fs;
use std::path::{Path, PathBuf};

#[get("/cwd/<file..>")]
pub fn current_dir(file: PathBuf, args: &State<Cli>) -> RawHtml<String> {
    let file_to_get = Path::new(".").join(file);
    info!("File to get: {}", file_to_get.to_str().unwrap());
    let html = if file_to_get.is_file() {
        format!("<pre>{}</pre>", fs::read_to_string(file_to_get).unwrap())
    } else if file_to_get.is_dir() {
        let a = file_to_get
            .read_dir()
            .expect("Could not read dir")
            .map(|it| it.unwrap())
            .filter(|it| !it.file_name().to_str().unwrap().starts_with("."))
            .map(|it| {
                let file = it.file_name();
                let path = it.path();
                format!("<a href={:?}>{:?}</a>", path, file)
            })
            .collect::<Vec<String>>()
            .join("<br/>\n");
        format!(
            r#"
        <head>
            <base href="http://127.0.0.1:{}/cwd/">
        </head>
        <body>
        {}
        </body>"#,
            args.port, a
        )
    } else {
        r#"
        <div role="main" align="center">
            <h1>404: Not Found</h1>
            <p>The requested resource could not be found.</p>
            <hr>
        </div>
        "#
        .to_string()
    };

    RawHtml(html)
}
