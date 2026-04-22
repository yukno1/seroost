use std::env;
use std::process::exit;
use std::{
    collections::HashMap,
    fs::{self, File},
    io,
    path::{Path, PathBuf},
};
use xml::reader::{EventReader, XmlEvent};

use crate::lexer::Lexer;

use serde_json::{Deserializer, Result, Serializer};

mod lexer;
// fn index_document(doc_content: &str) -> HashMap<String, usize> {
//     todo!()
// }

fn read_entire_xml_file<P: AsRef<Path>>(file_path: P) -> io::Result<String> {
    let file = File::open(file_path)?;
    let er = EventReader::new(file);

    let mut content = String::new();
    for event in er {
        if let XmlEvent::Characters(text) = event.expect("TODO") {
            content.push_str(&text);
            content.push_str(" ");
        }
    }
    Ok(content)
}

type TermFreq = HashMap<String, usize>;
type TermFreqIndex = HashMap<PathBuf, TermFreq>;

fn check_index(index_path: &str) -> io::Result<()> {
    let index_file = File::open(index_path)?;
    println!("Reading {index_path}...");
    let tf_index: TermFreqIndex = serde_json::from_reader(index_file).expect("serde read success");
    println!(
        "{index_path} contains {count} files",
        count = tf_index.len()
    );
    Ok(())
}

fn index_folder(dir_path: &str) -> io::Result<()> {
    let dir = fs::read_dir(dir_path)?;
    let _top_n = 10;
    let mut tf_index = TermFreqIndex::new();

    for file in dir {
        let file_path = file?.path();

        println!("Indexing {:?}...", &file_path);

        let content = read_entire_xml_file(&file_path)?
            .chars()
            .collect::<Vec<_>>();

        // term frequency table
        let mut tf = TermFreq::new();

        let lexer = Lexer::new(&content);

        for token in lexer {
            let term = token
                .iter()
                .map(|x| x.to_ascii_uppercase())
                .collect::<String>();

            if let Some(freq) = tf.get_mut(&term) {
                *freq += 1;
            } else {
                tf.insert(term, 1);
            }
        }

        // let mut stats = tf.iter().collect::<Vec<_>>();
        // stats.sort_by_key(|(_, f)| *f);
        // stats.reverse();

        tf_index.insert(file_path, tf);
        // println!("{file_path:?}");
        // for (t, f) in stats.iter().take(top_n) {
        //     println!("    {t} => {f}");
        // }
    }

    let index_path = "index.json";
    println!("Saving {index_path}...");
    let index_file = File::create(index_path)?;
    serde_json::to_writer(index_file, &tf_index).expect("serde works fine");

    Ok(())
}

fn main() {
    let mut args = env::args();
    let _program = args.next().expect("path to program is provided");

    let subcommand = args.next().unwrap_or_else(|| {
        println!("ERROR: no subcommand is provided");
        exit(0)
    });

    match subcommand.as_str() {
        "index" => {
            let dir_path = args.next().unwrap_or_else(|| {
                println!("ERROR: no directory is provided for {subcommand} subcommand");
                exit(0);
            });

            index_folder(&dir_path).unwrap_or_else(|err| {
                println!("ERROR: could not index folder {dir_path}: {err}");
                exit(0);
            });
        }
        "search" => {
            let index_path = args.next().unwrap_or_else(|| {
                println!("ERROR: no path to index is provided for {subcommand} subcommand");
                exit(0);
            });
            check_index(&index_path).unwrap_or_else(|err| {
                println!("ERROR: could not check index file {index_path}: {err}");
                exit(0);
            });
        }
        _ => {
            println!("ERROR: unknown subcommand {subcommand}");
            exit(0)
        }
    }
}
