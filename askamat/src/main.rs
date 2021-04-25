use askama::Template;

#[derive(Template)]
#[template(path = "settings.html")]
struct Settings {
    settings: Vec<Setting>,
}

#[derive(Debug)]
enum Setting {
    Section {
        description: &'static str,
        sub_settings: Vec<Setting>,
    },
    Toggle {
        js_data_name: &'static str,
        description: &'static str,
        default_value: bool,
    },
    Select {
        js_data_name: &'static str,
        description: &'static str,
        default_value: &'static str,
        options: Vec<(String, String)>,
    },
}

fn main() {
    let s = Settings {
        settings: vec![Setting::Toggle {
              js_data_name: "foo",
              description: "bar",
              default_value: true,
        }],
    };
    println!("{}", s.render().unwrap());
}
