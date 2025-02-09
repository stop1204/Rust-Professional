pub fn time_info(date: &str) -> String {
    let (year, month, day) = parse_date(date);
    let week_number = calculate_week_number(year, month, day);
    let day_of_week = calculate_day_of_week(year, month, day);
    let day_of_year = calculate_day_of_year(year, month, day);
    let days_left_in_year = calculate_days_left_in_year(year, month, day);
    let days_to_spring_festival = calculate_days_to_spring_festival(year, month, day);
    let days_to_stock_market = calculate_days_to_stock_market(year, month, day);

    format!("{},{},{},{},{},{}",
            week_number,
            day_of_week,
            day_of_year,
            days_left_in_year,
            days_to_spring_festival,
            days_to_stock_market)
}


fn parse_date(date: &str) -> (u32, u32, u32) {
    let parts: Vec<&str> = date.split('-').collect();
    (parts[0].parse().unwrap(), parts[1].parse().unwrap(), parts[2].parse().unwrap())
}


fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}


fn days_in_month(year: u32, month: u32) -> u32 {
    match month {
        4 | 6 | 9 | 11 => 30,
        2 => if is_leap_year(year) { 29 } else { 28 },
        _ => 31
    }
}


fn calculate_day_of_year(year: u32, month: u32, day: u32) -> u32 {
    (1..month).map(|m| days_in_month(year, m)).sum::<u32>() + day
}


fn calculate_days_left_in_year(year: u32, month: u32, day: u32) -> u32 {
    (if is_leap_year(year) { 366 } else { 365 })
        - calculate_day_of_year(year, month, day)
}


fn calculate_week_number(year: u32, month: u32, day: u32) -> u32 {
    let day_of_year = calculate_day_of_year(year, month, day);
    let first_day = calculate_day_of_week(year, 1, 1);
    ((day_of_year + first_day - 2) / 7) + 1
}


fn calculate_day_of_week(year: u32, month: u32, day: u32) -> u32 {
    let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let mut y = year;
    if month < 3 { y -= 1; }
    ((y + y / 4 - y / 100 + y / 400 + t[(month - 1) as usize] + day) % 7) + 1
}


fn calculate_days_to_spring_festival(year: u32, month: u32, day: u32) -> u32 {
    let target_year = if month > 2 || (month == 2 && day > 1) { year + 1 } else { year };
    if target_year == year {
        32 - day
    } else {
        let days_to_year_end = calculate_days_left_in_year(year, month, day);
        days_to_year_end + 32
    }
}


fn calculate_days_to_stock_market(year: u32, month: u32, day: u32) -> u32 {
    let day_of_week = calculate_day_of_week(year, month, day);
    match day_of_week {
        6 => 2,
        7 => 1,
        _ => 0,
    }
}