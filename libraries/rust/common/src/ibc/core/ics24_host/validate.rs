use crate::ibc::prelude::*;

use super::error::ValidationError as Error;

/// Path separator (ie. forward slash '/')
const PATH_SEPARATOR: char = '/';
const VALID_SPECIAL_CHARS: &str = "._+-#[]<>";

/// Default validator function for identifiers.
///
/// A valid identifier only contain lowercase alphabetic characters, and be of a given min and max
/// length.
pub fn validate_identifier(id: &str, min: usize, max: usize) -> Result<(), Error> {
    assert!(max >= min);

    // Check identifier is not empty
    if id.is_empty() {
        return Err(Error::Empty);
    }

    // Check identifier does not contain path separators
    if id.contains(PATH_SEPARATOR) {
        return Err(Error::ContainSeparator { id: id.into() });
    }

    // Check identifier length is between given min/max
    if id.len() < min || id.len() > max {
        return Err(Error::InvalidLength {
            id: id.into(),
            length: id.len(),
            min,
            max,
        });
    }

    // Check that the identifier comprises only valid characters:
    // - Alphanumeric
    // - `.`, `_`, `+`, `-`, `#`
    // - `[`, `]`, `<`, `>`
    if !id
        .chars()
        .all(|c| c.is_alphanumeric() || VALID_SPECIAL_CHARS.contains(c))
    {
        return Err(Error::InvalidCharacter { id: id.into() });
    }

    // All good!
    Ok(())
}

/// Default validator function for Client identifiers.
///
/// A valid identifier must be between 9-64 characters and only contain lowercase
/// alphabetic characters,
pub fn validate_client_identifier(id: &str) -> Result<(), Error> {
    validate_identifier(id, 9, 64)
}

/// Default validator function for Connection identifiers.
///
/// A valid Identifier must be between 10-64 characters and only contain lowercase
/// alphabetic characters,
pub fn validate_connection_identifier(id: &str) -> Result<(), Error> {
    validate_identifier(id, 10, 64)
}

/// Default validator function for Port identifiers.
///
/// A valid Identifier must be between 2-128 characters and only contain lowercase
/// alphabetic characters,
pub fn validate_port_identifier(id: &str) -> Result<(), Error> {
    validate_identifier(id, 2, 128)
}

/// Default validator function for Channel identifiers.
///
/// A valid identifier must be between 8-64 characters and only contain
/// alphabetic characters,
pub fn validate_channel_identifier(id: &str) -> Result<(), Error> {
    validate_identifier(id, 8, 64)
}
