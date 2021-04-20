use std::collections::HashMap;

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
    pub fn get(&self, key: &str) -> Option<&Value> {
        let lowercase_key = key.to_lowercase();

        for (&k, v) in self.data.iter() {
            if k.to_lowercase() == lowercase_key {
                return Some(v);
            }
        }

        None
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
