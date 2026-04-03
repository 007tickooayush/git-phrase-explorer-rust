use std::path::Path;

use git2::{DiffFormat, DiffOptions, Repository, Sort};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!("\n-----------------------------------------------------\n");

    // X------------------------------------------ EXTERNAL VARIABLES ------------------------------------------X

    // // testing basic implementation:
    // let repo_path = "/home/hellsent/PRJs/REACT/portfolio-vite-react";
    // let file_path =  "src/_components/About.jsx"; // "src/App.jsx";
    // let check_phrase = "<Text>{degree.degree}: {degree.major}</Text>";

    let repo_path = "/home/hellsent/ZedProjects/git-phrase-explorer/git-commits-track-test";
    let file_path =  "file1.txt"; // "src/App.jsx";
    let check_phrase = "COMMIT FILE.";
    let mut result_phrase_line = String::new();
    let single_discovery = true;

    // X-----------------------------------------X EXTERNAL VARIABLES X-----------------------------------------X

    let repo = Repository::open(repo_path)?;
    let target_file_path = Path::new(file_path);

    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(Sort::TOPOLOGICAL | Sort::REVERSE)?; // utilizing reverse topological sorting

    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;

        // if commit.parent_count() != 1 { continue; } // Skips merges/initial commits
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

        for delta in diff.deltas() {
            if let Some(new_path) = delta.new_file().path() {
                if new_path == target_file_path {
                    let commit_summary = match commit.summary() {
                        Some(s) => s,
                        None => "",
                    };
                    let mut found = false;
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
                        println!("FILE CONTENTS:\n\n{}", content);
                        println!("\n-----------------------------------------\n");
                        println!("LINE COTNENTS:\n\n{}", result_phrase_line);
                        println!("\n-----------------------------------------\n");
                    }
                }
            }
        }

        if single_discovery {
            break;
        }
    }

    // // personal import
    // let repo = git_commits::open(repo_path)?;
    // for commit in repo.commits()? {
    //     let commit = commit?;
    //     println!("\n\n{}\n\n", commit);
    //     for change in commit.changes()? {
    //         let change = change?;
    //         println!(" {} ", change);
    //     }
    //     break;
    // }

    // // git2 scratch approach
    // let repo = Repository::open(repo_path)?;
    // let mut revwalk = repo.revwalk()?;
    // revwalk.push_head()?;
    // revwalk.set_sorting(Sort::NONE)?;
    // let oid = revwalk.next();
    // if let Some(oid) = oid {
    //     let oid = oid?;
    //     let commit = repo.find_commit(oid)?;
    //     println!("{} ", commit.author());
    // }

    // // git-commits approach
    // let repo = git_commits::open(repo_path)?;
    // for commit in repo.commits()? {
    //     let commit = commit?;
    //     println!("{}", commit);
    //     println!("SHA: {}", commit.sha());
    //     for change in commit.changes()? {
    //         let change = change?;
    //         println!("{}", change);
    //         if let Some(added) = change.as_added() {
    //             println!("Added: {}", added);
    //         }
    //         break;
    //     }
    //     break;
    // }


    Ok(())
}

fn print_test_commit_diff(repo_path: &str) -> Result<(), git2::Error> {
    let repo = Repository::open(repo_path)?;

    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(Sort::TOPOLOGICAL)?;

    for oid in revwalk {
        let commit = repo.find_commit(oid?)?;
        println!("Commit: {} by '{}' ({:?})", commit.id(), commit.author(), commit.summary());

        if commit.parent_count() > 0 {
            let parent = commit.parent(0)?;
            let mut opts = DiffOptions::new();

            let diff = repo.diff_tree_to_tree(Some(&mut parent.tree()?), Some(&mut commit.tree()?), Some(&mut opts))?;
            diff.print(git2::DiffFormat::Patch, | delta, hunk, line | {
                let hunk = match hunk {
                    Some(hunk) => hunk,
                    None => return true,
                };
                let default_decode = "[invalid utf]";

                let path_bytes = match delta.new_file().path_bytes() {
                    Some(path_bytes) => path_bytes,
                    None => return true,
                };
                println!(
                    "
                    \n
                    \nDELTA: {}
                    \n_HUNK: {}
                    \n_LINE: {}
                    \n
                    ",
                    str::from_utf8(path_bytes).unwrap_or(default_decode),
                    str::from_utf8(hunk.header()).unwrap_or(default_decode),
                    str::from_utf8(line.content()).unwrap_or(default_decode)
                );
                true
            })?;
        }

        break;
    }
    Ok(())
}
