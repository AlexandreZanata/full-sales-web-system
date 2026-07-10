#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActorType {
    PlatformAdmin,
    User,
}

impl ActorType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PlatformAdmin => "PlatformAdmin",
            Self::User => "User",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "PlatformAdmin" => Some(Self::PlatformAdmin),
            "User" => Some(Self::User),
            _ => None,
        }
    }
}
