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
    slugs: Slugs,
    path: String,
    date: DateTime<Local>,
}

#[derive(Debug)]
struct Slugs {
    file: String,
    raw: String,
}

fn main() {
    let opt: Opt = Opt::from_args();
    let info: FileInfo = get_file_info(&opt.path);
    create_file(info).unwrap();
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

    let slugs: Slugs = gen_slugs(&title, &extensions[ext], &date);
    let save_path: String = format!("{}/{}", opt_path, slugs.file);

    FileInfo {
        title,
        excerpt,
        slugs,
        path: save_path,
        date,
    }
}

fn gen_slugs(title: &str, ext: &str, date: &DateTime<Local>) -> Slugs {
    let separator: char = '-';
    let title: String = title.to_lowercase();
    let mut raw_slug: String = String::new();
    for s in title.chars() {
        if s.is_whitespace() {
            raw_slug.push(separator);
            continue;
        }

        match s {
            'a'..='z' | '0'..='9' | '-' => raw_slug.push(s),
            _ => (),
        }
    }

    let raw_slug: String = raw_slug.trim_matches(separator).into();
    let raw_slug: String = format!("{}-{}", date.format("%Y-%m-%d"), raw_slug);
    let file_slug: String = format!("{}.{}", &raw_slug, ext);

    Slugs {
        file: file_slug,
        raw: raw_slug,
    }
}

fn create_file(info: FileInfo) -> std::io::Result<()> {
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
    file.write_all(&info.slugs.raw.as_bytes())?;

    file.write_all(b"\n---")?;

    file.flush()?;
    Ok(())
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn gen_slugs_test() {
        let slugs: Slugs = gen_slugs("slug test", "md", &Local::now());
        assert_eq!(
            slugs.file,
            format!("{}-{}", Local::now().format("%Y-%m-%d"), "slug-test.md")
        );
        assert_eq!(
            slugs.raw,
            format!("{}-{}", Local::now().format("%Y-%m-%d"), "slug-test")
        );

        let slugs: Slugs = gen_slugs("slug! test.", "md", &Local::now());
        assert_eq!(
            slugs.file,
            format!("{}-{}", Local::now().format("%Y-%m-%d"), "slug-test.md")
        );
        assert_eq!(
            slugs.raw,
            format!("{}-{}", Local::now().format("%Y-%m-%d"), "slug-test")
        );
    }
}
