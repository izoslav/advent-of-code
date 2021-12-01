use std::collections::{BTreeMap, BTreeSet};
use std::fs;

fn main() {
    let mut steps: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

    fs::read_to_string("res/day07_1.txt")
        .unwrap()
        .lines()
        .for_each(|l| {
            let words: Vec<&str> = l.split_ascii_whitespace().collect();

            let prereq = words[1].to_string();
            let step = words[7].to_string();

            steps.entry(prereq.clone()).or_default();

            let entry = steps.entry(step).or_default();
            entry.insert(prereq);
        });

    {
        // part 1
        let mut steps = steps.clone();
        let mut output = String::new();
        let mut current_steps: BTreeSet<String> = steps
            .iter()
            .filter(|(_, requirements)| requirements.is_empty())
            .map(|(step, _)| step.to_owned())
            .collect();

        while !current_steps.is_empty() {
            let current_step = current_steps.iter().take(1).next().unwrap().to_owned();

            current_steps.remove(&current_step);
            steps.remove_entry(&current_step);

            steps.iter_mut().for_each(|(_, requirements)| {
                requirements.remove(&current_step);
            });

            steps
                .iter()
                .filter(|(_, requirements)| requirements.is_empty())
                .for_each(|(step, _)| {
                    current_steps.insert(step.clone());
                });

            output += &current_step[..];
        }

        println!("Part 1 answer: {}", output);
    }

    {
        // part 2
        let mut steps = steps.clone();
        let mut workers: Vec<(usize, Option<String>)> = vec![(0, None); 5];
        let delay = 60usize;

        while !steps.is_empty() {
            let t = workers
                .iter()
                .filter(|&(_, step)| step.is_some())
                .min_by(|&(a, _), &(b, _)| a.cmp(&b))
                .map(|(wt, _)| wt)
                .unwrap_or(&0)
                .to_owned();

            for worker in &mut workers {
                if worker.0 <= t {
                    if let Some(step) = &worker.1 {
                        steps.iter_mut().for_each(|(_, requirements)| {
                            requirements.remove(step);
                        });

                        worker.1 = None;
                    }
                }
            }

            let free_workers_iter = workers.iter().filter(|&(wt, _)| *wt <= t);

            let free_workers_count = free_workers_iter.clone().count();

            let current_steps: Vec<String> = steps
                .iter_mut()
                .filter(|(_, requirements)| requirements.is_empty())
                .take(free_workers_count)
                .map(|(step, _)| step.to_owned())
                .collect();

            let mut current_steps_iter = current_steps.iter();

            workers
                .iter_mut()
                .filter(|(wt, _)| *wt <= t)
                .for_each(|entry| {
                    if let Some(current_step) = current_steps_iter.next() {
                        steps.remove(current_step);

                        let current_step = current_step.clone();
                        let current_step_char = current_step.chars().next().unwrap();
                        let current_step_value = current_step_char as usize - 'A' as usize + 1;

                        entry.0 = t + delay + current_step_value;
                        entry.1 = Some(current_step);
                    }
                });
        }

        let total_time = workers
            .into_iter()
            .max_by(|&(a, _), &(b, _)| a.cmp(&b))
            .unwrap()
            .0;

        println!("Part 2 answer: {:?}", total_time);
    }
}
