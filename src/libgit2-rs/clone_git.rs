/*
 * libgit2 "clone" example
 *
 * Written by the libgit2 contributors
 *
 * To the extent possible under law, the author(s) have dedicated all copyright
 * and related and neighboring rights to this software to the public domain
 * worldwide. This software is distributed without any warranty.
 *
 * You should have received a copy of the CC0 Public Domain Dedication along
 * with this software. If not, see
 * <http://creativecommons.org/publicdomain/zero/1.0/>.
 */

use crate::lib::check_if_remote_exists;
use git2::{
    build::{CheckoutBuilder, RepoBuilder},
    FetchOptions, Progress, RemoteCallbacks,
};
use mytools::question_yes_no;
use std::{
    cell::RefCell,
    io::{self, Write},
    path::{Path, PathBuf},
};

struct State {
    progress: Option<Progress<'static>>,
    total: usize,
    current: usize,
    path: Option<PathBuf>,
    newline: bool,
}

fn print(state: &mut State) {
    let stats = state.progress.as_ref().unwrap();
    let network_pct = (100 * stats.received_objects()) / stats.total_objects();
    let index_pct = (100 * stats.indexed_objects()) / stats.total_objects();
    let co_pct = if state.total > 0 {
        (100 * state.current) / state.total
    } else {
        0
    };
    let kbytes = stats.received_bytes() / 1024;
    if stats.received_objects() == stats.total_objects() {
        if !state.newline {
            println!();
            state.newline = true;
        }
        print!(
            "Resolving deltas {}/{}\r",
            stats.indexed_deltas(),
            stats.total_deltas()
        );
    } else {
        print!(
            "net {:3}% ({:4} kb, {:5}/{:5})  /  idx {:3}% ({:5}/{:5})  \
             /  chk {:3}% ({:4}/{:4}) {}\r",
            network_pct,
            kbytes,
            stats.received_objects(),
            stats.total_objects(),
            index_pct,
            stats.indexed_objects(),
            stats.total_objects(),
            co_pct,
            state.current,
            state.total,
            state
                .path
                .as_ref()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_default()
        )
    }
    io::stdout().flush().unwrap();
}

pub fn run(git_path: &str, path: &Path) -> Result<(), git2::Error> {
    // Create folder if not exists
    if !path.exists() {
        question_yes_no!("Folder does not exist. Create it?");

        match std::fs::create_dir_all(path) {
            Ok(_) => {
                println!("Folder created: {path:?}");
            }
            Err(e) => {
                println!("Error: {e}");
                return Err(git2::Error::from_str("Error creating folder"));
            }
        }
    }

    check_if_remote_exists(git_path.to_string());

    let state = RefCell::new(State {
        progress: None,
        total: 0,
        current: 0,
        path: None,
        newline: false,
    });
    let mut cb = RemoteCallbacks::new();
    cb.transfer_progress(|stats| {
        let mut state = state.borrow_mut();
        state.progress = Some(stats.to_owned());
        print(&mut state);
        true
    });

    let mut co = CheckoutBuilder::new();
    co.progress(|path, cur, total| {
        let mut state = state.borrow_mut();
        state.path = path.map(|p| p.to_path_buf());
        state.current = cur;
        state.total = total;
        print(&mut state);
    });

    let mut fo = FetchOptions::new();
    fo.remote_callbacks(cb);
    let result = RepoBuilder::new()
        .fetch_options(fo)
        .with_checkout(co)
        .clone(git_path, path);
    println!();

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
