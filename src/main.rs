extern crate rusqlite;

use rusqlite::Connection;
use std::path::Path;
use std::str;

struct Scanner {
    id: i32,
    name: String,
    status: String,
    open: bool,
    access_by: Vec<u8>,
}



fn str_vec_into_byte_vec_vec(input: &Vec<&str>) -> Vec<Vec<u8>> {
    // rusqlite doesn't support writing vecs of strings to the db. This function
    // turns the vector of strings into vectors of vectors of bytes, which can be
    // written to the db.
    
    let mut return_vec: Vec<Vec<u8>> = Vec::new();

    for slice in input {
        let mut temp_vec: Vec<u8> = Vec::new();
        for byte in slice.as_bytes() {
            temp_vec.push(*byte);
        };
        return_vec.push(temp_vec);
    };
    return return_vec;
}

fn byte_vec_vec_into_str_vec(input: &Vec<Vec<u8>>) -> Vec<String> {
    
    let mut return_vec: Vec<String> = Vec::new();

    for byte_vector in input {
        let slice = str::from_utf8(&byte_vector).unwrap();
        return_vec.push(slice.to_string());
    }

    return return_vec;
}

fn collapse_byte_vec(input: &Vec<Vec<u8>>) -> Vec<u8> {
    // Turn an array of byte arrays into a single null delimited array.
    // this just feels dirty and a bad idea

    let mut return_vec: Vec<u8> = Vec::new();

    for vec in input {
        for byte in vec {
            return_vec.push(*byte);
        };
        // i feel like using a null unicode as a delimiter is a bad idea...
        return_vec.push(0);
    }

    return return_vec;
}

fn reinflate_byte_vec(input: &Vec<u8>) -> Vec<Vec<u8>> {
    let mut return_vec: Vec<Vec<u8>> = Vec::new();
    
    let mut i = 0; // Index of the start of the current string
    for mut byte_index in 0..input.len() {
        if input[byte_index] == 0 {
            return_vec.push(array_to_vec(&input[i..byte_index]));
            i = byte_index + 1;
            // skip over the zero
            byte_index += 1;
        };
        //println!("start of current_string : {:?}\n
        //          start of next string    : {:?}",
        //          &i,&byte_index)
    };

    return return_vec;
}
            
fn array_to_vec(arr: &[u8]) -> Vec<u8> {
    return arr.iter().cloned().collect::<Vec<u8>>();
}

fn prepare_string_vec(input: &Vec<&str>) -> Vec<u8> {
    collapse_byte_vec(&str_vec_into_byte_vec_vec(&input))
}

fn read_output_blob(input: &Vec<u8>) -> Vec<String> {
    byte_vec_vec_into_str_vec(&reinflate_byte_vec(&input))
}

fn serialize_test() {
    // "test"
    
    let vec_s = vec!("ha fd","hudf  ");
    let x = prepare_string_vec(&vec_s);
    println!("{:?}", &x);
    let y = read_output_blob(&x);
    println!("{:?}", &y);
    assert!(y == vec_s);
}

fn sql_stuff() {
    let db_path = Path::new("home/oliver/prog/consult/scanning/scan_serv/test.db");
    let conn = Connection::open(&db_path).unwrap();
    
    conn.execute("CREATE TABLE scanners (
                  id                INTEGER PRIMARY KEY AUTOINCREMENT,
                  name              TEXT NOT NULL,
                  status            TEXT NOT NULL,
                  open              INTEGER NOT NULL,
                  access_by         BLOB NOT NULL
                  )", &[]).unwrap(); // Cmon man... why cant rust have optional args
    let test_scan = Scanner {
        id: 0,
        name: "Front door".to_string(),
        status: "Working normally".to_string(),
        open: false,
        access_by: prepare_string_vec(&vec!("hafd","hudf")),
    };

    conn.execute("INSERT INTO scanners (name, status, open, access_by)
                  VALUES ($1, $2, $3, $4)",
                 &[&test_scan.name, &test_scan.status, 
                 &test_scan.open, &test_scan.access_by]).unwrap();
}

fn main() {
    serialize_test();
    sql_stuff();
}

