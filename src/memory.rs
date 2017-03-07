//
// memory.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 07 Mar 2017 18:34:15 +0100 (CET)
//

use std::collections::HashMap;

struct Page {
  swapped_to_disk: bool,
}

pub struct Memory {
  page_size: usize,
  vm_page_directory: HashMap<u32, Page>,
  phys_mem_size_left: usize,
  page_fault_num: usize,
  page_access_num: usize,
}

pub fn new(phys_mem_size: usize, virt_mem_size: usize, page_size: usize) -> Memory {
  Memory {
    page_size: page_size,
    vm_page_directory: HashMap::with_capacity(virt_mem_size / page_size),
    phys_mem_size_left: phys_mem_size,
    page_fault_num: 0,
    page_access_num: 0,
  }
}

impl Memory {
  pub fn access_page(&mut self, page: u32) {
    self.page_access_num += 1;
    if self.vm_page_directory.contains_key(&page) {
      println!("accessed page 0x{:x}", page);
    } else {
      println!("page fault on page: 0x{:x}", page);
      self.page_fault_num += 1;
      if self.phys_mem_size_left >= self.page_size {
        // 'allocate' a page frame
        self.phys_mem_size_left -= self.page_size;
        self.vm_page_directory.insert(page, Page { swapped_to_disk: false });
        println!("-- page allocated");
      } else {
        // TODO obviously
        println!("no physical memory left! swapping a page to disk");
      }
    }
  }
}

impl Drop for Memory {
  fn drop(&mut self) {
    println!("summary: on {} page accesses there was {} page faults",
      self.page_access_num, self.page_fault_num);
  }
}

/*
 * vi: ts=2 sw=2 expandtab
 */

