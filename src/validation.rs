use crate::{Error, Result};

/// Pragmatic ASCII mailbox validation; this is not an RFC 5322 parser.
pub fn email(input: &str) -> Result<String> {
    let normalized = input.trim().to_ascii_lowercase();
    let parts: Vec<_> = normalized.split('@').collect();
    if normalized.len() > 254
        || !normalized.is_ascii()
        || normalized
            .chars()
            .any(|c| c.is_whitespace() || c.is_control())
        || parts.len() != 2
        || parts[0].is_empty()
        || parts[0].len() > 64
        || parts[1].is_empty()
        || !parts[1].contains('.')
        || parts[1].starts_with('.')
        || parts[1].ends_with('.')
    {
        return Err(Error::BadRequest("Invalid email address"));
    }
    Ok(normalized)
}

pub fn password(input: &str) -> Result<()> {
    if !(12..=128).contains(&input.len()) {
        return Err(Error::BadRequest("Password must contain 12 to 128 bytes"));
    }
    Ok(())
}

pub fn title(input: &str) -> Result<String> {
    let trimmed = input.trim();
    if !(1..=200).contains(&trimmed.chars().count()) || trimmed.chars().any(char::is_control) {
        return Err(Error::BadRequest(
            "Title must contain 1 to 200 non-control characters",
        ));
    }
    Ok(trimmed.into())
}
