//
// util.rs
// Copyright (C) 2017 Szymon Urbaś <szymon.urbas@aol.com>
// Distributed under terms of the BSD (2-clause) license.
//
// Created on: 05 Mar 2017 22:14:58 +0100 (CET)
//

macro_rules! KiB { ($value:expr) => ($value * 1024); }
macro_rules! MiB { ($value:expr) => (KiB!($value) * 1024); }
macro_rules! GiB { ($value:expr) => (MiB!($value) * 1024); }

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

/*
 * vi: ts=2 sw=2 expandtab
 */

