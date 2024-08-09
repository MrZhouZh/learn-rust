use num::complex::Complex;

fn greet_world() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界, 你好!";
    let english = "Hello, World!";
    let regions = [southern_germany, chinese, english];
    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn main() {
    let guess = "42".parse::<i32>().expect("Not a number");
    println!("{}", &guess);
    greet_world();
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;
    // 13.2 + 21i
    println!("{} + {}i", result.re, result.im);
    let x = '中';
    // 字符'中'占用了4字节的内存大小
    println!("字符'中'占用了{}字节的内存大小", std::mem::size_of_val(&x));
}
