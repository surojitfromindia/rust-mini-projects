use crate::book::{Book, BookState, LibBook, Library};
use std::error::Error;
use std::io;
use std::io::{Read, Stdin, Write};
use std::num::ParseIntError;

mod book;

struct Choices<'a> {
    id: u8,
    description: &'a str,
}

enum PromptAction {
    Exit,
    Help,
    Options,
}

enum PromptInput {
    Action(PromptAction),
    Choice(u8),
    UnknownAction,
}

fn main() {
    welcome_messages_and_choices();
    let mut home_library = Library::new();
    let choices = vec![
        Choices {
            id: 1,
            description: "Register new book",
        },
        Choices {
            id: 2,
            description: "Borrow a book",
        },
    ];
    print_options(&choices);
    print_help_options();
    loop {
        match prompt(choices.len()) {
            Ok(pm_in) => match pm_in {
                PromptInput::Action(T) => match T {
                    PromptAction::Exit => break,
                    PromptAction::Help => {
                        print_help_options();
                    }
                    PromptAction::Options => {
                        print_options(&choices);
                    }
                },
                PromptInput::Choice(ch_op) => match ch_op {
                    1 => {
                        enter_new_book(&mut home_library);
                    }
                    2 => {
                        borrow_book(&mut home_library);
                    }
                    _ => {
                        println!("Not an option");
                        continue;
                    }
                },
                PromptInput::UnknownAction => {
                    println!("Not an option");
                    continue;
                }
            },
            Err(_) => {
                println!("Not an option");
                continue;
            }
        }
    }
}

/// read from stdio io and save data
fn enter_new_book(library: &mut Library) -> &LibBook {
    let mut book_name = String::new();
    let mut author_id = 0;
    let mut std_inp = io::stdin();

    loop {
        print!("Enter book name: ");
        io::stdout().flush().unwrap();
        match std_inp.read_line(&mut book_name).is_err() {
            false => {
                break;
            }
            _ => continue,
        }
    }
    loop {
        print!("Enter author id: ");
        io::stdout().flush().unwrap();
        match read_as_number(&mut std_inp) {
            Ok(num) => {
                author_id = num;
                break;
            }
            Err(e_r) => {
                println!("Error {:?} ", e_r);
                continue;
            }
        }
    }
    library.register_new_book(book_name, author_id)
}

fn borrow_book(library: &mut Library) {
    loop {
        print!("Enter book id: ");
        let mut std_inp = io::stdin();
        io::stdout().flush().unwrap();
        match read_as_number(&mut std_inp) {
            Ok(num) => {
                match library.borrow_book(num) {
                    BookState::Found(book) => println!("Book borrowed {:?}", book.name),
                    BookState::NotFound => println!("Book not found!"),
                    BookState::Borrowed => println!("Book is already borrowed"),
                };
                break;
            }
            Err(e_r) => {
                println!("Error {:?} ", e_r);
                continue;
            }
        }
    }
}

fn welcome_messages_and_choices() {
    println!("Book keeper, Welcome");
    println!("--------------------");
    println!(
        "This CLI program store library data in memory \nso after closing it removes everything.\n"
    )
}

fn print_options(choices: &Vec<Choices>) {
    for choice in choices {
        println!("[{:?}] - {} ", choice.id, choice.description)
    }
}

fn print_help_options() {
    println!("[o] - options\t[h] - help\t[q] - quit");
}

fn prompt(max_input_case: usize) -> Result<PromptInput, &'static str> {
    let mut std_inp = io::stdin();
    print!("> ");
    io::stdout().flush().unwrap();
    return match read_action_input(&mut std_inp) {
        PromptInput::Choice(num) => {
            if num as usize > max_input_case {
                return Err("Not an option");
            }

            Ok(PromptInput::Choice(num))
        }
        P => Ok(P),
    };
}

fn read_as_number(std_input: &mut Stdin) -> Result<u8, &'static str> {
    let mut temp_r_string = String::with_capacity(5);
    // read from input buffer
    match std_input.read_line(&mut temp_r_string) {
        Ok(_) => match temp_r_string.trim().parse::<u8>() {
            Ok(num) => Ok(num),
            Err(_) => return Err("Not a number"),
        },
        Err(_) => {
            return Err("expecting an input");
        }
    }
}

fn read_action_input(std_input: &mut Stdin) -> PromptInput {
    let mut temp_r_string = String::with_capacity(5);
    match std_input.read_line(&mut temp_r_string) {
        Ok(_) => match temp_r_string.trim().parse::<u8>() {
            Ok(num) => PromptInput::Choice(num),
            _ => match temp_r_string.trim() {
                "q" => PromptInput::Action(PromptAction::Exit),
                "o" => PromptInput::Action(PromptAction::Options),
                "h" => PromptInput::Action(PromptAction::Help),
                _ => PromptInput::UnknownAction,
            },
        },
        Err(_) => PromptInput::UnknownAction,
    }
}
