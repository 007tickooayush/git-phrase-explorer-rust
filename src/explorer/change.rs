use std::path::PathBuf;

use git2::{Delta, Error as Git2Error};

use crate::explorer::change_type::ChangeType;

pub const NULL_CHANGES_CONSTANT: &str = "---[NULL]---";

pub struct Change {
    line_contents: Option<String>,
    change_type: ChangeType,
    old_file_size: Option<usize>,
    new_file_size: Option<usize>,
    old_file_path: Option<PathBuf>,
    new_file_path: Option<PathBuf>,
}

impl Change {
    pub fn new(
        delta_enum: Delta, 
        old_file_size: Option<usize>, 
        new_file_size: Option<usize>,
        old_file_path: Option<PathBuf>,
        new_file_path: Option<PathBuf>
    ) -> Self {
        let change_type = Self::enum_from_delta(delta_enum);
        Self {
            line_contents: Some(String::new()),
            change_type,
            old_file_size,
            new_file_size,
            old_file_path,
            new_file_path
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

    pub fn update_old_file_path(&mut self, file_path: Option<PathBuf>) {
        self.old_file_path = file_path
    }

    pub fn update_new_file_path(&mut self, file_path: Option<PathBuf>) {
        self.new_file_path = file_path
    }

    pub fn update_old_file_size(&mut self, file_size: Option<usize>) {
        self.old_file_size = file_size
    }
    
    pub fn update_new_file_size(&mut self, file_size: Option<usize>) {
        self.new_file_size = file_size
    }

    fn enum_from_delta(delta_enum: Delta) -> ChangeType {
        ChangeType::from(delta_enum)
    }
}

impl std::fmt::Display for Change {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CHANGE TYPE: {}\n\n", self.change_type)?;

        if let Some(line_contents) = self.line_contents() {
            if line_contents.is_empty() {
                write!(f, "CHANGE CONTENTS:\n{}\n", NULL_CHANGES_CONSTANT)?;
            } else {
                write!(f, "CHANGE CONTENTS:\n{}\n", line_contents)?;
            }
        } else {
            write!(f, "CHANGE CONTENTS:\n{}\n", NULL_CHANGES_CONSTANT)?;
        }

        Ok(())
    }
}