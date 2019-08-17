fn main() {
    // bool型
    let b1 = true;
    let b2: bool = false;
    println!("b1 is {}. \nb2 is {}.", b1, b2);
    
    // char型 シングルクオートで囲む
    let c1 = 'c';
    // Rustのchar型はユニコードのスカラ値を示し、4バイト
    let c2: char = '💕';
    println!("c1 is {}. \nc2 is {}", c1, c2);

    // 数値型
    
    //// 符号あり固定長

    // i8 -128 ~ 127
    println!("i8 min: {}, max: {}", i8::min_value(), i8::max_value());
    // i16 -32768 ~ 32767
    println!("i16 min: {}, max: {}", i16::min_value(), i16::max_value());
    // i32 -2147483648 ~ 2147483647
    println!("i32 min: {}, max: {}", i32::min_value(), i32::max_value());
    // i64 -9223372036854775808 ~ 9223372036854775807
    println!("i64 min: {}, max: {}", i64::min_value(), i64::max_value());

    //// 符号なし固定長
    // u8 0 ~ 255
    println!("u8 min: {}, max: {}", u8::min_value(), u8::max_value());
    // u16 0 ~ 65535
    println!("u16 min: {}, max: {}", u16::min_value(), u16::max_value());
    // u32 0 ~ 4294967295
    println!("u32 min: {}, max: {}", u32::min_value(), u32::max_value());
    // u64 0 ~ 18446744073709551615
    println!("u64 min: {}, max: {}", u64::min_value(), u64::max_value());

    //// 可変長
    let mut i_size: isize = 32767;
    let mut u_size: usize = 65535;
    println!("i_size is {}", i_size);
    println!("u_size is {}", u_size);

    i_size = i_size + 1;
    u_size = u_size + 1;
    println!("i_size + 1 is {}", i_size);
    println!("u_size + 1 is {}", u_size);

    // 固定長で宣言した変数が最大値を超えるとオーバーフローで例外発生
    // let mut i_16: i16 = 32767;
    // let mut u_16: u16 = 65535;
    // println!("i_16 is {}", i_16);
    // println!("u_16 is {}", u_16);

    // i_16 = i_16 + 1;
    // u_16 = u_16 + 1;
    // println!("i_16 + 1 is {}", i_16);
    // println!("u_16 + 1 is {}", u_16);

    // 浮動小数点型(省略)


    // 配列
    let a = [1, 2, 3]; // a: [i32; 3]
    // len()で配列の長さを取得する
    println!("a has {} elements.", a.len());
    // 添え字記法で取得する
    println!("first element is {}.", a[0]);
    // タプルの取得方法と異なる（後述）
    // println!("second element is {}.", a.1);

    // スライス
    let a2 = [0, 1, 2, 3, 4];
    let complete = &a2[..]; // all
    let middle = &a2[1..4]; // 1 2 3

    // タプル
    let t = (1, "Hello");
    // 型を宣言する
    let t: (i32, &str) = (2, "Good Bye");
    // アリティが同じであれば再代入可能
    let mut t2 = (3, "Good Morning");
    t2 = t;
    // エラー
    // let t3 = (true ,false);
    // t2 = t3;
    let tuple = (1, 2, 3);
    println!("first element is {}", tuple.0);
    // 配列の取得方法と異なる
    // println!("second element is {}", tuple[1]);

    // 関数型
    fn foo(x: i32) -> i32 { x }
    let function: fn(i32) -> i32 = foo;
    let ret = function(1);
    println!("ret is {}", ret);

}
