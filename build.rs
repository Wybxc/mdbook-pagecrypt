use std::path::Path;

fn main() {
    let out = std::env::var("OUT_DIR").unwrap();
    let out = Path::new(&out);

    println!("cargo:rerun-if-changed=src/decrypt.html");
    minify_html("src/decrypt.html", &out.join("decrypt.minify.html"));

    println!("cargo:rerun-if-changed=src/decrypt.js");
    minify_js("src/decrypt.js", &out.join("decrypt.minify.js"));
}

fn minify_html(input_path: impl AsRef<Path>, output_path: &Path) {
    let cfg = minify_html::Cfg {
        preserve_brace_template_syntax: true,
        minify_js: true,
        ..minify_html::Cfg::spec_compliant()
    };

    let html = std::fs::read(input_path).expect("Failed to read HTML template");
    let html = minify_html::minify(&html, &cfg);
    std::fs::write(output_path, html).expect("Failed to write minified HTML template");
}

fn minify_js(input_path: impl AsRef<Path>, output_path: &Path) {
    let code = std::fs::read(input_path).expect("Failed to read JS template");
    let session = minify_js::Session::new();
    let mut code_minified = Vec::new();
    minify_js::minify(
        &session,
        minify_js::TopLevelMode::Global,
        &code,
        &mut code_minified,
    )
    .expect("Failed to minify JS template");
    std::fs::write(output_path, code_minified).expect("Failed to write minified JS template");
}
