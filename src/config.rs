use clap::{clap_app, ArgMatches};
use std::{fs, io, path::Path};

crate fn get_config() -> ArgMatches<'a> {
    clap_app!(blogg =>
        (version: "0.1.0")
        (author: "Anton F. <a.filippov@protonmail.com>")
        (about: "Static blog generator")
        (@arg open: -o --open "Opens generated index file in default browser upon completion")
        (@arg SOURCE: --src <SOURCE> "Sets path to directory with blog post sources")
        (@arg TARGET: --tgt [TARGET] "Sets path to target directory (defaults to './blogg_target')")
    ).get_matches()
}

crate fn get_source_dir(config: &'a ArgMatches<'b>) -> io::Result<&'a Path> {
    let src_dir = Path::new(config.value_of("SOURCE").unwrap());

    if !src_dir.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "provided source path does not point to a directory",
        ));
    }

    Ok(src_dir)
}

crate fn get_target_dir(config: &'a ArgMatches<'b>) -> io::Result<&'a Path> {
    let tgt_dir = Path::new(config.value_of("TARGET").unwrap_or("blogg_target"));

    if tgt_dir.exists() {
        fs::remove_dir_all(tgt_dir)?;
    }
    fs::create_dir(tgt_dir)?;

    Ok(tgt_dir)
}
