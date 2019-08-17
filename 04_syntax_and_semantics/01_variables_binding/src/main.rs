fn main() {
    // 変数宣言 letで宣言するとデフォルトでイミュータブル
    let a = 5;
    // 複数宣言するパターン
    let (b, c) = (1, 2);
    
    println!("a is {}.", a);
    println!("b is {}.", b);
    println!("c is {}.", c);

    // 型アノテーション 変数名のあとにコロンを付けて型の名前
    let d: i32 = 5;

    println!("d is {}.", d);

    // 可変な変数を宣言するにはmutキーワード
    // mutを付けずに再代入するとコンパイルエラー
    let mut e = 6;
    println!("before: e is {}.", e);
    e = 3;
    println!("after: e is {}.", e);


    // 変数の初期化
    let f: i32;
    // 初期化されていない変数を使用するとコンパイルエラー
    // println!("f is {}.", f)

    // スコープ
    let g: i32 = 17;
    {
        let h: i32 = 18;
        println!("g is {}. h is {}.", g, h);
    }
    println!("g is {}.", g);
    // {}内で宣言された変数は{}で囲まれた部分でのみ有効
    // println!("h is {}.", h);

    // シャドーイング イミュータブルな変数でもletで再宣言可能
    let i: i32 = 19;
    {
        // i is 19.
        println!("i is {}.", i);
        let i = 20;
        // i is 20.
        println!("i is {}.", i);
    }
    // i is 19.
    println!("i is {}.", i);
    let i = 100;
    // i is 100.
    println!("i is {}.", i);

    // シャドーイングとミュータブルな変数を宣言することは異なる概念

    // ミュータブルな変数を宣言する場合は型の変更等ができない、スコープを超えても変数が戻らない
    let mut j: i32 = 1;
    j = 7;

    // シャドーイングする場合は型も変更できる、スコープによって値が戻る
    let k = 32;
    let k = "aaaaaaaaaaaa";

}
