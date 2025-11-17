use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    NavigateToEpicDetail { epic_id: u32 },
    NavigateToStoryDetail { epic_id: u32, story_id: u32 },
    NavigateToPreviousPage,
    CreateEpic,
    UpdateEpicStatus { epic_id: u32 },
    DeleteEpic { epic_id: u32 },
    CreateStory { epic_id: u32 },
    UpdateStoryStatus { story_id: u32 },
    DeleteStory { epic_id: u32, story_id: u32 },
    Exit,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Copy, Clone)]
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Open => write!(f, "OPEN"),
            Status::InProgress => write!(f, "IN PROGRESS"),
            Status::Resolved => write!(f, "RESOLVED"),
            Status::Closed => write!(f, "CLOSED"),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<u32>,
}

impl Epic {
    pub fn new<N: Into<String>, D: Into<String>>(name: N, description: D) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            status: Status::Open,
            stories: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Story {
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Story {
    pub fn new<N: Into<String>, D: Into<String>>(name: N, description: D) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            status: Status::Open,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Default)]
pub struct DBState {
    pub last_item_id: u32,
    pub epics: std::collections::HashMap<u32, Epic>,
    pub stories: std::collections::HashMap<u32, Story>,
}
