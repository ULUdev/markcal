/// a Mark
#[derive(Ord, Clone, PartialEq, Eq)]
pub struct Mark {
    subject: String,
    value: i32,
    class: String,
    comment: Option<String>,
}

impl Mark {
    pub fn new(subject: &str, value: i32, class: &str, comment: Option<String>) -> Mark {
        Mark {
            subject: subject.to_owned(),
            value,
            class: class.to_owned(),
            comment,
        }
    }

    pub fn to_string(&self) -> String {
        match &self.comment {
            Some(n) => format!("{},{},{},{}", self.subject, self.value, self.class, n),
            None => format!("{},{},{}", self.subject, self.value, self.class),
        }
    }

    pub fn get_comment(&self) -> Option<&String> {
        self.comment.as_ref()
    }

    pub fn get_subject(&self) -> &str {
        &self.subject
    }

    pub fn get_class(&self) -> &str {
        &self.class
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }
}

impl PartialOrd for Mark {
    fn lt(&self, other: &Self) -> bool {
        self.value < other.value
    }
    fn le(&self, other: &Self) -> bool {
        self.value <= other.value
    }
    fn gt(&self, other: &Self) -> bool {
        self.value > other.value
    }
    fn ge(&self, other: &Self) -> bool {
        self.value >= other.value
    }
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.lt(other) {
            Some(std::cmp::Ordering::Less)
        } else if self.gt(other) {
            Some(std::cmp::Ordering::Greater)
        } else {
            Some(std::cmp::Ordering::Equal)
        }
    }
}

fn parse_line(line: &str) -> Option<Mark> {
    if line.matches(',').count() < 1 {
        return None;
    }
    let mut fields = line.split(',');
    let sub = fields.next()?;
    let mark = fields.next()?;
    let class = fields.next()?;
    let leftover: String = fields.collect::<Vec<&str>>().join(",");
    let comment = match leftover.is_empty() {
        true => None,
        false => Some(leftover),
    };
    Some(Mark::new(sub, mark.parse().ok()?, class, comment))
}

pub fn parse_csv_dat(file: &str) -> Option<Vec<Mark>> {
    std::fs::read_to_string(file)
        .ok()?
        .lines()
        .map(parse_line)
        .collect()
}

pub fn to_csv(elms: &[Mark]) -> String {
    elms.iter()
        .map(Mark::to_string)
        .collect::<Vec<String>>()
        .join("\n")
}
