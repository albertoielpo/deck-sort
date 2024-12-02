use std::{
    cmp::Ordering,
    env,
    error::Error,
    fs::{self, File},
    io::{self, IsTerminal, Read},
};
use yaml_rust2::{YamlEmitter, YamlLoader};

enum FlushType {
    STDOUT,
    FILE(String, bool), // file path, backup
}

/// Given a &str containing a yaml file it sorts it and flush
fn sort_and_flush(content: &str, flush_type: FlushType) -> Result<(), Box<dyn Error>> {
    let mut docs = YamlLoader::load_from_str(content)?;

    // Multi document support, doc is a yaml::Yaml
    let doc = &mut docs[0];

    if doc["services"].is_array() {
        // Sort by reference
        if let Some(vec) = &mut doc["services"].as_mut_vec() {
            vec.sort_by(|a, b| {
                let host_a = a["host"].as_str().unwrap_or_default();
                let host_b = b["host"].as_str().unwrap_or_default();
                let ordering = host_a.cmp(&host_b);
                if ordering == Ordering::Greater || ordering == Ordering::Less {
                    return ordering;
                }
                // if are equals then order by path
                let a = a["path"].as_str().unwrap_or_default();
                let b = b["path"].as_str().unwrap_or_default();
                return a.cmp(&b);
            });
        }
    }

    let mut dumped_str = String::new();
    let mut emitter = YamlEmitter::new(&mut dumped_str);
    emitter.dump(doc)?; // dump the YAML object to a String

    match flush_type {
        FlushType::FILE(file_name, backup) => {
            if backup {
                // backup
                let backup_to = format!("{}.bk", &file_name);
                fs::copy(&file_name, &backup_to)?;
                println!("{} backup created", backup_to);
            }

            // flush
            let flush_to = format!("{}", &file_name);
            fs::write(format!("{}", &flush_to), dumped_str)?;
            println!("{} sorted", flush_to);
        }
        FlushType::STDOUT => {
            println!("{}", dumped_str);
        }
    }

    Ok(())
}

/// Sort deck yaml file for kong
/// File input
/// if STDIN then output will be STDOUT
/// if FILE then output will be save file
fn main() -> Result<(), Box<dyn Error>> {
    let mut flush_type = FlushType::STDOUT;
    let args: Vec<String> = env::args().collect();
    let mut input: Box<dyn Read> = if args.len() > 1 {
        let mut backup = false;
        if args.len() > 2 {
            if &args[2] == "true" {
                backup = true;
            }
        }

        flush_type = FlushType::FILE(String::from(&args[1]), backup);
        Box::new(File::open(&args[1])?)
    } else {
        let stdin: io::Stdin = io::stdin();

        if stdin.is_terminal() {
            // No input available
            return Err(Box::from("Standard input is empty"));
        }

        Box::new(stdin)
    };
    let mut content = String::new();
    input.read_to_string(&mut content)?;

    sort_and_flush(&content, flush_type)
}
