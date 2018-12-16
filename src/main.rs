use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    // /// Input directory. Defaults to the current directory
    // #[structopt(name = "input", long, short, parse(from_os_str))]
    // output: Option<PathBuf>,
    //
    // /// Output file. If absent the result will be printed.
    // #[structopt(name = "output", long, short, parse(from_os_str))]
    // output: Option<PathBuf>,

    /// A regex to extract the node names from within the file.
    extract_regex: String,

    // /// A replacement regex applied to the result of the extract-find regex.
    // #[structopt(long, short="er")]
    // extract_replace: Option<String>,
    //
    // /// A regex to extract the node names from the path of the files.
    // /// If not specified, the file stem is used. (ex: `/dir/foo.txt` becomes `foo`)
    // #[structopt(long, short)]
    // path_regex: Option<String>,
    //
    // /// A replacement regex applied to the result of the path regex.
    // #[structopt(long, short="pr")]
    // path_replace: Option<String>,
}

use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() -> Result<(), Box<Error>> {
    let opt = Opt::from_args();

    let re = Regex::new(&opt.extract_regex)?;

    let mut edges = Vec::new();

    use walkdir::WalkDir;

    let search_files = WalkDir::new(".").into_iter().filter_map(|e| e.ok());

    for dir_entry in search_files {
        let path = dir_entry.path();
        let file_node: String = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("file_tgf_unknown")
            .to_owned();

        let reader = {
            if let Ok(f) = File::open(path){
                BufReader::new(f)
            } else {
                continue;
            }
        };

        for line in reader.lines().filter_map(|l| l.ok()) {
            for capture in re.captures_iter(&line) {
                edges.push((file_node.clone(), (&capture[1]).to_owned()));
            }
        }
    }

    edges.sort();

    let tgf = get_tgf(&edges);

    println!("{}", tgf);

    Ok(())
}

//this fn was originally from https://github.com/Ryan1729/lua_call_tgf
fn get_tgf<S1: AsRef<str>, S2: AsRef<str>>(edges: &Vec<(S1, S2)>) -> String {
    use std::collections::HashMap;

    let mut node_labels = HashMap::new();

    let mut counter = 0;
    for &(ref s1, ref s2) in edges.iter() {
        node_labels.entry(s1.as_ref()).or_insert_with(|| {
            counter += 1;
            counter
        });
        node_labels.entry(s2.as_ref()).or_insert_with(|| {
            counter += 1;
            counter
        });
    }

    let mut tgf = String::new();

    let mut node_label_pairs: Vec<_> = node_labels.iter().collect();

    node_label_pairs.sort();

    for &(node, label) in node_label_pairs.iter() {
        tgf.push_str(&format!("{} {}\n", label, node));
    }

    tgf.push_str("#\n");

    for edge in edges.iter() {
        let label1: usize = *node_labels.get(edge.0.as_ref()).unwrap_or(&0);
        let label2: usize = *node_labels.get(edge.1.as_ref()).unwrap_or(&0);

        tgf.push_str(&format!("{} {}\n", label1, label2))
    }

    tgf
}
