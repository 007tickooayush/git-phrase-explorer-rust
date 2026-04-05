
use git2::Error as Git2Error;

use crate::explorer::change_type::ChangeType;

pub struct Change {
    pub delta_idx: usize,
    line_contents: Option<String>,
    change_type: ChangeType
}

impl Change {
    pub fn new(delta_idx: usize, change_type: ChangeType) -> Self {
        Self {
            delta_idx,
            line_contents: Some(String::new()),
            change_type
        }
    }

    pub fn append_line_contents(&mut self, line: &str) -> Result<(), Git2Error> {
        if let Some(contents)  = &mut self.line_contents {
            contents.push_str(line);
        };
        Ok(())
    }
}

impl std::fmt::Display for Change {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CHANGE {}", self.delta_idx)?;

        Ok(())
    }
}