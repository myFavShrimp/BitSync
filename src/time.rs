use chrono::{DateTime, Datelike, Timelike, Utc};
use time::{Date, Month, PrimitiveDateTime, Time};

pub fn datetime_chrono_to_time(datetime: DateTime<Utc>) -> PrimitiveDateTime {
    let chrono_date = datetime.date_naive();
    let chrono_time = datetime.time();

    let time_date = Date::from_calendar_date(
        chrono_date.year_ce().1 as i32,
        Month::try_from(chrono_date.month0() as u8 + 1).expect("month is valid"),
        chrono_date.day0() as u8 + 1,
    )
    .expect("date is valid");

    let time_time = Time::from_hms(
        chrono_time.hour() as u8,
        chrono_time.minute() as u8,
        chrono_time.second() as u8,
    )
    .expect("time is valid");

    PrimitiveDateTime::new(time_date, time_time)
}
