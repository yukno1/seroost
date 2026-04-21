use std::{
    collections::HashMap,
    fs::{self, File},
    io,
    path::{Path, PathBuf},
};
use xml::reader::{EventReader, XmlEvent};

use crate::lexer::Lexer;

mod lexer;
fn index_document(doc_content: &str) -> HashMap<String, usize> {
    todo!()
}

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

fn main() -> io::Result<()> {
    let dir_path = "docs.gl\\gl4";
    let dir = fs::read_dir(dir_path)?;
    let top_n = 10;
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

    for (path, tf) in tf_index {
        println!("{path:?} has {count} unique tokens", count = tf.len());
    }

    Ok(())
}
