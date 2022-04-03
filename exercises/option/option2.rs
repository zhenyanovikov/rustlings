// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints

fn main() {
    let optional_word = Some(String::from("rustlings"));
    if let Some(word) = optional_word {
        println!("The word is: {}", word);
    } else {
        println!("The optional word doesn't contain anything");
    }

    let mut optional_integers_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_integers_vec.push(Some(x));
    }

    while let Some(integer) = optional_integers_vec.pop() {
        if let Some(integer) = integer {
            println!("current value: {}", integer);
        }
    }
}
