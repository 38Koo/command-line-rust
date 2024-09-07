mod lib; // src/bin/catr/lib/mod.rs を指します

use lib::catr; // src/bin/catr/lib/catr.rs にあるモジュールをインポート

fn main() {
    if let Err(e) = catr::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}