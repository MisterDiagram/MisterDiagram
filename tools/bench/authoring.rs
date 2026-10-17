//! Contributor-original authoring benchmark, not a CI performance gate.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
use misterdiagram_engine::Session;
use serde_json::{Value, json};
use std::{io::Read, time::Instant};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    let input = args.get(1).ok_or("usage: authoring-bench COMMANDS.json OUTPUT.vsdx")?;
    let output = args.get(2).ok_or("missing output")?;
    let commands = read_commands(std::fs::File::open(input)?)?;
    let mut session = Session::new();
    let start = Instant::now();
    let mut slice = start;
    for (index, (id, params)) in commands.iter().enumerate() {
        let command_start = Instant::now();
        session.execute(id, params)?;
        println!("{}", json!({"kind":"command","index":index+1,"id":id,"seconds":command_start.elapsed().as_secs_f64()}));
        if (index + 1) % 250 == 0 || index + 1 == commands.len() {
            println!("{}", json!({"kind":"slice","end":index+1,"seconds":slice.elapsed().as_secs_f64()}));
            slice = Instant::now();
        }
    }
    session.execute("file.save", &json!({"path":output}))?;
    println!(
        "{}",
        json!({"kind":"total","commands":commands.len(),"shapes":session.document().pages.first().map(|p|p.shapes.len()),"seconds":start.elapsed().as_secs_f64()})
    );
    Ok(())
}

fn read_commands(input: impl Read) -> Result<Vec<(String, Value)>, Box<dyn std::error::Error>> {
    let mut bytes = Vec::new();
    input.take(16 * 1024 * 1024 + 1).read_to_end(&mut bytes)?;
    if bytes.len() > 16 * 1024 * 1024 {
        return Err("benchmark input byte limit".into());
    }
    let commands: Vec<(String, Value)> = serde_json::from_slice(&bytes)?;
    if commands.len() > 8192 {
        return Err("benchmark command count limit".into());
    }
    Ok(commands)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hostile_workloads_are_refused() {
        assert!(read_commands(&b"not json"[..]).is_err());
        assert!(read_commands(&b"[[1,{}]]"[..]).is_err());
        let deep = format!("{}0{}", "[".repeat(200), "]".repeat(200));
        assert!(read_commands(deep.as_bytes()).is_err());
        let oversized = std::io::repeat(b' ').take(16 * 1024 * 1024 + 1);
        assert!(read_commands(oversized).unwrap_err().to_string().contains("byte limit"));
        let commands = serde_json::to_vec(&vec![("file.new", json!({})); 8193]).unwrap();
        assert!(read_commands(commands.as_slice()).unwrap_err().to_string().contains("command count limit"));
        assert_eq!(read_commands(&b"[[\"file.new\",{}]]"[..]).unwrap().len(), 1);
    }
}
