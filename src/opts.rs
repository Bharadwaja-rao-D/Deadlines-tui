use std::{fs::File, io, path::PathBuf};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opts {
    #[structopt(long)]
    pub add: bool,

    //TODO: Later change this to Date struct
    #[structopt(long, short, required_if("add", "true"))]
    pub deadline: Option<String>,

    #[structopt(long, short, required_if("add", "true"))]
    pub course: Option<String>,

    #[structopt(long, short, required_if("add", "true"))]
    pub assignment: Option<String>,

    #[structopt(parse(from_os_str), default_value = "info.json")]
    pub deadlines_file: PathBuf,
}

#[derive(Debug)]
pub struct Args {
    pub deadline: String,
    pub course: String,
    pub assignment: String,
    pub deadlines_file: PathBuf,
}

fn get_file(file: &PathBuf) -> Result<PathBuf, io::Error> {
    if file.exists() {
        return Ok(file.to_path_buf());
    } else {
        let _ = File::create(file).expect("File creation error");
        return Ok(file.to_path_buf());
    }
}

impl TryFrom<Opts> for Args {
    type Error = io::Error;

    fn try_from(value: Opts) -> Result<Self, Self::Error> {
        let deadline = value.deadline.clone().unwrap();
        let course = value.course.clone().unwrap();
        let assignment = value.assignment.clone().unwrap();
        return Ok(Self {
            deadline,
            course,
            assignment,
            deadlines_file: get_file(&value.deadlines_file)?,
        });
    }
}
