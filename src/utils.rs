use chrono::Local;

pub fn get_today_date() -> String {
    // 获取当前日期
    let today = Local::now().naive_local().date();
    
    // 格式化为 YYYY-MM-DD
    let formatted_date = today.format("%Y-%m-%d").to_string();
    formatted_date
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_get_today_date() {
    let date = get_today_date();
    assert_eq!(date, "2024-10-31");
  }
}