use std::str;

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

pub fn prepare_string_vec(input: &Vec<&str>) -> Vec<u8> {
    collapse_byte_vec(&str_vec_into_byte_vec_vec(&input))
}

pub fn read_output_blob(input: &Vec<u8>) -> Vec<String> {
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

fn main() {
    serialize_test();
}

