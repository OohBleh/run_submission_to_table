use regex::{Regex, internal::Char};
use time::Date;


#[cfg(test)]
mod regex_correctness {
    use crate::parser::{interpret, Difficulty};

    #[test]
    fn tickler_a20_wr() {
let data = 
"Ascension-20 Unseeded - 4-Character in 28m 22s by Tickler - 2nd place

Version: 2.3 03/07/2022
Notes:

defect 1HAU17U5B4NQW
silent I39G06NHKNIY
watcher 9NI379ZS8JPV
ironclad 2D9L3DCZMK2FZ
Submitted by:
Tickler on 2023-01-08, 11:44
Played on:
PC on 2023-01-08 ";
        let data = interpret(data).unwrap();
        assert_eq!(data, Difficulty::A20)
    }
}

fn interpret(data: &str) -> Result<Difficulty, ParseError> {
    let data = data.replace("\n", " ");
    let prefix = Regex::new(r"(?x)
        (Any%|Ascension-20)\s
        (Unseeded|Seeded)\s-\s
        (Ironclad|Silent|Defect|Watcher|4-Character)\s
        (.*)$
    ").unwrap();
    let captures = prefix.captures(&data)
        .unwrap();
    for i in 0..captures.len() {
        println!("{:?}", captures.get(i).unwrap().as_str())
    }

    let diff: Difficulty = captures.get(1)
        .unwrap()
        .as_str()
        .try_into()?;

    let seeding: Seeding = captures.get(2)
        .unwrap()
        .as_str()
        .try_into()?;

    let character: Character = captures.get(3)
        .unwrap()
        .as_str()
        .try_into()?;



    return Ok(diff)
}

#[derive(Debug)]
enum ParseError {
    Difficulty,
    Seeding,
    Character,
    Glitching
}

struct RunData {
    category: Category,
    times: Times,
    version: Version,
    placing: usize,
    runner: String,
    dates: Dates,
    seed: Option<String>,
    unlocks: Option<Unlocks>
}

struct Category {
    character: Character,
    diff: Difficulty,
    seeding: Seeding,
    glitching: Glitching
}

#[derive(Debug)]
enum Character { Ironclad, Silent, Defect, Watcher, Four }

impl TryFrom<&str> for Character {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Ironclad" => Ok(Self::Ironclad),
            "Silent" => Ok(Self::Silent),
            "Defect" => Ok(Self::Defect),
            "Watcher" => Ok(Self::Watcher),
            "4-Character" => Ok(Self::Four),
            _ => Err(ParseError::Character)
        }
    }
}

#[derive(Debug, PartialEq)]
enum Difficulty { Any, A20 }

impl TryFrom<&str> for Difficulty {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Any%" => Ok(Self::Any),
            "Ascension-20" => Ok(Self::A20),
            _ => Err(ParseError::Difficulty)
        }
    }
}

#[derive(Debug)]
enum Seeding { Seeded, Unseeded }

impl TryFrom<&str> for Seeding {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Unseeded" => Ok(Self::Unseeded),
            "Seeded" => Ok(Self::Seeded),
            _ => return Err(ParseError::Seeding)
        }
    }
}


#[derive(Debug)]
enum Glitching { Glitchless, Glitched }

impl TryFrom<&str> for Glitching {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Glitchless" => Ok(Self::Glitchless),
            "Seeded" => Ok(Self::Glitched),
            _ => return Err(ParseError::Glitching)
        }
    }
}

struct RunSetup {
    seed: Option<String>,
    version: Version,
    unlocks: Option<Unlocks>
}

struct Version {
    major: usize,
    minor: usize,
    patch: usize
}

struct Unlocks {
    levels: [usize; 4],
    bosses: [usize; 3]
}

struct Times {
    rta: Time,
    igt: Time
}

struct Time {
    hour: usize,
    minute: usize,
    second: usize,
    milisecond: usize
}

struct Dates {
    submission: Date,
    run: Date
}
