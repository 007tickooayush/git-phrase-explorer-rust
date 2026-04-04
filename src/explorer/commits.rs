use git2::{Commit, Repository, Revwalk, Sort};
pub use git2::Error as Git2Error;

pub struct Commits<'repo> {
    repo: &'repo Repository,
    revwalk: Revwalk<'repo>
}

impl<'repo> Commits<'repo> {
    /// Iterate the Commits of the opted repository
    /// 
    /// Provide your own customized Sorting Strategy for displaying the commits
    /// in the required order.
    pub fn new(repo: &'repo Repository, sort_mode: Sort) -> Result<Self, Git2Error> {
        let mut revwalk = repo.revwalk()?;
        revwalk.push_head()?;
        revwalk.set_sorting(sort_mode)?;
        Ok(Self {
            repo,
            revwalk
        })
    }
}

impl<'repo> Iterator for Commits<'repo> {
    type Item = Result<Commit<'repo>, Git2Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let oid = self.revwalk.next()?;
        let oid = match oid {
            Ok(oid) => oid,
            Err(err) => return Some(Err(err))
        };

        let commit = match self.repo.find_commit(oid) {
            Ok(commit) => commit,
            Err(err) => return Some(Err(err))
        };

        Some(Ok(commit))
    }
}