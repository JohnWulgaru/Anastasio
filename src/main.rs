#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

#[get("/")]
fn index() -> Option<NamedFile> {
    let path = Path::new("index.html"); // Update this with your HTML file's path
    NamedFile::open(path).ok()
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}
