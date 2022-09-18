trait Person {
    fn name(&self) -> &str;
}

trait Student: Person {
    fn university(&self) -> &str;
}

trait Programmer {
    fn fav_language(&self) -> &str;
}

trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> &str;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username(),
    );
}
