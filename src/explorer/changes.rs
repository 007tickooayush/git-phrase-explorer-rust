use git2::{Delta, Diff, DiffDelta, DiffFindOptions, DiffOptions, Repository};

use crate::explorer::{change::{Change, NULL_CHANGES_CONSTANT}, commit::Commit, delta_file_ref::DeltaFileRef};
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
        let mut diff_tree;
        let current_tree = commit.commit.tree()?;

        // fix for handling renames
        let mut diff_find_options = DiffFindOptions::new();
        diff_find_options.renames(true);
        diff_find_options.rename_threshold(100);


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

        // fix for handling renames
        diff_tree.find_similar(Some(&mut diff_find_options))?;

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

    let new_file = DeltaFileRef::new(repo, delta.new_file());
    let old_file = DeltaFileRef::new(repo, delta.old_file());

    match delta.status() {
        Delta::Added | Delta::Copied => {
            let Some(new_file) = new_file else {
                return Ok(None);
            };

            let mut change = Change::new(
                Delta::Added,
                Some(new_file.size()),
                None,
                None,
                Some(new_file.get_path().to_path_buf())
            );

            diff.print(git2::DiffFormat::Patch, | _delta, _hunk, line | {
                let origin = line.origin();
                let line_contents = String::from_utf8_lossy(line.content());

                change.append_line_contents(&origin.to_string()).unwrap();
                change.append_line_contents(&line_contents.to_string()).unwrap();

                true
            }).unwrap();
            
            Ok(Some((change, None)))
        },
        Delta::Modified => {
            let Some(new_file) = new_file else {
                return Ok(None);
            };

            let Some(old_file) = old_file else {
                return Ok(None);
            };

            let mut change = Change::new(
                Delta::Modified, 
                Some(old_file.size()), 
                Some(old_file.size()), 
                Some(old_file.get_path().to_path_buf()), 
                Some(new_file.get_path().to_path_buf())
            );

            diff.print(git2::DiffFormat::Patch, | _delta, _hunk, line | {
                let origin = line.origin();
                let line_contents = String::from_utf8_lossy(line.content());

                change.append_line_contents(&origin.to_string()).unwrap();
                change.append_line_contents(&line_contents.to_string()).unwrap();

                true
            }).unwrap();

            Ok(Some((change, None)))
        },
        Delta::Deleted => {
            let Some(old_file) = old_file else {
                return Ok(None);
            };

            let mut change = Change::new(
                Delta::Deleted, 
                Some(old_file.size()), 
                None, 
                Some(old_file.get_path().to_path_buf()), 
                None
            );

            diff.print(git2::DiffFormat::Patch, | _delta, _hunk, line | {
                let origin = line.origin();
                let line_contents = String::from_utf8_lossy(line.content());

                change.append_line_contents(&origin.to_string()).unwrap();
                change.append_line_contents(&line_contents.to_string()).unwrap();

                true
            }).unwrap();

            Ok(Some((change, None)))
        },
        Delta::Renamed => {
            let Some(new_file) = new_file else {
                return Ok(None);
            };

            let Some(old_file) = old_file else {
                return Ok(None);
            };

            let change_modified = if old_file.size() != new_file.size() {
                Some(Change::new(
                    Delta::Modified, 
                    Some(old_file.size()), 
                    Some(new_file.size()), 
                    Some(old_file.get_path().to_path_buf()), 
                    Some(new_file.get_path().to_path_buf())
                ))
            } else {
                None
            };

            let mut change_renamed = Change::new(
                Delta::Renamed, 
                Some(old_file.size()), 
                Some(new_file.size()), 
                Some(old_file.get_path().to_path_buf()), 
                Some(new_file.get_path().to_path_buf())
            );

            let change = match change_modified {
                Some(mut change_modified) => {

                    diff.print(git2::DiffFormat::Patch, | _delta, _hunk, line | {
                        let origin = line.origin();
                        let line_contents = String::from_utf8_lossy(line.content());

                        change_modified.append_line_contents(&origin.to_string()).unwrap();
                        change_modified.append_line_contents(&line_contents.to_string()).unwrap();

                        true
                    }).unwrap();

                    (change_modified, Some(change_renamed))
                },
                None => (change_renamed, None)
            };

            Ok(Some(change))
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