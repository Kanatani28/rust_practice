fn main() {
    // if文
    let x = 5;

    if x == 5 {
        println!("x is 5");
    } else if x == 6 {
        println!("x is 6");
    } else {
        println!("x is not 5 or 6");
    }

    // if式
    let y = if x == 5 {
        "five"
    } else if x == 6 {
        "six"
    } else {
        "other"
    };
    println!("y is {}", y);

    // loop
    // 無限ループ
    // loop {
    //     println!("LOOP!!!");
    // }
    // while true と書く場合があるならloopを使うべき

    // while
    let mut number = 5;
    let mut done = false;

    // numberが5の倍数になるまでループして終了
    while !done {
        number += number - 3;
        println!("{}", number);
        if number % 5 == 0 {
            done = true;
        }
    }

    // for文
    // 0から9まで繰り返すループ
    for i in 0..10 {
        println!("{}", i);
    }
    // inのあとにはレンジやイテレータを書く
    // Rustは伝統的なCスタイルのループ（ for(int i = 0; i < 10 i++) ）を持たない

    // 列挙forループ
    for (i, j) in (5..10).enumerate() {
        println!("i = {}, j = {}", i , j);
    }
    let list = ["one", "two", "three"];
    for (i, s) in list.iter().enumerate() {
        println!("i = {}, s = {}", i, s);
    }

    // break
    let mut x = 5;
    loop {
        x += x - 3;
        println!("x is {}", x);

        if x % 5 == 0 { break ;}
    }
    // continue
    for i in 0..10 {
        if i % 2 == 0 { continue; }
        println!("i is {}", i);
    }

    // ラベル
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 { continue 'outer; }
            if y % 2 == 0 { continue 'inner; }
            println!("x is {}, y is {}", x, y);
        }
    }

}
