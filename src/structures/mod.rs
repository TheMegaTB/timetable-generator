const DAY_LENGTH: usize = 10;
const SHORT_DAY_LENGTH: usize = 6;
const PREALLOC_CLASSES: usize = 1;
const PREALLOC_LESSONS: usize = 2;
const PREALLOC_TEACHERS: usize = 2;
pub type ID = usize;
pub type Day = Vec<ID>;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Lesson {
    pub id: ID, // Numeric ID of the lesson to be used in the timetable
    pub amount: u8, // Amount of this lesson per Week
    pub max_per_day: u8 // TODO: DEPRECATED Max amount of this lesson per day (unneccesary -> should be replaced with double_lesson bool)
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Teacher {
    id: ID, // Numeric ID of the teacher
    subjects: Vec<ID>, // Subjects of the teacher referring to Lesson ID's
    timetable: Vec<Day> // Timetable containing ID's of classes
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Class {
    id: ID, // Numeric ID of the class
    week: Vec<Day>, // Week containing lesson ID's
    teachers: Vec<ID> // Teachers teaching this class
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct School {
    pub classes: Vec<Class>,
    pub teachers: Vec<Teacher>,
    pub lessons: Vec<Lesson>
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

    fn print(&self) {
        for lesson in 0..DAY_LENGTH {
            if lesson == 7 { println!("- - - - - - - - -") }
            print!("{} | ", lesson);
            for day in 0..5 {
                print!("{}  ", self.timetable[day][lesson]);
            }
            println!("");
        }
    }
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

            let ref mut teacher = t[self.teachers[l.id] - 1];

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

impl School {
    pub fn new() -> School {
        School{
            classes: Vec::with_capacity(PREALLOC_CLASSES),
            teachers: Vec::with_capacity(PREALLOC_TEACHERS),
            lessons: Vec::with_capacity(PREALLOC_LESSONS)
        }
    }

    pub fn new_teacher(&mut self, subjects: &Vec<ID>) -> usize {
        let id = self.teachers.len() + 1;
        self.teachers.push(Teacher {
            id: id as ID,
            subjects: subjects.clone(),
            timetable: vec![vec![0; DAY_LENGTH]; 5]
        });
        id
    }

    pub fn new_class(&mut self) -> usize {
        let id = self.classes.len();
        self.classes.push(Class::new(self.lessons.len(), (id+1)));
        id
    }

    pub fn generate_timetables(&mut self) {
        for class in self.classes.iter_mut() {
            class.add_lessons(&self.lessons, &mut self.teachers);
        }
    }

    pub fn print_timetables(&self) {
        for class in self.classes.iter() {
            println!(" --------------- CLASS {} --------------- ", class.id);
            class.print();
            println!("");
        }
        for teacher in self.teachers.iter() {
            println!(" --------------- TEACHER {} --------------- ", teacher.id);
            teacher.print();
            println!("");
        }
    }
}
