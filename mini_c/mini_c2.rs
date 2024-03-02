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
    print!("{}[2J", 27 as char); //clears screen || Start of welcome screen
    println!("\t\ti\n\t       iii\n\t      iiiii\n\t     iiiiiii\n\t      iiiii\n\t       iii\n\t\ti");
    print!("\t    interiOS\n");
    thread::sleep(time::Duration::from_secs(2)); //End of welcome screen
    while boot_tick == 1 { //Start of boot process
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
            for _i in 0..pass.chars().count() { //Fancy password hider
                print!("*");
            }
            println!("\n");
            thread::sleep(time::Duration::from_secs(2));
            interi_os = 1; //Leads to main software
            boot_tick = 0; //End of sign-in process

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
    while interi_os == 1 { //Start of main software
        print!("{}[2J", 27 as char); 
        println!("               - Home -                ");
        println!("Mail | Weather | Apps | Settings | Quit"); //Home layout
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
            if u2b == "Inbox" || u2b == "inbox" || u2b == "i" { //Checking the inbox in mail
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
                    if u2d == "Home" || u2d == "home" || u2d == "h" { //leaving inbox
                        continue
                    } else {
                        continue
                    }
                }

            } if u2b == "Send" || u2b == "send" || u2b == "s" { //Sending a message through mail
                print!("{}[2J", 27 as char); // --
                println!("Recipitent(s): ");
                let mut recip = String::new(); // *
                io::stdin()
                    .read_line(&mut recip)
                    .expect("Failed to read input");
                let recip = recip.trim(); // --
                print!("{}[2J", 27 as char); // --
                println!("Subject: ");
                let mut subject = String::new(); // *
                io::stdin()
                    .read_line(&mut subject)
                    .expect("Failed to read line");
                let subject = subject.trim(); // --
                print!("{}[2J", 27 as char);
                println!("Body: ");
                let mut body = String::new(); // *
                io::stdin()
                    .read_line(&mut body)
                    .expect("Failed to read line"); 
                let body = body.trim(); // --
                print!("{}[2J", 27 as char);
                println!("(Type Send to Send your letter | Type Home to return Home)\n");
                println!("Subject: {subject}\nRecipitent(s): {recip}\n{body}");
                let mut u2c = String::new();
                io::stdin()
                    .read_line(&mut u2c)
                    .expect("Failed to read line");
                let u2c = u2c.trim();
                if u2c == "Send" || u2c == "send" || u2c == "s" { // Send animation
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

            } if u2b == "Home" || u2b == "home" || u2b == "h" { // Leaving send
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
            print!("{}[2J", 27 as char); // end of animation
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
            print!("{}[2J", 27 as char);
            println!("Weather Converter | Notes | Terminal | Snake | Home");
            let mut u2c = String::new();
            io::stdin()
                .read_line(&mut u2c)
                .expect("Failed to read input");
            let u2c = u2c.trim();
            if u2c=="Weather Converter"||u2c=="weather converter"||u2c=="Weather converter"||u2c=="w"||u2c=="wc" {
                print!("{}[2J", 27 as char);
                let mut u1a = String::new();
                    println!("The Temperature Conversion app"); //This was copy and pasted from my
                                                                //own weather converter code
                    println!("Type F for: Fahrenheit -> Celsius || C for: Celsius -> Fahrenheit");
                    let _u1b = io::stdin()
                        .read_line(&mut u1a)
                        .expect("It broke, bud");
                if u1a == "F\r\n" || u1a == "f\r\n" { // fahrenheit -> celsius
                    let mut u2a = String::new();
                    println!("Known Fahrenheit: ");
                    let _u2b = io::stdin()
                        .read_line(&mut u2a)
                        .expect("It broke, bud");
                    let u2a: f32 = u2a.trim().parse().expect("oops");
                    println!("C: {}",c_celsius(u2a as f32));
                    thread::sleep(time::Duration::from_secs(3));
                    continue
                                        
                } else if u1a == "C\r\n" || u1a == "c\r\n" { // Celsius -> Fahrenheit
                    let mut u2a = String::new();
                    println!("Known Celsius: ");
                    let _u2b = io::stdin()
                        .read_line(&mut u2a)
                        .expect("It broke, bud");
                    let u2a: f32 = u2a.trim().parse().expect("oops");
                    println!("F: {}",c_fahrenheit(u2a as f32));
                    thread::sleep(time::Duration::from_secs(3));
                    continue

                } else {
                    println!("Not a correct statement");
                    thread::sleep(time::Duration::from_secs(2));
                    continue
                }
                fn c_fahrenheit(c: f32) -> f32 {
                        c*(9.0/5.0)+32.0
                }
                fn c_celsius(f: f32) -> f32 {
                        (f-32.0)*(5.0/9.0)
                }

            } else if u2c=="Notes"||u2c=="notes"||u2c=="note"||u2c=="Note"||u2c=="n" {
                print!("{}[2J", 27 as char);
                println!("Welcome to Notes");
                //C:\Users\EnUni\Ex_Folder\randFile_relocate
                //Ask for amount of lines & to open or write files
                let mut l_count = String::new();
                println!("How many lines do you want in your document?");
                io::stdin()
                    .read_line(&mut l_count)
                    .expect("F A I L E D - t o - R E A D - l i n e");
                let l_count: i32 = l_count.trim().parse().expect("ruh roh");
                let mut page = vec![];
                for i in 1..=l_count {
                    print!("{}[2J", 27 as char);
                    let mut write_up = String::new();
                    println!("Line {i}: ");
                    io::stdin()
                        .read_line(&mut write_up)
                        .expect("F A I L E D - t o - R E A D - l i n e");
                    //let write_up = write_up.trim();
                    page.push(write_up);

                }
                print!("{}[2J", 27 as char);
                println!("{:?}", page);
                thread::sleep(time::Duration::from_secs(3));
                continue
                
            } else if u2c=="Terminal"||u2c=="terminal"||u2c=="t" {
                println!("Welcome to the mainframe... OH YEEEAAAAAHHHHH!!!!!!!!");

            } else if u2c=="Snake"||u2c=="snake"||u2c=="s" {
                println!("ol snakey snake");

            } else {
                continue
            }

        } else if u2a == "Settings" || u2a == "settings" || u2a == "s" { //settings process
            continue

        } else {
            println!("failed to read input");
            continue
        }
    }
}



//revised draft
//Lots of placeholder apps that need to be worked on
