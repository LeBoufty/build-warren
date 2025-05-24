use chrono::{NaiveDate, NaiveTime};
use std::fmt;
use std::str::FromStr;

pub enum Race {
    Terran,
    Protoss,
    Zerg,
    Any,
}

impl fmt::Display for Race {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Race::Terran => "T",
            Race::Protoss => "P",
            Race::Zerg => "Z",
            Race::Any => "X",
        };
        write!(f, "{}", value)
    }
}

impl FromStr for Race {
    type Err = ();

    fn from_str(input: &str) -> Result<Race, Self::Err> {
        match input {
            "T" => Ok(Race::Terran),
            "P" => Ok(Race::Protoss),
            "Z" => Ok(Race::Zerg),
            "X" => Ok(Race::Any),
            _ => Err(()),
        }
    }
}

pub enum BuildType {
    Cheese,
    AllIn,
    Timing,
    Economic,
    CoOp,
    None,
}

impl fmt::Display for BuildType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            BuildType::Cheese => "Cheese",
            BuildType::AllIn => "All-In",
            BuildType::Timing => "Timing Attack",
            BuildType::Economic => "Economic",
            BuildType::CoOp => "Co-op",
            BuildType::None => "None",
        };
        write!(f, "{}", value)
    }
}

impl FromStr for BuildType {
    type Err = ();

    fn from_str(input: &str) -> Result<BuildType, Self::Err> {
        match input {
            "Cheese" => Ok(BuildType::Cheese),
            "All-In" => Ok(BuildType::AllIn),
            "Timing Attack" => Ok(BuildType::Timing),
            "Economic" => Ok(BuildType::Economic),
            "Co-op" => Ok(BuildType::CoOp),
            "None" => Ok(BuildType::None),
            _ => Err(()),
        }
    }
}

pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
        };
        write!(f, "{}", value)
    }
}

impl FromStr for Difficulty {
    type Err = ();

    fn from_str(input: &str) -> Result<Difficulty, Self::Err> {
        match input {
            "Easy" => Ok(Difficulty::Easy),
            "Medium" => Ok(Difficulty::Medium),
            "Hard" => Ok(Difficulty::Hard),
            _ => Err(()),
        }
    }
}

pub struct Votes {
    score: u32,
    count: u32,
}

pub enum ActionType {
    Worker,
    Unit,
    Building,
    Upgrade,
    Action,
}

impl fmt::Display for ActionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            ActionType::Worker => "Worker",
            ActionType::Unit => "Unit",
            ActionType::Building => "Building",
            ActionType::Upgrade => "Upgrade",
            ActionType::Action => "Action",
        };
        write!(f, "{}", value)
    }
}

impl FromStr for ActionType {
    type Err = ();

    fn from_str(input: &str) -> Result<ActionType, Self::Err> {
        match input {
            "Worker" => Ok(ActionType::Worker),
            "Unit" => Ok(ActionType::Unit),
            "Building" => Ok(ActionType::Building),
            "Upgrade" => Ok(ActionType::Upgrade),
            "Action" => Ok(ActionType::Action),
            _ => Err(()),
        }
    }
}

pub struct OrderEntry {
    supply: u8,
    timestamp: NaiveTime,
    action_type: ActionType,
    action: String,
    comment: Option<String>,
}

pub struct BuildOrder {
    name: String,
    description: Option<String>,
    vod: Option<String>,
    player_race: Race,
    opponent_race: Race,
    build_type: BuildType,
    creator: String,
    votes: Option<Votes>,
    published: Option<NaiveDate>,
    patch: String,
    difficulty: Option<Difficulty>,
    entries: Vec<OrderEntry>,
}
