use std::{fmt, hash};

#[repr(transparent)]
pub struct Location {
    inner: str,
}

impl Location {
    const DEFAULT_NAMESPACE: &'static str = "minecraft";

    /// Creates a new `Location` by parsing the given string.
    pub fn new(s: &str) -> Result<&Self, ParseLocationError> {
        let is_legal = |c: char| {
            c.is_ascii_digit() || c.is_ascii_lowercase() || c == '_' || c == '-' || c == '.'
        };

        let (namespace, path) = match s.split_once(':') {
            Some(tuple @ (namespace, _)) => {
                if namespace.is_empty() {
                    return Err(ParseLocationError::empty(LocationPart::Namespace));
                }

                for c in namespace.chars() {
                    if !is_legal(c) {
                        return Err(ParseLocationError::illegal_char(LocationPart::Namespace, c));
                    }
                }

                tuple
            }
            None => ("", s),
        };

        if path.is_empty() {
            Err(ParseLocationError::empty(LocationPart::Path))
        } else {
            for c in namespace.chars() {
                if !is_legal(c) && c != '/' {
                    return Err(ParseLocationError::illegal_char(LocationPart::Path, c));
                }
            }

            // SAFETY: `str` and `Location` have the same layout.
            Ok(unsafe { &*(s as *const str as *const Location) })
        }
    }

    /// Returns the underlying string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.inner
    }

    /// Returns the namespace and path parts making up the resource location.
    #[must_use]
    pub fn as_parts(&self) -> (&str, &str) {
        self.inner
            .split_once(':')
            .unwrap_or((Self::DEFAULT_NAMESPACE, &self.inner))
    }

    /// Returns the path part of the resource location.
    #[must_use]
    pub fn path(&self) -> &str {
        self.as_parts().1
    }
}

impl fmt::Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (namespace, path) = self.as_parts();

        write!(f, "{namespace}:{path}")
    }
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.as_parts() == other.as_parts()
    }
}

impl Eq for Location {}

impl hash::Hash for Location {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.as_parts().hash(state);
    }
}

impl<'de: 'a, 'a> serde::Deserialize<'de> for &'a Location {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;

        Location::new(s).map_err(serde::de::Error::custom)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct ParseLocationError {
    part: LocationPart,
    kind: LocationErrorKind,
}

impl ParseLocationError {
    fn empty(part: LocationPart) -> Self {
        ParseLocationError {
            part,
            kind: LocationErrorKind::Empty,
        }
    }

    fn illegal_char(part: LocationPart, c: char) -> Self {
        ParseLocationError {
            part,
            kind: LocationErrorKind::IllegalCharacter(c),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum LocationPart {
    Namespace,
    Path,
}

#[derive(Clone, Copy, Debug)]
enum LocationErrorKind {
    Empty,
    IllegalCharacter(char),
}

impl fmt::Display for ParseLocationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let part = self.part;

        match self.kind {
            LocationErrorKind::Empty => {
                write!(f, "resource location {part} is empty")
            }
            LocationErrorKind::IllegalCharacter(c) => {
                write!(
                    f,
                    "resource location {part} contains illegal character `{c}`",
                )
            }
        }
    }
}

impl fmt::Display for LocationPart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Namespace => f.write_str("namespace"),
            Self::Path => f.write_str("path"),
        }
    }
}

impl std::error::Error for ParseLocationError {}
