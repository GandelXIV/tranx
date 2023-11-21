use lazy_static::lazy_static;
use tera::Context;
use tera::Tera;

lazy_static! {
    pub static ref TEMPLATES: Tera = Tera::new("templates/**/*.html").unwrap();
}

pub fn greet(name: &str, score: &usize) -> String {
    let mut context = Context::new();
    context.insert("name", name);
    context.insert("score", score);
    TEMPLATES.render("greet.html", &context).unwrap()
}

pub fn counter(score: &usize) -> String {
    let mut context = Context::new();
    context.insert("score", &score);
    TEMPLATES.render("counter.html", &context).unwrap()
}
