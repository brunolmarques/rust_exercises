use std::collections::BTreeMap;
use std::collections::BTreeSet;
pub struct School {
    roster: BTreeMap<u32, BTreeSet<String>>,
}
impl School {
    pub fn new() -> School {
        School {
            roster: BTreeMap::new(),
        }
    }
    pub fn add(&mut self, grade: u32, student: &str) {
        let students = self.roster.entry(grade).or_default();
        students.insert(student.to_string());
    }
    pub fn grades(&self) -> Vec<u32> {
        self.roster.keys().copied().collect()
    }
    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.roster
            .get(&grade)
            .unwrap_or(&BTreeSet::new())
            .iter()
            .cloned()
            .collect()
    }
}
impl Default for School {
    fn default() -> Self {
        Self::new()
    }
}

