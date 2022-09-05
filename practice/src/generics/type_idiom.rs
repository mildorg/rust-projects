pub fn learn() {
    let age = Years(20);
    let age_days = age.to_days();

    println!("Old enough: {}", old_enough(&age));
    println!("Old enough: {}", old_enough(&age_days.to_years()));

    let year = age.0;
    let Years(y) = age;
    println!("{}", year);
    println!("{}", y);
}

struct Years(i64);
struct Days(i64);

impl Years {
    fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}

impl Days {
    fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn old_enough(age: &Years) -> bool {
    age.0 > 18
}
