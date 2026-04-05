use git2::{Delta, Diff, DiffDelta, DiffOptions, Repository};

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
        loop {
            if let Some(change) = self.next_change.take() {
                return Some(Ok(change));
            }
    
            let delta = match self.diff.get_delta(self.idx_delta) {
                Some(delta) => delta,
                None => return None
            };
            self.idx_delta += 1;
    
    
            match get_diff_changes(&self.commit.repo, delta, &self.diff) {
                Ok(Some((change, next_change))) => {
                    self.next_change = next_change;
    
                    return Some(Ok(change));
                },
                Ok(None) => {},
                Err(err) => return Some(Err(err)),
            }
        }
    }
}

fn get_diff_changes<'repo>(
    repo: &Repository, 
    delta: DiffDelta<'_>, 
    diff: &Diff<'_>
) -> Result<Option<(Change, Option<Change>)>, Git2Error> {

    // diff.print(git2::DiffFormat::Patch, | _delta, _hunk, line | {
    //     let origin = line.origin();
    //     true
    // }).unwrap();

    // let acb = |delta_idx: i32, diff: &Diff<'_>| {
    // };

    match delta.status() {
        Delta::Added | Delta::Copied => {
            let mut change = Change::new(Delta::Added);
            diff.print(git2::DiffFormat::Patch, | _delta, _hunk, line | {
                let origin = line.origin();
                let line_contents = String::from_utf8_lossy(line.content());

                // change.append_line_contents(&origin.to_string()).unwrap();
                change.append_line_contents(&line_contents.to_string()).unwrap();

                true
            }).unwrap();
            
            Ok(Some((change, None)))
        },
        Delta::Modified => {
            todo!("IMPLEMENT ITERATION HANDLING FOR THIS Delta TYPE")
        },
        Delta::Deleted => {
            todo!("IMPLEMENT ITERATION HANDLING FOR THIS Delta TYPE")
        },
        Delta::Renamed => {
            todo!("IMPLEMENT ITERATION HANDLING FOR THIS Delta TYPE")
        },

        // All other Changes are not required to be monitored for current use case.
        Delta::Unmodified |
        Delta::Ignored |
        Delta::Untracked |
        Delta::Typechange |
        Delta::Unreadable |
        Delta::Conflicted => {
            return Ok(None)
        },
    }
}