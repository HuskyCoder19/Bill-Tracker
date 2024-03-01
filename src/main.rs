use std::io::{self, Write, BufRead, BufReader};
use bill_tracker::bill_handler;
use bill_tracker::cmd_handler;
use std::fs::{OpenOptions, File};

fn main() {

    let path = "bills.txt"; // use bills.txt as default file

    // Open file for read & write, create one if it doesn't exist
    let f = OpenOptions::new()
        .read(true)
        .open(path)
        .expect("Failed to open file");

    // Open bill file
    let mut bills = parse_file(&f);

    println!("Welcome to your monthly bill tracker!");

    cmd_handler::view_commands();

    let mut val = true;

    while val {

        print!("->");
        let _ = io::stdout().flush();

        // Read stdin
        let mut cmd = String::new();
        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to read line in");

        val = parse_command(cmd.trim(), &mut bills);

    }
}

fn parse_file(file: &File) -> Vec<bill_handler::Bill> {

    let mut bill_vec: Vec<bill_handler::Bill> = Vec::new(); 

    let reader = BufReader::new(file);

    // read file line by line
    for line in reader.lines() {
        
        let l = line.expect("Failed to read line");

        let v: Vec<&str> = l.split(',').collect(); // parse csv line
        if v.len() == 4 {
            let amount = v[1].parse::<f32>().unwrap(); // string to f32
            let due = v[2].parse::<u32>().unwrap(); // string to u32
            
            let mut paid = false;
            // parse bool
            if v[3] == "true" {
                paid = v[3].parse::<bool>()
                    .expect("Failed to parse bool");
            }

            let this_bill = bill_handler::build_bill(v[0], amount, due, paid);
            bill_vec.push(this_bill); // append to vector of bills
        }
    }

    bill_vec
}

fn parse_command(cmd: &str, bills: &mut Vec<bill_handler::Bill>) -> bool {

    let mut val = true;

    if cmd == "view -all" {
        cmd_handler::view_all(bills);
    } else if cmd.contains("view -b") {
        // remove command from bill name
        let substr = "view -b ";
        let bill_name = cmd.replace(substr, "");
        cmd_handler::view_bill(&bill_name, bills);
    } else if cmd.contains("pay -b") {
        // remove command from bill name
        let substr = "pay -b ";
        let bill_name = cmd.replace(substr, "");
        cmd_handler::pay_bill(&bill_name, bills);
    } else if cmd.contains("edit -b") {
        // remove command from bill name
        let substr = "edit -b ";
        let bill_name = cmd.replace(substr, "");
        cmd_handler::edit_bill(&bill_name, bills);
    } else if cmd.contains("delete -b") {
        // remove command from bill name
        let substr = "delete -b ";
        let bill_name = cmd.replace(substr, "");
        cmd_handler::delete_bill(&bill_name, bills);
    } else if cmd == "add -b" {
        cmd_handler::add_bill(bills);
    } else if cmd == "view -unpaid" {
        cmd_handler::view_unpaid(bills);
    } else if cmd == "view -paid" {
        cmd_handler::view_paid(bills);
    } else if cmd == "amount -unpaid" {
        cmd_handler::view_amount_unpaid(bills);
    } else if cmd == "amount -paid" {
        cmd_handler::view_amount_paid(bills);
    } else if cmd == "view -cmd" {
        cmd_handler::view_commands();
    } else if cmd == "save" {
        cmd_handler::save(bills);
        println!("Saved!");
    } else if cmd == "exit" {
        println!("Saving and exiting...");
        cmd_handler::save(bills);
        val = false;
    } else {
        println!("ERROR: '{cmd}' command not found");
    }
    val
}
