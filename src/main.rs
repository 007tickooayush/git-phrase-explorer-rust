mod explorer;
mod parser;
mod tests;

use std::{path::{Path, PathBuf}, sync::{Arc, atomic::AtomicUsize}, time::Instant};
use std::sync::atomic::Ordering;
use clap::Parser;
use git2::{DiffFormat, DiffOptions, Repository, Sort};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{explorer::{commit, utils::bytes_to_path}, parser::CommandArgs};

// todo: IMPLEMENT the NON-CHUNK PARALLELISM BASED APPROACH FOR TRAVERSAL

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = CommandArgs::parse();

    if args.verbose {
        println!("\n*******************************************************");
        println!("repo_path:            {}", args.repo_path);
        println!("file_path:            {}", args.file_path);
        println!("phrase:               {}", args.phrase);
        println!("max_count:            {}", args.max_count);
        println!("verbose:              {}", args.verbose);
        println!("*******************************************************\n");
    }

    let start = Instant::now();

    // X------------------------------------------ EXTERNAL VARIABLES ------------------------------------------X
    let repo_path = &args.repo_path;
    let file_path = &args.file_path;
    let check_phrase = &args.phrase;
    let max_count = args.max_count;

    // let repo_path = "/home/hellsent/ZedProjects/email-newsletter-rust";
    // let file_path =  "tests/api/helpers.rs";
    // let check_phrase = "reqwest::Response";

    // let repo_path = "/home/hellsent/ZedProjects/git-phrase-explorer/git-commits-track-test-mark2/ppub";
    // let file_path =  "o.txt";
    // let check_phrase = "'SANDTMN': {'bids': [[249010.0, 85.488281544]";
    // let max_count = Some(5);
    // X-----------------------------------------X EXTERNAL VARIABLES X-----------------------------------------X

    let repo = Repository::open(&repo_path)?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(Sort::TOPOLOGICAL | Sort::REVERSE)?; // utilizing reverse topological sorting

    let repo = Repository::open(repo_path)?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(Sort::TOPOLOGICAL | Sort::REVERSE)?;

    let oids: Vec<_> = revwalk.filter_map(Result::ok).collect();
    let repo_path = Arc::new(repo_path.to_string());
    let target_file_path = Arc::new(PathBuf::from(file_path));
    let check_phrase = Arc::new(check_phrase.to_string());
    let curr_count = Arc::new(AtomicUsize::new(0));

    let results: Vec<_> = oids.par_iter()
        .filter_map(|&oid| {
            if curr_count.load(Ordering::Relaxed) >= max_count {
                return None;
            }
            

            let repo = Repository::open(repo_path.as_str()).unwrap();
            let commit = repo.find_commit(oid).unwrap();

            let mut diff_opts = {
                let mut opts = DiffOptions::new();
                opts.pathspec(target_file_path.as_path());
                opts
            };

            let diff = if commit.parent_count() > 0 {
                let parent = commit.parent(0).unwrap();
                repo.diff_tree_to_tree(
                    Some(&mut parent.tree().unwrap()),
                    Some(&mut commit.tree().unwrap()),
                    Some(&mut diff_opts),
                ).unwrap()
            } else {
                repo.diff_tree_to_tree(
                    None,
                    Some(&mut commit.tree().unwrap()),
                    Some(&mut diff_opts),
                ).unwrap()
            };

            let mut found_phrase = String::new();
            let mut has_changes = false;

            diff.print(DiffFormat::Patch, |_d, _h, line| {
                if let Ok(line_contents) = std::str::from_utf8(line.content()) {
                    if line_contents.contains(&*check_phrase) {
                        found_phrase = line_contents.to_string();
                        curr_count.fetch_add(1, Ordering::Relaxed);
                        has_changes = true;
                    }
                }
                true
            }).unwrap();

            if has_changes {
                Some((oid, commit.summary().map(|s| s.to_string()), found_phrase, commit.time()))
            } else {
                None
            }
        })
        .collect();

    println!("-----------------------------------------------------\n");
    for (oid, commit_summary, line_contents, time) in results {
        let time = chrono::DateTime::from_timestamp_secs(time.seconds());
        if let Some(time) = time {
            println!("COMMIT ID: {} | COMMIT TIME: {}", oid, time);
        }
        let summary = if let Some(summary) = &commit_summary {
            summary
        } else {
            ""
        };
        println!("COMMIT MESSAGE: {}", summary);
        if line_contents.len() <= 200 {
            println!("COMMIT LINE CONTENTS:\n{}", line_contents);
        } else {
            println!("COMMIT LINE CONTENTS:\n{}...", line_contents.get(..200).unwrap());
        }
        println!("-----------------------------------------------------\n");
    }

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);

    Ok(())
}