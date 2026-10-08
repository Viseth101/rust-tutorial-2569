trait Order {
    fn order(&self);
}

struct Student;
struct Teacher;
struct Staff;
struct Guest;

impl Order for Student {
    fn order(&self) {
        println!("Student ordered food");
    }
}

impl Order for Teacher {
    fn order(&self) {
        println!("Teacher ordered food");
    }
}

impl Order for Staff {
    fn order(&self) {
        println!("Staff ordered food");
    }
}

impl Order for Guest {
    fn order(&self) {
        println!("Guest ordered food");
    }
}

fn make_order<T: Order>(customer: T) {
    customer.order();
}

fn main() {
    let student = Student;
    let teacher = Teacher;
    let staff = Staff;
    let guest = Guest;

    make_order(student);
    make_order(teacher);
    make_order(staff);
    make_order(guest);
}
