mod pair_store;

use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Input directory. Defaults to the current directory
    #[structopt(name = "input", long, short, parse(from_os_str))]
    input: Option<PathBuf>,

    /// Output file. If absent the result will be printed.
    #[structopt(name = "output", long, short, parse(from_os_str))]
    output: Option<PathBuf>,

    /// A regex to extract the node names from within the file.
    extract_regex: String,

    /// A replacement regex applied to the result of the extract-find regex.
    #[structopt(long)]
    extract_replace_regex: Option<String>,

    /// The string the matches of extraction replace regex will be replaced with.
    /// Defaults to "".
    #[structopt(long)]
    extract_replace_string: Option<String>,

    /// A regex to extract the node names from the path of the files.
    /// If not specified, the file stem is used. (ex: `/dir/foo.txt` becomes `foo`)
    #[structopt(long)]
    path_regex: Option<String>,

    /// A replacement regex applied to the result of the path regex.
    #[structopt(long)]
    path_replace_regex: Option<String>,

    /// The string the matches of path replace regex will be replaced with.
    /// Defaults to "".
    #[structopt(long)]
    path_replace_string: Option<String>,

    /// Allow multiple edges with the same source and target node.
    #[structopt(long, short)]
    multiple: bool,

    /// Stop removing # from node names. Note that enabling this option can cause the `.tgf` to be misinterpreted by other programs.
    #[structopt(long, short)]
    disable_hash_removal: bool,

    /// A regex which filenames must match to be included in the scan.
    /// If not specified, all files in the directory are used.
    /// Note that only the file stem and extension are matched agains this regex.
    /// So ".*e.*" matches `file_tgf/src/pair_store.rs` but not `file_tgf/src/main.rs`,
    /// and ".*c.*" would match neither of them.
    #[structopt(long, short)]
    filename_regex: Option<String>,
}

use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

use crate::pair_store::PairStore;

fn compile_optional_regex(option: Option<String>) -> Result<Option<Regex>, impl Error> {
    match option.map(|r| Regex::new(&r)) {
        Some(Ok(x)) => Ok(Some(x)),
        Some(Err(e)) => Err(e),
        None => Ok(None),
    }
}

fn make_empty_string() -> String {
    "".to_string()
}

fn main() -> Result<(), Box<Error>> {
    let opt = Opt::from_args();

    let re = Regex::new(&opt.extract_regex)?;
    let extract_replace = compile_optional_regex(opt.extract_replace_regex)?;
    let extract_replace_string = opt.extract_replace_string.unwrap_or_else(make_empty_string);

    let file_re = compile_optional_regex(opt.filename_regex)?;

    let path_re = compile_optional_regex(opt.path_regex)?;
    let path_replace = compile_optional_regex(opt.path_replace_regex)?;
    let path_replace_string = opt.path_replace_string.unwrap_or_else(make_empty_string);

    let mut edges: PairStore = if opt.multiple {
        PairStore::Vec(Vec::new())
    } else {
        PairStore::HashMap(HashMap::new())
    };

    use walkdir::WalkDir;

    let search_files = WalkDir::new(opt.input.unwrap_or_else(|| PathBuf::from(".")))
        .into_iter()
        .filter_map(|e| e.ok());

    let mut file_nodes: Vec<String> = Vec::new();

    for dir_entry in search_files {
        let path = dir_entry.path();

        if let Some(ref r) = file_re {
            if path.file_name()
                .and_then(|os_str| os_str.to_str())
                .map(|f| !r.is_match(f))
                .unwrap_or(false) {
                continue;
            }
        }

        if let Some(ref r) = path_re {
            if let Some(path) = path.to_str() {
                for capture in r.captures_iter(&path) {
                    let mut node: String = if opt.disable_hash_removal {
                        (&capture[1]).to_owned()
                    } else {
                        (&capture[1]).replace("#", "")
                    };

                    if let Some(ref replace) = path_replace {
                        let s: &str = &path_replace_string;
                        node = replace.replace(&node, s).to_string();
                    }

                    file_nodes.push(node);
                }
            } else {
                continue;
            }
        } else {
            file_nodes.push(
                path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("file_tgf_unknown")
                .to_owned()
            );
        }

        for file_node in file_nodes.iter() {
            let reader = {
                if let Ok(f) = File::open(path){
                    BufReader::new(f)
                } else {
                    continue;
                }
            };

            for line in reader.lines().filter_map(|l| l.ok()) {
                for capture in re.captures_iter(&line) {
                    let mut target = if opt.disable_hash_removal {
                        (&capture[1]).to_owned()
                    } else {
                        (&capture[1]).replace("#", "")
                    };

                    if let Some(ref replace) = extract_replace {
                        let s: &str = &extract_replace_string;
                        target = replace.replace(&target, s).to_string();
                    }

                    edges.add_pair(
                        file_node.clone(),
                        target
                     );
                }
            }
        }

        file_nodes.clear();
    }

    let sorted_edges = edges.sorted_pairs();

    let tgf = get_tgf(&sorted_edges);

    use std::fs::OpenOptions;
    if let Some(Ok(mut f)) = opt.output.map(|path|  OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)) {
            use std::io::Write;
        f.write_all(tgf.as_bytes())?;
        f.flush()?;
    } else {
        println!("{}", tgf);
    }

    Ok(())
}

//the original version of this fn was from https://github.com/Ryan1729/lua_call_tgf
fn get_tgf<S1, S2>(edges: &Vec<(S1, S2)>) -> String
where
    S1: AsRef<str>,
    S2: AsRef<str>
 {
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
