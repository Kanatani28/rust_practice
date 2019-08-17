
// すべてのRustプログラムには最低でも一つの関数がある（main関数）
fn main() {
    // Foo is executed!
    foo();
    // x is 1
    print_number(1);
    // sum is 5
    print_sum(2, 3);
    // print_sum2(2, 3);
    // x is 6
    let x: i32 = add_one(5);
    println!("x is {}", x);
    // y is 6
    let y: i32 = add_recursive(3);
    println!("y is {}", y);

    // z is 10
    let z: i32 = return_sample(10);
    println!("z is {}", z);

    // 発散する関数
    //diverges();

    // 関数ポインタ
    // 型推論無し
    let f: fn(i32) -> i32 = add_one;
    println!("f() result is {}" , f(5));
    // 型推論あり
    let f = add_one;
    println!("f() result is {}" , f(10));
}

// 引数なしのfoo関数
fn foo() {
    println!("Foo is executed!");
}

// 引数ありの場合
fn print_number(x: i32) {
    println!("x is {}", x);
}

fn print_sum(x: i32, y: i32) {
    println!("sum is {}", x + y);
}

// 引数に型を宣言しない場合はコンパイルエラー
// fn print_sum2(x, y) {
//     println!("sum is ", x + y);
//}

// 戻り値　式ベース①
fn add_one(x: i32) -> i32 {
    x + 1
} 
// 戻り値がある場合は最後にコロンを付けない　式ベース②
fn add_recursive(x: i32) -> i32 {
    let mut ret = 0;
    for i in 1..x + 1 {
        ret += i
    }
    ret
} 

// 戻り値　早期リターン
fn return_sample(x: i32) -> i32 {
    return x;

    // こっちは走らない
    x + 1

    // return を関数の最後で使うのはあまり推奨されない
    // return x + 1;
}

// 発散する関数(diverge: ディバージ、発散する、弾ける)
fn diverges() -> ! {
    // panic 与えられたメッセージと共にクラッシュする
    panic!("This function never returns!");
}