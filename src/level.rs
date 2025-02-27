// std imports
use std::cmp::Ord;
use std::ops::Deref;
use std::result::Result;

// third-party imports
use clap::{
    builder::{EnumValueParser, TypedValueParser, ValueParserFactory},
    ValueEnum,
};
use enum_map::Enum;
use serde::{Deserialize, Serialize};

// ---

#[derive(ValueEnum, Clone, Copy, Debug, Deserialize, Serialize, Eq, Hash, Ord, PartialEq, PartialOrd, Enum)]
#[serde(rename_all = "kebab-case")]
pub enum Level {
    Error,
    Warning,
    Info,
    Debug,
}

// ---

#[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, Hash, Ord, PartialEq, PartialOrd, Enum)]
pub struct RelaxedLevel(Level);

impl Into<Level> for RelaxedLevel {
    fn into(self) -> Level {
        self.0
    }
}

impl Deref for RelaxedLevel {
    type Target = Level;

    fn deref(&self) -> &Level {
        &self.0
    }
}

impl ValueParserFactory for RelaxedLevel {
    type Parser = LevelValueParser;
    fn value_parser() -> Self::Parser {
        LevelValueParser
    }
}

// ---

#[derive(Clone, Debug)]
pub struct LevelValueParser;

impl TypedValueParser for LevelValueParser {
    type Value = RelaxedLevel;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<RelaxedLevel, clap::Error> {
        for (level, values) in Self::alternate_values() {
            if values.iter().cloned().any(|x| value.eq_ignore_ascii_case(x)) {
                return Ok(RelaxedLevel(*level));
            }
        }

        let inner = EnumValueParser::<Level>::new();
        let val = inner.parse_ref(cmd, arg, value)?;
        Ok(RelaxedLevel(val))
    }
}

impl LevelValueParser {
    fn alternate_values<'a>() -> &'a [(Level, &'a [&'a str])] {
        &[
            (Level::Error, &["err", "e"]),
            (Level::Warning, &["warn", "wrn", "w"]),
            (Level::Info, &["inf", "i"]),
            (Level::Debug, &["dbg", "d"]),
        ]
    }
}
