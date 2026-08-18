use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=static/css");

    let files = [
        "static/css/base/reset.css",
        "static/css/components/buttons.css",
        "static/css/components/cards.css",
        "static/css/components/forms.css",
        "static/css/components/modals.css",
        "static/css/components/search.css",
        "static/css/components/toasts.css",
        "static/css/components/toolbars.css",
        "static/css/layouts/sidebar.css",
    ];

    let mut merged = String::new();
    for file in files {
        let content = fs::read_to_string(file)
            .unwrap_or_else(|e| panic!("failed to read {file}: {e}"));
        merged.push_str(&format!("/* --- {file} --- */\n"));
        merged.push_str(&content);
        merged.push('\n');
    }

    fs::write("static/css/main.css", merged)
        .expect("failed to write global.css");
}