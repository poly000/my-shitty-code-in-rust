fn what_century(year: &str) -> String {
    let year_num: usize = year.parse::<usize>().unwrap();
    let year_num = year_num / 100 + if year_num % 100 == 0 { 0 } else { 1 };
    year_num.to_string() + match year_num {
        4..=20 => "th",
        _ => {
            ["th","st","nd","rd"][if year_num % 10 > 3 { 0 } else { year_num % 10}]
        },
    }
}

fn main() {
    assert_eq!(what_century("1999"), "20th");
    assert_eq!(what_century("2011"), "21st");
    assert_eq!(what_century("2154"), "22nd");
    assert_eq!(what_century("2259"), "23rd");
    assert_eq!(what_century("1234"), "13th");
    assert_eq!(what_century("1023"), "11th");
    assert_eq!(what_century("2000"), "20th");
    assert_eq!(what_century("3210"), "33rd");    
}
