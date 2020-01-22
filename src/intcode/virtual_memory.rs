use std::collections::HashMap;
use std::ops::{Index, IndexMut};

pub struct VirtMem {
    pages: HashMap<usize, [i64; 100]>,
}

fn get_indices(index: usize) -> (usize, usize) {
    return (index / 100, index % 100);
}

impl Index<usize> for VirtMem {
    type Output = i64;

    fn index(&self, index: usize) -> &Self::Output {
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
        let (page_index, page_offset) = get_indices(index);
        if !self.pages.contains_key(&page_index) {
            debug!("page fault, creating page {}", page_index);
            self.pages.insert(page_index, [0; 100]);
        }
        let page = self.pages.get_mut(&page_index);

        return page.unwrap().get_mut(page_offset).unwrap();
    }
}

impl From<Vec<i64>> for VirtMem {
    fn from(inp: Vec<i64>) -> Self {
        let mut mem = VirtMem {
            pages: HashMap::new(),
        };
        for (idx, value) in inp.iter().enumerate() {
            mem[idx] = *value;
        }
        mem
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocate_mem_on_write() {
        let mut mem = VirtMem {
            pages: HashMap::new(),
        };
        // read uninitialized address returns zero, no allocation
        assert_eq!(mem[0], 0);
        assert_eq!(mem[1000], 0);
        assert_eq!(mem.pages.len(), 0);
        println!("Finished read only test");

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

    #[test]
    fn init_memory() {
        let starting = vec![0, 1, 2, 3];
        let mem = VirtMem::from(starting.clone());
        for (idx, _) in starting.iter().enumerate() {
            println!("checking position {}", idx);
            assert_eq!(mem[idx], starting[idx] as i64)
        }
    }

    #[test]
    fn large_memory() {
        let starting = [12; 1000].to_vec();
        let mem = VirtMem::from(starting.clone());
        for (idx, _) in starting.iter().enumerate() {
            assert_eq!(
                mem[idx], starting[idx] as i64,
                "mem[{}]={} != {}(expected)",
                idx, mem[idx], starting[idx]
            )
        }
    }
}
