fn main() {
    let word = String::from("gamer");
    let translated = pig_latin(&word);
    println!("\"{}\" translated into pig latin is \"{}\"", word, translated);
}
fn pig_latin(str: &String) -> String {
    let mut chars = str.chars();
    let first = str.chars().next().unwrap();
    let ending = match first {
        'a' => String::from("-hay"),
        'e' => String::from("-hay"),
        'i' => String::from("-hay"),
        'o' => String::from("-hay"),
        'u' => String::from("-hay"),
        _ => {
            chars.next();
            format!("-{}ay", first)
        }
    };
    format!("{}{}", chars.as_str(), ending)
}
