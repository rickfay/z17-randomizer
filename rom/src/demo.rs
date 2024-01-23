use crate::scene::SpawnPoint;
use crate::{files::IntoBytes, Result};
use game::Course;
use serde::{ser::SerializeSeq, Serialize, Serializer};
use std::vec;

/// A cutscene file (CSV)
#[derive(Debug)]
pub struct Demo {
    commands: Vec<TimedCommand>,
}

impl Demo {
    pub fn new() -> Self {
        Self { commands: Vec::new() }
    }

    /// Change the background music
    pub fn bgm_start(&mut self, timestamp: usize, bgm: &str) {
        self.commands.push(TimedCommand::new(timestamp, Command::BgmStart { bgm: bgm.to_owned() }));
    }

    /// Adds an event flag to be set during the cutscene.
    pub fn set_event_flag(&mut self, timestamp: usize, flag: u16) {
        self.commands.push(TimedCommand::new(timestamp, Command::SetEventFlag(flag.into())));
    }

    pub fn finish(&mut self, timestamp: usize, sp: SpawnPoint) {
        self.commands.push(TimedCommand::new(
            timestamp,
            Command::Finish { course: sp.course, scene: sp.scene as u16 - 1, index: sp.spawn as u16 },
        ));
    }
}

impl IntoBytes for Demo {
    fn into_bytes(self) -> Box<[u8]> {
        let mut buf = vec![];
        let mut writer = csv::WriterBuilder::new().flexible(true).from_writer(&mut buf);

        for event in self.commands {
            writer.serialize(event).expect("Could not write CSV file.");
        }

        drop(writer);
        buf.into()
    }
}

/// A cutscene command.
#[derive(Debug, Eq, PartialEq)]
enum Command {
    BgmStart { bgm: String },
    Finish { course: Course, scene: u16, index: u16 },
    SetEventFlag(u16),
}

/// A cutscene command that happens at a specific time.
#[derive(Debug, Eq, PartialEq)]
struct TimedCommand {
    pub timestamp: usize,
    pub command: Command,
}

impl TimedCommand {
    pub fn new(timestamp: usize, command: Command) -> Self {
        Self { timestamp, command }
    }
}

impl Serialize for TimedCommand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(None)?;
        seq.serialize_element(&self.timestamp)?;
        match &self.command {
            Command::BgmStart { bgm } => {
                seq.serialize_element("BgmStart")?;
                seq.serialize_element(bgm)?;
            },
            Command::Finish { course, scene, index } => {
                seq.serialize_element("Finish")?;
                seq.serialize_element(course)?;
                seq.serialize_element(scene)?;
                seq.serialize_element(index)?;
            },
            Command::SetEventFlag(flag) => {
                seq.serialize_element("SetEventFlag")?;
                seq.serialize_element(flag)?;
            },
        }
        seq.end()
    }
}
