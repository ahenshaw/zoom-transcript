use anyhow::{bail, Result};
use chrono::NaiveTime;
use prse::{parse, try_parse};

#[derive(Debug)]
pub struct Snippet {
    pub index: u64,
    pub start: NaiveTime,
    pub end: NaiveTime,
    pub speaker: Option<String>,
    pub text: String,
}

impl Snippet {
    pub fn from_block(block: &str) -> Result<Snippet> {
        let mut lines = block.lines();
        let Some(index) = lines.next() else {
            bail!("Expected first line of snippet")
        };
        let Some(time) = lines.next() else {
            bail!("Expected second line of snippet")
        };
        let Some(text) = lines.next() else {
            bail!("Expected third line of snippet")
        };

        let index = index.parse::<u64>()?;
        let t = parse!(time, "{}:{}:{}.{} --> {}:{}:{}.{}");
        let start = NaiveTime::from_hms_milli_opt(t.0, t.1, t.2, t.3).unwrap();
        let end = NaiveTime::from_hms_milli_opt(t.4, t.5, t.6, t.7).unwrap();
        let (speaker, text) = match try_parse!(text, "{}: {}") {
            Ok((speaker, leftover)) => (Some(speaker), leftover),
            _ => (None, text.to_string()),
        };

        Ok(Snippet {
            index,
            start,
            end,
            speaker,
            text,
        })
    }
}
