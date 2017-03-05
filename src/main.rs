//
// main.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 05 Mar 2017 21:52:28 +0100 (CET)
//

#[macro_use]
mod util;

const PHYS_RAM_SIZE: u64 = MiB!(128);
const VIRT_RAM_SIZE: u64 = GiB!(4);

fn main() {
  println!("Physical address space size: {}", util::to_nice_unit(PHYS_RAM_SIZE));
  println!("Virtual address space size:  {}", util::to_nice_unit(VIRT_RAM_SIZE));
}

//
// vi: ts=2 sw=2 expandtab
//

