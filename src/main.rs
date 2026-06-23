use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
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

#[derive(Serialize, Deserialize)]
struct Account {
    account_number: u32,
    name: String,
    balance_minor_units: i64,
}

#[derive(Serialize, Deserialize)]
struct Bank {
    accounts: HashMap<u32, Account>,
    next_id: u32,
}

fn prompt(message: &str) -> String {
    println!("{}", message);

    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read Input");
    thread::sleep(Duration::from_millis(500));

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

fn format_money(balance_minor_units: i64) -> String {
    let major = balance_minor_units / 100;
    let minor = balance_minor_units % 100;

    format!("{}.{:02}", major, minor)
}

fn get_money_minor_units(_message: &str) -> i64 {
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

        let account = Account {
            account_number: self.next_id,
            name,
            balance_minor_units: 0,
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
                    format_money(account.balance_minor_units,)
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
                    format_money(account.balance_minor_units,)
                );

                let withdraw_money =
                    get_money_minor_units("Enter the Amount of Money (eg: 2.50): ");

                if withdraw_money > account.balance_minor_units {
                    println!("Insufficient Funds!");
                    return;
                }

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

    fn find_account(&self) {
        let account_number: u32 = get_u32("Enter the Account Number: ");
        match self.accounts.get(&account_number) {
            Some(account) => {
                println!("Account Number: {}", account.account_number);
                println!("Account Holder Name: {}", account.name);
                println!(
                    "Account Balance: {}",
                    format_money(account.balance_minor_units,)
                );
            }
            None => {
                println!("Account not found!");
            }
        }
    }

    fn transfer_account(&mut self) {
        let sender_account_number = get_u32("Enter the Sender's Account Number: ");
        let receiver_account_number = get_u32("Enter the Receiver's Account Number: ");
        let transfer_amount = get_money_minor_units("Enter the Transfer Amount(eg: 2.50): ");

        if sender_account_number == receiver_account_number {
            println!("Cannot Transfer to the Same Account!");
            return;
        }

        if !self.accounts.contains_key(&sender_account_number) {
            println!("Sender Account not Found!");
            return;
        }

        if !self.accounts.contains_key(&receiver_account_number) {
            println!("Receiver Account not Found!");
            return;
        }

        {
            let sender = self.accounts.get_mut(&sender_account_number).unwrap();

            if sender.balance_minor_units < transfer_amount {
                println!("Insufficient Funds!");
                return;
            }

            sender.balance_minor_units -= transfer_amount;
        }

        {
            let receiver = self.accounts.get_mut(&receiver_account_number).unwrap();

            receiver.balance_minor_units += transfer_amount;
        }

        println!("Successfully transferred {}", format_money(transfer_amount));
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
            println!("Balance: {}", format_money(account.balance_minor_units,));
        }
    }

    fn batch_save(&self) {
        let json = serde_json::to_string_pretty(self).expect("Failed to Serialize Data");

        fs::write("ferrobank.json", json).expect("Failed to Save Data");

        println!("Data Saved");
    }

    fn reload_save() -> Self {
        match fs::read_to_string("ferrobank.json") {
            Ok(json) => serde_json::from_str(&json).expect("Corrupted Save File")
        

            Err(_) => {
                println!("No Save File Found!");

                Self::new()
            }
        }
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

    let mut bank: Bank = Bank::reload_save();

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
                bank.batch_save();
                return;
            }

            _ => {
                println!("Bro! You had to choose from 1 to 8... Still you couldn't do it...");
            }
        }
    }
}
