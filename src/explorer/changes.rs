use git2::{Diff, DiffOptions};

use crate::explorer::{change::Change, commit::Commit};
use git2::Error as Git2Error;

pub struct Changes<'repo, 'commit> {
    pub commit: &'commit Commit<'repo>,
    diff: Diff<'repo>,
    idx_delta: usize,
    next_change: Option<Change>
}

impl<'repo, 'commit> Changes<'repo, 'commit>  {

    /// UPDATED IMPLEMENTATION OF FETCHING Changes
    /// 
    pub fn from_commit(commit: &'commit Commit<'repo>, diff_options: &'repo mut DiffOptions) -> Result<Self, Git2Error> {

        let parent_tree;
        let diff_tree;
        let current_tree = commit.commit.tree()?;


        // Checking if the commit has a parent 
        // Handling for initial/merge commit
        if commit.commit.parent_count() > 0 { 
            parent_tree = commit
                .commit
                .parent(0)
                .ok()
                .map(|parent| parent.tree())
                .transpose()?;

            diff_tree = commit
                .repo
                .diff_tree_to_tree(parent_tree.as_ref(), Some(&current_tree), Some(diff_options))?;
        } else {
            diff_tree = commit
                .repo
                .diff_tree_to_tree(None, Some(&current_tree), Some(diff_options))?;
        }

        Ok(Self {
            commit,
            diff: diff_tree,
            idx_delta: 0,
            next_change: None
        })
    }
}

impl<'repo, 'commit> Iterator for Changes<'repo, 'commit> {
    type Item = Result<Change, Git2Error>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}