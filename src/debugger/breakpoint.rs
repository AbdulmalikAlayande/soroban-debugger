use std::collections::HashSet;

/// Manages breakpoints during debugging
pub struct BreakpointManager {
    breakpoints: HashSet<String>,
}

impl BreakpointManager {
    /// Create a new breakpoint manager
    pub fn new() -> Self {
        Self {
            breakpoints: HashSet::new(),
        }
    }

    /// Add a breakpoint at a function name
    pub fn add(&mut self, function: &str) {
        self.breakpoints.insert(function.to_string());
    }

    /// Remove a breakpoint
    pub fn remove(&mut self, function: &str) -> bool {
        self.breakpoints.remove(function)
    }

    /// Check if execution should break at this function
    pub fn should_break(&self, function: &str) -> bool {
        self.breakpoints.contains(function)
    }

    /// List all breakpoints
    pub fn list(&self) -> Vec<String> {
        self.breakpoints.iter().cloned().collect()
    }

    /// Clear all breakpoints
    pub fn clear(&mut self) {
        self.breakpoints.clear();
    }

    /// Check if there are any breakpoints set
    pub fn is_empty(&self) -> bool {
        self.breakpoints.is_empty()
    }

    /// Get count of breakpoints
    pub fn count(&self) -> usize {
        self.breakpoints.len()
    }

    /// Parse a condition string into a Condition object
    /// Note: This feature is not yet fully implemented
    #[allow(dead_code)]
    pub fn parse_condition(s: &str) -> crate::Result<()> {
        use crate::DebuggerError;

        let trimmed = s.trim();
        let Some((op, pos)) = find_operator(trimmed) else {
            return Err(DebuggerError::BreakpointError(
                "Condition must contain a comparison operator".to_string(),
            )
            .into());
        };

        let lhs = trimmed[..pos].trim();
        let rhs = trimmed[pos + op.len()..].trim();

        if lhs.is_empty() || rhs.is_empty() {
            return Err(DebuggerError::BreakpointError(
                "Condition must include non-empty left and right operands".to_string(),
            )
            .into());
        }

        Err(DebuggerError::BreakpointError(
            "Conditional breakpoints are not yet implemented".to_string(),
        )
        .into())
    }
}

#[allow(dead_code)]
fn find_operator(s: &str) -> Option<(&'static str, usize)> {
    let ops = [">=", "<=", "==", "!=", ">", "<"];
    for op in ops {
        if let Some(pos) = s.find(op) {
            return Some((op, pos));
        }
    }
    None
}

impl Default for BreakpointManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
 mod tests {
    fn test_add_breakpoint() {
        let mut manager = BreakpointManager::new();
        manager.add("transfer");
        assert!(manager.should_break("transfer"));
        assert!(!manager.should_break("mint"));
    }

    #[test]
    fn test_remove_breakpoint() {
        let mut manager = BreakpointManager::new();
        manager.add("transfer");
        assert!(manager.remove("transfer"));
        assert!(!manager.should_break("transfer"));
    }

    #[test]
    fn test_list_breakpoints() {
        let mut manager = BreakpointManager::new();
        manager.add("transfer");
        manager.add("mint");
        let list = manager.list();
        assert_eq!(list.len(), 2);
        assert!(list.contains(&"transfer".to_string()));
        assert!(list.contains(&"mint".to_string()));
    }

    #[test]
    fn test_parse_condition_missing_operator_fails() {
        let result = BreakpointManager::parse_condition("balance 1000");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("comparison operator"));
    }

    #[test]
    fn test_parse_condition_missing_lhs_fails() {
        let result = BreakpointManager::parse_condition("> 1000");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("left and right operands"));
    }

    #[test]
    fn test_parse_condition_missing_rhs_fails() {
        let result = BreakpointManager::parse_condition("balance > ");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("left and right operands"));
    }

    #[test]
    fn test_parse_condition_valid_structure_still_not_implemented() {
        let result = BreakpointManager::parse_condition("balance > 1000");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("not yet implemented"));
    }
}