pub fn learn() {
    let a = 3;
    let b = 10;

    let container = Container(a, b);

    println!(
        "Does container contain {} and {}: {}",
        a,
        b,
        container.contains(&a, &b)
    );

    println!("First number: {}", container.first());
    println!("last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}

struct Container(i32, i32);

// trait Contains<A, B> {
//     fn contains(&self, _: &A, _: &B) -> bool;
//     fn first(&self) -> i32;
//     fn last(&self) -> i32;
// }

// impl Contains<i32, i32> for Container {
//     fn contains(&self, a: &i32, b: &i32) -> bool {
//         &self.0 == a && &self.1 == b
//     }

//     fn first(&self) -> i32 {
//         self.0
//     }

//     fn last(&self) -> i32 {
//         self.1
//     }
// }

// fn difference<A, B, C>(container: &C) -> i32
// where
//     C: Contains<A, B>,
// {
//     container.last() - container.first()
// }

trait Contains {
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> Self::A;
    fn last(&self) -> Self::B;
}

impl Contains for Container {
    type A = i32;
    type B = i32;

    fn contains(&self, a: &Self::A, b: &Self::B) -> bool {
        &self.0 == a && &self.1 == b
    }

    fn first(&self) -> Self::A {
        self.0
    }

    fn last(&self) -> Self::B {
        self.1
    }
}

fn difference(container: &Container) -> i32 {
    container.last() - container.first()
}
