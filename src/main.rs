#![feature(rust_2018_preview)]
use clap::clap_app;
use std::{fs, path::Path};

fn main() -> std::io::Result<()> {
    let matches = clap_app!(blogg =>
        (version: "0.1.0")
        (author: "Anton F. <a.filippov@protonmail.com>")
        (about: "Static blog generator")
        (@arg open: -o --open "Opens generated index file in default browser upon completion")
        (@arg SOURCE: --src <SOURCE> "Sets path to directory with blog post sources")
        (@arg TARGET: --tgt [TARGET] "Sets path to target directory (defaults to './blogg_target')")
    ).get_matches();

    let src_dir = Path::new(matches.value_of("SOURCE").unwrap());
    if !src_dir.exists() {
        panic!("unable to locate source directory");
    }

    let tgt_dir = Path::new(matches.value_of("TARGET").unwrap_or("blogg_target"));
    if tgt_dir.exists() {
        fs::remove_dir_all(tgt_dir)?;
    }
    fs::create_dir(tgt_dir)?;

    let mut index = String::new();
    let mut entries = fs::read_dir(src_dir)?
        .map(Result::unwrap)
        .map(|e| {
            let created = fs::metadata(e.path())
                .and_then(|meta| meta.created())
                .unwrap();
            (created, e)
        }).collect::<Vec<_>>();

    entries.sort_by_key(|(created, _)| *created);
    entries.reverse();

    for (created, entry) in entries {
        let contents = fs::read_to_string(entry.path())?;

        let out_f_path = entry.path().with_extension("html");
        let out_f_filename = out_f_path.file_name().unwrap();

        let out_path = tgt_dir.join(out_f_filename);
        fs::write(
            out_path,
            format!(
                include_str!("templates/post.html"),
                entry.file_name().to_str().unwrap(),
                contents
            ),
        )?;

        index.push_str(&format!(
            include_str!("templates/index_item.html"),
            out_f_filename.to_str().unwrap(),
            entry.file_name().to_str().unwrap(),
            chrono::DateTime::<chrono::offset::Utc>::from(created).format("%B %e, %Y, %I:%M %p")
        ));
    }

    fs::write(
        tgt_dir.join("index.html"),
        format!(
            include_str!("templates/index.html"),
            src_dir.display(),
            index
        ),
    )?;

    let styles_dir = tgt_dir.join("styles");
    fs::create_dir(&styles_dir)?;
    fs::write(
        styles_dir.join("styles.css"),
        include_str!("styles/styles.css"),
    )?;

    println!("Blog generated!");
    if matches.is_present("open") {
        open::that(tgt_dir.join("index.html"))?;
        println!("Opened in browser!");
    }

    Ok(())
}
