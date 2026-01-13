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
