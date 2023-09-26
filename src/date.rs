use chrono::NaiveDate;

pub trait DateString {
    fn try_parse_date(&self) -> Option<NaiveDate>;
    fn parse_date(&self) -> NaiveDate;
}

impl DateString for Option<String> {
    fn try_parse_date(&self) -> Option<NaiveDate> {
        self.as_ref()
            .and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
    }
    fn parse_date(&self) -> NaiveDate {
        self.as_ref()
            .and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
            .unwrap_or_else(|| panic!("Unknown date format: {:?}", self))
    }
}

impl DateString for str {
    fn try_parse_date(&self) -> Option<NaiveDate> {
        NaiveDate::parse_from_str(self, "%Y-%m-%d").ok()
    }
    fn parse_date(&self) -> NaiveDate {
        NaiveDate::parse_from_str(self, "%Y-%m-%d")
            .unwrap_or_else(|_| panic!("Unknown date format: {:?}", self))
    }
}
