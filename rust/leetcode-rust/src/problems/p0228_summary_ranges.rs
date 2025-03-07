pub fn interval_intersecion_with_point(s: &i32, e: &i32, t: &i32) -> bool {
    s >= t && t >= e
}

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub start: i32,
    pub end: i32,
}

impl Interval {
    pub fn new(start: i32, end: i32) -> Self {
        Self { start, end }
    }

    pub fn does_intersect(&self, test_point: &i32) -> bool {
        interval_intersecion_with_point(&self.start, &self.end, test_point)
    }

    pub fn to_string(&self) -> String {
        match self.start == self.end {
            true => format!("{:?}", &self.start),
            false => format!("{:?} -> {:?}", &self.start, &self.end),
        }
    }

    pub fn check_intersection_and_expand(&mut self, test_point: &i32) -> bool {
        match self.does_intersect(test_point) {
            true => true,
            false => match test_point - &self.end {
                1 => {
                    self.end += 1;
                    true
                }
                _ => false,
            },
        }
    }
}

pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    println!("{:?}", nums);
    let n = nums.len();
    match nums.first().cloned() {
        Some(first) => nums
            .into_iter()
            .map(|x| Some(x))
            .chain(std::iter::once(None))
            .enumerate()
            .scan(
                Interval::new(first.clone(), first.clone()),
                |mut acc, (i, x)| match x {
                    Some(x) => match acc.check_intersection_and_expand(&x) {
                        true => Some(None),
                        false => {
                            let retval = acc.clone();
                            *acc = Interval::new(x.clone(), x.clone());
                            Some(Some(retval))
                        }
                    },
                    None => Some(Some(*acc)),
                },
            )
            .flat_map(|x| x.map(|x| x.to_string()))
            .collect(),
        None => Vec::new(),
    }
}
