macro_rules! calculate {
    (eval $e: expr) => {
        {
            let val: usize = $e;
            println!("{} = {}",stringify!($e),val);
        }
    };

    (eval $e:expr, $(eval $es:expr), +) => {
       {
        calculate! {eval $e}
        calculate! {$(eval $es),+}
       }
    }
}

pub fn learn() {
    calculate! {
        eval 1 + 2
    }

    calculate! {
        eval (1+2)*(3/4)
    }

    println!("\nVariadic calculate!");

    calculate! {
        eval 1+2,
        eval 3+ 4,
        eval (2*3) +1
    }
}
