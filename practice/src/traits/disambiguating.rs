use std::mem;

pub fn learn() {
    // let form = Form {
    //     username: "abcdef".to_owned(),
    //     age: 28,
    // };

    // let username = UsernameWidget::get(&form);
    // let age = AgeWidget::get(&form, 8);

    // println!("username is {}", username);
    // println!("age is {}", age);

    println!("size of form: {}", mem::size_of::<Form>());
}

trait UsernameWidget {
    fn get(&self) -> String;
}

trait AgeWidget {
    fn get(&self, addition: u8) -> u32;
}

struct Form {
    id: String,
    token: String,
    username: String,
    password: String,
    sax: String,
    age: u8,
}

impl Default for Form {
    fn default() -> Self {
        Self {
            id: Default::default(),
            token: Default::default(),
            username: Default::default(),
            password: Default::default(),
            sax: Default::default(),
            age: Default::default(),
        }
    }
}

// impl UsernameWidget for Form {
//     fn get(&self) -> String {
//         self.username.clone()
//     }
// }

// impl AgeWidget for Form {
//     fn get(&self, addition: u8) -> u32 {
//         (self.age + addition) as u32
//     }
// }
