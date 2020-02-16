use std::io::{stdout, Write};
use std::cmp::max;
use nalgebra::{clamp, Vector2};
use crate::roster::{Roster, Slot};
use std::unreachable;

pub struct ViewState {
    pub roster: Roster,
    pub job_names: Vec<String>,
    pub person_names: Vec<String>,
    pub focused_field: Vector2<u32>,
}

impl ViewState {
    pub fn job_cols_needed(&self) -> u32 {
        self.roster.jobs.iter().map(|job| job.num_people).sum()
    }

    pub fn job_col_names(&self) -> Vec<String> {
        (0..self.job_cols_needed())
            .map(|col| self.col_job_name(col))
            .collect()
    }

    pub fn col_job_name(&self, mut col: u32) -> String {
        assert!(col < self.job_cols_needed());

        for (i, job) in self.roster.jobs.iter().enumerate() {
            if col < job.num_people {
                // Column falls under this job
                return self.job_names[i].clone()
            } else {
                col -= job.num_people;
            }
        }

        unreachable!();
    }

    pub fn max_col_width(&self) -> u32 {
        let job_person_max = max(
            self.job_names.iter().map(|job_name| job_name.len()).max().unwrap(),
            self.person_names.iter().map(|person_name| person_name.len()).max().unwrap()
        ) as u32;

        let week_max = max(
            2,
            (self.roster.num_weeks - 1).to_string().len() as u32,
        );

        max(job_person_max, week_max)
    }

    pub fn get_fields_for_week(&self, week: u32) -> Vec<Option<String>> {
        let mut res = vec![];

        for job in &self.roster.jobs {
            let slot = Slot {
                week,
                job: *job
            };
            let mut persons: Vec<Option<String>> =  self.roster.get_assigned(&slot).iter().map(|&person| Some(self.person_names[person as usize].clone())).collect();
            let empty_fields = job.num_people - (persons.len() as u32);
            res.append(&mut persons);
            for _ in 0..empty_fields {
                res.push(None)
            }
        }

        res
    }
}
