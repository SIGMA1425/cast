fn main() {
    let x: i32 = 42;

    //i32とi64は別の型でありそのまま束縛することはできない
    //let y: i64 = x;

    //intoメソッドを用いた型変換
    let y:i64 = x.into();

    //fromメソッドを用いた型変換
    let y = i64::from(x);

    // asを用いた型変換(コンパイラが知っている範囲での安全な型変換)
    let y = x as i64;

    println!("y = {}", y);

    /* from into asによる型変換は失敗してはいけない */
    /* ビット幅が広い型から狭い型への変換はできない */
    /* TryInto, TryFromを用いた失敗しうる型変換 */
    
}
