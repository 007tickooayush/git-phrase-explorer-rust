mod explorer;
mod parser;
mod tests;

use std::path::Path;

use clap::Parser;
use git2::{DiffFormat, DiffOptions, Repository, Sort};

use crate::parser::CommandArgs;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = CommandArgs::parse();

    if args.verbose {
        println!("\n*******************************************************");
        println!("repo_path:            {}", args.repo_path);
        println!("file_path:            {}", args.file_path);
        println!("phrase:               {}", args.phrase);
        println!("single_discovery:     {}", args.single_discovery);
        println!("verbose:              {}", args.verbose);
        println!("*******************************************************\n");
    }


    println!("\n-----------------------------------------------------\n");

    // X------------------------------------------ EXTERNAL VARIABLES ------------------------------------------X
    let repo_path = &args.repo_path;
    let file_path = &args.file_path;
    let check_phrase = &args.phrase;
    let single_discovery = args.single_discovery;

    // let repo_path = "/home/hellsent/ZedProjects/git-phrase-explorer/git-commits-track-test";
    // let file_path =  "file1.txt"; // "src/App.jsx";
    // let check_phrase = "UPDATED FILE IN branch2 changes";
    // let single_discovery = true;
    // X-----------------------------------------X EXTERNAL VARIABLES X-----------------------------------------X

    let mut result_phrase_line = String::new();

    let repo = Repository::open(repo_path)?;
    let target_file_path = Path::new(file_path);

    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(Sort::TOPOLOGICAL | Sort::REVERSE)?; // utilizing reverse topological sorting

    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;

        let mut has_parent = false;
        if commit.parent_count() > 0 {
            has_parent = true;
        }

        let mut diff_opts = DiffOptions::new();
        let parent;
        let diff;

        if has_parent {
            parent = commit.parent(0)?;
            diff = repo.diff_tree_to_tree(
                Some(&mut parent.tree()?),
                Some(&mut commit.tree()?),
                Some(&mut diff_opts)
            )?;
        } else {
            diff = repo.diff_tree_to_tree(
                None,
                Some(&mut commit.tree()?),
                Some(&mut diff_opts)
            )?;
        }
        let mut found = false;

        for delta in diff.deltas() {
            if let Some(new_path) = delta.new_file().path() {
                if new_path == target_file_path {
                    let commit_summary = match commit.summary() {
                        Some(s) => s,
                        None => "",
                    };
                    diff.print(DiffFormat::Patch, |d, _h, line| {
                        if d.new_file().path() == Some(target_file_path) && line.origin() == '+' {
                            let line_str = String::from_utf8_lossy(line.content());
                            if line_str.contains(check_phrase) {
                                result_phrase_line = line_str.to_string();
                                found = true;
                            }
                        }
                        true
                    })?;

                    if found {
                        println!(
                            "Commit: {} || Summary: {:?}",
                            commit.id(),
                            commit_summary
                        );

                        // todo: handle repeating occurances
                        let tree = commit.tree()?;
                        let entry = tree.get_path(target_file_path)?;
                        let blob = repo.find_blob(entry.id())?;
                        let content = String::from_utf8_lossy(blob.content());
                        if args.verbose {
                            println!("FILE CONTENTS:\n\n{}", content);
                        }
                        println!("\n-----------------------------------------\n");
                        println!("LINE COTNENTS:\n\n{}", result_phrase_line);
                        println!("\n-----------------------------------------\n");
                    }
                }
            }
        }

        if single_discovery && found {
            break;
        }
    }

    Ok(())
}