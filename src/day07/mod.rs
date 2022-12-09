use crate::utils::ParseError;
use std::str::FromStr;

#[derive(Debug)]
pub struct File {
    size: usize,
}

impl FromStr for File {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');

        let size = usize::from_str(split.next().ok_or(ParseError::new("Not a file size"))?)?;
        Ok(File { size })
    }
}

#[derive(Debug)]
pub struct Folder {
    name: String,
    id: usize,
    folders: Vec<usize>,
    files: Vec<File>,
    parent: Option<usize>,
}

impl Folder {
    fn root() -> Folder {
        let name = "";
        Folder::new(name, None, 0)
    }

    fn new(name: &str, parent: Option<usize>, id: usize) -> Folder {
        let name = name.to_owned();
        let folders = vec![];
        let files = vec![];
        Folder {
            name,
            id,
            folders,
            files,
            parent,
        }
    }
}

#[aoc_generator(day07)]
pub fn input_generator(input: &str) -> Result<Vec<Folder>, ParseError> {
    let mut lines = input.lines().rev().collect::<Vec<_>>();
    let mut entries: Vec<Folder> = vec![Folder::root()];
    let root = 0;
    let mut current = 0;

    while let Some(line) = lines.pop() {
        if line == "$ cd /" {
            current = root;
            continue;
        }

        if line == "$ cd .." {
            current = entries
                .get(current)
                .ok_or(ParseError::new("Invalid entry"))?
                .parent
                .ok_or(ParseError::new("Cannot move past /"))?;
            continue;
        }

        if line.starts_with("$ cd") {
            let name = line.chars().skip(5).collect::<String>();
            let entry = entries
                .get(current)
                .ok_or(ParseError::new("Folder not found"))?;
            // TODO proper error handling
            current = *entry
                .folders
                .iter()
                .find(|fid| entries.get(**fid).unwrap().name == name)
                .unwrap();
            continue;
        }

        if line == "$ ls" {
            let mut new_folders = vec![];
            let mut offset = 0;
            while let Some(entry) = lines.pop() {
                if entry.starts_with('$') {
                    lines.push(entry);
                    break;
                }
                let len = entries.len();
                let current_folder = entries
                    .get_mut(current)
                    .ok_or(ParseError::new("Folder not found"))?;
                if entry.starts_with("dir") {
                    let new_index = len + offset;
                    let name = entry.chars().skip(4).collect::<String>();
                    let new_folder = Folder::new(name.as_str(), Some(current), new_index);
                    new_folders.push(new_folder);
                    current_folder.folders.push(new_index);
                    offset += 1;
                    continue;
                }

                let file = File::from_str(entry)?;
                current_folder.files.push(file);
            }

            entries.append(&mut new_folders);
        }
    }

    Ok(entries)
}

fn size(drive: &Vec<Folder>, folder_id: usize) -> usize {
    let file_size: usize = drive[folder_id].files.iter().map(|file| file.size).sum();
    file_size
        + drive
            .iter()
            // TODO: fix unwrap
            .filter(|f| f.parent.is_some() && f.parent.unwrap() == folder_id)
            .fold(0_usize, |acc, folder| acc + size(drive, folder.id))
}

#[aoc(day07, part1)]
pub fn solve_part1(input: &Vec<Folder>) -> Result<usize, ParseError> {
    let sizes: usize = input
        .iter()
        .enumerate()
        .map(|(i, _)| size(input, i))
        .filter(|size| *size <= 100_000)
        .sum();

    Ok(sizes)
}

#[aoc(day07, part2)]
pub fn solve_part2(input: &Vec<Folder>) -> Result<usize, ParseError> {
    let total = 70_000_000_usize;
    let needed = 30_000_000_usize;
    let used = size(input, 0);
    let free_space = total - used;

    let mut sizes = input
        .iter()
        .enumerate()
        .map(|(i, _)| size(input, i))
        .filter(|size| free_space + size >= needed)
        .collect::<Vec<_>>();
    sizes.sort();

    Ok(*sizes.first().unwrap())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::ParseError;

    fn sample() -> &'static str {
        ""
    }

    fn input() -> Result<Vec<Folder>, ParseError> {
        input_generator(sample())
    }

    #[test]
    fn part1_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(0, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(0, solve_part2(&data)?))
    }
}
