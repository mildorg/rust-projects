pub fn learn() {
    let form = Form {
        username: "rustacean".to_owned(),
        age: 28,
    };

    let username = UsernameWidget::get(&form);
    let age = AgeWidget::get(&form, 8);

    println!("username is {}", username);
    println!("age is {}", age);
}

trait UsernameWidget {
    fn get(&self) -> String;
}

trait AgeWidget {
    fn get(&self, addition: u8) -> u32;
}

struct Form {
    username: String,
    age: u8,
}

impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}

impl AgeWidget for Form {
    fn get(&self, addition: u8) -> u32 {
        (self.age + addition) as u32
    }
}
