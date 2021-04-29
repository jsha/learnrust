use askama::Template;

#[derive(Template)]
#[template(path = "settings.html")]
struct Settings {
    settings: Vec<Setting>,
}

enum Setting {
    Toggle(bool),
}

fn main() {
    let s = Settings {
        settings: vec![Setting :: Toggle(true) ],
    };
    println!("{}", s.render().unwrap());
}
