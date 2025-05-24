use chrono::{NaiveDate, NaiveTime};

pub enum Race {
    Terran = "T",
    Protoss = "P",
    Zerg = "Z",
    Any = "X",
}

pub enum BuildType {
    Cheese = "Cheese",
    AllIn = "All-In",
    Timing = "Timing Attack",
    Economic = "Economic",
    CoOp = "Co-op",
    None = "None",
}

pub enum Difficulty {
    Easy = "Easy",
    Medium = "Medium",
    Hard = "Hard",
}

pub struct Votes {
    score: u32,
    count: u32,
}

pub enum ActionType {
    Worker = "Worker",
    Unit = "Unit",
    Building = "Building",
    Upgrade = "Upgrade",
    Action = "Action",
}

pub struct OrderEntry {
    supply: u8,
    timestamp: NaiveTime,
    actionType: ActionType,
    action: String,
    comment: Option<String>,
}

pub struct BuildOrder {
    name: String,
    description: Option<String>,
    vod: Option<String>,
    playerRace: Race,
    opponentRace: Race,
    buildType: BuildType,
    creator: String,
    votes: Option<Votes>,
    published: Option<NaiveDate>,
    patch: String,
    difficulty: Option<Difficulty>,
    entries: Vec<OrderEntry>,
}
