wit_bindgen_rust::export!("../wits/module1.wit");

struct Module1;

impl module1::Module1 for Module1 {
    fn f1(s: String) {
        println!("message: {}", s);
    }

    fn f2() -> String {
        "sdf".into()
    }

    fn f3() {
        println!("implemeted in module");
    }

    fn f4(slice: Vec<u8>) {
        println!("{:?}", slice);
    }
}

fn main() {
    println!("from module main");
}
