use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResourceType {
    Playlist,
    Track,
    Album,
    Artist,
    Show,
    Episode,
    Audiobook,
}

impl ResourceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ResourceType::Playlist => "playlist",
            ResourceType::Track => "track",
            ResourceType::Album => "album",
            ResourceType::Artist => "artist",
            ResourceType::Show => "show",
            ResourceType::Episode => "episode",
            ResourceType::Audiobook => "audiobook",
        }
    }
}

impl std::str::FromStr for ResourceType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "playlist" => Ok(ResourceType::Playlist),
            "track" => Ok(ResourceType::Track),
            "album" => Ok(ResourceType::Album),
            "artist" => Ok(ResourceType::Artist),
            "show" => Ok(ResourceType::Show),
            "episode" => Ok(ResourceType::Episode),
            "audiobook" => Ok(ResourceType::Audiobook),
            _ => Err(format!(
                "Invalid resource type '{}'. Valid types: playlist, track, album, artist, show, episode, audiobook",
                s
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn as_str_all_types() {
        assert_eq!(ResourceType::Playlist.as_str(), "playlist");
        assert_eq!(ResourceType::Track.as_str(), "track");
        assert_eq!(ResourceType::Album.as_str(), "album");
        assert_eq!(ResourceType::Artist.as_str(), "artist");
        assert_eq!(ResourceType::Show.as_str(), "show");
        assert_eq!(ResourceType::Episode.as_str(), "episode");
        assert_eq!(ResourceType::Audiobook.as_str(), "audiobook");
    }

    #[test]
    fn from_str_valid() {
        assert_eq!(ResourceType::from_str("playlist").unwrap(), ResourceType::Playlist);
        assert_eq!(ResourceType::from_str("track").unwrap(), ResourceType::Track);
        assert_eq!(ResourceType::from_str("album").unwrap(), ResourceType::Album);
        assert_eq!(ResourceType::from_str("artist").unwrap(), ResourceType::Artist);
        assert_eq!(ResourceType::from_str("show").unwrap(), ResourceType::Show);
        assert_eq!(ResourceType::from_str("episode").unwrap(), ResourceType::Episode);
        assert_eq!(ResourceType::from_str("audiobook").unwrap(), ResourceType::Audiobook);
    }

    #[test]
    fn from_str_case_insensitive() {
        assert_eq!(ResourceType::from_str("PLAYLIST").unwrap(), ResourceType::Playlist);
        assert_eq!(ResourceType::from_str("Track").unwrap(), ResourceType::Track);
        assert_eq!(ResourceType::from_str("ALBUM").unwrap(), ResourceType::Album);
    }

    #[test]
    fn from_str_invalid() {
        let result = ResourceType::from_str("invalid");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid resource type"));
    }

    #[test]
    fn serialize_deserialize() {
        let rt = ResourceType::Track;
        let json = serde_json::to_string(&rt).unwrap();
        assert_eq!(json, "\"track\"");

        let deserialized: ResourceType = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, ResourceType::Track);
    }

    #[test]
    fn equality() {
        assert_eq!(ResourceType::Track, ResourceType::Track);
        assert_ne!(ResourceType::Track, ResourceType::Album);
    }

    #[test]
    fn copy_trait() {
        let rt = ResourceType::Album;
        let copied1 = rt; // ResourceType implements Copy
        let copied2 = rt;
        assert_eq!(rt, copied1);
        assert_eq!(rt, copied2);
    }
}
