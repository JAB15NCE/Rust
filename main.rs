/*
Jon Bennett
01/28/2024
CISS 301
Truth table and logical propostion.
*/

use std::io;

fn main() {
/*This is the start of the loop.This provides instructions 
and to have the user input only 0's and 1'. This Section 
will also remove any spaces that user inputs.*/
    loop{
    let mut input: String = String::new();
    let  _first_run = true; 
    println!("Press enter again, and type 'yes' to exit other wise follow prompt below.");
    println!("\n");
    println!("Enter groups of up to four 0's or 1's separated by a comma: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.replace(" ", "");//Gets rid of spaces. 
    println!("{}", input);

/* This section Validate that the input contains only 0s and 1s 
and is no more then 4 characters between the commas. 
IF the user didn't foollow instructions this will kick back an error and have them redo their input.*/ 
    if input
        .split(',')
        .map(|group| group.trim())
        .all(|group| group.chars().all(|c| c == '0' || c == '1' || c ==',') && group.len() <= 4){
        
        let mut _output = String::new();

/*This section of code decipher the input of the user and corrects 
it to a logic propostion utilizing only ABCD.*/
        for (idx, j) in input.trim().split(',').enumerate() {
            if idx > 0 {
                print!(" + ");
            }
            for i in 0..j.len() {
                print!("{}", if j.as_bytes()[i] == b'0' { "/" } else { "" });
                print!("{}", (i as u8 + b'A') as char);
                }
            }
        println!("\n");


/*This section of code will ask if they user wants to provide another
 input and will either loop back or exit the program.*/
        loop{
        println!("Do you want to exit? (yes/no)");
        let mut exit_input = String::new();
        io::stdin().read_line(&mut exit_input).expect("Failed to read line");

        let exit_command = exit_input.trim().to_lowercase();
        match exit_command.as_str(){
            "yes" => {
            println!("See you later!! Have a wonderful day :)");
            return; // Exit the loop
        }
        "no" =>break,
        _ => println!("Sorry thats not a 'Yes' or 'No' lets try this again now."),
        
    }
    }
/*Print an error message if the input contains characters other 
than 0 and 1 and if user didn't follow the provided instructions above.*/
        } else { 
        println!("Error: Input must contain only 0s and 1s and no more then 4 between each comma. You didn't do this, so Try again.");
        println!("\n")
        }
    }
}
