// TODO: Add a Global Functon which Formats the Money back to it's balance_minor_units instead of storing
// the stuff in minor units. (1st Priority) Done
//
// TODO: Create the Withdraw Funtion. (2nd Priority) Done
//
// TODO: Add a Thread Sleep Function Gloabally and Use it in the Places where the TUI is screwed and
// the Words are just coming and going. (3rd Priority)
//
// TODO: Polish the Whole TUI. (4th Priority)

use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use std::collections::HashMap;
use std::io;
use std::str::FromStr;
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
    //    currency_code: String,
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
    // fn format_money(balance_minor_units: i64, currency_code: &str) -> String {

    let major = balance_minor_units / 100;
    let minor = balance_minor_units % 100;

    format!("{}.{:02}", major, minor) //     format!("{} {}.{:02}", currency_code, major, minor)
}

fn get_money_minor_units(message: &str) -> i64 {
    let input = prompt("Enter the Amount of Money (format: 2.50): ");

    let decimal = Decimal::from_str(&input).expect("Invalid Money Amount");

    let minor_units = (decimal * Decimal::new(100, 0))
        .round()
        .to_i64()
        .expect("Amount Too Large");

    if minor_units < 1 {
        println!("Amount too Low!");
    }

    minor_units
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
            //    currency_code,
        };

        self.accounts.insert(self.next_id, account);

        println!("Account Created!");
        println!("Account Number: {}", self.next_id);

        self.next_id += 1;
    }

    fn delete_account(&mut self) {
        // Placeholder
    }

    fn deposit(&mut self) {
        let account_number: u32 = get_u32("Enter the Account Number: ");
        match self.accounts.get_mut(&account_number) {
            Some(account) => {
                println!("Account Number: {}", account.account_number);
                println!("Account Holder Name: {}", account.name);
                println!(
                    "Account Balance: {}",
                    format_money(account.balance_minor_units,) //                     format_money(account.balance_minor_units, &account.currency_code,)
                );

                let deposit_money = get_money_minor_units("Enter the Amount of Money (eg: 2.50): ");

                account.balance_minor_units += deposit_money;

                println!(
                    "Added {} to your Account. Your Total Balance: {}",
                    deposit_money,
                    format_money(account.balance_minor_units)
                );
            }
            None => {
                println!("Account not found!");
            }
        }
    }

    fn withdraw(&mut self) {
        let account_number: u32 = get_u32("Enter the Account Number: ");
        match self.accounts.get_mut(&account_number) {
            Some(account) => {
                println!("Account Number: {}", account.account_number);
                println!("Account Holder Name: {}", account.name);
                println!(
                    "Account Balance: {}",
                    format_money(account.balance_minor_units,) //                     format_money(account.balance_minor_units, &account.currency_code,)
                );

                let withdraw_money =
                    get_money_minor_units("Enter the Amount of Money (eg: 2.50): ");

                account.balance_minor_units -= withdraw_money;

                println!(
                    "{} Withdrawn from your Account. Your Total Balance: {}",
                    withdraw_money,
                    format_money(account.balance_minor_units)
                );
            }
            None => {
                println!("Account not found!");
            }
        }
    }

    fn see_balance(&self) {
        // Placeholder
    }

    //   fn get_account_mut(&mut self) {
    //        let account_number: u32 = get_u32("Enter the Account Number: ");
    //        match self.accounts.get_mut(&account_number) {
    //            Some(account) => {}
    //            None => {
    //                println!("Account Not Found!")
    //            }
    //        }
    //    }
    //    Commented Just Because I wanted to see if I really need Abstraction instead of
    //    Repetition

    fn find_account(&self) {
        let account_number: u32 = get_u32("Enter the Account Number: ");
        match self.accounts.get(&account_number) {
            Some(account) => {
                println!("Account Number: {}", account.account_number);
                println!("Account Holder Name: {}", account.name);
                println!(
                    "Account Balance: {}",
                    format_money(account.balance_minor_units,) //                     format_money(account.balance_minor_units, &account.currency_code,)
                );
            }
            None => {
                println!("Account not found!");
            }
        }
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
                "Balance: {}",
                format_money(account.balance_minor_units,) //                 format_money(account.balance_minor_units, &account.currency_code,)
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
