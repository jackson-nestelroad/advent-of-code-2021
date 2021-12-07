use crate::common::{AocError, AocResult};
use std::env::Args;
use std::fmt::{Display, Formatter, Result as DisplayResult};

#[derive(Copy, Clone)]
pub enum SolutionPart {
    A,
    B,
}

impl SolutionPart {
    pub fn from_string(string: &str) -> AocResult<Self> {
        match string {
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            _ => Err(AocError::new("part must be either A or B")),
        }
    }
}

impl Display for SolutionPart {
    fn fmt(&self, f: &mut Formatter) -> DisplayResult {
        let string = match self {
            Self::A => String::from("A"),
            Self::B => String::from("B"),
        };
        write!(f, "{}", string)
    }
}

pub struct ProgramArgs {
    day: u8,
    part: SolutionPart,
    filename: Option<String>,
}

impl ProgramArgs {
    pub fn new(day: u8, part: SolutionPart, filename: Option<String>) -> Self {
        ProgramArgs {
            day,
            part,
            filename,
        }
    }

    pub fn day(&self) -> u8 {
        return self.day;
    }

    pub fn part(&self) -> SolutionPart {
        return self.part;
    }

    pub fn filename(&self) -> &Option<String> {
        return &self.filename;
    }

    fn get_next_string_optional(args: &mut Args) -> Option<String> {
        args.next()
    }

    fn get_next_string(args: &mut Args, name: &str) -> AocResult<String> {
        match Self::get_next_string_optional(args) {
            None => Err(AocError::new(format!("missing {}", name))),
            Some(parsed) => Ok(parsed),
        }
    }

    fn get_next_integer(args: &mut Args, name: &str) -> AocResult<u8> {
        match Self::get_next_string(args, name)?.parse::<u8>() {
            Err(_) => Err(AocError::new(format!("{} must be an integer", name))),
            Ok(parsed) => Ok(parsed),
        }
    }

    pub fn parse_from_args(mut args: Args) -> AocResult<Self> {
        let day = Self::get_next_integer(&mut args, "day")?;
        if day <= 0 || day > 31 {
            return Err(AocError::new("day must be between 1 and 31"));
        }

        let part = SolutionPart::from_string(&Self::get_next_string(&mut args, "part")?)?;

        let filename = Self::get_next_string_optional(&mut args);

        Ok(ProgramArgs::new(day, part, filename))
    }

    pub fn usage(program_name: &str) -> String {
        format!("{} [1-31] [A|B]", program_name)
    }
}
