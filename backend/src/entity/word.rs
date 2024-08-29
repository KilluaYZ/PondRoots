use std::ops::Index;
use mysql::{from_value, Row, Value};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Word{
    id: u32,
    british_word: String,
    american_word: String,
    british_soundmark: String,
    american_soundmark: String,
    comment: String,
    notes: String,
    create_time: String,
    update_time: String,
}

impl Word{
    pub(crate) fn new() -> WordBuilder {
        WordBuilder{
            word: Word{
                id: 0,
                british_word: String::from(""),
                american_word: String::from(""),
                british_soundmark: String::from(""),
                american_soundmark: String::from(""),
                comment: String::from(""),
                create_time: String::from(""),
                update_time: String::from(""),
                notes: String::from(""),
            }
        }
    }
}

pub struct WordBuilder {
    word: Word
}

impl  WordBuilder{
    pub fn id(mut self, id: u32) -> Self {
        self.word.id = id;
        self
    }

    pub fn british_word(mut self, british_word: String) -> Self {
        self.word.british_word = british_word;
        self
    }

    pub fn american_word(mut self, american_word: String) -> Self {
        self.word.american_word = american_word;
        self
    }

    pub fn british_soundmark(mut self, british_soundmark: String) -> Self {
        self.word.british_soundmark = british_soundmark;
        self
    }

    pub fn american_soundmark(mut self, american_soundmark: String) -> Self {
        self.word.american_soundmark = american_soundmark;
        self
    }

    pub fn comment(mut self, comment: String) -> Self {
        self.word.comment = comment;
        self
    }

    pub fn notes(mut self, notes: String) -> Self {
        self.word.notes = notes;
        self
    }


    pub fn create_time(mut self, create_time: String) -> Self {
        self.word.create_time = create_time;
        self
    }

    pub fn update_time(mut self, update_time: String) -> Self {
        self.word.update_time = update_time;
        self
    }

    pub fn from_row(self, row: &Row) -> Word{
        let mut word = Word::new();
        let id = row.index(0).clone();
        let british_word = row.index(1).clone();
        let american_word = row.index(2).clone();
        let british_soundmark = row.index(3).clone();
        let american_soundmark = row.index(4).clone();
        let comment = row.index(5).clone();
        let notes = row.index(6).clone();
        let create_time = row.index(7).clone();
        let update_time = row.index(8).clone();
        if id != Value::NULL{
            word = word.id(from_value::<u32>(id));
        }
        if british_word != Value::NULL{
            word = word.british_word(from_value::<String>(british_word));
        }
        if american_word != Value::NULL{
            word = word.american_word(from_value::<String>(american_word));
        }
        if british_soundmark != Value::NULL{
            word = word.british_soundmark(from_value::<String>(british_soundmark));
        }
        if american_soundmark != Value::NULL{
            word = word.american_soundmark(from_value::<String>(american_soundmark));
        }
        if comment != Value::NULL{
            word = word.comment(from_value::<String>(comment));
        }
        if notes != Value::NULL{
            word = word.notes(from_value::<String>(notes));
        }
        if create_time != Value::NULL{
            word = word.create_time(from_value::<String>(create_time));
        }
        if update_time != Value::NULL{
            word = word.update_time(from_value::<String>(update_time));
        }
        word.build()
    }

    pub fn build(self) -> Word {
        self.word
    }
}