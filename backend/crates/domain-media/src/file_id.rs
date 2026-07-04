use std::fmt;
use std::str::FromStr;

use uuid::Uuid;

use crate::error::MediaError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileId(Uuid);

impl FileId {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(self) -> Uuid {
        self.0
    }
}

impl FromStr for FileId {
    type Err = MediaError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(value)
            .map(Self)
            .map_err(|_| MediaError::InvalidFileId)
    }
}

impl fmt::Display for FileId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
