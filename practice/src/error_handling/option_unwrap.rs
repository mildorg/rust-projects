pub fn learn() {
    // test_base_use();
    test_question_mark();
}

// base use
fn test_base_use() {
    let water = Some("water");
    let lemonade = Some("lemonade");
    let void = None;

    give_adult(water);
    give_adult(lemonade);
    give_adult(void);

    let coffee = Some("coffee");
    let nothing = None;

    drink(coffee);
    drink(nothing);
}

fn give_adult(drink: Option<&str>) {
    match drink {
        Some("lemonade") => println!("Yuck! Too sugary"),
        Some(inner) => println!("{}? How nice.", inner),
        None => println!("No drink? Oh well"),
    }
}

fn drink(drink: Option<&str>) {
    let inside = drink.unwrap();

    if inside == "lemonade" {
        panic!("AAAaaaaa!!!");
    }

    println!("I love {}s!!!", inside);
}

// use ? operator
fn test_question_mark() {
    let p = Person::form_phone_number(PhoneNumber {
        area_code: Some(61),
        number: 439222222,
    });

    assert_eq!(p.work_phone_area_code(), Some(61));
}

struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {
    fn form_phone_number(phone_number: PhoneNumber) -> Self {
        Person {
            job: Some(Job {
                phone_number: Some(phone_number),
            }),
        }
    }

    fn work_phone_area_code(&self) -> Option<u8> {
        self.job?.phone_number?.area_code
    }
}
