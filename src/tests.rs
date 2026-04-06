use git2::DiffOptions;

use crate::explorer::changes::Changes;


#[tokio::test]
async fn test_structured_changes() -> Result<(), git2::Error> {
    use crate::explorer::repo::Repo;

    let repo_path = "/home/hellsent/ZedProjects/git-phrase-explorer/git-commits-track-test"; // "/home/hellsent/PRJs/REACT/portfolio-vite-react";
    let file_path =  "file1.txt"; // "src/App.jsx";
    let check_phrase = "UPDATED FILE IN branch2 changes";
    let single_discovery = true;

    let repo = Repo::open(repo_path)?;
    let mut diff_options = DiffOptions::new();

    println!("-------------------------------------------------------------------");
    for commit in repo.commits()? {
        let commit = commit?;
        // let changes = Changes::from_commit(&commit, &mut diff_options)?;

        let message = match commit.message() {
            Some(message) => message,
            None => "N/A",
        };

        println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
        println!("COMMIT DETAILS\nCOMMIT ID:\n{}\nCOMMIT SUMMARY:\n{}\n", commit.sha(), message);
        println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
        for change in commit.changes(&mut diff_options)? {
            let change = change?;
            println!("{}", change);

        }
        println!("-------------------------------------------------------------------");
    }

    Ok(())
}