use std::char::from_digit;


pub fn convert_base(num_str: &str, to_base: u32) -> String {
    /// 基本邏輯
    /// 遍歷每一位置的數字，將其乘以對應源進制的次冪,求和 結果為 10 進制
    /// 將 10 進制轉為目標進制
    /// 數據反復除以目標進製數,記錄每次餘數,餘數下往上排列
    ///

    // 來源數據  數字(進制), 需要將字符串提取數字和進制
    let (num, base) = parse_num_str(num_str);
    println!("num_str:{num}, base:{base}");
    let mut decimal = 0;
    decimal = num.to_string().chars().fold(0, |acc, val| {
        println!("acc:{},val:{}", acc, val);
        acc * base as i32 + val.to_digit(base).unwrap() as i32
    });
    if decimal == 0 {
        return String::from("0");
    }
    let mut result = Vec::new();
    while decimal > 0 {
        let remainder = decimal / to_base as i32;
        result.push(from_digit((decimal - remainder * to_base as i32) as u32, to_base).unwrap());
        decimal /= to_base as i32;
    }

    result.iter().rev().collect()
}


/// 來源數據格式 數字(進制)
fn parse_num_str(str: &str) -> (i32, u32) {
    let mut num_str = str.split('(');
    let num = num_str.next().unwrap().parse::<i32>().unwrap();
    let base = num_str.next().unwrap().split(')').next().unwrap().parse::<u32>().unwrap();
    (num, base)
}