use chrono::prelude::*;
use dialoguer::{theme::ColorfulTheme, Input};
use std::fs::File;
use std::io::prelude::*;
use str_slug::slug;

fn main() {
    // user input for post title and excerpt
    let title: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Post title")
        .interact_text()
        .unwrap();

    let excerpt: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Post excerpt")
        .interact_text()
        .unwrap();

    let date: DateTime<Local> = Local::now();

    // generate the file name, and post slug
    let slug: String = slug(&title);
    let file_slug: String = format!("{}-{}.mdx", date.format("%Y-%m-%d"), slug);

    let frontmatter = format!(
        "---\ntitle: {}\nexcerpt: {}\ndate: {}\nslug: {}\n---",
        &title, &excerpt, &date, &slug
    );

    create_mdx_file(&file_slug, &frontmatter).unwrap();
}

fn create_mdx_file(slug: &String, frontmatter: &String) -> std::io::Result<()> {
    let mut file = File::create(slug)?;
    file.write_all(frontmatter.as_bytes())?;
    Ok(())
}
