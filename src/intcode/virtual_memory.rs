use std::collections::HashMap;
use std::ops::{Index, IndexMut};

pub struct VirtMem {
    pages: HashMap<usize, [i32; 100]>
}

fn get_indices(index: usize) -> (usize, usize){
    return (index / 100, index % 100);
}

impl Index<usize> for VirtMem {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        println!("start Index");
        let (page_index, page_offset) = get_indices(index);
        let page = &self.pages.get(&page_index);
        if page.is_none() {
            return &0;
        }
        return &page.unwrap().get(page_offset).unwrap_or(&0);
    }
}

impl IndexMut<usize> for VirtMem {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {

        println!("start IndexMut");
        let (page_index, page_offset) = get_indices(index);
        if !self.pages.contains_key(&page_index) {
            println!("page fault");
            self.pages.insert(page_index, [0;100]);
        }
        let page = self.pages.get_mut(&page_index);

        return page.unwrap().get_mut(page_offset).unwrap();
    }
}

impl From<Vec<i32>> for VirtMem {
    fn from(inp: Vec<i32>) -> Self {
        let mut hmap = HashMap::new();
        let page = [0; 100];
        hmap.insert(0, page);
        return VirtMem { pages: hmap };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocate_mem_on_write() {
        let mut mem = VirtMem { pages: HashMap::new() };
        // read uninitialized address returns zero, no allocation
        assert_eq!(mem[0], 0);
        assert_eq!(mem[1000], 0);
        assert_eq!(mem.pages.len(), 0);
        println!("Finished read only");

        // allocate pages on write
        mem[0] = 1;
        mem[1000] = 3;
        println!("Finished write");
        assert_eq!(mem[0], 1);
        assert_eq!(mem[1000], 3);
        println!("Read of written");
        assert!(mem.pages.len() > 0);

        // write/read value at an offset
        mem[10] = 2;
        assert_eq!(mem[10], 2);
    }
}
