use askama::Template;
//template struct for index
#[derive(Template)]
#[template(path="index.html")]
struct IndexTemplate<'a> {
    name :&'a str,
}
//templating for index
pub fn index_template() -> String{
    let index = IndexTemplate{name:"hell"};
    index.render().unwrap()
}
