use chrono::prelude::*;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::fs::File;
use std::io::prelude::*;
use str_slug::slug;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "post_gen")]
struct Opt {
    #[structopt(short, long)]
    path: String,
}

fn main() {
    let opt = Opt::from_args();

    // user input for post title and excerpt
    let title: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Post title")
        .interact_text()
        .unwrap();

    let excerpt: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Post excerpt")
        .interact_text()
        .unwrap();

    let extensions: Vec<&str> = vec!["mdx", "md"];
    let ext: usize = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Markdown or MDX")
        .default(0)
        .items(&extensions[..])
        .interact()
        .unwrap();

    let date: DateTime<Local> = Local::now();

    // generate the file name, and post slug
    let slug: String = slug(&title);
    let file_slug: String = format!("/{}-{}", date.format("%Y-%m-%d"), slug);

    let frontmatter = format!(
        "---\ntitle: {}\nexcerpt: {}\ndate: {}\nslug: {}\n---",
        &title, &excerpt, &date, &slug
    );

    let save_path: String = format!("{}{}.{}", opt.path, file_slug, extensions[ext]);

    create_mdx_file(&frontmatter, &save_path).unwrap();
}

fn create_mdx_file(frontmatter: &str, save_path: &str) -> std::io::Result<()> {
    let mut file: File = File::create(save_path)?;
    file.write_all(frontmatter.as_bytes())?;
    Ok(())
}
