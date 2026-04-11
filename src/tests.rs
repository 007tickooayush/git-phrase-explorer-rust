use std::{path::{Path, PathBuf}, sync::{Arc, RwLock, atomic::{AtomicUsize, Ordering}}, thread, time::Instant};

use git2::{Diff, DiffFormat, DiffOptions, Repository, Sort};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};



#[test]
fn test_structured_changes() {
    use crate::explorer::repo::Repo;

    let start = Instant::now();
    // let repo_path = "/home/hellsent/ZedProjects/git-phrase-explorer/git-commits-track-test";
    // let file_path = "file1.txt";
    // let check_phrase = "UPDATED FILE IN branch2 changes";
    // let single_discovery = true;

    // let repo_path = "/home/hellsent/PRJs/REACT/portfolio-vite-react";
    // let file_path =  "src/_components/About.jsx";
    // let check_phrase = "maxW=\"container.md\" borderRadius={\"2rem\"}";
    // let single_discovery = true;

    let repo_path = "/home/hellsent/ZedProjects/email-newsletter-rust";
    let file_path =  "tests/api/helpers.rs";
    let check_phrase = "reqwest::Response";
    let single_discovery = true;

    let repo = Repo::open(repo_path).unwrap();
    let target_file_path = Path::new(file_path);
    let mut diff_options = DiffOptions::new();

    
    println!("-------------------------------------------------------------------");
    for commit in repo.commits().unwrap() {
        let commit = commit.unwrap();
        // let changes = Changes::from_commit(&commit, &mut diff_options).unwrap();
        
        let message = match commit.message() {
            Some(message) => message,
            None => "N/A",
        };
        
        // println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
        // println!("COMMIT DETAILS\nCOMMIT ID:\n{}\nCOMMIT SUMMARY:\n{}\n", commit.sha(), message);
        // println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
        let mut found_changes = false;
        let mut phrase_line_contents = String::new();
        for change in commit.changes(&mut diff_options).unwrap() {
            let change = change.unwrap();

            if let Some(line_contents) = change.line_contents() {
                if let Some(change_file_path) = change.new_file_path() {

                    for line in line_contents.lines() {
                        if line.contains(check_phrase) && change_file_path.iter().eq(target_file_path) {
                            let line_origin = line.chars().next().unwrap();
                            if line_origin == '+' {
                                found_changes = true;
                                // println!("FOUND CHANGE || LINE ORIGIN == '+'");
                                // println!("><><><><><><><><><><><><><");
                                // println!("CHANGE CONTENTS:\n{}",change);
                                // println!("><><><><><><><><><><><><><");
                            }
                        }
                    }
                }
            }
        }

        if found_changes {
            println!(
                "***---COMMIT DETAILS---***\nCOMMIT ID:\n{}\nCOMMIT SUMMARY:\n{}\n", 
                commit.sha(), 
                message
            );


            let file_contents = commit.get_file(target_file_path).unwrap();
            file_contents.lines().for_each(|line| {
                if line.contains(check_phrase) {
                    phrase_line_contents = line.to_string();
                }
            });
            // println!("FILE CONTENTS:\n{}", file_contents);

            println!("LINE CONTENTS:\n\n{}",phrase_line_contents);
            println!("-------------------------------------------------------------------");
        }
    }
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}

#[test]
fn test_default_implementation () {
    let start = Instant::now();

    println!("\n-----------------------------------------------------\n");

    // X------------------------------------------ EXTERNAL VARIABLES ------------------------------------------X
    // let repo_path = &args.repo_path;
    // let file_path = &args.file_path;
    // let check_phrase = &args.phrase;
    // let single_discovery = args.single_discovery;

    // let repo_path = "/home/hellsent/ZedProjects/git-phrase-explorer/git-commits-track-test";
    // let file_path =  "file1.txt"; // "src/App.jsx";
    // let check_phrase = "UPDATED FILE IN branch2 changes";

    let repo_path = "/home/hellsent/ZedProjects/email-newsletter-rust";
    let file_path =  "tests/api/helpers.rs";
    let check_phrase = "reqwest::Response";

    let single_discovery = false;
    let verbose = false;
    // X-----------------------------------------X EXTERNAL VARIABLES X-----------------------------------------X

    let mut result_phrase_line = String::new();

    let repo = Repository::open(repo_path).unwrap();
    let target_file_path = Path::new(file_path);

    let mut revwalk = repo.revwalk().unwrap();
    revwalk.push_head().unwrap();
    revwalk.set_sorting(Sort::TOPOLOGICAL | Sort::REVERSE).unwrap(); // utilizing reverse topological sorting

    for oid in revwalk {
        let oid = oid.unwrap();
        let commit = repo.find_commit(oid).unwrap();

        let mut has_parent = false;
        if commit.parent_count() > 0 {
            has_parent = true;
        }

        let mut diff_opts = DiffOptions::new();
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
                    }).unwrap();

                    if found {
                        println!(
                            "Commit: {} || Summary: {:?}",
                            commit.id(),
                            commit_summary
                        );

                        // todo: handle repeating occurances
                        let tree = commit.tree().unwrap();
                        let entry = tree.get_path(target_file_path).unwrap();
                        let blob = repo.find_blob(entry.id()).unwrap();
                        let content = String::from_utf8_lossy(blob.content());
                        if verbose { // args.verbose {
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

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}

#[test]
fn test_targetted_file_spec_filter() {
    let start = Instant::now();

    // X------------------------------------------ EXTERNAL VARIABLES ------------------------------------------X
    // let repo_path = &args.repo_path;
    // let file_path = &args.file_path;
    // let check_phrase = &args.phrase;
    // let single_discovery = args.single_discovery;

    // let repo_path = "/home/hellsent/ZedProjects/git-phrase-explorer/git-commits-track-test";
    // let file_path =  "file1.txt"; // "src/App.jsx";
    // let check_phrase = "UPDATED FILE IN branch2 changes";

    // let repo_path = "/home/hellsent/ZedProjects/email-newsletter-rust";
    // let file_path =  "tests/api/helpers.rs";
    // let check_phrase = "reqwest::Response";

    let repo_path = "/home/hellsent/ZedProjects/git-phrase-explorer/git-commits-track-test-mark2/ppub";
    let file_path =  "o.txt";
    let check_phrase = "'SANDTMN': {'bids': [[249010.0, 85.488281544]";

    let max_count = Some(5);
    let verbose = false;
    // X-----------------------------------------X EXTERNAL VARIABLES X-----------------------------------------X

    // let mut result_phrase_line = String::new();
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
}

#[tokio::test]
async fn test_concurrent_trav_diffs() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();

    // X------------------------------------------ EXTERNAL VARIABLES ------------------------------------------X
    // let repo_path = &args.repo_path;
    // let file_path = &args.file_path;
    // let check_phrase = &args.phrase;
    // let single_discovery = args.single_discovery;

    // let repo_path = "/home/hellsent/ZedProjects/git-phrase-explorer/git-commits-track-test";
    // let file_path =  "file1.txt"; // "src/App.jsx";
    // let check_phrase = "UPDATED FILE IN branch2 changes";

    // let repo_path = "/home/hellsent/ZedProjects/email-newsletter-rust";
    // let file_path =  "tests/api/helpers.rs";
    // let check_phrase = "reqwest::Response";

    let repo_path = "/home/hellsent/ZedProjects/git-phrase-explorer/git-commits-track-test-mark2/ppub";
    let file_path =  "o.txt";
    let check_phrase = "'SANDTMN': {'bids': [[249010.0, 85.488281544]";
    let max_count = Some(5);
    // let verbose = false;
    // X-----------------------------------------X EXTERNAL VARIABLES X-----------------------------------------X

    // let mut result_phrase_line = String::new();
    // let mut matching_phrase_lines: Vec<(String, String)> = Vec::new();

    let repo_path = repo_path.to_string();
    let target_file_path = Path::new(file_path);

    let repo = Repository::open(&repo_path).unwrap();
    let mut revwalk = repo.revwalk().unwrap();
    revwalk.push_head().unwrap();
    revwalk.set_sorting(Sort::TOPOLOGICAL | Sort::REVERSE).unwrap(); // utilizing reverse topological sorting

    // Avoiding `Mutex` and using `AtomicUsize` atomic counter to remove synchronization overhead
    let curr_count = Arc::new(AtomicUsize::new(0));
    let mut handles: Vec<_> = Vec::new();

    for oid in revwalk {
        let oid = oid?;
        let repo_path = repo_path.clone();
        let curr_count = curr_count.clone();
        
        if let Some(count) = max_count {
            if curr_count.load(Ordering::Relaxed) >= count { break; }
        }

        handles.push(
            thread::spawn(move || {
                let repo = Repository::open(&repo_path).unwrap();
                let commit = repo.find_commit(oid.clone()).unwrap();

                let diff;
                let parent;

                let mut diff_opts = DiffOptions::new();
                diff_opts.pathspec(target_file_path);
                

                let mut found_phrase = String::new();
                let mut has_changes = false;

                if commit.parent_count() > 0 {
                    parent = commit.parent(0).unwrap();
                    diff = repo.diff_tree_to_tree(
                        Some(&mut parent.tree().unwrap()), 
                        Some(&mut commit.tree().unwrap()), 
                        Some(&mut diff_opts)).unwrap();
                } else {
                    diff = repo.diff_tree_to_tree(
                        None, 
                        Some(&mut commit.tree().unwrap()), 
                        Some(&mut diff_opts)).unwrap();
                }

                diff.print(DiffFormat::Patch, |_d, _h, line| {
                    if let Ok(line_contents) = str::from_utf8(line.content()) {
                        if line_contents.contains(check_phrase) {
                            found_phrase = line_contents.to_string();
                            curr_count.fetch_add(1, Ordering::Relaxed);
                            has_changes = true;
                        }
                    }
                    true
                }).unwrap();

                if has_changes {
                    Some((oid, found_phrase, commit.time()))
                } else {
                    None
                }
            })
        );
    }

    println!("-----------------------------------------------------\n");
    for handle in handles {
        if let Some((oid, line_contents, time)) = handle.join().unwrap() {
            println!(
                "COMMIT ID: {} | COMMIT TIME: {} (offset {} min, sign {})",
                oid,
                time.seconds(),
                time.offset_minutes(),
                time.sign()
            );
    
            println!("COMMIT LINE CONTENTS:\n{}", line_contents);
            println!("-----------------------------------------------------\n");
        }
    }

    let duration = start.elapsed();
    println!("test_concurrent_trav_diffs elapsed: {:?}", duration);

    Ok(())
}


#[tokio::test]
/// Significant increase in performance
async fn test_concurrent_pathspec_line_chunk() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();

    // X------------------------------------------ EXTERNAL VARIABLES ------------------------------------------X
    // let repo_path = &args.repo_path;
    // let file_path = &args.file_path;
    // let check_phrase = &args.phrase;
    // let single_discovery = args.single_discovery;

    // let repo_path = "/home/hellsent/ZedProjects/git-phrase-explorer/git-commits-track-test";
    // let file_path =  "file1.txt"; // "src/App.jsx";
    // let check_phrase = "UPDATED FILE IN branch2 changes";

    // let repo_path = "/home/hellsent/ZedProjects/email-newsletter-rust";
    // let file_path =  "tests/api/helpers.rs";
    // let check_phrase = "reqwest::Response";

    let repo_path = "/home/hellsent/ZedProjects/git-phrase-explorer/git-commits-track-test-mark2/ppub";
    let file_path =  "o.txt";
    let check_phrase = "'SANDTMN': {'bids': [[249010.0, 85.488281544]";
    let max_count = Some(5);
    // let verbose = false;
    // X-----------------------------------------X EXTERNAL VARIABLES X-----------------------------------------X

    
    let repo = Repository::open(&repo_path)?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(Sort::TOPOLOGICAL | Sort::REVERSE)?; // utilizing reverse topological sorting
    
    
    let oids: Vec<_> = revwalk.filter_map(Result::ok).collect();
    let repo_path = Arc::new(repo_path.to_string());
    let target_file_path = Arc::new(PathBuf::from(file_path));
    let check_phrase = Arc::new(check_phrase.to_string());
    let curr_count = Arc::new(AtomicUsize::new(0));

    let results: Vec<_> = oids.par_iter()
        .filter_map(|&oid| {
            if let Some(max) = max_count {
                if curr_count.load(Ordering::Relaxed) >= max {
                    return None;
                }
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
                Some(&mut diff_opts)).unwrap()
            } else {
                repo.diff_tree_to_tree(
                    None, 
                    Some(&mut commit.tree().unwrap()), 
                Some(&mut diff_opts)).unwrap()
            };

            let mut found_changes = String::new();
            let mut has_changes = false;

            diff.print(DiffFormat::Patch, | _d, _h, line | {
                if let Ok(line_contents) = std::str::from_utf8(line.content()) {
                    if line_contents.contains(check_phrase.as_str()) {
                        found_changes = line_contents.to_string();
                        curr_count.fetch_add(1, Ordering::Relaxed);
                        has_changes = true;
                    }
                }
                true
            }).unwrap();

            if has_changes {
                Some((oid, found_changes, commit.time()))
            } else {
                None
            }
        })
        .collect();

    for (oid, line_contents, time) in results {
        println!("COMMIT ID: {} | COMMIT TIME: {}", oid, time.seconds());
        println!("COMMIT LINE CONTENTS:\n{}", line_contents);
    }

    let duration = start.elapsed();
    println!("test_concurrent_trav_diffs elapsed: {:?}", duration);

    Ok(())
}