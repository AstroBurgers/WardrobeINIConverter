use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::structs::*;
use rayon::prelude::*;
use once_cell::sync::Lazy;
use dashmap::DashMap;

static INTERNED_NAMES: Lazy<DashMap<String, &'static str>> = Lazy::new(DashMap::new);

pub fn parse_file(file_path: &str) -> Vec<Entry> {
    println!("Initializing parser...");

    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let sections = chunk_sections(reader);
    println!("Chunked file into {} sections...", sections.len());

    let parsed_entries: Vec<Entry> = sections
        .into_par_iter()
        .map(|section| parse_entry(&section))
        .collect();

    println!("Finished parsing file...");
    parsed_entries
}

fn chunk_sections<R: BufRead>(reader: R) -> Vec<Vec<String>> {
    let mut sections = Vec::new();
    let mut current_section = Vec::new();

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(l) => l,
            Err(_) => continue,
        };

        let trimmed = line.trim();
        if trimmed.len() > 1
            && trimmed.starts_with('[')
            && trimmed.ends_with(']')
            && !current_section.is_empty()
        {
            sections.push(std::mem::take(&mut current_section));
        }

        current_section.push(line);
    }

    if !current_section.is_empty() {
        sections.push(current_section);
    }

    sections
}

fn parse_entry(lines: &[String]) -> Entry {
    let mut combo_buffer = Vec::with_capacity(16);

    let mut gender = String::new();
    let mut entry_name = String::new();

    for raw in lines {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }

        if line.len() > 2 && line.starts_with('[') && line.ends_with(']') {
            entry_name = line[1..line.len() - 1].to_string();
            continue;
        }

        let Some(eq) = line.find('=') else { continue };
        let (comp_name_str, value_str) = line.split_at(eq);
        let value_str = &value_str[1..];

        if comp_name_str.eq_ignore_ascii_case("Gender") {
            gender = value_str.trim().to_string();
            continue;
        }

        let Some((id_str, tex_str)) = value_str.split_once(':') else {
            continue;
        };
        if let (Some(id), Some(tex)) = (try_parse_int(id_str), try_parse_int(tex_str)) {
            combo_buffer.push(CompCombo {
                comp_name: intern_comp_name(comp_name_str),
                comp_id: id as i16,
                comp_tex: tex as i8,
            });
        }
    }

    Entry {
        entry_name,
        combos: combo_buffer,
        gender,
    }
}

fn try_parse_int(s: &str) -> Option<i32> {
    s.trim().parse::<i32>().ok()
}

fn intern_comp_name(name: &str) -> &'static str {
    let lowered = name.to_ascii_lowercase();
    if let Some(existing) = INTERNED_NAMES.get(&lowered) {
        return *existing;
    }

    let static_str: &'static str = Box::leak(lowered.clone().into_boxed_str());
    INTERNED_NAMES.insert(lowered, static_str);
    static_str
}
