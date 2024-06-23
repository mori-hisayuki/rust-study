#[allow(clippy::print_literal)]

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

fn main() {
  //`{}` はどんな引数でも文字列に置き換えられます
  println!("{} days", 31);
  // インデックス付きの引数を使うこともできます
  println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
  // 名前での指定も可能です
  println!("{subject}, {verb}, {object}",
    object="the lazy dog",
    subject="the quick brown fox",
    verb="jumps over"
  );

  // 10進数(69420)
  println!("Base 10: {}", 69420);
  // 2進数(1010101011111100)
  println!("Base 2 (binary): {:b}", 69420);
  // 8進数(152674)
  println!("Base 8 (octal): {:o}", 69420);
  // 16進数-英字小文字(10f2c)
  println!("Base 16 (hexadecimal): {:x}", 69420);
  // 16進数-英字大文字(10F2C)
  println!("Base 16 (hexadecimal): {:X}", 69420);

  // 指定した幅の中に、右寄せで文字列を挿入
  // "     1". というように、５つの半角空白のあとに"1"が入る
  println!("{number:>width$}", number=1, width=6);
  // 数字を0埋めすることも可能
  // "000001". というように、５つの0のあとに"1"が入る
  println!("{number:0>width$}", number=1, width=5);
  // 記号を反対にすると左寄せになります。以下は"10000"と出力されます
  println!("{number:0<5}", number=1);

  // 引数の数が正しいかのチェックも行ってくれます。
  println!("My name is {0}, {1} {0}", "Bond", "James");
  // FIXME ^ 不足している引数"James"を追加しましょう。

  /*
  `{}`でフォーマットできるのは、fmt::Displayを実装している型のみです。
  ユーザーが定義した型はデフォルトではfmt::Displayを実装していません。
  */

  // `Structure` は `fmt::Display`を実装していないので、エラーになります。
  println!("This struct `{:?}` won't print...", Structure(3));
  println!("Now {:?} will print!", Structure(3));
  println!("Now {:?} will print!", Deep(Structure(7)));

  // Rust 1.58以降では周囲の変数から直接引数をキャプチャできます。
  let number: f64 =1.0;
  let width: usize = 5;
  println!("{number:>width$}");

  // 円周率を表示
  let pi = 3.14159;
  println!("Pi is roughly {0:.3}", pi);
}
