use chrono::prelude::*;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::io::prelude::*;
use std::{fs::File, io::LineWriter};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "post_gen")]
struct Opt {
    #[structopt(short, long)]
    path: String,
}

#[derive(Debug)]
struct FileInfo {
    title: String,
    excerpt: String,
    slug: String,
    path: String,
    date: DateTime<Local>,
}

fn main() {
    let opt = Opt::from_args();
    let info: FileInfo = get_file_info(&opt.path);
    create_mdx_file(info).unwrap();
}

fn get_file_info(opt_path: &str) -> FileInfo {
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

    let slug: String = build_slug(&date, &title);
    let save_path: String = format!("{}/{}.{}", opt_path, slug, extensions[ext]);

    FileInfo {
        title,
        excerpt,
        slug,
        path: save_path,
        date,
    }
}

fn slug(name: &str) -> String {
    let separator: char = '-';
    let name: String = name.to_lowercase();
    let mut slug: String = String::new();
    for s in name.chars() {
        if s.is_whitespace() {
            slug.push(separator);
            continue;
        }

        match s {
            'a'..='z' | '0'..='9' | '-' => slug.push(s),
            _ => (),
        }
    }
    slug.trim_matches(separator).into()
}

fn build_slug(date: &DateTime<Local>, title: &str) -> String {
    let combine: String = format!("{}-{}", date.format("%Y-%m-%d"), title);
    slug(&combine)
}

fn create_mdx_file(info: FileInfo) -> std::io::Result<()> {
    let file: File = File::create(&info.path)?;
    let mut file: LineWriter<File> = LineWriter::new(file);

    file.write_all(b"---\n")?;

    file.write_all(b"title: ")?;
    file.write_all(&info.title.as_bytes())?;

    file.write_all(b"\nexcerpt: ")?;
    file.write_all(&info.excerpt.as_bytes())?;

    file.write_all(b"\ndate: ")?;
    file.write_all(&info.date.to_string().as_bytes())?;

    file.write_all(b"\nslug: ")?;
    file.write_all(&info.slug.as_bytes())?;

    file.write_all(b"\n---")?;

    file.flush()?;
    Ok(())
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn slug_builder() {
        assert_eq!(
            build_slug(&Local::now(), "slug test"),
            format!(
                "{}-{}",
                Local::now().format("%Y-%m-%d"),
                String::from("slug-test")
            )
        );
    }

    #[test]
    fn gen_slug_test() {
        assert_eq!(slug("hello world"), "hello-world");
        assert_eq!(slug("HeLLo WorlD"), "hello-world");
        assert_eq!(slug("!hello world."), "hello-world");
    }
}
