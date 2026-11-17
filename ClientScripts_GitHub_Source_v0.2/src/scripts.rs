//! Minimal, dependency-free .csk syntax checker and separate script state.
//! It deliberately does NOT execute attack/place actions until genuine
//! version-appropriate client input hooks are implemented.

use std::{collections::BTreeMap, fs, path::Path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event { Load, Every(u64), Key(String) }

#[derive(Debug, Clone, PartialEq)]
pub enum Action { Say(String), Attack, Place, Look(f32, f32) }

#[derive(Debug, Clone)]
pub struct Rule { pub event: Event, pub actions: Vec<(usize,Action)> }

#[derive(Debug, Clone)]
pub struct Script { pub name: String, pub rules: Vec<Rule> }

#[derive(Default)]
pub struct Store { pub scripts: BTreeMap<String,Script>, pub errors: Vec<String> }

pub fn parse(name: &str, source: &str) -> Result<Script, Vec<String>> {
    let mut rules:Vec<Rule> = Vec::new();
    let mut errors:Vec<String> = Vec::new();
    for (i, raw) in source.lines().enumerate() {
        let n = i + 1;
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') { continue }
        if raw.starts_with(' ') || raw.starts_with('\t') {
            let action = if let Some(v) = line.strip_prefix("say ") { Some(Action::Say(v.to_owned())) }
                else if line == "attack" { Some(Action::Attack) }
                else if line == "place" { Some(Action::Place) }
                else if let Some(rest) = line.strip_prefix("look ") {
                    let v: Vec<_> = rest.split_whitespace().collect();
                    if v.len() == 2 {
                        match (v[0].parse::<f32>(), v[1].parse::<f32>()) {
                            (Ok(y), Ok(p)) if y.is_finite() && p.is_finite() => Some(Action::Look(y,p)),
                            _ => None
                        }
                    } else { None }
                } else { None };
            match (rules.last_mut(), action) {
                (Some(rule), Some(action)) => rule.actions.push((n, action)),
                (None, _) => errors.push(format!("{name}:{n}: action without event")),
                (_, None) => errors.push(format!("{name}:{n}: unknown action: {line}")),
            }
        } else {
            let event = line.strip_suffix(':').and_then(|head| {
                if head == "on load" { Some(Event::Load) }
                else if let Some(v) = head.strip_prefix("every ").and_then(|v|v.strip_suffix(" ticks")) {
                    v.parse::<u64>().ok().filter(|n|*n>0).map(Event::Every)
                } else {head.strip_prefix("on key ").filter(|s|!s.is_empty()).map(|s|Event::Key(s.to_owned()))}
            });
            if let Some(event) = event {rules.push(Rule{event,actions:Vec::new()})}
            else {errors.push(format!("{name}:{n}: unknown event: {line}"))}
        }
    }
    if errors.is_empty() {Ok(Script{name:name.to_owned(),rules})} else {Err(errors)}
}

impl Store {
    pub fn reload(&mut self, root: &Path) -> Result<(), String> {
        let mut newer = BTreeMap::new();
        let mut errors = Vec::new();
        fs::create_dir_all(root).map_err(|e|e.to_string())?;
        for file in fs::read_dir(root).map_err(|e|e.to_string())? {
            let file=file.map_err(|e|e.to_string())?;
            let path=file.path();
            if path.extension().and_then(|v|v.to_str()) != Some("csk") {continue}
            let Some(name)=path.file_stem().and_then(|v|v.to_str()) else {continue};
            if name.is_empty() || name.len()>48 || !name.bytes().all(|v|v.is_ascii_alphanumeric() || v==b'-' || v==b'_') {
                errors.push(format!("invalid script filename: {}",path.display()));continue;
            }
            match fs::read_to_string(&path) {
                Ok(source)=>match parse(name,&source) {Ok(script)=>{newer.insert(name.to_owned(),script);},Err(err)=>errors.extend(err)},
                Err(e)=>errors.push(format!("{name}: read failed: {e}")),
            }
        }
        self.scripts=newer;
        self.errors=errors;
        Ok(())
    }
}

pub fn example_diagnostics() -> String {
    let source = "on load:\n    say ClientScripts parser ready!\n";
    match parse("hello",source) {
        Ok(script)=>format!("Rust .csk parser ready ({} rule). Game input hooks not connected.",script.rules.len()),
        Err(errors)=>format!("Parser errors: {}", errors.join("; ")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn good_script() {
        let s = parse("demo", "on load:\n    say hello\nevery 20 ticks:\n    attack\n").unwrap();
        assert_eq!(s.rules.len(),2);
    }
    #[test]
    fn line_errors() {
        assert!(parse("demo", "on load:\n    teleport anywhere\n").unwrap_err()[0].contains("demo:2"));
    }
}
