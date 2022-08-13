pub mod display;
pub mod fetch;
pub mod info;
use std::{collections::VecDeque, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::opts::Args;

pub enum Operations {
    Add,
    Display,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Deadline {
    pub course: String,
    pub assignment: String,
    pub date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    pub deadlines: VecDeque<Deadline>,
}

impl TryFrom<&str> for Deadline {
    type Error = serde_json::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        return Ok(serde_json::from_str(value)?);
    }
}

impl TryFrom<&Deadline> for String {
    type Error = serde_json::Error;

    fn try_from(value: &Deadline) -> Result<Self, Self::Error> {
        return Ok(serde_json::to_string(value)?);
    }
}

//TODO: Need to optimize it..
//Instead of reading whole file, appending and then writing to the file ...
//check for append option.
pub fn add(args: Args) {
    let deadline = Deadline {
        course: args.course,
        assignment: args.assignment,
        date: args.deadline,
    };

    //Add this struct into AllDeadlines obj in the args.deadline_file

    let mut info_struct: Info = get_deadlines(&args.deadlines_file);
    info_struct.deadlines.push_back(deadline);
    let info = serde_json::to_string(&info_struct).unwrap();

    fs::write(&args.deadlines_file, info).unwrap();
}

pub fn get_deadlines(file: &PathBuf) -> Info {
    let info = fs::read_to_string(file).unwrap();
    let info_struct;
    if info.len() > 0 {
        info_struct = serde_json::from_str(&info).unwrap();
    } else {
        //create a Info struct and insert it
        info_struct = Info {
            deadlines: VecDeque::new(),
        };
    }

    return info_struct;
}
