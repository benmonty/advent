use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use std::fmt;
use std::collections::HashMap;

pub mod constants {
    pub const INPUT_PATH: &str = "day9/input.txt";
}

type FileId = usize;
type BlockId = usize;
type BlockIndex = usize;

struct DiskMap {
    entries: Vec<DiskMapEntry>,
}


#[derive(Debug, PartialEq)]
enum DiskMapEntry {
    File(usize),
    Free(usize),
}

impl DiskMap {
    fn from(input: &String) -> Self {
        let mut entries = Vec::new();
        let mut is_file = true;
        for c in input.chars().take(input.len() - 1) {
            let num = usize::try_from(c.to_digit(10).unwrap()).unwrap();
            if is_file {
                entries.push(DiskMapEntry::File(num));
            } else {
                entries.push(DiskMapEntry::Free(num));
            }
            is_file = !is_file;
        }
        Self {
            entries
        }
    }
}

#[derive(Debug, PartialEq)]
struct Disk {
    blocks: Vec<Rc<Block>>,
    files: Vec<File>,
    file_ptrs: HashMap<FileId, BlockIndex>,
}

impl Disk {

    fn new() -> Self {
        Disk {
            blocks: Vec::new(),
            files: Vec::new(),
            file_ptrs: HashMap::new(),
        }
    }

    pub fn append_file(&mut self, file: File) {
        self.file_ptrs.entry(file.id).or_insert(self.blocks.len());
        for block in file.blocks.iter() {
            self.blocks.push(Rc::clone(&block))
        }
        self.files.push(file);
    }

    pub fn append_free(&mut self, num_blocks: usize) {
        for _i in 0..num_blocks {
            self.blocks.push(Rc::new(Block::Free))
        }
    }

    pub fn from(disk_map: &DiskMap) -> Self {
        let mut disk = Self::new();

        let mut file_id_counter = 0;
        for entry in disk_map.entries.iter() {
            match entry {
                DiskMapEntry::File(num_blocks) => {
                    let mut f = File {
                        id: file_id_counter,
                        blocks: Vec::new(),
                    };

                    for i in 0..*num_blocks {
                        let block = Block::File(f.id, i);
                        f.blocks.push(Rc::new(block))
                    }
                    disk.append_file(f);
                    file_id_counter += 1;
                }
                DiskMapEntry::Free(num_blocks) => {
                    disk.append_free(*num_blocks);
                }
            }
        }
        disk
    }

    pub fn find_first_free(&self, starting_block: usize) -> Option<usize> {
        if starting_block >= self.blocks.len() {
            panic!("attempting to access out of bounds block");
        }
        for i in starting_block..self.blocks.len() {
            match *self.blocks[i] {
                Block::Free => return Some(i),
                _ => (),
            }
        }
        return None
    }

    pub fn find_first_contiguous_free(&self, num_blocks: usize, ending_at: BlockIndex) -> Option<usize> {
        let mut free_count = 0;
        let mut range_start = 0;
        for i in 0..ending_at {
            match *self.blocks[i] {
                Block::Free => {
                    free_count += 1;
                    if free_count == num_blocks {
                        return Some(range_start);
                    }
                },
                _ => {
                    range_start = i + 1;
                    free_count = 0;
                },
            }
        }
        return None

    }

    pub fn find_last_data(&self, end_block: usize) -> Option<usize> {
        if end_block > self.blocks.len() || end_block == 0 {
            panic!("attempting to access out of bounds block");
        }

        for i in (0..end_block).rev() {
            match *self.blocks[i] {
                Block::File(_, _) => return Some(i),
                _ => (),
            }
        }
        None
    }

    pub fn checksum(&self) -> usize {
        let mut result = 0;
        for (idx, block_rc) in self.blocks.iter().enumerate() {
            let block = block_rc.clone();
            match &*block {
                Block::File(file_id, _block_id) => {
                    result += idx * file_id;
                },
                Block::Free => (),
            }
        }
        result
    }

    pub fn swap_blocks(&mut self, idx_a: usize, idx_b: usize) {
        self.blocks.swap(idx_a, idx_b);
    }

    pub fn mv_file(&mut self, file_id: FileId, new_idx: BlockIndex) {
        let current_file_idx = self.file_ptrs.get(&file_id).unwrap().clone();
        let num_blocks = self.files[file_id].blocks.len();
        for offset in 0..num_blocks {
            self.swap_blocks(new_idx + offset, current_file_idx + offset);
        }
        self.file_ptrs.entry(file_id).and_modify(|ptr| *ptr = new_idx);
    }

    pub fn compress(&mut self) {
        match (self.find_first_free(0), self.find_last_data(self.blocks.len())) {
            (Some(mut free_ptr), Some(mut data_ptr)) => {
                while free_ptr < data_ptr {
                    self.swap_blocks(free_ptr, data_ptr);
                    match (self.find_first_free(free_ptr + 1), self.find_last_data(data_ptr)) {
                        (Some(next_free), Some(next_data)) => {
                            free_ptr = next_free;
                            data_ptr = next_data;
                        },
                        _ => return,
                    }
                }
            },
            _ => (),
        }
    }

    pub fn compress_no_frag(&mut self) {
        for i in (0..self.files.len()).rev() {
            let free_at = self.find_first_contiguous_free(
                self.files[i].blocks.len(),
                *self.file_ptrs.get(&self.files[i].id).unwrap(),
            );
            match free_at {
                Some(free_at) => {
                    let file_id = self.files[i].id;
                    self.mv_file(file_id, free_at);
                },
                None => (),
            }

        }
    }
}

impl fmt::Display for Disk {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for b_rc in self.blocks.iter() {
            let block = b_rc.clone();
            match &*block {
                Block::File(file_id, _block_id) => write!(f, "{}", file_id).unwrap(),
                Block::Free => write!(f, ".").unwrap(),
            }
        }
        write!(f, "")
    }
}


#[derive(Debug)]
struct File {
    id: usize,
    blocks: Vec<Rc<Block>>
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.blocks.len() == other.blocks.len()
    }
}

#[derive(Debug)]
enum Block {
    File(FileId, BlockId),
    Free,
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Block::File(file_id_a, file_idx_a), Block::File(file_id_b, file_idx_b)) => {
                file_id_a == file_id_b && file_idx_a == file_idx_b
            },
            (Block::Free, Block::Free) => true,
            _ => false,
        }
    }
}

/*

there are files
files have
  * ids
  * blocks
  * blocks of free space following the file

files' blocks can be moved around

file blocks at the end of the disk must be moved 1 by 1
to the free space closest to the beginning of the disk

need to be able to do a checksum of the disk
  * for a given block (or block index) need to get the file id
  * need to get block index
  * need to know where the next free space is (and how much?)
*/

pub fn solution1(path: &PathBuf) -> usize {
    let input =  fs::read_to_string(path).unwrap();
    _solution1(&input)
}


pub fn _solution1(input: &String) -> usize {
    let disk_map = DiskMap::from(&input);
    let mut disk = Disk::from(&disk_map);
    disk.compress();
    disk.checksum()
}

pub fn solution2(path: &PathBuf) -> usize {
    let input =  fs::read_to_string(path).unwrap();
    _solution2(&input)
}

pub fn _solution2(input: &String) -> usize {
    let disk_map = DiskMap::from(&input);
    let mut disk = Disk::from(&disk_map);
    disk.compress_no_frag();
    disk.checksum()
}

#[cfg(test)]
mod tests  {
    use super::*;
    use crate::common;

    #[test]
    fn example_day9_disk_map() {
        let disk_map = DiskMap::from(&String::from("12345\n"));
        assert_eq!(
            disk_map.entries,
            vec![
                DiskMapEntry::File(1),
                DiskMapEntry::Free(2),
                DiskMapEntry::File(3),
                DiskMapEntry::Free(4),
                DiskMapEntry::File(5),
            ]
        );
    }

    #[test]
    fn example_day9_disk() {
        let disk_map = DiskMap::from(&String::from("12345\n"));
        let disk = Disk::from(&disk_map);
        assert_eq!(
            disk,
            Disk {
                files: vec![
                    File {
                        id: 0,
                        blocks: vec![
                            Rc::new(Block::File(0, 0)),
                        ],
                    },
                    File {
                        id: 1,
                        blocks: vec![
                            Rc::new(Block::File(1, 0)),
                            Rc::new(Block::File(1, 1)),
                            Rc::new(Block::File(1, 2)),
                        ],
                    },
                    File {
                        id: 2,
                        blocks: vec![
                            Rc::new(Block::File(2, 0)),
                            Rc::new(Block::File(2, 1)),
                            Rc::new(Block::File(2, 2)),
                            Rc::new(Block::File(2, 3)),
                            Rc::new(Block::File(2, 4)),
                        ],
                    }
                ],
                blocks: vec![
                    Rc::new(Block::File(0, 0)),
                    Rc::new(Block::Free),
                    Rc::new(Block::Free),
                    Rc::new(Block::File(1, 0)),
                    Rc::new(Block::File(1, 1)),
                    Rc::new(Block::File(1, 2)),
                    Rc::new(Block::Free),
                    Rc::new(Block::Free),
                    Rc::new(Block::Free),
                    Rc::new(Block::Free),
                    Rc::new(Block::File(2, 0)),
                    Rc::new(Block::File(2, 1)),
                    Rc::new(Block::File(2, 2)),
                    Rc::new(Block::File(2, 3)),
                    Rc::new(Block::File(2, 4)),
                ],
                file_ptrs: HashMap::from([
                    (0, 0),
                    (1, 3),
                    (2, 10),
                ]),
            }
        );
    }

    #[test]
    fn example_day9_1_compress() {
        let path = common::get_test_data_path("day9/case1.txt").unwrap();
        let input = fs::read_to_string(&path).unwrap();
        let disk_map = DiskMap::from(&input);
        let mut disk = Disk::from(&disk_map);
        disk.compress();
        assert_eq!(solution1(&path), 1928);
    }

    #[test]
    fn example_day9_1() {
        let path = common::get_test_data_path("day9/case1.txt").unwrap();
        assert_eq!(solution1(&path), 1928);
    }

    #[test]
    fn example_day9_2() {
        let path = common::get_test_data_path("day9/case1.txt").unwrap();
        assert_eq!(solution2(&path), 99999);
    }
}
