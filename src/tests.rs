use std::path::Path;



#[tokio::test]
async fn test_structured_changes() -> Result<(), git2::Error> {
    use git2::DiffOptions;
    use crate::explorer::changes::Changes;
    use crate::explorer::repo::Repo;


    // let repo_path = "/home/hellsent/ZedProjects/git-phrase-explorer/git-commits-track-test";
    // let file_path = "file1.txt";
    // let check_phrase = "UPDATED FILE IN branch2 changes";

    let repo_path = "/home/hellsent/PRJs/REACT/portfolio-vite-react";
    let file_path =  "src/_components/About.jsx";
    let check_phrase = "maxW=\"container.md\" borderRadius={\"2rem\"}";
    let single_discovery = true;

    let repo = Repo::open(repo_path)?;
    let target_file_path = Path::new(file_path);
    let mut diff_options = DiffOptions::new();

    
    println!("-------------------------------------------------------------------");
    for commit in repo.commits()? {
        let commit = commit?;
        // let changes = Changes::from_commit(&commit, &mut diff_options)?;
        
        let message = match commit.message() {
            Some(message) => message,
            None => "N/A",
        };
        
        // println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
        // println!("COMMIT DETAILS\nCOMMIT ID:\n{}\nCOMMIT SUMMARY:\n{}\n", commit.sha(), message);
        // println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
        let mut found_changes = false;
        let mut phrase_line_contents = String::new();
        for change in commit.changes(&mut diff_options)? {
            let change = change?;

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


            let file_contents = commit.get_file(target_file_path)?;
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

    Ok(())
}