use crate::{DomainError, DomainResult};

pub trait Item: HasTitle + Estimatable + Assingable + Send {}

pub trait Estimatable {
    fn point(&self) -> Option<StoryPoint>;
    fn estimate(&mut self, point: StoryPoint);
}

pub trait HasTitle {
    fn title(&self) -> &str;
}

pub trait Assingable {
    fn assignee(&self) -> Option<&str>;
    fn assign(&mut self, assignee: &str);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct StoryPoint(u8);

impl StoryPoint {
    pub fn new(point: u8) -> DomainResult<Self> {
        match point {
            1 | 2 | 3 | 5 | 8 => Ok(Self(point)),
            _ => Err(DomainError::InvalidStoryPoint),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(
        input,
        expected,
        case(0, false),
        case(1, true),
        case(2, true),
        case(3, true),
        case(4, false),
        case(5, true),
        case(6, false),
        case(7, false),
        case(8, true),
        case(9, false)
    )]
    fn story_point(input: u8, expected: bool) {
        let point = StoryPoint::new(input);
        assert_eq!(point.is_ok(), expected);
    }
}
