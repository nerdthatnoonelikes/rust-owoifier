use std::io::{stdin, stdout, Write};

fn input(input: &mut String) {
    stdout().flush()
        .expect("failed to flush");
    stdin().read_line(input)
        .expect("failed to read");
}

fn main() {
    let mut text = String::new();
    let mut finalText = String::new();
    print!("Please input text: ");
    input(&mut text);

    for x in text.chars() {
        if x == 'R' {
            finalText = finalText + "W"
        } else if x == 'r' {
            finalText = finalText + "w"
        } else if x == 'L' {
            finalText = finalText + "W";
        } else if x == 'l' {
            finalText = finalText + "w";
        } else {
            finalText.push(x);
        }
    }
    println!("{}", finalText)
}
