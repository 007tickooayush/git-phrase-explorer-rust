use git2::DiffOptions;

use crate::explorer::changes::Changes;


#[tokio::test]
async fn test_structured_changes() -> Result<(), git2::Error> {
    use crate::explorer::repo::Repo;

    let repo_path = "/home/hellsent/ZedProjects/git-phrase-explorer/git-commits-track-test";
    let file_path =  "file1.txt"; // "src/App.jsx";
    let check_phrase = "UPDATED FILE IN branch2 changes";
    let single_discovery = true;

    let repo = Repo::open(repo_path)?;
    let mut diff_options = DiffOptions::new();


    for commit in repo.commits()? {
        println!("-------------------------------------------------------------------");
        let commit = commit?;
        // let changes = Changes::from_commit(&commit, &mut diff_options)?;
        
        for change in commit.changes(&mut diff_options)? {
            let change = change?;
            println!("{}", change);

            todo!("TEST THE TRAVERSAL IMPLEMENTATION");
        }
        println!("-------------------------------------------------------------------");

        break;
    }

    Ok(())
}