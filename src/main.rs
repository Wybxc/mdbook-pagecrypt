use std::io;
use std::path::Path;

use anyhow::{Context, Result};
use mdbook_html::HtmlHandlebars;
use mdbook_pagecrypt::PageCrypt;
use mdbook_renderer::book::BookItem;
use mdbook_renderer::{RenderContext, Renderer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct PageCryptConfig {
    pub password: String,
    pub rounds: u32,
}

impl Default for PageCryptConfig {
    fn default() -> Self {
        Self {
            password: "pagecrypt".to_string(),
            rounds: 600_000,
        }
    }
}

fn main() -> Result<()> {
    let mut stdin = io::stdin();
    let ctx =
        RenderContext::from_json(&mut stdin).with_context(|| "Failed to read JSON from stdin")?;

    let cfg: PageCryptConfig = ctx
        .config
        .get("output.pagecrypt")
        .with_context(|| "Failed to parse config entry 'output.pagecrypt'")?
        .unwrap_or_default();

    let pagecrypt = PageCrypt::builder()
        .password(cfg.password)
        .rounds(cfg.rounds)
        .build()?;

    // Render HTML with default template
    HtmlHandlebars::new().render(&ctx)?;

    // Patch up the HTML with password protection
    for item in ctx.book.iter() {
        if let BookItem::Chapter(ref ch) = *item {
            if ch.is_draft_chapter() {
                continue;
            }

            // Get the path to the HTML file, from the default backend
            let path = ch.path.as_ref().unwrap();
            let ctx_path = path
                .to_str()
                .with_context(|| "Could not convert path to str")?;
            let filepath = Path::new(&ctx_path).with_extension("html");

            // Encrypt the HTML
            let file = std::fs::read(&filepath).with_context(|| "Failed to read HTML file")?;
            let file = pagecrypt.encrypt_html(&file)?;
            std::fs::write(&filepath, file).with_context(|| "Failed to write HTML file")?;
        }
    }

    // Encrypt the index.html
    if Path::new("index.html").exists() {
        let index = std::fs::read("index.html")?;
        let index = pagecrypt.encrypt_html(&index)?;
        std::fs::write("index.html", index).with_context(|| "Failed to write index.html")?;
    }

    // Encrypt the print.html
    if Path::new("print.html").exists() {
        let index = std::fs::read("print.html")?;
        let index = pagecrypt.encrypt_html(&index)?;
        std::fs::write("print.html", index).with_context(|| "Failed to write print.html")?;
    }

    // Encrypt the search index
    if Path::new("searchindex.json").exists() {
        std::fs::remove_file("searchindex.json")
            .with_context(|| "Failed to remove search index")?;
    }
    if Path::new("searchindex.js").exists() {
        let index = std::fs::read("searchindex.js")?;
        let index = pagecrypt.encrypt_js(&index)?;
        std::fs::write("searchindex.js", index).with_context(|| "Failed to write search index")?;
    }

    Ok(())
}
