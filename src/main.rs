mod explorer;
mod parser;
mod tests;

use std::{path::Path, time::Instant};

use clap::Parser;
use git2::{DiffFormat, DiffOptions, Repository, Sort};

use crate::{explorer::utils::bytes_to_path, parser::CommandArgs};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // let args = CommandArgs::parse();

    // if args.verbose {
    //     println!("\n*******************************************************");
    //     println!("repo_path:            {}", args.repo_path);
    //     println!("file_path:            {}", args.file_path);
    //     println!("phrase:               {}", args.phrase);
    //     println!("single_discovery:     {}", args.single_discovery);
    //     println!("verbose:              {}", args.verbose);
    //     println!("*******************************************************\n");
    // }

    let start = Instant::now();

    // X------------------------------------------ EXTERNAL VARIABLES ------------------------------------------X
    // let repo_path = &args.repo_path;
    // let file_path = &args.file_path;
    // let check_phrase = &args.phrase;
    // let single_discovery = args.single_discovery;

    let repo_path = "/home/hellsent/ZedProjects/email-newsletter-rust";
    let file_path =  "tests/api/helpers.rs";
    let check_phrase = "reqwest::Response";
    let max_count = Some(5);
    let single_discovery = false;
    // X-----------------------------------------X EXTERNAL VARIABLES X-----------------------------------------X

    let mut matching_phrase_lines: Vec<(String, String)> = Vec::new();

    let repo = Repository::open(repo_path).unwrap();
    let target_file_path = Path::new(file_path);

    let mut revwalk = repo.revwalk().unwrap();
    revwalk.push_head().unwrap();
    revwalk.set_sorting(Sort::TOPOLOGICAL | Sort::REVERSE).unwrap(); // utilizing reverse topological sorting

    let mut curr_count = 0;
    for oid in revwalk {

        if let Some(count) = max_count {
            if curr_count >= count { break; }
        }

        let oid = oid.unwrap();
        let commit = repo.find_commit(oid).unwrap();

        let mut has_parent = false;
        if commit.parent_count() > 0 {
            has_parent = true;
        }

        let mut diff_opts = DiffOptions::new();
        diff_opts.pathspec(target_file_path);

        let parent;
        let diff;
        

        if has_parent {
            parent = commit.parent(0).unwrap();
            diff = repo.diff_tree_to_tree(
                Some(&mut parent.tree().unwrap()),
                Some(&mut commit.tree().unwrap()),
                Some(&mut diff_opts)
            ).unwrap();
        } else {
            diff = repo.diff_tree_to_tree(
                None,
                Some(&mut commit.tree().unwrap()),
                Some(&mut diff_opts)
            ).unwrap();
        }

        diff.print(DiffFormat::Patch, |_d, _h, line| {
            if let Ok(content) = std::str::from_utf8(line.content()) {
                if content.contains(check_phrase) {
                    matching_phrase_lines.push((oid.to_string(), content.to_string()));
                    curr_count += 1;
                }
            }
            true
        }).unwrap();


        // let deltas: Vec<_> = diff.deltas().collect();
        // if !deltas.is_empty() {  // File actually changed
        //     curr_count += 1;
        // }
    }


    println!("-----------------------------------------------------\n");
    for (oid, line_contents) in matching_phrase_lines {
        println!("COMMIT ID: {}", oid);
        println!("COMMIT LINE CONTENTS:\n{}", line_contents);
        println!("-----------------------------------------------------\n");
    }

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);

    Ok(())
}