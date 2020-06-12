use std::env;
use koto::interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 && &args[1] == "run" {
        let path = &args[2];
        let result = interpreter::interpreters::read_file(path);
        println!("{:?}", result);
        interpreter::interpreters::run(result);
    }else if args.len() == 3 && &args[1] == "compile"{
        //TODO中間言語(C言語)にコンパイルして実行する
    } else {
        println!("file run");
        println!("./koto run [file name]");
    }
}
