mod student;
use student::{CourseGrade, Grade, Student, StudentDatabase};

fn main() {
    println!("Student Management System\n");

    let mut db = StudentDatabase::new();

    let mut alice = Student::new(
        String::from("S001"),
        String::from("Alice Johnson"),
        String::from("alice@example.com"),
    );
    alice.add_grade(CourseGrade::new(
        String::from("IS4010"),
        String::from("App Dev with AI"),
        3,
        Grade::A,
    ));
    alice.add_grade(CourseGrade::new(
        String::from("IS3050"),
        String::from("Database Design"),
        3,
        Grade::B,
    ));
    alice.add_grade(CourseGrade::new(
        String::from("IS2000"),
        String::from("Intro to IS"),
        3,
        Grade::A,
    ));

    let mut bob = Student::new(
        String::from("S002"),
        String::from("Bob Smith"),
        String::from("bob@example.com"),
    );
    bob.add_grade(CourseGrade::new(
        String::from("IS3050"),
        String::from("Database Design"),
        3,
        Grade::B,
    ));

    match db.add_student(alice) {
        Ok(()) => println!("Added Alice"),
        Err(e) => println!("Error: {}", e),
    }
    match db.add_student(bob) {
        Ok(()) => println!("Added Bob"),
        Err(e) => println!("Error: {}", e),
    }

    println!("\nDatabase Statistics:");
    println!("Total students: {}", db.student_count());
    println!("Average GPA: {:.2}", db.average_gpa());

    println!("\nAll Students:");
    for student in db.list_students() {
        println!(
            "  {} - {} (GPA: {:.2})",
            student.id,
            student.name,
            student.calculate_gpa()
        );
    }

    if let Some(student) = db.find_student("S001") {
        println!("\nFound: {}", student.name);
        println!("  Credits: {}", student.credits_earned);
        println!("  GPA: {:.2}", student.calculate_gpa());
        println!("  Standing: {}", student.class_standing());
    }
}
