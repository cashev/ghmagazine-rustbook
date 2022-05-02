fn main() {
    // 変数expをRPN形式の文字列に束縛する
    // このRPNは数式 6.1 + 5.2 * 4.3 - 3.4 / 2.5 * 1.6 と等しい
    let exp = "6.1 5.2 4.3 * + 3.4 2.5 / 1.6 * -";

    let ans = rpn(exp);

    // debug build時のみ、答えが正しいかチェック
    // 浮動小数点の計算誤差を考慮し、ここでは小数点以下4桁までの値を文字列に変換
    debug_assert_eq!("26.2840", format!("{:.4}", ans));

    // expとansの値を表示する
    println!("{} = {:.4}", exp, ans);
}

fn rpn(exp: &str) -> f64 {
    // 変数stackを空のスタックに束縛
    // stackはmutableな変数
    let mut stack = Vec::new();

    // expをスペースで分割し、tokenを要素に束縛
    // 要素がなくなるまで繰り返す
    for token in exp.split_whitespace() {
        // tokenがf64型ならスタックに積む
        if let Ok(num) = token.parse::<f64>() {
            stack.push(num);
        } else {
            // tokenが数値でないなら、演算子なのか調べる
            match token {
                // tokenが演算子ならapply2関数で計算する
                // |x, y| x + y はクロージャ
                // 引数x, yを受け取り、x + yを計算して答えを返す
                "+" => apply2(&mut stack, |x, y| x + y),
                "-" => apply2(&mut stack, |x, y| x - y),
                "*" => apply2(&mut stack, |x, y| x * y),
                "/" => apply2(&mut stack, |x, y| x / y),
                // tokenが演算子でないなら、エラーを起こして終了
                _ => panic!("Unknown operator: {}", token),
            }
        }
    }
    // スタックから計算結果を取り出す。失敗したらエラーを起こして終了
    stack.pop().expect("Stack underflow")
}

// スタックから数値を2つ取り出し、F型のクロージャfunで計算し、その結果をスタックに積む
fn apply2<F>(stack: &mut Vec<f64>, fun: F)
// F型のトレイト境界
where
    F: Fn(f64, f64) -> f64,
{
    // 変数yとxをスタックの最後の2要素に束縛
    // y
    // x
    // の順でスタックされているため、Some(y), Some(x)の順番となる
    if let (Some(y), Some(x)) = (stack.pop(), stack.pop()) {
        // クロージャfunで計算し、変数zをその結果に束縛
        let z = fun(x, y);
        // 変数zの値をスタックに積む
        stack.push(z);
    } else {
        // スタックから要素が取り出せなかったときはエラーを起こして終了
        panic!("Stack underflow");
    }
}
