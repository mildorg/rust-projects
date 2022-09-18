pub fn learn() {
    // test_base_use();
    // test_question_mark();
    test_map_combinator();
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

// map combinator
fn test_map_combinator() {
    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    // let potato: Option<Food> = None;
    let potato: Option<Food> = Some(Food::Potato);

    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));
    let cooked_potato = process(potato);

    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);
}

#[derive(Debug)]
enum Food {
    Apple,
    Carrot,
    Potato,
}

#[derive(Debug)]
struct Peeled(Food);

#[derive(Debug)]
struct Chopped(Food);

#[derive(Debug)]
struct Cooked(Food);

fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None => None,
    }
}

fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
        Some(Peeled(food)) => Some(Chopped(food)),
        None => None,
    }
}

fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food)| Cooked(food))
}

fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

fn eat(food: Option<Cooked>) {
    match food {
        Some(food) => println!("Mmm. I love {:?}", food),
        None => println!("Oh no! It wasn't edible."),
    }
}
