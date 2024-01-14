fn main() {
    // 1. 变量
    let age = 10;
    let mut name = String::from("John");
    name = String::from("Doe");
    println!("name: {}, age: {}", name, age);

    // 2. 常量
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS: {}", MAX_POINTS);

    // 3. shadowing
    let num = 20;
    // 允许重新声明变量，但使用 let 关键字
    let num = num + 1;

    // 4. 数据类型
    // 4.1 标量类型
    // 4.1.1 整型
    let int8: i8 = 127;
    let int16: i16 = 32767;
    let int32: i32 = 2147483647;
    let int64: i64 = 9223372036854775807;
    let int128: i128 = 170141183460469231731687303715884105727;
    let uint8: u8 = 255;
    let uint16: u16 = 65535;
    let uint32: u32 = 4294967295;
    let uint64: u64 = 18446744073709551615;
    let uint128: u128 = 340282366920938463463374607431768211455;
    let isize: isize = 9223372036854775807;
    let usize: usize = 18446744073709551615;

    // 4.1.2 浮点型

    // struct Person {
    //     name: String,
    //     age: i32,
    //     height: f32,
    //     is_married: bool,
    //     is_graduated: bool,
    //     hobbies: Vec<String>,
    // }

    // fn main() {
    //     let person = Person {
    //         name: String::from("John"),
    //         age: 20,
    //         height: 1.81,
    //         is_married: false,
    //         is_graduated: true,
    //         hobbies: vec![String::from("music"), String::from("movies"), String::from("sports")],
    //     };

    //     println!("name: {}", person.name);
    //     println!("age: {}", person.age);
    //     println!("height: {}", person.height);
    //     println!("is_married: {}", person.is_married);
    //     println!("is_graduated: {}", person.is_graduated);
    //     println!("hobbies: {:?}", person.hobbies);
    // }

    // // Path: 08-arrays/index.rs

    // fn main() {
    //     let numbers = [1, 2, 3, 4, 5];

    //     println!("numbers: {:?}", numbers);
    // }

    // // Path: 09-vectors/index.rs

    // fn main() {
    //     let numbers = vec![1, 2, 3, 4, 5];

    //     println!("numbers: {:?}", numbers);
    // }

    // // Path: 10-strings/index.rs

    // fn main() {
    //     let name = String::from("John");

    //     println!("name: {}", name);
    // }

    // // Path: 11-ownership/index.rs

    // fn main() {
    //     let name = String::from("John");

    //     println!("name: {}", name);
    // }

    // // Path: 12-borrowing/index.rs

    // fn main() {
    //     let name = String::from("John");

    //     println!("name: {}", name);
    // }

    // // Path: 13-references/index.rs

    // fn main() {
    //     let name = String::from("John");

    //     println!("name: {}", name);
    // }

    // // Path: 14-slices/index.rs

    // fn main() {
    //     let name = String::from("John");

    //     println!("name: {}", name);
    // }

    // // Path: 15-structs-methods/index.rs

    // struct Person {
    //     name: String,
    //     age: i32,
    //     height: f32,
    //     is_married: bool,
    //     is_graduated: bool,
    //     hobbies: Vec<String>,
    // }

    // impl Person {
    //     fn get_name(&self) -> &String {
    //         &self.name
    //     }

    //     fn get_age(&self) -> &i32 {
    //         &self.age
    //     }

    //     fn get_height(&self) -> &f32 {
    //         &self.height
    //     }

    //     fn get_is_married(&self) -> &bool {
    //         &self.is_married
    //     }

    //     fn get_is_graduated(&self) -> &bool {
    //         &self.is_graduated
    //     }

    //     fn get_hobbies(&self) -> &Vec<String> {
    //         &self.hobbies
    //     }
    // }

    // fn main() {
    //     let person = Person {
    //         name: String::from("John"),
    //         age: 20,
    //         height: 1.81,
    //         is_married: false,
    //         is_graduated: true,
    //         hobbies: vec![String::from("music"), String::from("movies"), String::from("sports")],
    //     };

    //     println!("name: {}", person.get_name());
    //     println!("age: {}", person.get_age());
    //     println!("height: {}", person.get_height());
    //     println!("is_married: {}", person.get_is_married());
    //     println!("is_graduated: {}", person.get_is_graduated());
    //     println!("hobbies: {:?}", person.get_hobbies());
    // }
}
