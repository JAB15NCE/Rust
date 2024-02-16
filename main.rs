/*
Jon Bennett
02/04/2024
CISS 301 Lab 2

Discription:
This Rust program implements a logical deduction system. 
Users can input premises, and the program applies various logical rules to derive new premises. 
The deduction validity is checked, and a proof of the deduction is printed, including the rules applied at each step.
The program uses enums and structs to represent logical laws and premise information, respectively.
*/

#[warn(non_snake_case)]
use std::io;

// Enum to represent logical laws
#[derive(Debug)]
#[derive(Clone)]
enum LogicalLaw {
    ModusPonens,
    ModusTollens,
    LawOfSyllogism,
    IdempotentLaw,
    AssociativeLaw,
    CommutativeLaw,
    DistributiveLaw,
    IdentityLawForConjunction,
    IdentityLawForDisjunction,
    ComplementLaw,
    DeMorgansLaw,
    AbsorptionLaw,
    ImplicationElimination,
}

// Struct to represent information about a premise
#[derive(Debug, Clone)]
struct PremiseInfo {
    premise: String,
    law_used: Option<LogicalLaw>,
}

// Rule: Modus Ponens
fn modus_ponens(premise1: &str, premise2: &str, deduction: &str) -> Option<PremiseInfo> {
    if premise1.ends_with(deduction) && premise2 == &(premise1[..premise1.len() - deduction.len()]).to_string() {
        Some(PremiseInfo {
            premise: deduction.to_string(),
            law_used: Some(LogicalLaw::ModusPonens),
        })
    } else {
        None
    }
}

// Rule: Modus Tollens
fn modus_tollens(premise1: &str, premise2: &str, deduction: &str) -> Option<PremiseInfo> {
    if premise1 == &(deduction.to_string() + "->" + premise2) {
        Some(PremiseInfo {
            premise: format!("~{}", premise1),
            law_used: Some(LogicalLaw::ModusTollens),
        })
    } else {
        None
    }
}

// Rule: Law of Syllogism
fn law_of_syllogism(premise1: &str, premise2: &str, deduction: &str) -> Option<PremiseInfo> {
    if premise2.starts_with(premise1) && premise2.ends_with(deduction) {
        Some(PremiseInfo {
            premise: format!("{}->{}", premise1, deduction),
            law_used: Some(LogicalLaw::LawOfSyllogism),
        })
    } else {
        None
    }
}

// Rule: Idempotent Law
fn idempotent_law(premise1: &str, premise2: &str, _deduction: &str) -> Option<PremiseInfo> {
    if premise1 == premise2 {
        Some(PremiseInfo {
            premise: premise1.to_string(),
            law_used: Some(LogicalLaw::IdempotentLaw),
        })
    } else {
        None
    }
}

// Rule: Associative Law
fn associative_law(premise1: &str, premise2: &str, deduction: &str) -> Option<PremiseInfo> {
    if premise1.starts_with('(') && premise1.ends_with(')') && premise2.starts_with('(') && premise2.ends_with(')') {
        let inner_premise1 = &premise1[1..premise1.len() - 1];
        let inner_premise2 = &premise2[1..premise2.len() - 1];
        if format!("({})({})", inner_premise1, inner_premise2) == deduction {
            Some(PremiseInfo {
                premise: deduction.to_string(),
                law_used: Some(LogicalLaw::AssociativeLaw),
            })
        } else {
            None
        }
    } else {
        None
    }
}

// Rule: Commutative Law
fn commutative_law(premise1: &str, premise2: &str, deduction: &str) -> Option<PremiseInfo> {
    if premise1.starts_with('(') && premise1.ends_with('>') && premise2.starts_with('(') && premise2.ends_with('>') {
        let inner_premise1 = &premise1[1..premise1.len() - 1];
        let inner_premise2 = &premise2[1..premise2.len() - 1];
        if format!("({})({})->{}", inner_premise2, inner_premise1, deduction) == deduction {
            Some(PremiseInfo {
                premise: deduction.to_string(),
                law_used: Some(LogicalLaw::CommutativeLaw),
            })
        } else {
            None
        }
    } else {
        None
    }
}

// Rule: Distributive Law
fn distributive_law(premise1: &str, premise2: &str, deduction: &str) -> Option<PremiseInfo> {
    if premise1.starts_with('(') && premise1.ends_with('>') && premise2.starts_with('(') && premise2.ends_with('>') && deduction.starts_with('(') && deduction.ends_with('>') {
        let inner_premise1 = &premise1[1..premise1.len() - 1];
        let inner_premise2 = &premise2[1..premise2.len() - 1];
        let inner_deduction = &deduction[1..deduction.len() - 1];
        if format!("({})({})->{}", inner_premise1, inner_premise2, inner_deduction) == deduction {
            Some(PremiseInfo {
                premise: deduction.to_string(),
                law_used: Some(LogicalLaw::DistributiveLaw),
            })
        } else {
            None
        }
    } else {
        None
    }
}

// Rule: Identity Law
fn identity_law(premise1: &str, premise2: &str, deduction: &str) -> Option<PremiseInfo> {
    if premise1 == premise2 {
        // Identity Law for Conjunction (P ∧ P => P)
        if premise1.ends_with('∧') && deduction == &premise1[..premise1.len() - 1] {
            Some(PremiseInfo {
                premise: deduction.to_string(),
                law_used: Some(LogicalLaw::IdentityLawForConjunction),
            })
        }
        // Identity Law for Disjunction (P ∨ P => P)
        else if premise1.ends_with('∨') && deduction == &premise1[..premise1.len() - 1] {
            Some(PremiseInfo {
                premise: deduction.to_string(),
                law_used: Some(LogicalLaw::IdentityLawForDisjunction),
            })
        }
        // Add more cases for other logical operations if needed
        else {
            None
        }
    } else {
        None
    }
}

// Rule: Complement Law
fn complement_law(premise1: &str, premise2: &str, deduction: &str) -> Option<PremiseInfo> {
    if premise1 == format!("~{}", deduction) && premise2 == deduction {
        Some(PremiseInfo {
            premise: "None".to_string(),
            law_used: Some(LogicalLaw::ComplementLaw),
        })
    } else {
        None
    }
}

// Rule: DeMorgan's Law
fn demorgans_law(premise1: &str, premise2: &str, deduction: &str) -> Option<PremiseInfo> {
    if premise1 == format!("~({}∧{})", deduction, deduction) && premise2 == format!("~{}∨~{}", deduction, deduction) {
        Some(PremiseInfo {
            premise: "None".to_string(),
            law_used: Some(LogicalLaw::DeMorgansLaw),
        })
    } else {
        None
    }
}

// Rule: Absorption Law
fn absorption_law(premise1: &str, premise2: &str, deduction: &str) -> Option<PremiseInfo> {
    if premise1.starts_with('(') && premise1.ends_with('>') && premise2.starts_with('(') && premise2.ends_with('>') && deduction.starts_with('(') && deduction.ends_with('>') {
        let inner_premise1 = &premise1[1..premise1.len() - 1];
        let inner_premise2 = &premise2[1..premise2.len() - 1];
        let inner_deduction = &deduction[1..deduction.len() - 1];
        if inner_premise1 == format!("({}>{})->{}", inner_premise1, inner_premise2, inner_deduction) {
            Some(PremiseInfo {
                premise: format!("({}>{})->{}", inner_premise1, inner_premise2, inner_deduction),
                law_used: Some(LogicalLaw::AbsorptionLaw),
            })
        } else if inner_premise1 == format!("({}>{})->{}", inner_premise1, inner_premise2, inner_deduction) {
            Some(PremiseInfo {
                premise: format!("({}>{})->{}", inner_premise1, inner_premise2, inner_deduction),
                law_used: Some(LogicalLaw::AbsorptionLaw),
            })
        } else {
            None
        }
    } else {
        None
    }
}

// Rule: Implication Elimination
fn implication_elimination(premise1: &str, premise2: &str, deduction: &str) -> Option<PremiseInfo> {
    if premise1.starts_with('(') && premise1.ends_with("->") && premise2 == &format!("{}{}", &premise1[1..premise1.len() - 1], deduction) {
        let _inner_premise1 = &premise1[1..premise1.len() - 1];
        Some(PremiseInfo {
            premise: deduction.to_string(),
            law_used: Some(LogicalLaw::ImplicationElimination),
        })
    } else {
        None
    }
}

// Function to substitute characters
fn substitute_characters(input: &str) -> String {
    input
        .replace(">", "\u{2192}")
        .replace("~", "\u{00AC}")
        .replace("*", "\u{2227}")
        .replace("+", "\u{2228}")
        .replace("A", "\u{2200}")
        .replace("E", "\u{2203}")
        .replace("R", "\u{2234}")
}

// Function to print the proof (demonstration)
fn print_proof(premises: &Vec<PremiseInfo>) {
    println!("Proof:");

    for premise_info in premises.iter().rev() {
        let substituted_premise = substitute_characters(&premise_info.premise);
        let rule_name = premise_info.law_used.clone().map(|law| match law {
            LogicalLaw::ImplicationElimination => LogicalLaw::ModusPonens,
            _ => law.clone(),
        });
        match &rule_name {
            Some(law) => {
                println!("Premise: {} - Using: {:?}", substituted_premise, law);
            }
            None => {
                println!("Premise: {} - No law applied", substituted_premise);
            }
        }
    }
}

// Function to check the validity of a deduction
fn check_deduction(premises: &Vec<PremiseInfo>, deduction: &str) -> Option<String> {
    let new_premises = apply_rules(premises, deduction);

    if new_premises.is_empty() {
        Some("Insufficient information".to_string())
    } else {
        let last_premise = &new_premises[new_premises.len() - 1].premise;

        if last_premise == deduction {
            Some("Deduction is valid!".to_string())
        } else if last_premise == &format!("~{}", deduction) {
            Some("Deduction is invalid!".to_string())
        } else {
            None
        }
    }
}

// Function to check the validity of a premise
fn check_premise_validity(_premise: &str) -> Option<String> {
    // Add more checks if needed
    None
}

// Function to apply rules and expand the list of known premises
fn apply_rules(premises: &Vec<PremiseInfo>, deduction: &str) -> Vec<PremiseInfo> {
    let mut new_premises: Vec<PremiseInfo> = Vec::new();

    // Consider the last premise
    let last_premise = &premises[premises.len() - 1].premise;

    // Iterate over known premises and apply rules
    for known_premise_info in premises.iter().rev() {
        let known_premise = &known_premise_info.premise;

        // Apply rules
        if let Some(new_premise) = modus_ponens(known_premise, last_premise, deduction) {
            new_premises.push(new_premise);
        }
        if let Some(new_premise) = modus_tollens(known_premise, last_premise, deduction) {
            new_premises.push(new_premise);
        }
        if let Some(new_premise) = law_of_syllogism(known_premise, last_premise, deduction) {
            new_premises.push(new_premise);
        }
        if let Some(new_premise) = idempotent_law(known_premise, last_premise, deduction) {
            new_premises.push(new_premise);
        }
        if let Some(new_premise) = associative_law(known_premise, last_premise, deduction) {
            new_premises.push(new_premise);
        }
        if let Some(new_premise) = commutative_law(known_premise, last_premise, deduction) {
            new_premises.push(new_premise);
        }
        if let Some(new_premise) = distributive_law(known_premise, last_premise, deduction) {
            new_premises.push(new_premise);
        }
        if let Some(new_premise) = identity_law(known_premise, last_premise, deduction) {
            new_premises.push(new_premise);
        }
        if let Some(new_premise) = complement_law(known_premise, last_premise, deduction) {
            new_premises.push(new_premise);
        }
        if let Some(new_premise) = demorgans_law(known_premise, last_premise, deduction) {
            new_premises.push(new_premise);
        }
        if let Some(new_premise) = absorption_law(known_premise, last_premise, deduction) {
            new_premises.push(new_premise);
        }
        if let Some(new_premise) = implication_elimination(known_premise, last_premise, deduction) {
            new_premises.push(new_premise);
        }
        // Add more rules here as needed
    }

    new_premises
}

fn main() {
    let mut entered_premises: Vec<String> = Vec::new();
    let mut known_premises: Vec<PremiseInfo> = Vec::new();

    loop {
        println!("Enter a premise (or type 'R' to finish):");
        let input = {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            input.trim().to_string()
        };

        if input.to_uppercase() == "R" {
            println!("Enter the deduction value:");
            let deduction = {
                let mut deduction = String::new();
                io::stdin().read_line(&mut deduction).expect("Failed to read line");
                deduction.trim().to_string()
            };

            for premise in entered_premises.iter() {
                // Check validity of entered premise
                match check_premise_validity(premise) {
                    Some(message) => {
                        println!("{}", message);
                        return;
                    }
                    None => {}
                }
            }

            let validity_message = check_deduction(&known_premises, &deduction);
            match validity_message {
                Some(message) => {
                    println!("{}", message);
                    print_proof(&known_premises);
                    return;
                }
                None => {
                    println!("Invalid deduction: Deduction must be '~{}'", deduction);
                    continue;
                }
            }
        }

        entered_premises.push(input.clone());

        known_premises.push(PremiseInfo {
            premise: substitute_characters(&input),
            law_used: Some(LogicalLaw::ImplicationElimination), // Initial premise
        });

        let new_premises = apply_rules(&known_premises, "t");

        if new_premises.is_empty() {
            println!("Insufficient information");
            continue;
        }

        for new_premise in &new_premises {
            match &new_premise.law_used {
                Some(law) => {
                    println!("New Premise: {} - Using: {:?}", substitute_characters(&new_premise.premise), law);
                }
                None => {
                    println!("New Premise: {} - No law applied", substitute_characters(&new_premise.premise));
                }
            }
        }

        known_premises.extend(new_premises);
    

    for (i, premise) in entered_premises.iter().enumerate() {
        println!("Entered Premise {}: {}", i + 1, substitute_characters(premise));
    }

    println!("Enter your next Premise");
}
}