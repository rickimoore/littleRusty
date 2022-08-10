use std::io;
use rand::Rng;
use std::{thread, time, process};

#[derive(Debug, PartialEq)]
enum Player {
    COMPUTER,
    USER,
    BOTH
}

static ROCK: &'static str = "ROCK";
static PAPER: &'static str = "PAPER";
static SCISSORS: &'static str = "SCISSORS";

fn wait(milliseconds: u64) {
    thread::sleep(time::Duration::from_millis(milliseconds));
}

fn show_instructions() {
    println!("Lets play rock, paper, scissors!\n\
              How to play:\n\
              I will signal `rock, paper, scissors, shoot!`\n\
              On `shoot` input your response to and the winner will be revealed!");
}
fn get_user_input() -> String {
    let mut response = String::new();

    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line");

    return response.trim().to_uppercase();
}
fn generate_computer_choice() -> String {
    let num = rand::thread_rng().gen_range(0..=2);
    match num {
        1 => PAPER.to_string(),
        2 => SCISSORS.to_string(),
        _ => ROCK.to_string(),

    }
}
fn validate_choice(value: &String) -> bool {
    return value.eq(ROCK) || value.eq(PAPER) || value.eq(SCISSORS);
}
fn init_game() -> Player {
    println!("{}", ROCK);
    wait(500);
    println!("{}", PAPER);
    wait(500);
    println!("{}", SCISSORS);
    wait(1000);
    println!("Shoot!!! (Input your response...)");

    let response = get_user_input();
    let computer_choice = generate_computer_choice();
    let is_valid = validate_choice(&response);

    if !is_valid {
        println!("Invalid Input");
        process::exit(1);
    }

    println!("Your response: {}", response);
    println!("Computer response: {:?}", computer_choice);

    let winner = calculate_winner(&response, &computer_choice);

    if winner == Player::BOTH {
        println!("It is a TIE. Try again!")
    } else {
        println!("Winner: {:?}", winner);
    }

    return winner;
}
fn calculate_winner(u_value: &String, c_value: &String) -> Player {
    if u_value.eq(c_value) {
        return Player::BOTH;
    }

    if c_value.eq(ROCK) && u_value.eq(PAPER)
        || c_value.eq(PAPER) && u_value.eq(SCISSORS)
        || c_value.eq(SCISSORS) && u_value.eq(ROCK) {
        return Player::USER;
    }

    return Player::COMPUTER;
}

fn main() {
    show_instructions();
    let mut user_wins = 0;
    let mut computer_wins = 0;
    loop {
        let result = init_game();

        match result {
            Player::USER => user_wins += 1,
            Player::COMPUTER => computer_wins +=1,
            _ => ()
        }

        if user_wins + computer_wins >= 3 {
            println!("Computer Wins: {}", computer_wins);
            println!("User Wins: {}", user_wins);
            break;
        }
    }
}
