use std::io;
use std::{thread, time};

/*
  i
 iii InteriOS* (*Not an actual operating system)
  i
    */

fn main() {
    let mut boot_tick = 1;
    let mut interi_os = 0;
    print!("{}[2J", 27 as char); //clears screen
    println!("\t\ti\n\t       iii\n\t      iiiii\n\t     iiiiiii\n\t      iiiii\n\t       iii\n\t\ti");
    print!("\t    interiOS\n");
    thread::sleep(time::Duration::from_secs(2));
    while boot_tick == 1 {
        print!("{}[2J", 27 as char);
        println!("\nAre you Logging in or creating a new user? [Sign-in: s | Log-in: l] ");
        let mut u1a = String::new();
        io::stdin()
            .read_line(&mut u1a)
            .expect("Failed to read input");

        if u1a == "s\r\n" { //sign in process
            print!("{}[2J", 27 as char);
            println!("Sign in\n---\nUsername: ");
            let mut user = String::new();
            io::stdin()
                .read_line(&mut user)
                .expect("Failed to read line");
            println!("\nPassword: ");
            let mut pass = String::new();
            io::stdin()
                .read_line(&mut pass)
                .expect("Failed to read line");
            print!("{}[2J", 27 as char);
            let pass = pass.trim();
            let user = user.trim();
            println!("Sign up process successful\n---");
            println!("Username:\n{user}");
            println!("Password:");
            for _i in 0..pass.chars().count() {
                print!("*");
            }
            println!("\n");
            thread::sleep(time::Duration::from_secs(2));
            interi_os = 1;
            boot_tick = 0;

        } else if u1a == "l\r\n" { //log in process
            println!("Log in\n---\nUsername: ");
            boot_tick = 0; //save for last

        } else if u1a == "q\r\n" { //quit process
            boot_tick = 0;

        } else { //Invalid process
            println!("Not a valid input");
            continue

        }
    }
    let mut new_mail = 1;
    while interi_os == 1 {
        print!("{}[2J", 27 as char); 
        println!("               - Home -                ");
        println!("Mail | Weather | Apps | Settings | Quit");
        let mut u2a = String::new();
        io::stdin()
            .read_line(&mut u2a)
            .expect("Failed to read input");
        let u2a = u2a.trim();
        if u2a == "Quit" || u2a == "quit" || u2a == "q" { //quit process
            break

        } else if u2a == "Mail" || u2a == "mail" || u2a == "m" { //mail process
            print!("{}[2J", 27 as char);
            if new_mail > 0 {
                println!("Inbox({new_mail}) | Send | Home");
                new_mail = 0;
            } else {
                println!("Inbox | Send | Home");  
            }                           
            let mut u2b = String::new();   
            io::stdin()
                .read_line(&mut u2b)
                .expect("Failed to read input");
            let u2b = u2b.trim();
            if u2b == "Inbox" || u2b == "inbox" || u2b == "i" {
                print!("{}[2J", 27 as char);
                println!("Inbox\nType a number to view a letter | Type Home to return Home\n1) Welcome to InteriOS | from: InteriOS Developers");
                let mut u2c = String::new();
                io::stdin()
                    .read_line(&mut u2c)
                    .expect("Failed to read input");
                let u2c = u2c.trim();
                if u2c == "1" {
                    print!("{}[2J", 27 as char);
                    println!("(Type home to return Home)\n");
                    println!("Subject: Welcome to InteriOS");
                    println!("From: InteriOS Developers");
                    println!("Thank you for using InteriOS, a text based operating system written is rust! \n i\niii ~InteriOS Developers\n i");
                    let mut u2d = String::new();
                    io::stdin()
                        .read_line(&mut u2d)
                        .expect("Failed to read input");
                    let u2d = u2d.trim();
                    if u2d == "Home" || u2d == "home" || u2d == "h" {
                        continue
                    } else {
                        continue
                    }
                }

            } if u2b == "Send" || u2b == "send" || u2b == "s" {
                print!("{}[2J", 27 as char);
                println!("Recipitent(s): ");
                let mut recip = String::new();
                io::stdin()
                    .read_line(&mut recip)
                    .expect("Failed to read input");
                let recip = recip.trim();
                print!("{}[2J", 27 as char);
                println!("Subject: ");
                let mut subject = String::new();
                io::stdin()
                    .read_line(&mut subject)
                    .expect("Failed to read line");
                let subject = subject.trim();
                print!("{}[2J", 27 as char);
                println!("Body: ");
                let mut body = String::new();
                io::stdin()
                    .read_line(&mut body)
                    .expect("Failed to read line");
                let body = body.trim();
                print!("{}[2J", 27 as char);
                println!("(Type Send to Send your letter | Type Home to return Home)\n");
                println!("Subject: {subject}\nRecipitent(s): {recip}\n{body}");
                let mut u2c = String::new();
                io::stdin()
                    .read_line(&mut u2c)
                    .expect("Failed to read line");
                let u2c = u2c.trim();
                if u2c == "Send" || u2c == "send" || u2c == "s" {
                    print!("{}[2J", 27 as char);
                    println!("sending\n[#####]");
                    thread::sleep(time::Duration::from_secs(1));
                    print!("{}[2J", 27 as char);
                    println!("sending\n[>####]");
                    thread::sleep(time::Duration::from_secs(1));
                    print!("{}[2J", 27 as char);
                    println!("sending\n[->###]");
                    thread::sleep(time::Duration::from_secs(1)); //SUFFER THE LONG WAIT MWHAHA!!!
                    print!("{}[2J", 27 as char);
                    println!("sending\n[-->##]");
                    thread::sleep(time::Duration::from_secs(1));
                    print!("{}[2J", 27 as char);
                    println!("sending\n[--->#]");
                    thread::sleep(time::Duration::from_secs(1));
                    print!("{}[2J", 27 as char);
                    println!("[-----]\n *SENT*");
                    thread::sleep(time::Duration::from_secs(1));
                    continue
                } else {
                    continue
                }

            } if u2b == "Home" || u2b == "home" || u2b == "h" {
                continue

            } else {
                println!("Failed to read input");
                continue
            }

        } else if u2a == "Weather" || u2a == "weather" || u2a == "w" { //weather process
            print!("{}[2J", 27 as char);
            println!("type Home to return");
            thread::sleep(time::Duration::from_secs(1));
            print!("{}[2J", 27 as char);
            println!("*");
            thread::sleep(time::Duration::from_millis(500));
            print!("{}[2J", 27 as char);
            println!("W*");
            thread::sleep(time::Duration::from_millis(500));
            print!("{}[2J", 27 as char);
            println!("WN*");
            thread::sleep(time::Duration::from_millis(500));
            print!("{}[2J", 27 as char);
            println!("WN2*");
            thread::sleep(time::Duration::from_millis(500));
            print!("{}[2J", 27 as char);
            println!("*WN28*");
            thread::sleep(time::Duration::from_secs(1));
            print!("{}[2J", 27 as char);
            println!("Down in our small county, Willowlake, Wyoming. Temperatures will reach as low as -12C, and a blizzard is said to be on its way");
            println!("Dress appropiately, and stay by your fireplace");
            let mut u2b = String::new();
            io::stdin()
                .read_line(&mut u2b)
                .expect("Failed to read input");
            if u2b == "Home" {
                continue
            } else {
                continue
            }

        } else if u2a == "Apps" || u2a == "apps" || u2a == "a" { //apps process
            continue

        } else if u2a == "Settings" || u2a == "settings" || u2a == "s" { //settings process
            continue

        } else {
            println!("failed to read input");
            continue
        }
    }
}



// Main Inteface | Interi
// Mail | Weather | Settings | Quit
// first asks you to create a username and password
// C -> F converter in Weather?
// Log-in functionality? using file io, if file ... has ... written on it, then a log in screen
//      will prompt
// At the start, maybe ask you to login or create a new user
// neofetch
// interacts with real terminal
// Using tabs, make everything print in the center of a full screen terminal
// save sent messages
