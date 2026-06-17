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
8. Exit the Program
";

struct Account {
    account_number: u32,
    name: String,
    balance_minor_units: i64,
    currency_code: String,
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

fn format_money(balance_minor_units: i64) -> String {
    let major = balance_minor_units / 100;
    let minor = balance_minor_units % 100;

    format!("{}.{:02}", major, minor)
}

fn menu() -> u32 {
    println!("{MENU}");

    get_u32("Enter Choice: ")
}

impl Bank {
    fn create_account(&mut self) {
        let name = prompt("Enter Account Name: ");
        let currency_code = prompt("Enter the Currency Code: ");

        let account = Account {
            account_number: self.next_id,
            name,
            balance_minor_units: 0,
            currency_code,
        };

        self.accounts.insert(self.next_id, account);

        println!("Account Created!");
        println!("Account Number: {}", self.next_id);

        self.next_id += 1;
    }

    fn delete_account(&mut self) {
        // Placeholder
    }

    fn deposit(&mut self) {}

    fn withdraw(&mut self) {
        // Placeholder
    }

    fn see_balance(&self) {
        // Placeholder
    }

    fn find_account(&self) {
        // Placeholder
    }

    fn transfer_account(&mut self) {
        // Placeholder
    }

    fn list_accounts(&self) {
        if self.accounts.is_empty() {
            println!("No Accounts Found!");
            return;
        }

        for account in self.accounts.values() {
            println!("---------------");
            println!("Account Number: {}", account.account_number);
            println!("Account Name: {}", account.name);
            println!(
                "Balance: {} {}",
                account.currency_code,
                format_money(account.balance_minor_units)
            );
        }
    }

    fn batch_save(&self) {
        // Placeholder
    }

    fn reload_save(&mut self) {
        // Placeholder
    }

    fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            next_id: 1,
        }
    }
}

fn main() {
    println!("Hello There! FerroBank Active");

    let mut bank: Bank = Bank::new();

    loop {
        let choice: u32 = menu();
        match choice {
            1 => {
                bank.create_account();
            }

            2 => {
                bank.delete_account();
            }

            3 => {
                bank.deposit();
            }

            4 => {
                bank.withdraw();
            }

            5 => {
                bank.find_account();
            }

            6 => {
                bank.transfer_account();
            }

            7 => {
                bank.list_accounts();
            }

            8 => {
                return;
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
