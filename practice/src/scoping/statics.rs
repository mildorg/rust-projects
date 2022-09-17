use std::fmt::Debug;

static NUM: i32 = 18;

pub fn learn() {
    {
        let static_string = "I'm in read-only memory";
        println!("static_string:{}", static_string);
    }

    {
        let lifetime_num = 9;

        let coerced_static = coerce_static(&lifetime_num);
        println!("coerced_static:{}", coerced_static);
    }

    println!("NUM:{} stays accessible!", NUM);

    let i = 5;
    print_it(i);
    // print_it(&i);
}

fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn print_it<D: Debug + 'static>(input: D) {
    println!("'Static value passed in is: {:?}", input);
}
