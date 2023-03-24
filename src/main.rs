/**
 * 华氏温度转摄氏度
 */
fn fahrenheit_to_celsius(temperature: i32) -> i32 {
    return (temperature - 32) * 5 / 9;
}

/**
 * 斐波那契数列
 */
fn fibonacci(index: i32) -> i32 {
    if index == 1 || index == 2 {
        return 1;
    }
    return fibonacci(index - 1) + fibonacci(index - 2);
}

// 圣诞歌列表
const CHRISTMAS_LIST: [&'static str; 11] = [
    "Twelve drummers drumming,",
    "Eleven pipers piping,",
    "Ten lords a-leaping,",
    "Nine ladies dancing,",
    "Eight maids a-milking,",
    "Seven swans a-swimming,",
    "Six geese a-laying,",
    "Five golden rings,",
    "Four calling birds,",
    "Three French hens,",
    "Two turtle doves,",
];

/**
 * “圣诞节的十二天” 歌词打印
 */
fn christmas_twelve_day(day: i32) -> String {
    if day == 1 {
        return "On the first day of Christmas,\nmy true love sent to me,\nA partridge in a pear tree.\n".to_owned();
    }
    if day > 12 {
        return "There are only 12 days to christmas".to_owned();
    }
    let christmas_lastday = christmas_twelve_day(day - 1);
    let christmas_content = &CHRISTMAS_LIST[(12 - day) as usize..11].join("\n");
    return format!("{}\n\nOn the first day of Christmas,\nmy true love sent to me,\n{}\nA partridge in a pear tree.\n", christmas_lastday, christmas_content);
}

fn main() {
    // 在华氏度和摄氏度之间转换温度
    let celsius = fahrenheit_to_celsius(100);
    println!("100华氏温度对应:{celsius}摄氏度");

    // 生成第n个斐波那契数
    let fibonacci_ten = fibonacci(10);
    println!("第10个斐波那契数字是:{fibonacci_ten}");

    // 利用歌曲中的重复打印圣诞颂歌“圣诞节的十二天”的歌词
    let christmas_twelve = christmas_twelve_day(12);
    println!("{christmas_twelve}");
}
