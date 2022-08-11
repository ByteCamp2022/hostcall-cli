wit_bindgen_rust::import!("../../imports.wit");
wit_bindgen_rust::export!("../../exports.wit");

struct Exports;

impl exports::Exports for Exports {

    fn proxy(name: String, param: String) -> String{
        match name.as_str() {
            "modulef1" => {
                return modulef1(param);
            }
            "modulef2" => {
                return modulef2(param);
            }
            "modulef2" => {
                return modulef2(param);
            }
            _ => return "{}".into(),
    }
    }

}

fn modulef1(s: String) -> String{
    println!("module a, message: {}", s);
    imports::proxy("f1", "implemented in host");
    "modulef1".into()
}

fn modulef2(s: String) -> String {
    "sdf".into()
}

fn modulef3(s: String) -> String {
    println!("implemeted in module");
    "modulef3".into()
}

fn main() {
    println!("from module main");
}
