use std::collections::HashMap;


pub struct School {
    grades: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School { grades: HashMap::new() }
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut ret = self.grades.keys().map(|&k| k).collect::<Vec<u32>>();
        ret.sort();
        ret
    }

    pub fn add(&mut self, grade: u32, name: &str) {
        let students = self.grades.entry(grade).or_insert(Vec::new());
        students.push(name.to_owned());
        students.sort();
    }

    pub fn grade(&self, grade: u32) -> Option<&Vec<String>> {
        self.grades.get(&grade)
    }
}
