//
// main.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 05 Mar 2017 21:52:28 +0100 (CET)
//

extern crate rand;

#[macro_use]
mod util;
mod memory;

const PHYS_RAM_SIZE: usize = MiB!(128);
const VIRT_RAM_SIZE: usize = GiB!(4);
const PAGE_SIZE: usize = KiB!(4);
const PHYS_PAGE_NUM: usize = PHYS_RAM_SIZE / PAGE_SIZE;
const VIRT_PAGE_NUM: usize = VIRT_RAM_SIZE / PAGE_SIZE;

fn main() {
  println!("Physical address space size: {}", util::to_nice_unit(PHYS_RAM_SIZE));
  println!("Virtual address space size: {}", util::to_nice_unit(VIRT_RAM_SIZE));
  println!("Page size: {}", util::to_nice_unit(PAGE_SIZE));
  println!("Physical page num: {}", PHYS_PAGE_NUM);
  println!("Virtual page num: {}", VIRT_PAGE_NUM);

  let mut memory = memory::new(PHYS_RAM_SIZE, VIRT_RAM_SIZE, PAGE_SIZE);

  for _ in 0..1_000_000 {
    let deviation = rand::random::<u32>() % 8 + rand::random::<u32>() % 8;
    let addr = 0xbabe0000u32 + PAGE_SIZE as u32 * deviation;
    memory.access_page(addr);
  }
}

//
// vi: ts=2 sw=2 expandtab
//

