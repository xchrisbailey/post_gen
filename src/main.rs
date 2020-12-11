use chrono::prelude::*;
use dialoguer::{theme::ColorfulTheme, Input};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use str_slug::slug;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    path: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt.path);
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

fn create_mdx_file(slug: &str, frontmatter: &str) -> std::io::Result<()> {
    let mut file = File::create(slug)?;
    file.write_all(frontmatter.as_bytes())?;
    Ok(())
}
