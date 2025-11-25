use git2::{FetchOptions, RemoteCallbacks, Repository};
use clap_complete::{generate, Generator};
use indicatif::{ProgressBar, ProgressStyle};

use std::io;
use std::fs;
use std::env;
use std::path::Path;

pub fn clone_pb(url: &str, p: &str) -> Repository {
    let pb = ProgressBar::new(0);
    pb.set_style(ProgressStyle::default_bar().template("{bar} {pos}/{len}").unwrap());

    let mut cb = RemoteCallbacks::new();
    cb.transfer_progress(|s| {
        if pb.length().is_none() {
            pb.set_length(s.total_objects() as u64);
        }
        pb.set_position(s.received_objects() as u64);
        true
    });

    let mut fo = FetchOptions::new();
    fo.remote_callbacks(cb);

    let mut o = git2::build::RepoBuilder::new();
    o.fetch_options(fo);

    let r = o.clone(url, std::path::Path::new(p)).unwrap();
    pb.finish_and_clear();
    r
}

pub fn config_folder() -> io::Result<()> {
    let home = env::var("HOME").unwrap();
    let p = format!("{}/.config/themey/themes", home);
    fs::create_dir_all(p)?;
    Ok(())
}

pub fn prep_dir(p: &str) {
    if Path::new(p).exists() {
        fs::remove_dir_all(p).unwrap();
    }
    fs::create_dir_all(p).unwrap();
}

pub fn list_themes(p: &str) -> Vec<String> {
    let mut v = Vec::new();
    if let Ok(entries) = fs::read_dir(Path::new(p)) {
        for e in entries.flatten() {
            let p = e.path();
            if p.is_dir() {
                let m = p.join("metadata.toml");
                if m.exists() {
                    if let Some(s) = p.file_name().and_then(|x| x.to_str()) {
                        v.push(s.to_string());
                    }
                }
            }
        }
    }
    v
}

pub fn print_completions<G: Generator>(r#gen: G, cmd: &mut clap::Command) {
    generate(r#gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}
