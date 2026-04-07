use std::path::Path;
use std::{borrow::Cow, fmt};

use chrono::{DateTime, FixedOffset, Local, TimeZone, Utc};
use git2::DiffOptions;
use git2::Repository;

use git2::Error as Git2Error;
use git2::Tree;

use crate::explorer::changes::Changes;

pub struct Commit<'repo> {
    pub repo: &'repo Repository,
    pub commit: git2::Commit<'repo>
}

impl<'repo> Commit<'repo> {

    pub fn new(repo: &'repo Repository, commit: git2::Commit<'repo>) -> Self  {
        Self { repo, commit }
    }

    pub fn sha(&self) -> String {
        self.commit.id().to_string()
    }

    pub fn tree(&self) -> Result<Tree<'repo>, Git2Error> {
        self.commit.tree()
    }

    pub fn message_bytes(&self) -> &[u8] {
        self.commit.message_bytes()
    }

    pub fn message(&self) -> Option<&str> {
        self.commit.message()
    }

    pub fn message_lossy(&self) -> String {
        let message_bytes = self.message_bytes();
        String::from_utf8_lossy(message_bytes).into_owned()
    }

    pub fn author(&self) -> Signature<'_> {
        Signature {
            signature: self.commit.author()
        }
    }

    pub fn committer(&self) -> Signature<'_> {
        Signature {
            signature: self.commit.committer()
        }
    }

    /// Returns the commit time (i.e. committer time) of a commit.
    ///
    /// Returns `(seconds, offset_minutes)`.
    ///
    /// _See also [`.time()`](Self::time) for a `chrono` `DateTime`._
    pub fn when(&self) -> (i64, i32) {
        let time = self.commit.time();
        (time.seconds(), time.offset_minutes())
    }

    /// Returns the commit time (i.e. committer time) of a commit.
    ///
    /// Returns `None` for an invalid timestamp.
    pub fn time(&self) -> Option<DateTime<FixedOffset>> {
        let time = self.commit.time();

        let offset = time.offset_minutes().checked_mul(60)?;
        let offset = FixedOffset::east_opt(offset)?;
        offset.timestamp_opt(time.seconds(), 0).single()
    }

    /// Returns the commit time (i.e. committer time) of a commit.
    ///
    /// Returns `None` for an invalid timestamp.
    pub fn time_utc(&self) -> Option<DateTime<Utc>> {
        let time = self.time()?.with_timezone(&Utc);
        Some(time)
    }

    /// Returns the commit time (i.e. committer time) of a commit.
    ///
    /// Returns `None` for an invalid timestamp.
    pub fn time_local(&self) -> Option<DateTime<Local>> {
        let time = self.time()?.with_timezone(&Local);
        Some(time)
    }

    /// Get the contents of a file base on the repo file path specified
    /// 
    /// The resulting version of the file has the change w.r.t the current `Commit`
    /// Returns `String` having the complete file contents
    pub fn get_file(&self, target_file_path: &Path) -> Result<String, Git2Error> {
        let tree = self.tree()?;
        let entry = tree.get_path(target_file_path)?;
        let blob = self.repo.find_blob(entry.id())?;
        let file_contents = String::from_utf8_lossy(blob.content()).into_owned();
        Ok(file_contents)
    }

    pub fn changes(&self, diff_options: &'repo mut DiffOptions) -> Result<Changes<'repo, '_>, Git2Error> {
        Changes::from_commit(self, diff_options)
    }

}

impl<'repo> fmt::Display for Commit<'repo> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.time() {
            Some(time) => write!(f, "[{time}]")?,
            None => write!(f, "[invalid time]")?
        }

        let msg = self.message_lossy();
        let first_line = msg.trim().lines().next().unwrap_or_default();

        write!(f, "{} {}", self.author().name_lossy(), first_line)?; // DFRT

        Ok(())
    }
}

pub struct Signature<'commit> {
    signature: git2::Signature<'commit>
}

impl<'commit> Signature<'commit> {

    pub fn name(&self) -> Option<&str> {
        self.signature.name()
    }

    pub fn name_bytes(&self) -> &[u8] {
        self.signature.name_bytes()
    }

    pub fn name_lossy(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(self.name_bytes())
    }

    pub fn email(&self) -> Option<&str> {
        self.signature.email()
    }

    pub fn email_bytes(&self) -> &[u8] {
        self.signature.email_bytes()
    }
    
    pub fn email_lossy(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(self.email_bytes())
    }

    /// Returns `(seconds, offset_minutes)`.
    ///
    /// _See also [`.time()`](Self::time) for a `chrono` `DateTime`._
    pub fn when(&self) -> (i64, i32) {
        let time = self.signature.when();
        (time.seconds(), time.offset_minutes())
    }

    /// Returns `None` for an invalid timestamp.
    pub fn time(&self) -> Option<DateTime<FixedOffset>> {
        let time = self.signature.when();

        let offset = time.offset_minutes().checked_mul(60)?;
        let offset = FixedOffset::east_opt(offset)?;
        offset.timestamp_opt(time.seconds(), 0).single()
    }

    /// Returns `None` for an invalid timestamp.
    pub fn time_utc(&self) -> Option<DateTime<Utc>> {
        let time = self.time()?.with_timezone(&Utc);
        Some(time)
    }

    /// Returns `None` for an invalid timestamp.
    pub fn time_local(&self) -> Option<DateTime<Local>> {
        let time = self.time()?.with_timezone(&Local);
        Some(time)
    }
}

impl<'commit> fmt::Display for Signature<'commit> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.time() {
            Some(time) => write!(f, "[{time}]")?,
            None => write!(f, "[invalid time]")?
        }

        write!(f, "{} <{}>", self.name_lossy(), self.email_lossy())?;

        Ok(())
    }
}