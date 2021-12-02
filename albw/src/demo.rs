use std::{
    convert::{TryFrom, TryInto},
    vec,
};

use serde::{
    de::{Error as _, SeqAccess, Visitor},
    ser::SerializeSeq,
    Deserialize, Serialize, Serializer,
};

use crate::{course, files::IntoBytes, Error, Result};

/// A skippable cutscene.
#[derive(Debug)]
pub struct Demo {
    commands: Vec<Timed<Command>>,
    finish: Timed<Finish>,
}

impl Demo {
    /// Attempt to parse a Demo CSV file.
    pub(crate) fn try_read(data: Box<[u8]>) -> Result<Self> {
        let mut commands = csv::ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_reader(data.as_ref())
            .into_byte_records()
            .map(|record| record.and_then(|record| record.deserialize(None)))
            .filter_map(|row: Result<Row, _>| row.map(|row| row.0).transpose())
            .collect::<Result<Vec<_>, csv::Error>>()
            .map_err(Error::new)?;
        let finish = commands
            .pop()
            .ok_or_else(|| Error::new("The demo file was empty."))?
            .try_map(TryInto::try_into)?;
        Ok(Self { commands, finish })
    }

    /// Retains only commands satisfying the provided predicate.
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&Timed<Command>) -> bool,
    {
        self.commands.retain(f)
    }

    /// Adds an event flag to be set during the cutscene.
    pub fn add_event_flag(&mut self, flag: u16) {
        self.commands
            .insert(0, Timed::new(Command::SetEventFlag(flag)));
    }

    /// Gets a mutable reference to the 'Finish' command.
    pub fn finish_mut(&mut self) -> &mut Timed<Finish> {
        &mut self.finish
    }
}

impl IntoBytes for Demo {
    fn into_bytes(self) -> Box<[u8]> {
        let mut buf = vec![];
        (|| {
            let mut writer = csv::WriterBuilder::new()
                .flexible(true)
                .from_writer(&mut buf);
            for event in self.commands {
                writer.serialize(event)?;
            }
            writer.serialize(self.finish.into_command())
        })()
        .expect("Could not write CSV file.");
        buf.into()
    }
}

/// A row in the CSV file.
#[derive(Debug, PartialEq, Eq)]
struct Row(Option<Timed<Command>>);

impl<'de> Deserialize<'de> for Row {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct RowVisitor;

        impl<'de> Visitor<'de> for RowVisitor {
            type Value = Row;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "A sequence of values.")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let timestamp = seq
                    .next_element::<isize>()?
                    .ok_or_else(|| A::Error::missing_field("timestamp"))?;
                (timestamp >= 0)
                    .then(|| {
                        let timestamp = timestamp as usize;
                        let command = seq
                            .next_element::<&[u8]>()?
                            .ok_or_else(|| A::Error::missing_field("name"))?;
                        let command = match command {
                            b"Finish" => {
                                let course = seq
                                    .next_element()?
                                    .ok_or_else(|| A::Error::missing_field("course"))?;
                                let scene = seq
                                    .next_element()?
                                    .ok_or_else(|| A::Error::missing_field("scene"))?;
                                let index = seq
                                    .next_element()?
                                    .ok_or_else(|| A::Error::missing_field("index"))?;
                                Command::Finish {
                                    course,
                                    scene,
                                    index,
                                }
                            }
                            b"SetEventFlag" => {
                                let flag = seq
                                    .next_element()?
                                    .ok_or_else(|| A::Error::missing_field("flag"))?;
                                Command::SetEventFlag(flag)
                            }
                            name => {
                                let mut args = vec![];
                                while let Some(arg) = seq.next_element::<&[u8]>()? {
                                    args.push(arg.into());
                                }
                                Command::Other {
                                    name: name.into(),
                                    args,
                                }
                            }
                        };
                        Ok(Timed {
                            timestamp,
                            value: command,
                        })
                    })
                    .transpose()
                    .map(Row)
            }
        }

        deserializer.deserialize_seq(RowVisitor)
    }
}

/// A timed event.
#[derive(Debug, Eq, PartialEq)]
pub struct Timed<T> {
    timestamp: usize,
    value: T,
}

impl<T> Timed<T> {
    pub fn new(value: T) -> Self {
        Self {
            timestamp: 0,
            value,
        }
    }

    pub fn set_timestamp(&mut self, timestamp: usize) {
        self.timestamp = timestamp;
    }

    fn try_map<U, F>(self, f: F) -> Result<Timed<U>>
    where
        F: FnOnce(T) -> Result<U>,
    {
        Ok(Timed {
            timestamp: self.timestamp,
            value: f(self.value)?,
        })
    }
}

impl Timed<Command> {
    pub fn is_known(&self) -> bool {
        !matches!(self.value, Command::Other { .. })
    }
}

impl Timed<Finish> {
    fn into_command(self) -> Timed<Command> {
        Timed {
            timestamp: self.timestamp,
            value: self.value.into(),
        }
    }
}

impl Serialize for Timed<Command> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(None)?;
        seq.serialize_element(&self.timestamp)?;
        match &self.value {
            Command::Finish {
                course,
                scene,
                index,
            } => {
                seq.serialize_element("Finish")?;
                seq.serialize_element(course)?;
                seq.serialize_element(scene)?;
                seq.serialize_element(index)?;
            }
            Command::SetEventFlag(flag) => {
                seq.serialize_element("SetEventFlag")?;
                seq.serialize_element(flag)?;
            }
            Command::Other { name, args } => {
                seq.serialize_element(name)?;
                for arg in args {
                    seq.serialize_element(arg)?;
                }
            }
        }
        seq.end()
    }
}

/// A cutscene command.
#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    Finish {
        course: course::Id,
        scene: u16,
        index: u16,
    },
    SetEventFlag(u16),
    Other {
        name: Vec<u8>,
        args: Vec<Box<[u8]>>,
    },
}

impl From<Finish> for Command {
    fn from(finish: Finish) -> Self {
        Self::Finish {
            course: finish.course,
            scene: finish.scene,
            index: finish.index,
        }
    }
}

/// The 'Finish' command.
#[derive(Debug)]
pub struct Finish {
    course: course::Id,
    scene: u16,
    index: u16,
}

impl TryFrom<Command> for Finish {
    type Error = Error;

    fn try_from(command: Command) -> Result<Self, Self::Error> {
        match command {
            Command::Finish {
                course,
                scene,
                index,
            } => Ok(Self {
                course,
                scene,
                index,
            }),
            _ => Err(Error::new("Not a 'Finish' command.")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reads_finish() -> Result<(), csv::Error> {
        let mut event = &b"0,Finish,0,1,2"[..];
        let event = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(&mut event)
            .into_deserialize()
            .next()
            .transpose()?;
        assert_eq!(
            event,
            Some(Row(Some(Timed {
                timestamp: 0,
                value: Command::Finish {
                    course: course::Id::FieldLight,
                    scene: 1,
                    index: 2
                },
            }))),
        );
        Ok(())
    }

    #[test]
    fn it_reads_set_event_flag() -> Result<(), csv::Error> {
        let mut event = &b"0,SetEventFlag,1"[..];
        let event = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(&mut event)
            .into_deserialize()
            .next()
            .transpose()?;
        assert_eq!(
            event,
            Some(Row(Some(Timed {
                timestamp: 0,
                value: Command::SetEventFlag(1),
            }))),
        );
        Ok(())
    }

    #[test]
    fn it_reads_other() -> Result<(), csv::Error> {
        let mut event = &b"0,Other,,"[..];
        let event = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(&mut event)
            .into_deserialize()
            .next()
            .transpose()?;
        assert_eq!(
            event,
            Some(Row(Some(Timed {
                timestamp: 0,
                value: Command::Other {
                    name: b"Other"[..].into(),
                    args: vec![[].into(); 2],
                }
            }))),
        );
        Ok(())
    }

    #[test]
    fn it_reads_comments() -> Result<(), csv::Error> {
        let mut events = &b"-1,comment\n0,Other"[..];
        let comment = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(&mut events)
            .into_deserialize()
            .next()
            .transpose()?;
        assert_eq!(comment, Some(Row(None)),);
        Ok(())
    }
}
