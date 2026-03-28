mod helpers;
use helpers::*;

pub fn get_potential_schedules() -> Vec<Vec<usize>> {
    //data obtained from frontend
    let (key, courses) = test_data();

    let mut results: Vec<Vec<usize>> = Vec::new();
    let mut current: Vec<usize> = Vec::new();
    let mut checks: u64 = 0;

    backtrack(0, &mut current, &courses, &mut results, &mut checks, &key);

    //sort results by seat quality
    results.sort_unstable_by_key(|schedule| {
        schedule
            .iter()
            .map(|id| {
                let section = &key[*id];
                section.seat_score()
            })
            .sum::<i32>()
    });

    return results;
}

///backtracking function for get_potential_schedules
fn backtrack(
    course_idx: usize,
    current_schedule: &mut Vec<usize>,
    courses: &[Box<[usize]>],
    results: &mut Vec<Vec<usize>>,
    checks: &mut u64,
    key: &[Section],
) {
    if course_idx == courses.len() {
        results.push(current_schedule.clone());
        return;
    }

    //keep computation time down
    if *checks >= 5000 {
        return;
    }

    for new_section in &courses[course_idx] {
        let mut has_conflict: bool = false;
        for existing_section in current_schedule.iter() {
            *checks += 1;

            if is_conflict(*new_section, *existing_section, key) {
                has_conflict = true;
                break;
            }
        }

        if !has_conflict {
            current_schedule.push(*new_section);
            backtrack(
                course_idx + 1,
                current_schedule,
                courses,
                results,
                checks,
                key,
            );
            current_schedule.pop();
        }
    }
}
