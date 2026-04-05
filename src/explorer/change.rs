
use git2::{Delta, Error as Git2Error};

use crate::explorer::change_type::ChangeType;

const NULL_CHANGES_CONSTANT: &str = "---[NULL]---";

pub struct Change {
    line_contents: Option<String>,
    change_type: ChangeType
}

impl Change {
    pub fn new(delta_enum: Delta) -> Self {
        let change_type = Self::enum_from_delta(delta_enum);
        Self {
            line_contents: Some(String::new()),
            change_type
        }
    }

    pub fn line_contents(&self) -> Option<&String> {
        if let Some(contents) = &self.line_contents {
            Some(contents)
        } else {
            None
        }
    }

    pub fn append_line_contents(&mut self, line: &str) -> Result<(), Git2Error> {
        if let Some(contents)  = &mut self.line_contents {
            contents.push_str(line);
        };
        Ok(())
    }

    fn enum_from_delta(delta_enum: Delta) -> ChangeType {
        ChangeType::from(delta_enum)
    }
}

impl std::fmt::Display for Change {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CHANGE TYPE: {}\n\n", self.change_type)?;

        if let Some(line_contents) = self.line_contents() {
            write!(f, "CHANGE CONTENTS:\n{}\n", line_contents)?;
        } else {
            write!(f, "CHANGE CONTENTS:\n{}\n", NULL_CHANGES_CONSTANT)?;
        }

        Ok(())
    }
}