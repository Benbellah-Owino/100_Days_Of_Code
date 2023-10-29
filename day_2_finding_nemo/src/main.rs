use std::io;
fn main() {
    let mut sentence = String::new();

    io::stdin()
        .read_line(&mut sentence)
        .expect("Failed to read line");

    println!("{sentence}");
    find_nemo(sentence);
}

fn find_nemo(sentence: String) {
    let vec_sent = sentence.split_whitespace();

    for (position, word) in vec_sent.enumerate() {
        if word == "nemo" {
            println!("I found Nemo at {}", position + 1);
            return;
        }
    }

    println!("I can't find Nemo");
}
