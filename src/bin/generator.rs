use url::Url;
use glob::glob;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use diesel::Insertable;
use diesel::insert_into;
use w0bmarkov::*;
use schema::words;

fn parse_line(line: String) -> Option<Vec<String>> {
    static BOTNAMES: [&'static str; 5] = [
        "<Uwe>",
        "<Sexy>",
        "<w0bm>",
        "<nxy>",
        "<elon>"
    ];
    let mut out = vec![];
    let mut it = line.split_whitespace().skip(1); // skip timestamp
    if let Some(name) = it.next() {
        if name == "***" || BOTNAMES.contains(&name) { // remove bot and status messages
            return None;
        }
    }

    for w in it {
        if let Ok(_) = Url::parse(w) {
            return None; // remove messages with urls
        }
        out.push(w.into());
    }

    return Some(out);
}

#[derive(PartialEq, Debug, Clone, Default, Insertable)]
#[table_name="words"]
struct Word {
    curr: String,
    next: Option<String>,
    start_sentinel: bool,
    end_sentinel: bool,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection = establish_connection();

    let mut entries = vec![];
    for file in glob("logs/*.log").expect("Pattern failure")
        .filter_map(Result::ok)
    {
        let f = File::open(file)?;
        let f = BufReader::new(f);
        for line in f.lines().filter_map(Result::ok)
            .filter_map(|l| parse_line(l))
        {
            let mut line = line.iter().peekable();
            if let Some(w) = line.next() { // First word
                if w.starts_with(".") {
                    continue; // filter out bot trigger
                }
                let mut entry = Word::default();
                entry.curr = w.clone();
                entry.start_sentinel = true;
                if let Some(next) = line.peek() {
                    entry.end_sentinel = false;
                    entry.next = Some((*next).clone());
                } else {
                    entry.end_sentinel = true;
                }
                entries.push(entry);
            }
            while let Some(w) = line.next() {
                let mut entry = Word::default();
                entry.curr = w.clone();
                entry.start_sentinel = false;
                if let Some(next) = line.peek() {
                    entry.end_sentinel = false;
                    entry.next = Some((*next).clone());
                } else {
                    entry.end_sentinel = true;
                }
                entries.push(entry);

            }
        }
    }

    for e in entries.chunks(256) {
        let _ = insert_into(words::table)
            .values(e)
            .execute(&connection);
    }



    Ok(())
}
