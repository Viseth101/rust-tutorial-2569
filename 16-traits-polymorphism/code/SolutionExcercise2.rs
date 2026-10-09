trait Booking {
    fn book(&self);
}

struct Student;
struct Staff;
struct Guest;
struct Athlete;

impl Booking for Student {
    fn book(&self) {
        println!("Student booked a badminton court");
    }
}

impl Booking for Staff {
    fn book(&self) {
        println!("Staff booked a badminton court");
    }
}

impl Booking for Guest {
    fn book(&self) {
        println!("Guest booked a badminton court");
    }
}

impl Booking for Athlete {
    fn book(&self) {
        println!("Athlete booked a badminton court");
    }
}

fn main() {
    let student = Student;
    let staff = Staff;
    let guest = Guest;
    let athlete = Athlete;

    student.book();
    staff.book();
    guest.book();
    athlete.book();
}
