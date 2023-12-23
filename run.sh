# コンパイルによってバイナリの実行可能ファイルを出力
rustc main.rs
# 実行
./main

# プロジェクトの作成
cargo new project_name
# ビルド＆実行
cargo build
./src/main.rs
# ビルド＆実行
cargo run
# コンパイル可能かチェック
cargo check
# リリースに向けたビルド（最適化された状態でコンパイルできる）
cargo build --release
# テストの実行 （#[test]がついているもの）
cargo test