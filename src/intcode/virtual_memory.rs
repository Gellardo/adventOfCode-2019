use std::collections::HashMap;
use std::ops::{Index, IndexMut};

pub struct VirtMem {
    pages: HashMap<usize, Vec<i32>>
}

impl Index<usize> for VirtMem {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        let page_index = index / 100;
        let page_offset = index % 100;
        return &self.pages[&page_index][page_offset];
    }
}

impl IndexMut<usize> for VirtMem {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let page_index = index / 100;
        let page_offset = index % 100;
        let page = self.pages.get_mut(&page_index).unwrap();
        return page.get_mut(page_offset).unwrap();
    }
}

impl From<Vec<i32>> for VirtMem {
    fn from(inp: Vec<i32>) -> Self {
        let mut hmap = HashMap::new();
        hmap.insert(0, inp);
        return VirtMem { pages: hmap };
    }
}
