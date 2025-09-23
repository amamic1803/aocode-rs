use crate::{Error, Solution};

day!(Day07, 2022, 7, "No Space Left On Device");

impl Solution for Day07 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let base_dir = parse_input(input);
        let mut total_size = 0;
        part1_recursion(&base_dir, &mut total_size);
        Ok(total_size.to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let base_dir = parse_input(input);
        let needed_space = 30_000_000 - (70_000_000 - base_dir.size());
        let mut min_del = 70_000_001;
        part2_recursion(&base_dir, needed_space, &mut min_del);
        Ok(min_del.to_string())
    }
}

fn part1_recursion(folder: &Folder, total_size: &mut u64) {
    let folder_size = folder.size();
    if folder_size <= 100_000 {
        *total_size += folder_size;
    }
    for subfolder in &folder.folders {
        part1_recursion(subfolder, total_size);
    }
}

fn part2_recursion(folder: &Folder, needed_space: u64, min_del: &mut u64) {
    let folder_size = folder.size();
    if (folder_size >= needed_space) && (folder_size < *min_del) {
        *min_del = folder_size;
    }
    for subfolder in &folder.folders {
        part2_recursion(subfolder, needed_space, min_del);
    }
}

fn parse_input(input: &str) -> Folder {
    let mut base_dir = Folder::new("/".to_string());

    let mut current_location: Vec<String> = vec!["/".to_string()];

    for line in input.trim().lines() {
        let line_contents: Vec<&str> = line.trim().split(' ').collect();

        if line_contents[0] == String::from('$') {
            if line_contents[1] == "cd" {
                if line_contents[2] == ".." {
                    if current_location.len() > 1 {
                        current_location.pop();
                    }
                } else if line_contents[2] == "/" {
                    current_location = vec!["/".to_string()];
                } else {
                    current_location.push(line_contents[2].to_string());
                }
            }
        } else if line_contents[0] == "dir" {
            base_dir.new_entity(&current_location, line_contents[1].to_string(), None);
        } else {
            base_dir.new_entity(
                &current_location,
                line_contents[1].to_string(),
                Some(line_contents[0].parse::<u64>().unwrap()),
            );
        }
    }

    base_dir
}

struct File {
    name: String,
    size: u64,
}

struct Folder {
    name: String,
    files: Vec<File>,
    folders: Vec<Folder>,
}

impl File {
    fn new(name: String, size: u64) -> File {
        File { name, size }
    }
}

impl Folder {
    fn new(name: String) -> Folder {
        Folder {
            name,
            files: Vec::new(),
            folders: Vec::new(),
        }
    }

    fn new_entity(&mut self, location: &[String], name: String, size: Option<u64>) {
        if location.len() > 1 {
            match self.folders.iter_mut().find(|f| f.name == location[1]) {
                Some(folder) => folder.new_entity(&location[1..], name, size),
                None => {
                    let mut new_folder = Folder::new(location[0].to_string());
                    new_folder.new_entity(&location[1..], name, size);
                    self.folders.push(new_folder);
                }
            }
        } else if !self.files.iter().any(|f| f.name == name) {
            self.new_entity_here(name, size);
        }
    }

    fn new_entity_here(&mut self, name: String, size: Option<u64>) {
        match size {
            Some(size) => self.files.push(File::new(name, size)),
            None => self.folders.push(Folder::new(name)),
        }
    }

    fn size(&self) -> u64 {
        self.files.iter().map(|f| f.size).sum::<u64>()
            + self.folders.iter().map(|f| f.size()).sum::<u64>()
    }
}
