use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub struct Headers<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> Headers<'buf> {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl<'buf> Headers<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        let lowercase_key = key.to_lowercase();

        for (&k, v) in self.data.iter() {
            if k.to_lowercase() == lowercase_key {
                return Some(v);
            }
        }

        None
    }

    pub fn add(&mut self, key: &'buf str, val: &'buf str) {
        // TODO: deduplicate code
        self.data
            .entry(key)
            .and_modify(|existing| match existing {
                Value::Single(prev) => *existing = Value::Multiple(vec![prev, val]),
                Value::Multiple(vec) => vec.push(val),
            })
            .or_insert(Value::Single(val));
    }
}

impl<'buf> From<&'buf str> for Headers<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();

        for line in s.lines() {
            if line == "" {
                break;
            }

            if let Some(i) = line.find(':') {
                let key = &line[..i];
                let val = (&line[i + 1..]).strip_prefix(' ').unwrap_or("");

                data.entry(key)
                    .and_modify(|existing| match existing {
                        Value::Single(prev) => *existing = Value::Multiple(vec![prev, val]),
                        Value::Multiple(vec) => vec.push(val),
                    })
                    .or_insert(Value::Single(val));
            }
        }

        Headers { data }
    }
}

impl<'buf> Display for Headers<'buf> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        for (&k, v) in self.data.iter() {
            match v {
                Value::Single(v) => write!(f, "{}: {}\r\n", k, v),
                Value::Multiple(v) => {
                    for &header_value in v.iter() {
                        write!(f, "{}: {}\r\n", k, header_value)?;
                    }
                    Ok(())
                }
            }?;
        }
        Ok(())
    }
}
