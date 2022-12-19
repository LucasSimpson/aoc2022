use std::fmt;
use regex::Regex;

#[derive(Debug, Clone)]
struct Path(Vec<String>);

impl Path {
    fn new() -> Path {
        Path(vec![])
    }

    fn root() -> Path {
        Path(vec![])
    }

    fn is_empty(&self) -> bool {
        return self.0.len() == 0
    }

    fn push(&self, segment: String) -> Path {
        let mut res = self.clone();
        res.0.push(segment);
        res
    }

    fn pop(&self) -> Path {
        let mut res = self.clone();
        res.0.pop();
        res
    }

    fn drop(&self, n: usize) -> Path {
        Path(self.0.iter().skip(n).cloned().collect())
    }
}

// Implement `Display` for `MinMax`.
impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res = self.0.iter().fold("".to_string(), |mut accum, segment| {
            accum.push_str("/");
            accum.push_str(segment);
            accum
        });

        match res.as_ref() {
            "" => write!(f, "/"),
            path => write!(f, "{}", path)
        }
    }
}

#[derive(Debug, Clone)]
struct Folder {
    name: String,

    files: Vec<(String, usize)>,
    folders: Vec<Folder>,
}

impl Folder {
    fn root() -> Folder {
        Folder{
            name: "R".to_string(),
            files: vec![],
            folders: vec![]
        }    
    }

    fn size(&self) -> usize {
        self.files.iter().map(|(_, size)| size ).sum::<usize>() + self.folders.iter().map(|f| f.size() ).sum::<usize>()
    }
    
    fn add_file(&self, location: &Path, name: String, size: usize) -> Folder{
        let mut res = self.clone();

        if location.is_empty() {
            res.files.push((name, size));
        } else {
            res.folders = res.folders.into_iter().map(|folder| {
                if folder.name == *location.0.first().unwrap() {
                    folder.add_file(&location.drop(1), name.clone(), size)
                } else {
                    folder
                }
            }).collect()
        }

        res
    }

    fn add_folder(&self, location: &Path, name: String) -> Folder {
        let mut res = self.clone();

        if location.is_empty() {
            res.folders.push(Folder{
                name,
                files: vec![],
                folders: vec![]
            });
        } else {
            res.folders = res.folders.into_iter().map(|folder| {
                if folder.name == *location.0.first().unwrap() {
                    folder.add_folder(&location.drop(1), name.clone())
                } else {
                    folder
                }
            }).collect()
        }

        res
    }

    fn all_folders(&self) -> Vec<Folder> {
        let mut res = vec![self.clone()];
        for folder in &self.folders {
            res.append(&mut folder.all_folders())
        }
        res
    }

    fn display(&self, location: Path) {
        println!("{}, size={}", location, self.size());

        for folder in &self.folders {
            let new_loc = location.push(folder.name.clone());
            println!("{}, size={}", new_loc, folder.size());
            folder.display(new_loc)
        }
        for (name, size) in &self.files {
            println!("{}/{}, size={}", location, name, size)
        }
    }
}

fn build_fs(lines: Vec<String>) -> Folder {
    let cd_re = Regex::new(r"\$ cd (.+)").unwrap();
    let ls_re = Regex::new(r"\$ ls").unwrap();
    let disp_dir = Regex::new(r"dir (\w+)").unwrap();
    let disp_file = Regex::new(r"(\d+) (.*)").unwrap();

    let mut path = Path::new();
    let mut root = Folder::root();

    for line in lines.into_iter() {
        if let Some(captures) = cd_re.captures(&line) {
            let folder = &captures[1];
            // println!("cd to directory={}", folder);
            path = match folder {
                "/" => Path::root(),
                ".." => path.pop(),
                location => path.push(location.to_string())
            };
            // println!("\t\tnew path={}", path);

        } else if let Some(_) = ls_re.captures(&line) {
            // println!("ls command");

        } else if let Some(captures) = disp_dir.captures(&line) {
            let name = &captures[1];
            // println!("displaying folder name={}", name);

            root = root.add_folder(&path, name.to_string());

        } else if let Some(captures) = disp_file.captures(&line) {
            let size = &captures[1].parse::<usize>().unwrap();
            let name = &captures[2];
            // println!("displaying file name={}, size={}", name, size);

            root = root.add_file(&path, name.to_string(), *size);

        } else {
            println!("line didn't match anything!!!: {:?}", line)
        }
    }

    root
}

pub fn solve_p1(lines: Vec<String>) -> u32 {
    let root = build_fs(lines);
    root.all_folders()
        .into_iter()
        .map(|f| f.size() )
        .filter(|s| *s <= 100000 )
        .sum::<usize>() as u32
}

pub fn solve_p2(lines: Vec<String>) -> u32 {
    let root = build_fs(lines);

    let disk_space: usize = 70000000;
    let update = 30000000;

    let total_size = root.size();
    let unused = disk_space - total_size;
    let needed = update - unused;

    let mut large_enough = root.all_folders()
        .into_iter()
        .map(|f| f.size() )
        .filter(|s| *s >= needed )
        .collect::<Vec<usize>>();

    large_enough.sort();

    *large_enough.first().unwrap() as u32
}

#[cfg(test)]
mod tests {
    use crate::{solve_p1, solve_p2};

    #[test]
    fn test_solve_p1() {
        let lines: Vec<String> = vec![
            "$ cd /".to_string(),
            "$ ls".to_string(),
            "dir a".to_string(),
            "14848514 b.txt".to_string(),
            "8504156 c.dat".to_string(),
            "dir d".to_string(),
            "$ cd a".to_string(),
            "$ ls".to_string(),
            "dir e".to_string(),
            "29116 f".to_string(),
            "2557 g".to_string(),
            "62596 h.lst".to_string(),
            "$ cd e".to_string(),
            "$ ls".to_string(),
            "584 i".to_string(),
            "$ cd ..".to_string(),
            "$ cd ..".to_string(),
            "$ cd d".to_string(),
            "$ ls".to_string(),
            "4060174 j".to_string(),
            "8033020 d.log".to_string(),
            "5626152 d.ext".to_string(),
            "7214296 k".to_string(),
        ];
        assert_eq!(solve_p1(lines), 95437)
    }

    #[test]
    fn test_solve_p2() {
        let lines: Vec<String> = vec![
            "$ cd /".to_string(),
            "$ ls".to_string(),
            "dir a".to_string(),
            "14848514 b.txt".to_string(),
            "8504156 c.dat".to_string(),
            "dir d".to_string(),
            "$ cd a".to_string(),
            "$ ls".to_string(),
            "dir e".to_string(),
            "29116 f".to_string(),
            "2557 g".to_string(),
            "62596 h.lst".to_string(),
            "$ cd e".to_string(),
            "$ ls".to_string(),
            "584 i".to_string(),
            "$ cd ..".to_string(),
            "$ cd ..".to_string(),
            "$ cd d".to_string(),
            "$ ls".to_string(),
            "4060174 j".to_string(),
            "8033020 d.log".to_string(),
            "5626152 d.ext".to_string(),
            "7214296 k".to_string(),
        ];
        assert_eq!(solve_p2(lines), 24933642)
    }
}

