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

impl fmt::Display for Votes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}% ({} votes)", self.score, self.count)
    }
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

pub struct Action {
    action_type: ActionType,
    name: String,
}

impl Action {
    pub fn new(action_type: ActionType, name: String) -> Self {
        Action { action_type, name }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.action_type, self.name)
    }
}

pub struct OrderEntry {
    supply: u8,
    timestamp: NaiveTime,
    actions: Vec<Action>,
    comment: Option<String>,
}

impl fmt::Display for OrderEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let comment_str = self.comment.as_deref().unwrap_or("");
        write!(
            f,
            "{} supply at {}: [{}] {}",
            self.supply,
            self.timestamp,
            self.actions
                .iter()
                .map(|action| action.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            comment_str
        )
    }
}

impl OrderEntry {
    pub fn new(supply: u8, time: String, actions: Vec<Action>, comment: String) -> Self {
        let timestamp = NaiveTime::parse_from_str(&time, "%H:%M:%S")
            .unwrap_or(NaiveTime::parse_from_str(&time, "%M:%S").unwrap());
        OrderEntry {
            supply,
            timestamp,
            actions,
            comment: if comment.is_empty() {
                None
            } else {
                Some(comment)
            },
        }
    }
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

impl BuildOrder {
    pub fn new() -> Self {
        BuildOrder {
            name: String::new(),
            description: None,
            vod: None,
            player_race: Race::Any,
            opponent_race: Race::Any,
            build_type: BuildType::None,
            creator: String::new(),
            votes: None,
            published: None,
            patch: String::new(),
            difficulty: None,
            entries: Vec::new(),
        }
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }
    pub fn set_vod(&mut self, vod: String) {
        self.vod = Some(vod);
    }
    pub fn set_player_race(&mut self, race: Race) {
        self.player_race = race;
    }
    pub fn set_opponent_race(&mut self, race: Race) {
        self.opponent_race = race;
    }
    pub fn set_build_type(&mut self, build_type: BuildType) {
        self.build_type = build_type;
    }
    pub fn set_creator(&mut self, creator: String) {
        self.creator = creator;
    }
    pub fn set_votes(&mut self, score: u32, count: u32) {
        self.votes = Some(Votes { score, count });
    }
    pub fn set_published(&mut self, date: NaiveDate) {
        self.published = Some(date);
    }
    pub fn set_patch(&mut self, patch: String) {
        self.patch = patch;
    }
    pub fn set_difficulty(&mut self, difficulty: Difficulty) {
        self.difficulty = Some(difficulty);
    }
    pub fn add_step(&mut self, entry: OrderEntry) {
        self.entries.push(entry);
    }
}

#[derive(Debug)]
pub enum BuildOrderError {
    ParseError(String),
    InvalidData(String),
    HttpError(String),
}
