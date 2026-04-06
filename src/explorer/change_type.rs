use git2::Delta;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum ChangeType {
    Added,
    Modified,
    Deleted,
    Renamed,
    Unmapped
}

impl From<Delta> for ChangeType {
    fn from(value: Delta) -> Self {
        match value {
            Delta::Added | Delta::Copied => ChangeType::Added,
            Delta::Modified => ChangeType::Modified,
            Delta::Deleted => ChangeType::Deleted,
            Delta::Renamed => ChangeType::Renamed,
            Delta::Unmodified | 
            Delta::Ignored | 
            Delta::Untracked | 
            Delta::Typechange | 
            Delta::Unreadable | 
            Delta::Conflicted => ChangeType::Unmapped,
        }
    }
}

impl std::fmt::Display for ChangeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let printable = match *self {
            ChangeType::Added => "Added",
            ChangeType::Modified => "Modified",
            ChangeType::Deleted => "Deleted",
            ChangeType::Renamed => "Renamed",
            ChangeType::Unmapped => "Unmapped"
        };
        write!(f, "{printable}")?;
        Ok(())
    }
}