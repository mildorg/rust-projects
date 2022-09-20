use std::collections::{HashMap, HashSet};

pub fn learn() {
    // test_base_use();
    // test_custom_key();
    test_hash_set();
}

fn call(number: &str) -> &str {
    match number {
        "798-1364" => {
            "We're sorry, the call cannot be completed as dialed. 
            Please hang up and try again."
        }
        "645-7689" => {
            "Hello, this is Mr. Awesome's Pizza. My name is Fred.
            What can I get for you today?"
        }
        _ => "Hi! Who is this again?",
    }
}

fn test_base_use() {
    let mut contacts = HashMap::new();

    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("Robert", "956-1745");

    // println!("> {:?}", contacts);

    match contacts.get("Daniel") {
        Some(&v) => println!("Calling Daniel: {}", call(v)),
        None => println!("Don't have Daniel's number"),
    }

    // contacts.insert("Daniel", "164-6743");
    // println!("> {:?}", contacts);

    match contacts.get("Ashley") {
        Some(&v) => println!("Calling Ashley: {}", call(v)),
        None => println!("Don't have Ashley's number"),
    }

    contacts.remove("Ashley");

    for (contact, number) in contacts.iter() {
        println!("Calling {}: {}", contact, call(number));
    }
}

// custom key
fn test_custom_key() {
    let mut accounts = HashMap::new();

    let account = Account {
        username: "j.everyman",
        password: "password123",
    };

    let account_info = AccountInfo {
        name: "John Everyman",
        email: "j.everyman@email.com",
    };

    accounts.insert(account, account_info);

    try_logon(&accounts, "j.everyman", "password12345");

    println!();
    try_logon(&accounts, "j.everyman", "password123");
}

#[derive(PartialEq, Eq, Hash)]
struct Account<'a> {
    username: &'a str,
    password: &'a str,
}

struct AccountInfo<'a> {
    name: &'a str,
    email: &'a str,
}

type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

fn try_logon<'a>(accounts: &Accounts<'a>, username: &str, password: &str) {
    println!("Username: {}", username);
    println!("Password: {}", password);
    println!("Attempting logon...");

    let logon_token = Account { username, password };

    match accounts.get(&logon_token) {
        Some(info) => {
            println!("Successful logon!");
            println!("Name: {}", info.name);
            println!("Email: {}", info.email);
        }
        _ => println!("Login failed!"),
    }
}

// hash set
fn test_hash_set() {
    let mut a: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    let mut b: HashSet<i32> = vec![2, 3, 4].into_iter().collect();

    assert!(a.insert(4));
    assert!(a.contains(&4));

    // assert!(b.insert(4), "Value 4 is already in set B");
    b.insert(5);

    println!("A: {:?}", a);
    println!("B: {:?}", b);

    println!("Union: {:?}", a.union(&b));

    println!("Difference: {:?}", a.difference(&b));

    println!("Intersection: {:?}", a.intersection(&b));

    println!("Symmetric Difference: {:?}", a.symmetric_difference(&b));
}
