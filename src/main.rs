use rand::Rng;
use std::io;
use std::io::Write;
use colored::*;

fn get_correct_conjugation(pronoun: &str, verb: &str) -> String {
    // The correct conjugation function allows to have all the possible matches 
    match (pronoun, verb) {
        // MANGIARE
        ("Io", "mangiare") => "Io mangio".to_string(),
        ("Tu", "mangiare") => "Tu mangi".to_string(),
        ("Lei", "mangiare") => "Lei mangia".to_string(),
        ("Lui", "mangiare") => "Lui mangia".to_string(),
        ("Noi", "mangiare") => "Noi mangiamo".to_string(),
        ("Voi", "mangiare") => "Voi mangiate".to_string(),
        ("Loro", "mangiare") => "Loro mangiano".to_string(),
        // DORMIRE
        ("Io", "dormire") => "Io dormo".to_string(),
        ("Tu", "dormire") => "Tu dormi".to_string(),
        ("Lei", "dormire") => "Lei dorme".to_string(),
        ("Lui", "dormire") => "Lui dorme".to_string(),
        ("Noi", "dormire") => "Noi dormiamo".to_string(),
        ("Voi", "dormire") => "Voi dormite".to_string(),
        ("Loro", "dormire") => "Loro dormono".to_string(),
        // PARTIRE
        ("Io", "partire") => "Io parto".to_string(),
        ("Tu", "partire") => "Tu parti".to_string(),
        ("Lei", "partire") => "Lei parte".to_string(),
        ("Lui", "partire") => "Lui parte".to_string(),
        ("Noi", "partire") => "Noi partiamo".to_string(),
        ("Voi", "partire") => "Voi partite".to_string(),
        ("Loro", "partire") => "Loro partono".to_string(),

        // VIVERE
        ("Io", "vivere") => "Io vivo".to_string(),
        ("Tu", "vivere") => "Tu vivi".to_string(),
        ("Lei", "vivere") => "Lei vive".to_string(),
        ("Lui", "vivere") => "Lui vive".to_string(),
        ("Noi", "vivere") => "Noi viviamo".to_string(),
        ("Voi", "vivere") => "Voi vivete".to_string(),
        ("Loro", "vivere") => "Loro vivono".to_string(),

        _ => "unknown".to_string(),
    }
}


fn main() {

    // Game loop
    println!("Welcome to BigBert's Italian Verb Conjugation Game!");
    println!("Type 'quit' to exit the game at any time.");
    println!();

    loop {

    // Generate random pronoun + verbs
    let pronouns = ["Io", "Tu", "Lei", "Lui", "Noi", "Voi", "Loro"];
    let verbs = ["mangiare", "dormire", "partire", "vivere", "prendere", "comprare", "vendere", "lavorare", "vedere", "studiare", "finire", "preparare", "preferire", "scrivere", "sentire", "parlare"];
    
    let number1 = rand::rng().random_range(0..=6);
    let number2 = rand::rng().random_range(0..=3);

    let selected_pronoun = pronouns[number1];
    let selected_verb = verbs[number2];

    println!("{} + {}", selected_pronoun, selected_verb);


    fn verb_conjugation(selected_verb: &str) -> String {
        let pronouns = ["Io", "Tu", "Lei", "Lui", "Noi", "Voi", "Loro"];
        let mut conjugations = String::new();
    
        for pronoun in pronouns {
            let conjugation = get_correct_conjugation(pronoun, selected_verb);
            conjugations.push_str(&format!("{}\n", conjugation));
        }

        // Print the conjugations
        println!("The congugation for {} is :", selected_verb);
        print!("{}", conjugations);
        conjugations
    }

    // Get user input

    println!("Please input your guess:");
    io::stdout().flush().expect("Failed to flush stdout");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

    // Trim whitespace from the guess
    let guess = guess.trim();

    // Check for quit
    if guess.to_lowercase() == "quit" {
        println!();
        println!("Grazie ! Ciao!");
        break;
    }

    // Get the correct conjugation
    let correct_answer = get_correct_conjugation(selected_pronoun, selected_verb);

    // Compare the answers
    if guess.to_lowercase() == correct_answer.to_lowercase() {
        println!("{}", "Correct! :-)".green());
    } else {
        println!("{} {}", "Incorrect.".red(), format!("The correct answer is: {}", correct_answer));
        verb_conjugation(selected_verb);   
    }
    println!();
}

} 
