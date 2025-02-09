use chrono::{Datelike, Duration as ChronoDuration, NaiveDate};


fn parse_date(date: &str) -> (i32, u32) {
    let parts: Vec<&str> = date.split('-').collect();
    (parts[0].parse().unwrap(), parts[1].parse().unwrap())
}


pub fn retire_time(time: &str, tp: &str) -> String {
    let (birth_year, birth_month) = parse_date(time);
    let birth_date = NaiveDate::from_ymd_opt(birth_year, birth_month, 1).expect("Invalid date");

    let (base_retirement_age, delay_months) = match tp {
        "原法定退休年龄55周岁女职工" => {
            if birth_year < 1972 {
                (55.0, 0)
            } else if birth_year < 2000 {
                (55.0 + (birth_year - 1972) as f64 / 3.0, ((birth_year - 1972) * 12 / 3) as i32)
            } else {
                (58.0, 36)
            }
        }
        "原法定退休年龄50周岁女职工" => {
            if birth_year < 1972 {
                (50.0, 0)
            } else if birth_year < 2000 {
                (50.0 + (birth_year - 1972) as f64 / 3.0, ((birth_year - 1972) * 12 / 3) as i32)
            } else {
                (55.0, 60)
            }
        }
        "男职工" => {
            if birth_year < 2000 {
                (60.0, 0)
            } else {
                (63.0, 36)
            }
        }
        _ => (60.0, 0), // 默认情况
    };

    let retirement_age = base_retirement_age + delay_months as f64 / 12.0;
    let retirement_date = birth_date + ChronoDuration::days((retirement_age * 365.25) as i64);
    let retirement_year = retirement_date.year();
    let retirement_month = retirement_date.month();

    format!("{:04}-{:02},{:.2},{}", retirement_year, retirement_month, retirement_age, delay_months)
}
