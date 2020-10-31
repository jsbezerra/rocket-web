#![feature(decl_macro)]
#[macro_use]
extern crate rocket;

use rocket::request::Form;
use rocket::response::content::Json;

#[derive(FromForm, Debug)]
struct Book {
  title: String,
  author: String,
  isbn: String,
}

#[get("/hello")]
fn hello() -> Json<&'static str> {
  Json("{
    'status': 'success',
    'message': 'Hello API!'
  }")
}

#[post("/book", data = "<book_form>")]
fn new_book(book_form: Form<Book>) -> String {
  let book: Book = book_form.into_inner();
  let mut dummy_db: Vec<Book> = Vec::new();
  dummy_db.push(book);
  format!("Book added successfully: {:?}", dummy_db)
}

fn main() {
  rocket::ignite()
    .mount("/api", routes![hello])
    .launch();
}
