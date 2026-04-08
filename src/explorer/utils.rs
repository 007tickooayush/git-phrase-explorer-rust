use std::path::PathBuf;

use encoding_rs::WINDOWS_1252;

/// Convert bytes to PathBuf, trying multiple encodings
pub fn bytes_to_path(bytes: &[u8]) -> Option<PathBuf> {
    // Try UTF-8 first
    if let Ok(s) = std::str::from_utf8(bytes) {
        return Some(PathBuf::from(s));
    }
    
    // Try Windows-1252 (common legacy encoding on Windows)
    let (decoded, _, had_errors) = WINDOWS_1252.decode(bytes);
    if !had_errors {
        return Some(PathBuf::from(decoded.as_ref()));
    }
    
    // Fallback: use lossy UTF-8 conversion (replaces invalid sequences with �)
    let decoded = String::from_utf8_lossy(bytes);
    Some(PathBuf::from(decoded.as_ref()))
}