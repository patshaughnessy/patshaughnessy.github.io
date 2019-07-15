extern crate maud;

use self::maud::html;

use post::Post;

pub fn render(_all_posts: &Vec<Post>) -> String {
    let rendered = html! {
      body {
          "to do"
      }
    };
    rendered.into_string()
}
