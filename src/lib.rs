#![allow(dead_code)]

use regex::Regex;
use std::ffi::{c_char, CStr};

use rusqlite::{named_params, Connection, Result};

#[no_mangle]
pub extern "C" fn rust_add(left: i32, right: i32, result: *mut i32) {
    unsafe { *result = add(left, right) }
}

#[no_mangle]
pub extern "C" fn rust_register_datasource(ds: *const *mut i32, length: i32) {
    unsafe {
        for x in 0..length {
            let ptr = ds.offset(x.clone() as isize);
            **ptr *= 10;
        }
    }
}
#[no_mangle]
pub extern "C" fn rust_set_bit(val: *mut i32, bit: i32) -> bool {
    let mut new_val = unsafe { *val };
    let ret = set_bit(&mut new_val, bit);
    unsafe {
        *val = new_val;
    }
    ret
}

#[no_mangle]
pub extern "C" fn rust_clear_bit(val: *mut i32, bit: i32) -> bool {
    let mut new_val = unsafe { *val };
    let ret = clear_bit(&mut new_val, bit);
    unsafe {
        *val = new_val;
    }
    ret
}

#[no_mangle]
pub extern "C" fn rust_read_db(file: *const c_char) -> bool {
    let c_file_path = unsafe { CStr::from_ptr(file) };
    let file_path = c_file_path.to_str().unwrap();

    match read_db(file_path.to_string()) {
        Err(_) => return false,
        _ => (),
    }
    true
}

fn set_bit(val: &mut i32, bit: i32) -> bool {
    if bit < 32 {
        *val |= 1 << bit;
        return true;
    }
    false
}

fn clear_bit(val: &mut i32, bit: i32) -> bool {
    if bit < 32 {
        *val &= !(1 << bit);
        return true;
    }
    false
}

fn add(left: i32, right: i32) -> i32 {
    left + right
}

#[derive(Debug)]
struct Record {
    id: i32,
    name: String,
    age: i32,
    breed: String,
}

fn read_db(file: String) -> Result<()> {
    let conn = Connection::open(file)?;
    let mut stmt = conn.prepare("SELECT id,name,age,breed from hello")?;

    let record_iter = stmt.query_map([], |row| {
        Ok(Record {
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
            breed: row.get(3)?,
        })
    })?;

    for record in record_iter {
        println!("{:?}", record.unwrap());
    }
    Ok(())
}

fn write_db(file: String) -> Result<()> {
    let conn = Connection::open(file)?;

    let mut stmt = conn.prepare("update hello set breed=:breed where name=:name")?;

    let name = "Nidhi";
    let breed = "unknown";

    let mut rows = stmt.query(named_params! {":name": name, ":breed": breed})?;

    while let Some(_row) = rows.next()? {}

    Ok(())
}

fn extract_func(func: String) {
    let re = Regex::new(r"(\w+)\((.+)\)").unwrap();
    println!("{:?}", re);

    // let parts: Vec<&str> = re.split(&func).collect();
    // println!("{:?}", parts);

    for cap in re.captures_iter(&func) {
        println!("{:?}", cap);
        println!("{}: {}, {}", &cap[0], &cap[1], &cap[2]);
    }
}

fn parse_params(paramstr: &str, parts: &mut Vec<String>) -> usize {
    let mut p_depth = 0;

    let mut sub = Vec::new();

    for c in paramstr.chars() {
        match c {
            '(' => {
                p_depth += 1;
                sub.push(c);
            }
            ')' => {
                p_depth -= 1;
                sub.push(c);
            }
            ',' => {
                if p_depth == 0 {
                    parts.push(sub.iter().collect::<String>());
                    sub.clear();
                } else {
                    sub.push(c);
                }
            }
            _ => sub.push(c),
        }
    }

    // Got to the end and didn't encounter a comma, so check if any
    // characters are in sub and add to parts list
    if sub.len() > 0 {
        parts.push(sub.iter().collect::<String>());
    }

    parts.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn func_parts_test() {
        let mut parts: Vec<String> = vec![];
        parse_params("DS(x,2),2", &mut parts);
        println!("{:?}", parts);
        assert!(true);
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn bit_set_test() {
        let mut val = 0;
        let res = set_bit(&mut val, 7);
        assert!(res);
        assert_eq!(val, 128);
    }

    #[test]
    fn bit_clear_test() {
        let mut val = 128;
        let res = clear_bit(&mut val, 7);
        assert!(res);
        assert_eq!(val, 0);
    }

    #[test]
    fn db_read_test() {
        assert_eq!(read_db("test.db".to_string()), Ok(()));
    }

    #[test]
    fn db_write_test() {
        assert_eq!(write_db("test.db".to_string()), Ok(()));
    }

    #[test]
    fn extract_func_test() {
        extract_func("ADD(1,2)".to_string());
        assert!(true)
    }
}
