use regex::Regex;
use uuid::Uuid;

/// Utility functions for validation
pub struct ValidationUtils;

impl ValidationUtils {
    /// Validates email format using regex
    pub fn is_valid_email(email: &str) -> bool {
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        email_regex.is_match(email)
    }
    
    /// Validates UUID format
    pub fn is_valid_uuid(uuid_str: &str) -> bool {
        Uuid::parse_str(uuid_str).is_ok()
    }
    
    /// Validates name format
    pub fn is_valid_name(name: &str) -> bool {
        !name.trim().is_empty() && name.len() <= 100
    }
    
    /// Sanitizes string input
    pub fn sanitize_string(input: &str) -> String {
        input.trim().to_string()
    }
}