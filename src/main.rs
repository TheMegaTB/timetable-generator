extern crate time;
extern crate rustc_serialize;
use rustc_serialize::json;
mod structures;
use structures::*;

fn main() {
    let start_time = time::precise_time_ns();


    let mut school = School::new();
    school.lessons.push(Lesson { id: 1, amount: 10, max_per_day: 2 });
    school.lessons.push(Lesson { id: 2, amount: 7, max_per_day: 2 });
    school.lessons.push(Lesson { id: 3, amount: 4, max_per_day: 1 });
    school.lessons.push(Lesson { id: 4, amount: 9, max_per_day: 2 });


    school.new_teacher(&vec![1, 2, 3, 4]);
    school.new_teacher(&vec![2]);
    school.new_teacher(&vec![3, 4]);
    school.new_teacher(&vec![1, 2, 3]);
    school.new_teacher(&vec![2, 3, 4]);
    school.new_teacher(&vec![4]);
    school.new_teacher(&vec![1]);


    school.new_class();
    school.new_class();
    school.new_class();
    school.generate_timetables();


    let time = (time::precise_time_ns() - start_time) / 1000;

    school.print_timetables();
    println!("The whole calculation took {}μs", time);

    let encoded = json::encode(&school).unwrap();
    println!("{}", encoded);

}
