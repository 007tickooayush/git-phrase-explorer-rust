
pub struct Change {
    pub delta_idx: usize
}

impl Change {
    pub fn new() -> Self {
        Self {
            delta_idx: 1
        }
    }
}

impl std::fmt::Display for Change {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CHANGE {}", self.delta_idx)?;

        Ok(())
    }
}