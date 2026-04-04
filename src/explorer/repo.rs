use std::path::Path;

use git2::{Repository, Sort};

pub use git2::Error as Git2Error;

use crate::explorer::commits::Commits;

pub struct Repo(Repository);

impl Repo {
    /// Attempt to open an already-existing repository at `path`.
    ///
    /// The path can point to either a normal or bare repository.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, Git2Error> {
        let repo = Repository::open(path)?;
        Ok(Self(repo))
    }

    /// Attempt to open an already-existing repository at or above `path`.
    ///
    /// This starts at `path` and looks up the filesystem hierarchy
    /// until it finds a repository.
    pub fn discover(path: impl AsRef<Path>) -> Result<Self, Git2Error> {
        let repo = Repository::discover(path)?;
        Ok(Self(repo))
    }

    /// Returns an iterator that produces all commits
    /// in the repo.
    /// Here thereverse topological format is the opted by default
    ///
    /// _See [`.commits_ext()`](Repo::commits_ext) to be
    /// able to specify the order._
    pub fn commits(&self) -> Result<Commits<'_>, Git2Error> {
        self.commits_ext(Sort::TOPOLOGICAL | Sort::REVERSE)
    }

    /// Returns an iterator that produces all commits
    /// in the repo.
    #[inline]
    pub fn commits_ext(&self, sort: Sort) -> Result<Commits<'_>, Git2Error> {
        Commits::new(&self.0, sort)
    }
}