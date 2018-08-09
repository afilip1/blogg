#![feature(rust_2018_preview)]

mod config;
mod osstrext;

use chrono::{offset::Utc, DateTime};
use crate::osstrext::OsStrExt;
use std::{fs, io};

fn main() -> io::Result<()> {
    let config = config::get_config();

    let src_dir = config::get_source_dir(&config)?;
    let tgt_dir = config::get_target_dir(&config)?;

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
        let title = entry.path().file_stem().unwrap().into_string().replace("_", " ");
        let contents = fs::read_to_string(entry.path())?;
        let created_datetime = DateTime::<Utc>::from(created);

        let target_filename = entry
            .path()
            .with_extension("html")
            .file_name()
            .unwrap()
            .into_string();

        let target_path = tgt_dir.join(&target_filename);
        fs::write(
            target_path,
            format!(
                include_str!("templates/post.html"),
                title,
                contents.replace("\n", "<br />")
            ),
        )?;

        index.push_str(&format!(
            include_str!("templates/index_item.html"),
            target_filename,
            title,
            created_datetime.format("%B %e, %Y, %I:%M %p")
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
    if config.is_present("open") {
        open::that(tgt_dir.join("index.html"))?;
        println!("Opened in browser!");
    }

    Ok(())
}
