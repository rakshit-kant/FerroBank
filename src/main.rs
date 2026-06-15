use std::collections::HashMap;
use std::io;
use std::thread;
use std::time::Duration;

const MENU: &str = "
Welcome to Banking Account System!

1. Create Account
2. Delete Account
3. Deposit Money
4. Withdraw Money
5. Find Account
6. Transfer Account
7. List all Accounts
"; // List Admin Options

struct Account {
    name: String,
    balance_cent: i64,
}

struct Bank {
    accounts: HashMap<u32, Account>,
    next_id: u32,
}

// struct Branches {}

fn prompt(message: &str) -> String {
    println!("{}", message);

    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read Input");

    input.trim().to_string()
}

fn get_u32(message: &str) -> u32 {
    println!("{}", message);

    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to Read Input");

    input.trim().parse::<u32>().unwrap()
}

fn get_i64(message: &str) -> i64 {
    println!("{}", message);

    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to Read Input");

    input.trim().parse::<i64>().unwrap()
}

fn menu() -> u32 {
    println!("{MENU}");

    get_u32("Enter Choice: ")
}

impl Bank {
    fn create_account() {
        // Placeholder
    }

    fn delete_account() {
        // Placeholder
    }

    fn deposit() {
        // Placeholder
    }

    fn withdraw() {
        // Placeholder
    }

    fn find_account() {
        // Placeholder
    }

    fn transfer_account() {
        // Placeholder
    }

    fn list_account() {
        // Placeholder
    }

    fn batch_save() {
        // Placeholder
    }

    fn reload_save() {
        // Placeholder
    }

    fn review_admin_authenticity() {
        // Placeholder
    }

    fn admin_options() {
        // Placeholder
    }
}

fn main() {
    println!("Hello There! FerroBank Active");

    loop {
        let choice: u32 = menu();
        match choice {
            1 => {
                Bank::create_account();
                break;
            }
            2 => {
                Bank::delete_account();
                break;
            }
            3 => {
                Bank::deposit();
                break;
            }
            4 => {
                Bank::withdraw();
                break;
            }
            5 => {
                Bank::find_account();
                break;
            }
            6 => {
                Bank::transfer_account();
                break;
            }
            7 => {
                Bank::list_account();
                break;
            }
            _ => {
                println!("Bro! You had to choose from 1 to 8... Still you couldn't do it...");
                println!("Loading the Menu Again");

                for _ in 0..3 {
                    print!(".");
                    thread::sleep(Duration::from_secs(1));
                }
            }
        }
    }
}
