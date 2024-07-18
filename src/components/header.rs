use maud::{html, Markup, DOCTYPE};

pub struct Header<'a> {
    pub title: &'a str,
}

pub fn generate_header(options: Header) -> Markup {
    let markup = html! {
        (DOCTYPE)
        meta charset="utf-8";
        meta name="viewport" content="width=device-width, initial-scale=1";
        link href="/style.css" rel="stylesheet" type="text/css";
        script src="https://unpkg.com/htmx.org@2.0.1" integrity="sha384-QWGpdj554B4ETpJJC9z+ZHJcA/i59TyjxEPXiiUgN2WmTyV5OEZWCD6gQhgkdpB/" crossorigin="anonymous"
        title { (options.title) };
    };

    markup
}
