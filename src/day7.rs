use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

enum File<'a> {
    Dir(PathAndName<'a>),
    PlainFile(&'a str, u32),
}

/// Path (parent) and the directory name
type PathAndName<'a> = (Vec<&'a str>, &'a str);

type Lookup<'a> = HashMap<PathAndName<'a>, Directory<'a>>;

type Directory<'a> = Vec<File<'a>>;

pub fn solve_part1<'a>(input: &'a str) -> u32 {
    let lookup = parse(&input);
    let mut output = Vec::new();
    count(&(Vec::new(), "/"), &lookup, &mut output);
    // And now…
    output.iter().map(|p| p.1).filter(|s| s <= &100_000).sum()
}

pub fn solve_part2<'a>(input: &'a str) -> u32 {
    let lookup = parse(&input);
    let mut output = Vec::new();
    count(&(Vec::new(), "/"), &lookup, &mut output);
    // And now…
    let total_size = output.iter().filter(|(n, _)| n == "/").nth(0).unwrap().1;
    let space_left = 70_000_000 - total_size;
    let need_space = 30_000_000 - space_left;
    output.iter().map(|p| p.1).filter(|s| s >= &need_space).sorted().nth(0).unwrap()
}

fn count<'a>(pan: &'a PathAndName, lookup: &'a Lookup<'a>, output: &mut Vec<(String, u32)>) -> u32 {
    let ls: &Directory = lookup.get(pan).expect("Lookup failed");
    let mut sum = 0;
    for f in ls {
        match f {
            File::Dir(pan2) => sum = sum + count(pan2, lookup, output),
            File::PlainFile(name, size) => sum = sum + size,
        }
    }
    // Clone to string in order to avoid lifetime headaches
    output.push((pan.1.to_string(), sum));
    sum
}

fn parse<'a>(input: &'a str) -> Lookup<'a> {
    lazy_static::lazy_static! {
        static ref root: Regex = Regex::new(r"\$ cd /").unwrap();
        static ref cd: Regex = Regex::new(r"\$ cd ([a-z]+)").unwrap();
        // need to check this before `cd <dir>`…
        static ref cd_back: Regex = Regex::new(r"\$ cd \.\.").unwrap();
        static ref ls: Regex = Regex::new(r"\$ ls").unwrap();
        static ref dir: Regex = Regex::new(r"dir (.*)").unwrap();
        static ref pf: Regex = Regex::new(r"(\d+) (.*)").unwrap();
    }

    // directory stack
    let mut curr_dirs: Vec<&'a str> = Vec::new();

    let mut all_dirs: Lookup = HashMap::new();

    // for each `ls` invocation
    let mut ls_dir_name: &str = "";
    let mut ls_dirs: Directory = Vec::new();

    for l in input.lines() {
        if l.starts_with("$") {
            // Need to commit `ls` output
            if !ls_dirs.is_empty() {
                // all but last
                // NOTE Too bad that I have to use a clone here?
                let path: Vec<&'a str> = if curr_dirs.len() < 2 {
                    Vec::new()
                } else {
                    curr_dirs
                        .clone()
                        .into_iter()
                        .take(curr_dirs.len() - 1)
                        .collect()
                };
                let path_and_name = (path, ls_dir_name);
                all_dirs.insert(path_and_name, ls_dirs);
                ls_dirs = Vec::new();
            }
            if let Some(raw_assgns) = root.captures(l) {
                curr_dirs.push("/");
            } else if let Some(raw_assgns) = cd_back.captures(l) {
                curr_dirs.pop();
            } else if let Some(raw_assgns) = cd.captures(l) {
                let cd_to = raw_assgns.get(1).unwrap().as_str();
                curr_dirs.push(cd_to);
            } else if let Some(raw_assgns) = ls.captures(l) {
                ls_dir_name = curr_dirs.last().unwrap();
                // ls_dirs has already been set to the empty Vec
            } else {
                panic!("Found unexpected line while parsing commands: {l}");
            }
        } else {
            if let Some(raw_assgns) = dir.captures(l) {
                let dir_name = raw_assgns.get(1).unwrap().as_str();
                let path_and_name = File::Dir((curr_dirs.clone(), dir_name));
                ls_dirs.push(path_and_name);
            } else if let Some(raw_assgns) = pf.captures(l) {
                let size = parse_u32(raw_assgns.get(1).unwrap().as_str());
                let file_name = raw_assgns.get(2).unwrap().as_str();
                ls_dirs.push(File::PlainFile(file_name, size));
            } else {
                panic!("Expected to parse a line of `ls` output but found line: {l}");
            }
        }
    }
    // NOTE duplicate code
    // Need to commit `ls` output
    if !ls_dirs.is_empty() {
        // all but last
        // NOTE Too bad that I have to use a clone here?
        let path: Vec<&'a str> = if curr_dirs.len() < 2 {
            Vec::new()
        } else {
            curr_dirs
                .clone()
                .into_iter()
                .take(curr_dirs.len() - 1)
                .collect()
        };
        let path_and_name = (path, ls_dir_name);
        all_dirs.insert(path_and_name, ls_dirs);
        ls_dirs = Vec::new();
    }
    all_dirs
}

fn parse_u32(i: &str) -> u32 {
    i.parse::<u32>().expect("not an integer")
}
