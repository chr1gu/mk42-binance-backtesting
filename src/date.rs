use chrono::NaiveDate;

pub trait DateString {
    fn as_date(&self) -> Option<NaiveDate>;
}

impl DateString for Option<String> {
    fn as_date(&self) -> Option<NaiveDate> {
        self.as_ref().and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
    }
}

impl DateString for str {
    fn as_date(&self) -> Option<NaiveDate> {
        NaiveDate::parse_from_str(self, "%Y-%m-%d").ok()
    }
}