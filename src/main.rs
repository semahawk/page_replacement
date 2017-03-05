//
// main.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 05 Mar 2017 21:52:28 +0100 (CET)
//

const PHYS_RAM_SIZE: u64 = 128 * 1024 * 1024;
const VIRT_RAM_SIZE: u64 = 4 * 1024 * 1024 * 1024;

pub fn to_nice_unit(value: u64) -> String {
  let mut units = vec!["EiB", "PiB", "TiB", "GiB", "MiB", "KiB", "B"];
  let mut final_unit = units.pop().unwrap();

  let mut value = value as f64;
  'a: while value >= 1024f64 {
    if let Some(unit) = units.pop() {
      final_unit = unit;
      value /= 1024f64;
    } else {
      break 'a;
    }
  }

  return format!("{} {}", value, final_unit);
}

fn main() {
  println!("Physical address space size: {}", to_nice_unit(PHYS_RAM_SIZE));
  println!("Virtual address space size:  {}", to_nice_unit(VIRT_RAM_SIZE));
}

//
// vi: ts=2 sw=2 expandtab
//

