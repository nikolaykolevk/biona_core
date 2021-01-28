use chrono::naive::NaiveDate;
use chrono::Local;

//impl default methood for the NaivaDate struct (used as a date type)
pub trait DefaultDate {
    fn default() -> Self;
}

impl DefaultDate for NaiveDate {
    fn default() -> NaiveDate {
        Local::now().naive_local().date()
    }
}