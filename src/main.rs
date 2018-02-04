#![feature(plugin)]
#![plugin(rocket_codegen)]
static SOME_STR: &'static str = "A static string";

extern crate rocket;

#[get("/")]
fn index() -> String {
    // "Hello, world!"
    let mut owned_string: String = "Hello, world!".to_owned();
    owned_string.push_str(SOME_STR);
    owned_string
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}


struct MyStruct {
    number: i32,
    string: &'static str,
}
