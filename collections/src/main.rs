#[derive(PartialEq, Eq)]
struct Student{
    name: String,
}

fn main(){
    let mut students = vec![Student{name: String::from("Akshat Jaimini")}, Student{name: String::from("Akshat 2")}];
    students.push(Student{name:String::from("Akshat Jaimini")});
    // Getting Elements
    assert!(
        &students[0]
            == &Student{
                name: "Akshat Jaimini".to_string()
            }
    );
    assert!(
        students.get(0)
            == Some(&Student{name: "Akshat Jaimini".to_string()})
    );
    assert!(
        students.get(100000) == None
    );
    // Traversal
    for student in students.iter(){
        println!("{}", student.name);
    }

    use std::collections::HashMap;

    let mut enrolment = HashMap::new();
    enrolment.insert("Biology", students);

    let bio_studs = enrolment.get("Biology");
    let students = enrolment.remove("biology");
}