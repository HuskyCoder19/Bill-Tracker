pub mod bill_handler {

    pub struct Bill {
        pub name: String,
        pub amount: f32,
        pub due: u32,
        pub paid: bool,
    }

    pub fn build_bill(n: &str, a: f32, d: u32, p: bool) -> Bill {
        Bill {
            name: (*n).to_string(),
            amount: a,
            due: d,
            paid: p,
        }
    }

    pub fn print_bill(b: &Bill){
        println!("  {}", b.name);
        println!("  ${}", b.amount);
        println!("  day due: {}", b.due);
        println!("  paid: {}", b.paid);
    }
}

pub mod cmd_handler {

    use std::io::{self, Write};
    use crate::bill_handler;
    use std::fs::File;

    // Prints full list of commands to the console.
    pub fn view_commands() {
        println!();
        println!("Commands:");
        println!("  View all bills:         'view -all'");
        println!("  View specific bill:     'view -b <bill_name>'");
        println!("  Pay bill:               'pay -b <bill_name>'");
        println!("  Edit bill:              'edit -b <bill_name>'");
        println!("  Delete bill:            'delete -b <bill_name>'");
        println!("  Add bill:               'add -b'");
        println!("  View unpaid bills:      'view -unpaid'");
        println!("  View paid bills:        'view -paid'");
        println!("  View amount paid:       'amount -paid'");
        println!("  View amount unpaid:     'amount -unpaid'");
        println!("  View commands:          'view -cmd'");
        println!("  Save to default file:   'save'");
        println!("  Exit applicaion:        'exit'");
        println!();
    }

    // Prints all bills in stored in bill vector to console.
    pub fn view_all(bills: &Vec<bill_handler::Bill>) {
        for b in bills {
            bill_handler::print_bill(b);
            println!();
        }
    }

    // Prints infomration of specific bill.
    pub fn view_bill(name: &str, bills: &Vec<bill_handler::Bill>) {

        let mut found = false;
        for b in bills {
            if name == b.name {
                bill_handler::print_bill(b);
                found = true;
                break;
            }
        }
        if !found {
            println!("Error: failed to find bill with name '{name}'");
        }
    }

    // Prints every unpaid bill to the console.
    pub fn view_unpaid(bills: &Vec<bill_handler::Bill>) {

        println!("Unpaid bills:\n");
        let mut count = 0;
        for b in bills {
            if !b.paid {
                bill_handler::print_bill(b);
                println!();
                count += 1;
            }
        }
        println!("Total: {count}");
    }

    // Prints every paid bill to the console.
    pub fn view_paid(bills: &Vec<bill_handler::Bill>) {
        
        println!("Paid bills:\n");
        let mut count = 0;
        for b in bills {
            if b.paid {
                bill_handler::print_bill(b);
                println!();
                count += 1;
            }
        }
        println!("Total: {count}");
    }

    // Get total amount of money of the unpaid bills.
    pub fn view_amount_unpaid(bills: &Vec<bill_handler::Bill>) {

        let mut tot = 0.0;
        for b in bills {
            if !b.paid {
                tot += b.amount;
            }
        }
        println!("Totoal amount unpaid: ${:.2}", tot);
    }

    pub fn view_amount_paid(bills: &Vec<bill_handler::Bill>) {

        let mut tot = 0.0;
        for b in bills {
            if b.paid {
                tot += b.amount;
            }
        }
        println!("Total amount paid: ${:.2}", tot);
    }

    // Edit information of specified bill.
    pub fn edit_bill(name: &str, bills: &mut Vec<bill_handler::Bill>) {

        let mut found = false;
        let mut ind = 0;

        let mut bill_name = String::new();
        let mut bill_amount = 0.0;
        let mut bill_due = 0;

        for b in &mut *bills {
            if name == b.name {

                println!("\nEditing bill:");
                bill_handler::print_bill(b);

                bill_name = b.name.to_string();
                bill_amount = b.amount;
                bill_due = b.due;

                found = true;
                break;
            }
            ind += 1;
        }

        if !found {
            println!("Error: failed to find bill with name '{name}'");
        } else {
            
            // edit bill name
            print!("Would you like to change bill name? ('yes'/'no'): ");
            let _ = io::stdout().flush();
            let mut s = String::new();
            io::stdin()
                .read_line(&mut s)
                .expect("Failed to read line in");
            
            if s.trim() == "yes" {
            
                let mut cond = true;
                while cond {
                    // read bill name
                    print!("New name: ");
                    let _ = io::stdout().flush();
                    let mut s1 = String::new();
                    io::stdin()
                        .read_line(&mut s1)
                        .expect("Failed to read line in");
                                
                    let n = s1.trim().to_string();
                        
                    // check for duplicate bill
                    if check_duplicate(&n, bills) {
                        println!("Error: bill name already exists - choose unique name.");
                    } else {
                        bill_name = n;
                        cond = false;
                    }
                }
            }

            // edit amount
            print!("Would you like to change bill amount? ('yes'/'no'): ");
            let _ = io::stdout().flush();
            let mut s2 = String::new();
            io::stdin()
                .read_line(&mut s2)
                .expect("Failed to read line in");

            if s2.trim() == "yes" {
                print!("Amount: ");
                let _ = io::stdout().flush();
                let mut s3 = String::new();
                io::stdin()
                    .read_line(&mut s3)
                    .expect("Failed to read line in");

                bill_amount = s3.trim().parse::<f32>().unwrap();
            }

            // edit due date
            print!("Would you like to change the bill due day? ('yes'/'no'): ");
            let _ = io::stdout().flush();
            let mut s3 = String::new();
            io::stdin()
                .read_line(&mut s3)
                .expect("Failed to read line in");

            if s3.trim() == "yes" {
                print!("Due day: ");
                let _ = io::stdout().flush();
                let mut s4 = String::new();
                io::stdin()
                    .read_line(&mut s4)
                    .expect("Failed to read line in");

                bill_due = s4.trim().parse::<u32>().unwrap();
            }

            println!("Updated bill information: ");
            println!("Bill name: {bill_name}");
            println!("Bill amount: {bill_amount}");
            println!("Bill due: {bill_due}");

            print!("Are you sure you want to update this bill information? ('yes'/'no'): ");
            let _ = io::stdout().flush();
            let mut s5 = String::new();
            io::stdin()
                .read_line(&mut s5)
                .expect("Failed to read line in");

            if s5.trim() == "yes" {
                bills[ind].name = bill_name.clone();
                bills[ind].amount = bill_amount.clone();
                bills[ind].due = bill_due.clone();
                println!("Bill information updated.");
            } else {
                println!("Bill update canceled.");
            }
        }
    }

    // Create new bill and add to bill vector.
    pub fn add_bill(bills: &mut Vec<bill_handler::Bill>) {

        let mut n = String::new();
        let mut b = true;
        while b {
            // read bill name
            print!("Name: ");
            let _ = io::stdout().flush();
            let mut s = String::new();
            io::stdin()
                .read_line(&mut s)
                .expect("Failed to read line in");
        
            n = s.trim().to_string();

            // check for duplicate bill
            if check_duplicate(&n, bills) {
                println!("Error: bill name already exists - choose unique name.");
            } else {
                b = false;
            }
        }

        // read bill amount
        print!("Amount: ");
        let _ = io::stdout().flush();
        let mut s1 = String::new();
        io::stdin()
            .read_line(&mut s1)
            .expect("Failed to read line in");

        let a = s1.trim().parse::<f32>().unwrap();

        // read bill due day
        print!("Due (month day #):");
        let _ = io::stdout().flush();
        let mut s2 = String::new();
        io::stdin()
            .read_line(&mut s2)
            .expect("Failed to read line in");

        let d = s2.trim().parse::<u32>().unwrap();

        println!("Bill:");
        println!("  Name - {n}");
        println!("  Amount - {a}");
        println!("  Due - {d}");
        
        print!("Would you like to add this bill? ('yes'/'no'): ");
        let _ = io::stdout().flush();
        let mut s3 = String::new();
        io::stdin()
            .read_line(&mut s3)
            .expect("Failed to read line in");
 
        let ans = s3.trim();

        if ans == "yes" {
            // push to bills vector
            let new_bill = bill_handler::build_bill(&n, a, d, false);
            bills.push(new_bill);
            println!("Bill added!");
        }   

    }

    // Delete specified bill from bill vector.
    pub fn delete_bill(name: &str, bills: &mut Vec<bill_handler::Bill>) {

        let mut ind = 0;
        let mut found = false;
        
        for b in &mut *bills {

            if name == b.name {
                println!("\nAbout to delete bill:");
                bill_handler::print_bill(b);

                print!("Are you sure? ('yes'/'no'): ");
                let _ = io::stdout().flush();
                let mut s = String::new();
                io::stdin()
                    .read_line(&mut s)
                    .expect("Failed to read line in");

                if s.trim() == "yes" {
                    bills.remove(ind);
                    println!("Bill deleted!");
                }

                found = true;
                break;
            }
            ind += 1;
        }

        if !found {
            println!("Unable to find bill with name: {name}");
        }
    }

    // Mark input bill as paid.
    pub fn pay_bill(name: &str, bills: &mut Vec<bill_handler::Bill>) {

        let mut found = false;
        for b in bills {
            if name == b.name {
                println!("Bill info: ");
                bill_handler::print_bill(b);
                
                print!("Would you like to mark this bill as paid? ('yes'/'no'): ");
                let _ = io::stdout().flush();
                let mut s = String::new();
                io::stdin()
                    .read_line(&mut s)
                    .expect("Failed to read line in");

                if s.trim() == "yes" {
                    // mark as paid
                    b.paid = true;
                    println!("Bill paid.");
                } else {
                    // mark as unpaid
                    b.paid = false;
                    println!("Bill not paid.");
                }

                found = true;
                break;
            }
        }
        if !found {
            println!("Error: failed to find bill with name '{name}'");
        }
    }

    // Print bill vector to default files.txt in csv format for each bill
    pub fn save(bills: &Vec<bill_handler::Bill>) {
        
        let mut file = File::create("bills.txt")
            .expect("Failed to create file");

        file.set_len(0)
            .expect("Failed to set file lenth");
        
        for b in bills {
            let csv = format!("{},{:.2},{},{}", b.name, b.amount, b.due, b.paid);
            file.write_all(csv.as_bytes())
                .expect("Failed to write to file");

            file.write_all(b"\n")
                .expect("Failed to write to file");
        }
    
    }

    // check for duplicate bill name when adding/editing bill.
    fn check_duplicate(name: &str, bills: &Vec<bill_handler::Bill>) -> bool {
        let mut is_dup = false;
        for b in bills {
            if name == b.name {
                is_dup = true;
                break;        
            }
        }
        is_dup
    }
}
