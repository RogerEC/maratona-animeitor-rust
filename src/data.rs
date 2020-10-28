
// use std::io::Error;
use std::fmt;
use std::collections::BTreeMap;
// use serde::Deserialize;
use serde::Serialize;
// use serde_json;


#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum Answer {
    Yes,
    No,
    Wait,
    Unk
}

#[derive(Debug)]
pub enum ContestError {
    // IO(Error),
    Parse(std::num::ParseIntError),
    // InvalidUri(String),
    // Hyper(String),
    Simple(String),
    UnmatchedTeam(String)
}

impl std::error::Error for ContestError {}

impl std::convert::From<std::num::ParseIntError> for ContestError {
    fn from(error: std::num::ParseIntError) -> Self {
        ContestError::Parse(error)
    }
}
// impl std::convert::From<Error> for ContestError {
//     fn from(error: Error) -> Self {
//         ContestError::IO(error)
//     }
// }

// impl std::convert::From<hyper::Error> for ContestError {
//     fn from(error: hyper::Error) -> Self {
//         ContestError::Hyper(error)
//     }

// }

// impl std::convert::From<warp::http::uri::InvalidUri> for ContestError {
//     fn from(error: warp::http::uri::InvalidUri) -> Self {
//         ContestError::InvalidUri(error)
//     }
// }

impl fmt::Display for ContestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Answer could not be parsed: {:?}", self)
    }
}


impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Answer::Yes => "Accepted",
            Answer::No => "Wrong Answer",
            Answer::Wait => "Judging...",
            _ => "Error!"
        })
    }
}

#[derive(Debug, Serialize)]
pub struct Problem {
    solved : bool,
    wait : bool,
    submissions : i64,
    penalty: i64
}

impl Problem {
    fn empty() -> Self {
        Problem { solved : false, wait : false, submissions : 0, penalty : 0 }
    }
    fn add_run_problem(&mut self, tim : i64, answer: Answer) {
        match answer {
            Answer::Yes => {
                self.solved = true;
                self.submissions += 1;
                self.penalty += tim;
            },
            Answer::No => {
                // TODO many corner cases!
                self.submissions += 1;
                self.penalty += 20;
                self.wait = false;
            },
            Answer::Wait => {
                self.wait = true;                
            },
            _ => {

            }
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Team {
    pub login : String,
    pub escola : String,
    pub name : String,
    pub placement : usize,
    pub problems : BTreeMap<String, Problem>,
}

impl Team {
    pub fn new(login : &str, escola : &str, name : &str) -> Self {
        Self {
            login : login.to_string(),
            escola : escola.to_string(),
            name : name.to_string(),
            placement : 0,
            problems : BTreeMap::new()
        }
    }

    pub fn dummy() -> Self {
        Self::new("<login>", "<escola>", "<nome>")
    }


    pub fn apply_run(&mut self, run : &RunTuple) {
        self.problems.entry(run.prob.clone())
            .or_insert(Problem::empty())
            .add_run_problem(run.time, run.answer.clone());
    }

    fn score(&self) -> (i64, i64) {
        let mut solved = 0;
        let mut penalty = 0;
        for (_, value) in self.problems.iter() {
            if value.solved {
                solved += 1;
                penalty += value.penalty;
            }
        }
        (solved, penalty)
    }
}

#[derive(Debug)]
pub struct ContestFile {
    pub contest_name : String,
    pub teams : BTreeMap<String, Team>,
    pub current_time : i64,
    pub maximum_time : i64,
    pub score_freeze_time : i64,
    pub penalty_per_wrong_answer : i64,
    pub score_board : Vec<String>,
    pub number_problems : usize
}

impl ContestFile {
    pub fn new(contest_name : String
        , teams : Vec<Team>
        , current_time : i64
        , maximum_time : i64
        , score_freeze_time : i64
        , penalty : i64
        , number_problems : usize) -> Self {

        let mut m = BTreeMap::new();
        for t in teams {
            m.insert(t.login.clone(), t);
        }
        Self {
            contest_name : contest_name,
            teams : m,
            current_time : current_time,
            maximum_time : maximum_time,
            score_freeze_time : score_freeze_time,
            penalty_per_wrong_answer : penalty,
            score_board : Vec::new(),
            number_problems : number_problems
        }
    }


    pub fn reload_score(&mut self) -> Result<(), ContestError> {
        let mut score_board = Vec::new();
        for (key, _) in self.teams.iter() {
            score_board.push(key.clone());
        }
        score_board.sort_by(|a,b| {
            let (solved_a, penalty_a) = self.teams.get(a).unwrap().score();
            let (solved_b, penalty_b) = self.teams.get(b).unwrap().score();

            if solved_a == solved_b {
                return penalty_a.cmp(&penalty_b);
            }
            return solved_b.cmp(&solved_a);
        });
        
        for (i, v) in score_board.iter().enumerate() {
            match self.teams.get_mut(v) {
                None => return Err(ContestError::UnmatchedTeam(v.clone())),
                Some(t) => t.placement = i+1
            }
        }

        self.score_board = score_board;
        Ok(())
    }


    pub fn dummy() -> Self {
        Self::new("Dummy Contest".to_string(), Vec::new(), 0, 0, 0, 0, 50)
    }
}

#[derive(Debug, Clone)]
pub struct RunTuple {
    pub id : i64,
    pub time : i64,
    pub team_login : String,
    pub prob : String,
    pub answer : Answer
}

#[derive(Debug)] 
pub struct RunsFile {
    pub runs : Vec<RunTuple>
}

#[derive(Debug, Serialize)]
pub struct RunsPanelItem {
    pub id : i64,
    pub placement : usize,
    pub color : i64,
    pub escola : String,
    pub team_name : String,
    pub team_login : String,
    pub problem : String,
    pub result : Answer
}

impl RunsFile {
    pub fn empty() -> Self {
        RunsFile {
            runs : Vec::new()
        }
    }

    pub fn latest_n(&self, n : usize) -> Vec<RunTuple> {
        let mut ret = self.runs.clone();
        ret.sort_by(|a, b| 
            b.time.cmp(&a.time)
            // a.time.cmp(&b.time)
        );
        ret.truncate(n);
        ret
    }

    pub fn add_run(&mut self, t : &RunTuple) {
        self.runs.push(t.clone())
    }
}


