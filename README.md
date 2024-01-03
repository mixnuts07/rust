### Rust の特徴 / 言語思想

エンパワーメントを根本原理としている。

メモリ管理やデータ表現、並行性などの低レベルな詳細を扱う「システムレベル」のプログラミングがある。

- メモリ安全性
  - コンパイル時にメモリ安全性を保証し、ランタイムエラーを減少させる。メモリ安全性を最優先し、データ競合やヌルポ、バッファオーバーフローなどの一般的なメモリエラーを防ぐ。
- データ競合の防止
  - 所有権システムにより、データ競合をコンパイル時に検出し防止する。
- 所有権システム
  - Rust の所有権によってガベージコレクションやランタイムコストなしにメモリ管理を行う。
- スタックとヒープ（実行時にコードが使用できるメモリの一部）
  - スタック...得た順と逆順で取り出す。高速。Push,Pop。既知の固定サイズ。(last in, first out.)
  - ヒープ...得た順と同順で取り出す。低速。可変。データを置く際に OS は空領域を確保(allocate)し、ポインタを返す。(first in, first out.)
- 所有者規則
  - Rust の各値は、所有者と呼ばれる変数と対応している。
  - いかなる時も所有者は 1 つである。
  - 所有者がスコープから外れたら、値は破棄される。
- メモリと確保（allocate, free）
  - メモリは実行時に OS に要求される。
  - String 型を使用し終わったら OS にメモリを返還する方法が必要（ガベージコレクタ）ただ Rust は自動的に返還する仕組み。
  - ムーブ(shallow copy に近い)
    ```rust
      let s1 = String::from("hello");
      let s2 = s1
    ```
    - s1 は s2 にムーブされる。(メモリ安全性のため)
      ![Alt text](<images/Screen Shot 2023-12-25 at 9.45.16 AM.png>)
  - クローン(deep copy に近い)
    ```rust
      let s1 = String::from("hello");
      let s2 = s1.clone(); // ヒープデータのコピー
    ```
  - コピー(スタックのみのデータ。サイズが既知ならコピーできる)
    ```rust
      let s1 = 5;
      let s2 = s1;
    ```
  - Copy 型
    - Copy 型...単純なスカラー値。タプル
    - Copy 型ではないもの...メモリ確保が必要だったり、何らかの形態のリソース。
- 参照(&)
  - 所有権をもらうことなく、値を参照することができる。
    ![Alt text](<images/Screen Shot 2023-12-25 at 1.08.22 PM.png>)
    ```rust
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    // この&s1記法によりs1の値を参照する参照を生成できる。
    // 参照がスコープを抜けてもs1はdropされなくなる
    ```
- 参照外し(\*)
- 借用

  - 関数の引数に参照を取ること。借用した何かを変更しようとしたら動かない。(参照は不変のため)

  ```rust
  let s = String::from("hello");
  change(&s);
  fn change(some_thing: &String) {
   some_thing.push_str(", world")l
  }
  ```

- スライス（所有権のない別のデータ型。コレクション全体ではなく、そのうちの一連の要素を参照することができる。）
  - 文字列スライス...String への一部への参照。
    ```rust
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..1];
    ```
    ![Alt text](<images/Screen Shot 2023-12-25 at 8.00.13 PM.png>)
- メソッド記法(impl)
  - 構造体の文脈。関数と似ているが最初に引数は必ず self になる。これはメソッドは呼び出されている構造体インスタンスを表す。
- パフォーマンス
  - C / C++に匹敵する高性能を持ち、システムプログラミングに適している。
- ゼロコスト抽象化
  - Rust の抽象化は実行時のパフォーマンスに影響を与えない。
- 型安全性と型推論
  - バグ発生の減少。
- 並行処理
  - 安全な並行処理を容易に実装できるツールを提供する。
- AOT コンパイル言語
  - プログラムをコンパイルし、生成された実行可能ファイルを誰かにあげると、もらった人は Rust をインストールしていなくても実行できる。

### Rust の開発ツール

- Cargo
  - 依存管理ツール。ビルドツール。Rust のエコシステム全体で一貫性を持たせる。
- Cargo.toml
  - Cargo の設定フォーマット。
- Cargo.lock
  - プロジェクト内の依存関係の正確なバージョンを記録。
- Rustfmt
  - 一貫したコーディングスタイルを保証する。
- Rust 言語サーバー
  - コード補完やインラインエラーメッセージに対応。
- rustup
  - Rust のインストール・管理。

### 用語集

- クレート
  - コードのパッケージのこと。
- プレリュード[prelude](https://doc.rust-lang.org/std/prelude/index.html)
  - 標準ライブラリで定義されているアイテムの中のいくつかを、全てのプログラムのスコープに取り込む。このセットのこと。プレリュードにない場合、use 文を使って明示的にスコープに入れる必要がある。
- マクロ(２種類ある)
  - コードの生成や繰り返しを効率化するための強力なツール。特定の構文に基づいてコードを動的に展開するメタプログラミング機能を提供する。
  - デクララティブマクロ
    - パターンマッチングに基づいてコードを生成する。シンプルな置換や繰り返しに使用。
  - 手続型マクロ
    - 複雑なコード生成に使用され、カスタム derive・属性マクロ・関数マクロが含まれる。Rust のコードをトークンとして操作し、コンパイル時に新しいコードを作成する。
- :: ... 関連関数（Ex. String::new()の::）
  - ある型（String）の関数のこと。new()関数は新しい空の文字列を作成する。
  - 構造体に定義できる関数（&self を引数に取らないもの）
  - 構造体の新規インスタンスを返すコンストラクタによく使用される。
- :: ... スコープ解決演算子
  - 特定のモジュールやトレイト内の関数・型・定数などにアクセスするために使用される。
- ＆ ... 参照（デフォルトで不変） -　コードの複数の部分が同じデータにアクセスしても、そのデータを何度もメモリにコピーしなくて済む。
- !
  - マクロを呼び出すときに使用される。（println!()）
  - コンパイル時にコードを生成する。
- const(定数＋型明記必須。全て大文字＋\_区切り), let(変数)
- シャドーイング
  - let で定義した変数を上書くこと。不変とは異なり、let を使いちょっとした加工ができる。
- データ型（2 つ）
  - スカラー（4 つ）
    - 整数、不動小数点、文字列、論理値
    - 複合(2 つ)
      - タプル
        - 複数の何らかの値を 1 つの複合型にまとめ上げる。
        ```rust
          let tup: (i32, f64, u8) = (500, 6.4, 1);
          let (x, y, z) = tup;
          let five_hundred = x.0 // 500
        ```
        - 配列(配列は固定長なので、一度宣言されたらサイズを変更できない！)
          - スタック上に確保される一塊のメモリ。
            ```rust
              let a: [i32, 5] = [1,2,3,4,5]
              let second = a[1] // 2
              let a = [3,5] // [3,3,3,3,3]
            ```
- ；...文はつく。式にはつかない。
- データ競合（Rust はデータ競合が起こるコードをコンパイルしない。）
  - 2 つ以上のポインタが同じデータにアクセスする。
  - 少なくとも 1 つのポインタがデータに書き込みを行なっている。
  - データへのアクセスを同期する機構が使用されていない。
- ダングリングポインタ

  - 他人に渡されてしまった可能性のあるメモリを指すポインタのことで、その箇所へのポインタを保持している間にメモリを解放することで発生してしまう。Rust ではコンパイル時に発生しないように保証してくれる。

- match...制御フロー演算子。一連のパターンに対して値を比較し、マッチしたパターンに応じてコードを実行する。
- \_ ... プレースホルダー。列挙してない値。

- if let(値が 1 つのパターンにマッチした時にコードを走らせ、他は無視する match への糖衣構文)

  ```rust
  if let Some(3) = some_u8_value {
    println!("three");
  }
  ```

- パッケージ (cargo new -- で作る)
  - クレートをビルドし、テストし、共有することができる Cargo の機能。
  - ある機能群を提供する 1 つ以上のクレート。
  - Cargo.toml というそれらのクレートをどのようにビルドするかを説明するファイルを持っている。
  - src/bin ディレクトリにファイルを置くと、パッケージは複数のバイナリクレートを持つことができる。
- クレート（バイナリかライブラリ）
  - ライブラリか実行可能ファイルを生成する、木構造をしたモジュール群。
  - クレートの機能は、そのスコープ内の名前空間に位置付けられている。
  - クレートルート(src/main.rs, src/lis.rs)
    - Rust コンパイラの開始点となり、クレートのルートモジュールを作るソースファイル。
    - main.rs, lib.rs の中身が crate というモジュールを形成する。
    ```
    // lib.rs
    crate
    └── front_of_house
        ├── hosting
        │   ├── add_to_waitlist
        │   └── seat_at_table
        └── serving
            ├── take_order
            ├── serve_order
            └── take_payment
    ```
- モジュール(mod)と use
  - モジュールはクレート内のコードをグループ化し、可読性と再利用性を上げる。
  - パスの構成、スコープ、公開するか否かを決定できる。
  - io と Write をスコープに持ち込んでいる。
    ```rust
    use std::io::[self, Write];
    ```
  - glob 演算子(パスに定義されているすべての公開要素をスコープに持ち込みたい時)
    ```rust
    use std::collections::*;
    ```
- パス
  - 要素（構造体、関数、モジュール）に名前をつける方法。
- Cargo Workspace
- 再公開(pub use)
  - 要素を use し、再度他の人が利用できるようにする。
- コレクション（3 つ）

  - ベクタ型(Vec<T>)(配列？)
    - 単体のデータ構造でありながら複数の値を保持でき、それらの値はメモリ上に隣り合わせに並べる。可変長の値を並べて保持できる
    - vec!...与えた値を保持する新しいベクタを生成するマクロ。
  - 文字列（文字のコレクション。）
  - ハッシュマップ（値を特定のキーと紐付けさせてくれる。）

- エラー処理
  - Rust には例外がない。
  - x 回復可能（Ex.ファイルがないなど。再試行が合理的）(Result<T,E>)
  - 回復不可能（Ex.配列境界を超えたアクセスなど）(panic!)
  - パニックに対するスタックの unwinding or abort
  - バックトレース
    - この時点までに呼び出されたすべての関数のリスト。
  -
