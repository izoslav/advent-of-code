use std::collections::BTreeMap;
use std::fs;

#[derive(Debug)]
enum EntryType {
    FallAsleep,
    WakeUp,
    BeginShift(usize),
}

#[derive(Debug)]
struct Entry {
    pub date: String,
    pub minute: usize,
    pub entry_type: EntryType,
}

impl Entry {
    pub fn new(log_entry: &str) -> Self {
        let date = log_entry[6..11].to_string();
        let minute = log_entry[15..17].parse::<usize>().unwrap();

        let rest: &str = &log_entry[19..];

        let entry_type = match rest {
            "falls asleep" => EntryType::FallAsleep,
            "wakes up" => EntryType::WakeUp,
            shift => {
                // ID length is between 3 and 4 digits
                let guard_id = shift[7..11].trim().parse::<usize>().unwrap();

                EntryType::BeginShift(guard_id)
            }
        };

        Entry {
            date,
            minute,
            entry_type,
        }
    }
}

trait Schedule {
    fn add_entry(&mut self, guard_id: usize, start: usize, end: usize);
}

impl Schedule for BTreeMap<usize, [usize; 60]> {
    fn add_entry(&mut self, guard_id: usize, start: usize, end: usize) {
        if !self.contains_key(&guard_id) {
            self.insert(guard_id, [0usize; 60usize]);
        }

        for i in start..end {
            self.get_mut(&guard_id).unwrap()[i] += 1;
        }
    }
}

fn main() {
    let logs: Vec<String> = fs::read_to_string("res/day04_1.txt")
        .unwrap()
        .lines()
        .into_iter()
        .map(|l| l.to_string())
        .collect();

    let mut sorted_logs = logs;
    sorted_logs.sort();

    let entries: Vec<Entry> = sorted_logs
        .into_iter()
        .map(|log| Entry::new(&log))
        .collect();

    let mut schedules = BTreeMap::new();
    schedules.insert(0usize, [0usize; 60]);

    let mut current_guard_id = 0usize;
    let mut current_start = 0usize;
    for entry in entries {
        match entry.entry_type {
            EntryType::BeginShift(guard_id) => current_guard_id = guard_id,
            EntryType::FallAsleep => current_start = entry.minute,
            EntryType::WakeUp => schedules.add_entry(current_guard_id, current_start, entry.minute),
        }
    }

    let (guard_id, _, chosen_minute) = schedules
        .iter()
        .map(|(&guard_id, &schedule)| {
            let minutes_asleep = schedule.into_iter().sum::<usize>();
            let max_asleep_minute = schedule.into_iter().max().unwrap();
            let chosen_minute = schedule
                .into_iter()
                .position(|e| e == max_asleep_minute)
                .unwrap();

            (guard_id, minutes_asleep, chosen_minute)
        })
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap();

    println!(
        "Part 1 answer: {} * {} = {}",
        guard_id,
        chosen_minute,
        guard_id * chosen_minute
    );

    let mut most_sleepy_guard = 0usize;
    let mut most_sleepy_minute = 0usize;
    let mut most_sleepy_index = 0usize;

    schedules.iter().for_each(|(&guard_id, &schedule)| {
        let max_sleep = schedule.into_iter().max().unwrap();

        if max_sleep > most_sleepy_minute {
            most_sleepy_minute = max_sleep;
            most_sleepy_guard = guard_id;
            most_sleepy_index = schedule.into_iter().position(|e| e == max_sleep).unwrap();
        }
    });

    println!(
        "Part 2 answer: {} * {} = {}",
        most_sleepy_guard,
        most_sleepy_index,
        most_sleepy_guard * &most_sleepy_index
    );
}
