use chrono::{Datelike, NaiveDate};


fn parse_date(date: &str) -> NaiveDate {
    NaiveDate::parse_from_str(date, "%Y-%m").unwrap()
}

pub fn retire_time(time: &str, tp: &str) -> String {
    let birth_date = parse_date(time);
    let birth_year = birth_date.year();

    let (base_retirement_age, delay_months) = match tp {
        "原法定退休年龄55周岁女职工" => {
            if birth_year <= 1963 {
                (55, 0)
            } else if birth_year <= 1965 {
                (55, (birth_year - 1963) * 4)
            } else if birth_year <= 2000 {
                (55, 12 + (birth_year - 1965) * 2)
            } else {
                (58, 36)
            }
        }
        "原法定退休年龄50周岁女职工" => {
            if birth_year <= 1963 {
                (50, 0)
            } else if birth_year <= 2000 {
                (50, (birth_year - 1963) * 2)
            } else {
                (55, 60)
            }
        }
        "男职工" => {
            if birth_year <= 1963 {
                (60, 0)
            } else if birth_year <= 1965 {
                (60, (birth_year - 1963) * 3)
            } else if birth_year <= 2000 {
                (60, 6 + (birth_year - 1965) * 2)
            } else {
                (63, 36)
            }
        }
        _ => (60, 0), // 默认情况
    };

    let retirement_date = birth_date + chrono::Months::new((base_retirement_age * 12 + delay_months) as u32);
    let retirement_age = base_retirement_age as f64 + delay_months as f64 / 12.0;

    format!("{},{:.2},{}",
            retirement_date.format("%Y-%m"),
            retirement_age,
            delay_months
    )
}
