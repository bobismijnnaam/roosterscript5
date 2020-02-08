use std::collections::{HashMap};

#[derive(Clone, Hash, Copy, Eq, Ord, PartialOrd, PartialEq, Debug)]
pub struct Job {
    pub id: JobID,
    pub num_people: u32,
    pub period: u32,
}

impl Job {
    pub fn new(id: JobID, num_people: u32, period: u32) -> Job {
        Job {
            id,
            num_people,
            period
        }
    }
}

pub type Person = u32;
pub type JobID = u32;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Roster {
    pub jobs: Vec<Job>,
    pub num_weeks: u32,
    pub slots: HashMap<Slot, Vec<Person>>,
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialOrd, PartialEq, Hash)]
pub struct Slot {
    pub week: u32,
    pub job: Job,
}

struct Field {
    slot: Slot,
    index: u32,
}

impl Roster {
    pub(crate) fn new(mut jobs: Vec<Job>, num_weeks: u32) -> Roster {
        let roster = Roster {
            jobs,
            num_weeks,
            slots: HashMap::new(),
        };

        roster
    }

    pub fn get_assigned(&self, slot: &Slot) -> Vec<Person> {
//        assert!(self.is_valid_slot(slot));

        if slot.week >= self.num_weeks {
            return vec![];
        }

        self.slots.get(slot).cloned().unwrap_or_default()
    }

    fn is_valid_slot(&self, slot: &Slot) -> bool {
        self.jobs.contains(&slot.job) && (slot.week % slot.job.period) == 0
    }

    fn get_previous(&self, slot: &Slot) -> Option<Slot> {
        if slot.job.period  > slot.week {
            None
        } else {
            Some(Slot {
                week: slot.week - slot.job.period,
                .. *slot
            })
        }
    }

    fn get_next(&self, slot: &Slot) -> Option<Slot> {
        if (slot.week + slot.job.period) >= self.num_weeks {
            None
        } else {
            Some(Slot {
                week: slot.week + slot.job.period,
                .. *slot
            })
        }
    }

    fn num_people_in_slot(&self, slot: &Slot) -> u32 {
        assert!(self.is_valid_slot(slot));

        if let Some(persons) = self.slots.get(slot) {
            persons.len() as u32
        } else {
            0
        }
    }

    fn is_slot_open(&self, slot: &Slot) -> bool {
        self.num_people_in_slot(slot) < slot.job.num_people
    }

    fn get_open_slot(&self) -> Option<Slot> {
        for week in 0..self.num_weeks {
            for job in &self.jobs {
                let slot = Slot {
                    week,
                    job: *job
                };

                if self.is_valid_slot(&slot) && self.is_slot_open(&slot) {
                    return Some(slot);
                }
            }
        }

        None
    }

    pub fn get_assigned_in_week(&self, week: u32) -> Vec<Person> {
        self.jobs
            .iter()
            .filter(|job| (week % job.period) == 0)
            .map(|job| self.get_assigned(&Slot { week, job: *job }))
            .flatten()
            .collect()
    }

    fn fits(&self, slot: &Slot, person: Person) -> bool {
        assert!(self.is_valid_slot(slot));

        let previous_assigned = self.get_previous(slot)
            .map(|slot| self.get_assigned(&slot))
            .unwrap_or(vec![]);

        let next_assigned = self.get_next(slot)
            .map(|slot| self.get_assigned(&slot))
            .unwrap_or(vec![]);

        let assigned_in_week = self.get_assigned_in_week(slot.week);

        !previous_assigned.contains(&person)
            && !next_assigned.contains(&person)
            && !assigned_in_week.contains(&person)
    }

    fn append(&mut self, person: Person) {
        let slot = self.get_open_slot().unwrap();
        self.slots.entry(slot).or_default().push(person);
    }

    fn append_to_slot(&mut self, person: Person, slot: &Slot) {
        assert!(self.is_slot_open(slot));
        self.slots.entry(*slot).or_default().push(person);
    }

    fn get_assigned_in_field(&self, field: &Field) -> Option<Person> {
        let persons = self.get_assigned(&field.slot);
        if (field.index as usize) < persons.len() {
            Some(persons[field.index as usize])
        } else {
            None
        }
    }

//    fn is_streak_field(&self, field: &Field) -> bool {
//        if field.slot.week == 0 {
//            false
//        } else if let Some(person) = self.get_assigned_in_field(field) {
//            self.get_assigned_in_week(field.slot.week - 1).contains(&person)
//        } else {
//            false
//        }
//    }

    fn get_streakers(&self, slot: &Slot) -> Vec<Person> {
        let previous_week = self.get_previous(slot)
            .map(|previous_slot| self.get_assigned_in_week(previous_slot.week))
            .unwrap_or_default();

        self.get_assigned(slot)
            .iter()
            .filter_map(|person| if previous_week.contains(person) {
                Some(*person)
            } else {
                None
            })
            .collect()
    }

//    fn count_streak_slots(&self) -> u32 {
//        self.all_slots().iter().filter(|slot| self.is_streak_slot(*slot)).count() as u32
//    }

//    fn set_slot(&mut self, slot: &Slot, person: Option<Person>) {
//        self.slots[slot.week as usize][slot.job_index as usize] = Some(person);
//    }

//    fn can_swap(&mut self, a: &Slot, b: &Slot) -> bool {
//        let person_a = self.get_slot(a);
//        let person_b = self.get_slot(b);
//
//        self.set_slot(a, None);
//        self.set_slot(b, None);
//
//        let person_a_fits = person_a.map(|person| self.fits(b, person)).unwrap_or(true);
//        let person_b_fits = person_b.map(|person| self.fits(a, person)).unwrap_or(true);
//
//        self.set_slot(a, person_a);
//        self.set_slot(b, person_b);
//
//        person_a_fits && person_b_fits
//    }
//
//    fn swap(&mut self, a: &Slot, b: &Slot) {
//        let person_a = self.get_slot(a);
//        let person_b = self.get_slot(b);
//
//        self.set_slot(a, person_b);
//        self.set_slot(b, person_a);
//    }

    fn pretty_str(&self, job_to_str: HashMap<JobID, String>, person_to_str: HashMap<Person, String>) -> String {
        let mut res = "".to_owned();

        let RED = "\033[31m";
        let STOP = "\033[0m";

        for week in 0..self.num_weeks {

        }

        res
    }
}
