// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.

use std::io;
use uuid::Uuid;

#[derive(Debug)]
struct Bill {
    id: Uuid,
    name: String,
    amount: i32,
}

fn get_input() -> io::Result<String> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

enum Menu {
    AddBill,
    ViewBills,
    RemoveBill,
    EditBill,
    Quit,
}

impl Menu {
    fn get_menu_choice() -> Menu {
        loop {
            println!("Select an option:");

            println!("1. Add bill");
            println!("2. View bills");
            println!("3. Remove bill");
            println!("4. Edit bill");
            println!("5. Quit");

            let input = get_input().unwrap();

            match input.as_str() {
                "1" => return Menu::AddBill,
                "2" => return Menu::ViewBills,
                "3" => return Menu::RemoveBill,
                "4" => return Menu::EditBill,
                "5" => return Menu::Quit,
                "q" => return Menu::Quit,
                _ => {
                    println!("Invalid input");
                }
            }
        }
    }
}

fn main() {
    let mut bills: Vec<Bill> = Vec::new();

    loop {
        let menu = Menu::get_menu_choice();
        match menu {
            Menu::AddBill => {
                println!("Enter name of bill: ");
                let name = get_input().unwrap();
                println!("Enter amount of bill: ");
                let amount = get_input().unwrap();
                let bill = Bill {
                    id: Uuid::new_v4(),
                    name,
                    amount: amount.parse().unwrap(),
                };
                bills.push(bill);
            }
            Menu::ViewBills => {
                println!("Bills:");
                for bill in bills.iter() {
                    println!("{:?}", bill.id);
                    println!("{:?}", bill.name);
                    println!("{:?}", bill.amount);
                }
            }
            Menu::EditBill => {
                println!("Enter id of bill to edit: ");
                let id = get_input().unwrap();
                let id = Uuid::parse_str(&id).unwrap();
                let index = bills.iter().position(|b| b.id == id).unwrap();
                println!("Enter name of bill: ");
                let name = get_input().unwrap();
                print!("Enter amount of bill: ");
                let amount = get_input().unwrap();
                let bill = Bill {
                    id,
                    name,
                    amount: amount.parse().unwrap(),
                };
                bills.remove(index);
                bills.push(bill);
            }
            Menu::RemoveBill => {
                println!("Enter id of bill to remove: ");
                let id = get_input().unwrap();
                let id = Uuid::parse_str(&id).unwrap();
                let index = bills.iter().position(|b| b.id == id).unwrap();
                bills.remove(index);
                println!("Bill removed");
            }
            Menu::Quit => {
                break;
            }
        }
    }
}
