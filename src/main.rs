use chrono::prelude::*;
use dialoguer::{theme::ColorfulTheme, Input};
use str_slug::slug;

fn main() {
    let title: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Post title")
        .interact_text()
        .unwrap();

    let slug: String = slug(&title);

    let excerpt: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Post excerpt")
        .interact_text()
        .unwrap();

    let date: DateTime<Local> = Local::now();

    let frontmatter = format!(
        "---\ntitle: {}\nexcerpt: {}\ndate: {}\nslug: {}\n---",
        &title, &excerpt, &date, &slug
    );

    println!("{}", frontmatter);
}
