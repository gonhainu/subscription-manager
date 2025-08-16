use crate::error::{DomainError, DomainResult};

#[derive(Debug, Clone, PartialEq)]
pub struct Year(i32);

impl Year {
    pub fn new(year: i32) -> DomainResult<Self> {
        if !(1900..=2200).contains(&year) {
            return Err(DomainError::InvalidYear(year));
        }
        Ok(Self(year))
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Month(u8);

impl Month {
    pub fn new(month: u8) -> DomainResult<Self> {
        if !(1..=12).contains(&month) {
            return Err(DomainError::InvalidMonth(month));
        }
        Ok(Self(month))
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct YearMonth {
    year: Year,
    month: Month,
}

impl YearMonth {
    pub fn new(year: Year, month: Month) -> Self {
        Self { year, month }
    }

    pub fn from_values(year: i32, month: u8) -> DomainResult<Self> {
        let year = Year::new(year)?;
        let month = Month::new(month)?;
        Ok(Self::new(year, month))
    }

    pub fn year(&self) -> &Year {
        &self.year
    }

    pub fn month(&self) -> &Month {
        &self.month
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_year() {
        assert!(Year::new(2020).is_ok());
        assert!(Year::new(1800).is_err());
        assert!(Year::new(2300).is_err());
    }

    #[test]
    fn test_month() {
        assert!(Month::new(5).is_ok());
        assert!(Month::new(0).is_err());
        assert!(Month::new(13).is_err());
    }

    #[test]
    fn test_year_month() {
        assert!(YearMonth::from_values(2020, 5).is_ok());
        assert!(YearMonth::from_values(1800, 5).is_err());
        assert!(YearMonth::from_values(2020, 0).is_err());
        assert!(YearMonth::from_values(2020, 13).is_err());
    }
}
