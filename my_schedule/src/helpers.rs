#[derive(Debug, Clone, Default)]
pub struct Section {
    /// [day][chunk] where chunk 0 = slots 0-63, 1 = 64-127, 2 = 128-191
    /// Each bit = 5 min slot starting at 8am
    /// Slot 0 = 8:00am, slot 1 = 8:05am, ... slot 167 = 9:55pm
    pub classtimes: [[u64; 3]; 5],
    pub course: String,
    pub section: String,
    pub open_seats: u32,
    pub waitlist: u32,
    pub holdfile: Option<u32>,
    pub building: Option<Building>,
}

impl PartialEq for Section {
    fn eq(&self, _other: &Self) -> bool {
        self.course == _other.course && self.section == _other.section
    }
}

impl Section {
    ///Lower numbers correspond to higher value sections
    pub fn seat_score(&self) -> i32 {
        -(self.open_seats as i32)
            + (self.waitlist as i32)
            + (self.holdfile.unwrap_or(0) as i32)        
    }
}

#[derive(Debug, Clone, Default)]
//Walking/distance conflict testing will be added in the future
pub struct Building {
    _lat: f32,
    _lon: f32,
}

///Checks if the meeting times of two given sections overlap
pub fn is_conflict(section1: usize, section2: usize, key: &[Section]) -> bool {
    key[section1]
        .classtimes
        .iter()
        .zip(key[section2].classtimes.iter())
        .any(|(day1, day2)| day1.iter().zip(day2.iter()).any(|(a, b)| a & b != 0))
}

/// This replicates the data that the frontend gets from the database
/// aka an array of arrays, with each item representing a course and section respectively
pub fn test_fetch() -> Box<[Box<[Section]>]> {
    let math_240 = Box::new([Section::default(), Section::default(), Section::default()]);
    let engl_101 = Box::new([Section::default(), Section::default(), Section::default()]);
    let phys_260 = Box::new([Section::default(), Section::default(), Section::default()]);

    return Box::new([math_240, engl_101, phys_260]);
}

///Returns the index key and the desired courses
/// This logic will happen on the frontend
pub fn test_data() -> (Box<[Section]>, Box<[Box<[usize]>]>) {
    let mut data = test_fetch();

    //sort courses by number of sections to speed up backtracking
    data.sort_unstable_by_key(|sections| sections.len());

    //priorize high quality sections by sorting by (-openseats + waitlist + holdfile)
    for course in data.iter_mut() {
        course.sort_unstable_by_key(|section|section.seat_score());
    }

    //generate key
    let key: Box<[Section]> = data.iter().flatten().cloned().collect();

    //generate course list via indexes
    let courses: Box<[Box<[usize]>]> = data
        .iter()
        .map(|course| {
            course
                .iter()
                .map(|section| key.iter().position(|s| s == section).unwrap())
                .collect()
        })
        .collect();

    return (key, courses);
}
