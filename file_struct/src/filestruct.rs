#![allow(unused_variables)]

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub data: Vec<u8>,   
}

impl File {
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    pub fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    pub fn read(self: &File, save_to: &mut Vec<u8>) -> usize {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        read_length
    }
}

pub fn open(f: &mut File) -> bool {
    true
}

pub fn close(f: &mut File) -> bool {
    true
}
