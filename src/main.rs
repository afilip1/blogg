#![feature(rust_2018_preview)]
use std::{path::Path, fs, io::prelude::*};
use clap::clap_app;

fn main() {
    let matches = clap_app!(blogg =>    
        (version: "0.1.0")
        (author: "Anton F. <a.filippov@protonmail.com>")
        (about: "Static blog generator")
        (@arg open: -o --open "Opens generated index file in default browser on completion")
    ).get_matches();

    let tgt_dir = Path::new("blogg_target");
    
    if tgt_dir.exists() {
        fs::remove_dir_all(tgt_dir).unwrap();
    }
    fs::create_dir(tgt_dir).unwrap();

    let dir = "blog_src";

    let mut index = String::new();
    for e in fs::read_dir(dir).unwrap() {
        let e = e.unwrap();
        let mut f = fs::File::open(e.path()).unwrap();
        let mut contents = String::new();
        f.read_to_string(&mut contents).unwrap();

        let out_f_path = format!("{}.html", e.path().file_stem().unwrap().to_str().unwrap());
        let out_path = tgt_dir.join(Path::new(&out_f_path));
        let mut out_f = fs::File::create(&out_path).unwrap();
        out_f.write_fmt(format_args!(include!("templates/post.html"), e.file_name().to_str().unwrap(), contents)).unwrap();

        index.push_str(&format!("<li><a href=\"{}\">{}</a></li>\n", out_f_path, e.file_name().to_str().unwrap()));
    }

    let mut index_f = fs::File::create(tgt_dir.join("index.html")).unwrap();
    index_f.write_fmt(format_args!(include!("templates/index.html"), dir, index)).unwrap();
    println!("Blog generated!");
    
    if matches.is_present("open") && open::that(tgt_dir.join("index.html")).is_ok() {
        println!("Opening!..");
    }
}
