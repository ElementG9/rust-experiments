use std::collections::HashMap;
fn main() {
    // Vectors are similar to arrays
    print!("\nVectors:\n");
    let mut v : Vec<i32> = Vec::new();
    v.push(1);
    v.push(86);
    v.push(23);
    v.push(56);
    print!("Elements in v:");
    for i in &mut v {
        print!(" {}", *i);
    }
    print!("\n");

    // Strings
    print!("\nStrings:\n");
    let str1 = "test"; // Primitive &str
    println!("{}", str1);
    let mut str2 = String::from("Hello"); // String class with methods
    // Appending
    str2.push_str(" world");
    str2.push('!');
    println!("{}", str2);
    // Formatting (template strings)
    let s1 = "tic";
    let s2 = "tac";
    let s3 = "toe";
    let str3 = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", str3);
    // String is a wrapper of Vec<u8>
    // This means no indexing into strings
    let str4 = String::from(" counts as two bytes");
    println!("\"{}\" is {} bytes long", str4, str4.len()); // Length
    let str4_modified = format!("д{}", str4);
    println!("\"{}\" is {} bytes long", str4_modified, str4_modified.len()); // Length
    let str4_clipped = &str4_modified[0..4];
    println!("The first 4 bytes of \"{}\" are \"{}\"", str4_modified, str4_clipped); // Length
    // Iterating
    let str5 = "नमस्ते";
    print!("The chars of {} are", str5);
    for c in str5.chars() {
        print!(" {}", c);
    }
    print!(" and the bytes are");
    for b in str5.bytes() {
        print!(" {}", b);
    }
    print!("\n");

    // HashMaps
    print!("\nHashMaps:\n");
    let mut scores = HashMap::new();
    // Inserting into the HashMap
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Green"), 25);
    let score = scores.get("Blue"); // Returns an Optional
    match score {
        Some(i) => println!("Blue: {}", i),
        _ => ()
    }
    println!("Unmodified: {:?}", scores);
    scores.insert(String::from("Yellow"), 30); // Overwrite the Yellow value
    println!("Modified: {:?}", scores);
    // Insert without possible overwrite
    scores.entry(String::from("Blue")).or_insert(45);
    scores.entry(String::from("Orange")).or_insert(20);
    println!("Modified without overwrite: {:?}", scores);
}
