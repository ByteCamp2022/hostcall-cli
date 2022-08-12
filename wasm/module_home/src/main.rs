use serde_json::json;

wit_bindgen_rust::import!("../../imports.wit");
wit_bindgen_rust::export!("../../exports.wit");

struct Exports;

impl exports::Exports for Exports {

    fn proxy(name: String, param: String) -> String{
        println!("\tUsing module_home");
        let v:serde_json::Value = serde_json::from_str(&param.as_str()).unwrap();
        match name.as_str() {
            "response" => {
                return response(&v).to_string();
            }
            _ => {
                "no such a func !".to_string()
            }
    }
    }

}

fn responseStatus() -> String {
    let status = String::from("HTTP/1.1 200 OK");
    return status;
}

fn response_HTML(path: &str) -> String {
    let v = imports::proxy("response_HTML", &json!({"path": path,}).to_string());
    let html:serde_json::Value = serde_json::from_str(&v).unwrap();
    html["html"].as_str().unwrap().to_string()
}

fn response(v: &serde_json::Value) -> serde_json::Value{
    println!("\t\tUsing function: response(path: String) in module_home");
    let status = responseStatus();
    let contents = response_HTML(v["path"].as_str().unwrap());
    let resp = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        contents.len(),
        contents
    );
    json!({"response": resp,})
    // return imports::proxy("response", &resp[..]);
}

fn main() {
    println!("Hello, world!");
}
