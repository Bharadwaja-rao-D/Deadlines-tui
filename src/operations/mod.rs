pub mod display;
pub mod fetch;
pub mod info;
use std::{fs, path::PathBuf, str::FromStr};

use chrono::NaiveDate;
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
    pub deadlines: Vec<Deadline>,
}

impl TryFrom<&str> for Deadline {
    type Error = serde_json::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let deadline = serde_json::from_str(value)?;
        return Ok(deadline);
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
    info_struct.deadlines.push(deadline);
    let info = serde_json::to_string(&info_struct).unwrap();

    fs::write(&args.deadlines_file, info).unwrap();
}

pub fn get_deadlines(file: &PathBuf) -> Info {
    let info = fs::read_to_string(file).unwrap();
    let mut info_struct: Info;
    if info.len() > 0 {
        info_struct = serde_json::from_str(&info).unwrap();

        //Sorting based on the nearest deadline
        info_struct.deadlines.sort_by(|a, b| {
            let date1 = NaiveDate::from_str(&a.date).unwrap();
            let date2 = NaiveDate::from_str(&b.date).unwrap();

            date2.cmp(&date1)
        })
    } else {
        //create a Info struct and insert it
        info_struct = Info {
            deadlines: Vec::new(),
        };
    }

    return info_struct;
}
