pub fn learn() {
    let mut dolly: Sleep = Animal::new("Dolly");

    dolly.talk();
    dolly.shear();
    dolly.talk();
}

struct Sleep<'a> {
    naked: bool,
    name: &'a str,
}

trait Animal<'a> {
    fn new(name: &'a str) -> Self;
    fn name(&self) -> &str;
    fn noise(&self) -> &str;

    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl<'a> Sleep<'a> {
    fn is_snaked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_snaked() {
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);
            self.naked = true;
        }
    }
}

impl<'a> Animal<'a> for Sleep<'a> {
    fn new(name: &'a str) -> Self {
        Self { naked: false, name }
    }

    fn name(&self) -> &str {
        self.name
    }

    fn noise(&self) -> &str {
        if self.is_snaked() {
            "baaaaah?"
        } else {
            "baaaaah!"
        }
    }

    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}
