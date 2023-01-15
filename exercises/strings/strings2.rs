// strings2.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings2` or use the `hint` watch subcommand for a hint.

fn main() {
    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

// fix the code below
// fn is_a_color_word(word: String) -> bool {
//     let color_words = vec!["red", "green", "blue"];
//     for color in color_words {
//         if color == word {
//             return true;
//         }
//     }
//     false
// }
fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
