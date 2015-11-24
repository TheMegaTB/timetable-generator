extern crate time;
extern crate rustc_serialize;
use rustc_serialize::json;

const DAY_LENGTH: usize = 10;
const SHORT_DAY_LENGTH: usize = 6;
const PREALLOC_CLASSES: usize = 1;
const PREALLOC_LESSONS: usize = 2;
const PREALLOC_TEACHERS: usize = 2;
type ID = u16;
type Day = Vec<ID>;

#[derive(RustcDecodable, RustcEncodable)]
struct Lesson {
    id: ID,
    amount: u8,
    max_per_day: u8
}

#[derive(RustcDecodable, RustcEncodable)]
struct Teacher {
    id: ID, // ID of the teacher
    subjects: Vec<ID>, // Subjects of the teacher referring to Lesson ID's
    timetable: Vec<Day> // Timetable containing ID's of classes
}

impl Teacher {
    fn blocked_percentage(&self) -> u32 {
        let mut total = 0;
        let mut blocked = 0;
        for day in self.timetable.iter() {
            for lesson in 0..day.len() {
                total += 1;
                if day[lesson] != 0 {
                    blocked += 1;
                }
            }
        }
        blocked * 100 / total // Percentage between 0 and 100
    }
}

#[derive(RustcDecodable, RustcEncodable)]
struct Class {
    id: ID,
    week: Vec<Day>, // Week containing lesson ID's
    teachers: Vec<ID> // Teachers teaching this class
}

impl Class {
    fn new(lesson_amount: usize, id: ID) -> Class {
        Class {
            id: id,
            week: vec![vec![0; DAY_LENGTH]; 5],
            teachers: vec![0; lesson_amount+1]
        }
    }

    pub fn add_lesson(&mut self, l: &Lesson, t: &mut Vec<Teacher>) {
        let mut long_day = false;
        loop {
            // Step 1 - Assign a teacher if no one exists
            if self.teachers[l.id as usize] == 0 {
                let mut last_percentage = 100;
                for teacher in t.iter() {
                    if teacher.subjects.iter().any(|subj| subj == &l.id) {
                        let perc = teacher.blocked_percentage();
                        if perc < last_percentage {
                            self.teachers[l.id as usize] = teacher.id;
                            last_percentage = perc;
                        }
                    }
                }
                if last_percentage == 100 { panic!("NO TEACHER FOUND FOR SUBJECT!") }
            }

            let ref mut teacher = t[self.teachers[l.id as usize] as usize - 1];

            // Step 2 - Fill the lessons in according to the following criteria
            let mut remaining = l.amount;
            for day in 0..self.week.len() {
                let mut current_day = 0;
                let iterator = if long_day { 0..self.week[day].len() } else { 0..SHORT_DAY_LENGTH+1 };
                for lesson in iterator {
                    if remaining == 0 || current_day >= l.max_per_day {
                        break;
                    } else if self.week[day][lesson] == 0 && teacher.timetable[day][lesson] == 0 {
                        self.week[day][lesson] = l.id.clone();
                        teacher.timetable[day][lesson] = self.id.clone();
                        remaining -= 1;
                        current_day += 1;
                    }
                }
            }

            if remaining == 0 {
                break;
            } else if !long_day {
                long_day = true;
            } else { panic!("Couldn't fit in all lessons!") }
        }
    }

    pub fn add_lessons(&mut self, lessons: &Vec<Lesson>, teachers: &mut Vec<Teacher>) {
        for lesson in lessons.iter() {
            self.add_lesson(lesson, teachers);
        }
    }

    pub fn print(&self) {
        println!("");
        println!("\\\\\\\\TIMETABLE////");
        println!("    M  T  W  T  F");
        println!("-----------------");
        for lesson in 0..DAY_LENGTH {
            if lesson == 7 { println!("- - - - - - - - -") }
            print!("{} | ", lesson);
            for day in 0..5 {
                print!("{}  ", self.week[day][lesson]);
            }
            println!("");
        }
        println!("");
        println!("\\\\\\\\TEACHERS////");
        for subject in 0..self.teachers.len() {
            if subject != 0 { println!("Teacher {} teaches subject {}.", self.teachers[subject], subject) }
        }
        println!("");
    }
}

#[derive(RustcDecodable, RustcEncodable)]
struct School {
    classes: Vec<Class>,
    teachers: Vec<Teacher>,
    lessons: Vec<Lesson>
}

impl School {
    fn new() -> School {
        School{
            classes: Vec::with_capacity(PREALLOC_CLASSES),
            teachers: Vec::with_capacity(PREALLOC_TEACHERS),
            lessons: Vec::with_capacity(PREALLOC_LESSONS)
        }
    }

    fn new_class(&mut self) -> usize {
        let id = self.classes.len();
        self.classes.push(Class::new(self.lessons.len(), (id+1) as u16));
        id
    }

    fn generate_timetables(&mut self) {
        for class in self.classes.iter_mut() {
            class.add_lessons(&self.lessons, &mut self.teachers);
        }
    }

    fn print_timetables(&self) {
        for class in self.classes.iter() {
            println!(" --------------- CLASS {} --------------- ", class.id);
            class.print();
            println!("");
        }
    }
}

fn main() {
    let start_time = time::precise_time_ns();


    let mut school = School::new();
    school.lessons.push(Lesson { id: 1, amount: 10, max_per_day: 2 });
    school.lessons.push(Lesson { id: 2, amount: 7, max_per_day: 2 });
    school.lessons.push(Lesson { id: 3, amount: 4, max_per_day: 1 });
    school.lessons.push(Lesson { id: 4, amount: 9, max_per_day: 2 });


    school.teachers.push(Teacher { id: 1, subjects: vec![1, 2, 3, 4], timetable: vec![vec![0; DAY_LENGTH]; 5] });
    school.teachers.push(Teacher { id: 2, subjects: vec![2], timetable: vec![vec![0; DAY_LENGTH]; 5] });
    school.teachers.push(Teacher { id: 3, subjects: vec![3, 4], timetable: vec![vec![0; DAY_LENGTH]; 5] });
    school.teachers.push(Teacher { id: 4, subjects: vec![1, 2, 3], timetable: vec![vec![0; DAY_LENGTH]; 5] });
    school.teachers.push(Teacher { id: 5, subjects: vec![2, 3, 4], timetable: vec![vec![0; DAY_LENGTH]; 5] });
    school.teachers.push(Teacher { id: 6, subjects: vec![4], timetable: vec![vec![0; DAY_LENGTH]; 5] });
    school.teachers.push(Teacher { id: 7, subjects: vec![1], timetable: vec![vec![0; DAY_LENGTH]; 5] });


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
