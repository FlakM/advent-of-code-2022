use std::{collections::HashMap, path::PathBuf, str::FromStr};

pub fn main() {
    let input = include_str!("../../day7a/input.txt");
    let response = perfom(input);

    // benchmark code should be quiet
    if std::option_env!("BENCH").is_none() {
        println!("response: {} ", response)
    }
}

#[derive(Debug)]
enum FileEntry {
    Dir(PathBuf),
    File(usize),
}

fn calculate_size(tree: &HashMap<PathBuf, Vec<FileEntry>>, path: &PathBuf) -> usize {
    tree[path]
        .iter()
        .map(|e| match e {
            FileEntry::Dir(path) => calculate_size(tree, path),
            FileEntry::File(size) => *size,
        })
        .sum()
}

fn perfom(input: &str) -> usize {
    // dir -> files map
    let mut state: HashMap<PathBuf, Vec<FileEntry>> = HashMap::new();
    let mut current_dir = PathBuf::new();

    for line in input.lines() {
        if line.starts_with("$ cd ..") {
            current_dir.pop();
            continue;
        }
        if line.starts_with("$ cd") {
            current_dir.push(line[5..].to_string());
            state.entry(current_dir.clone()).or_default();
            continue;
        }

        if line.starts_with("$ ls") {
            continue;
        }

        if line.starts_with("dir") {
            let name = &line[4..];
            let dir = current_dir.join(name);
            state.entry(dir.clone()).or_default();
            state
                .get_mut(&current_dir)
                .unwrap()
                .push(FileEntry::Dir(dir));
        } else {
            let file = line
                .split_once(' ')
                .map(|(size, _)| size.parse::<usize>().unwrap())
                .unwrap();
            state
                .get_mut(&current_dir)
                .unwrap()
                .push(FileEntry::File(file));
        }
    }

    let free_space = 70000000 - calculate_size(&state, &PathBuf::from_str("/").unwrap());
    state
        .iter()
        .map(|e| calculate_size(&state, e.0))
        .filter(|size| *size >= 30000000 - free_space)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let test_input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        assert_eq!(perfom(test_input), 24933642);
    }
}
