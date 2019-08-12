extern crate maud;

use chrono::offset::Utc;
use chrono::DateTime;
use chrono::SecondsFormat;
use std::time::SystemTime;

use self::maud::html;

use post::Post;

pub fn render(all_posts: &Vec<Post>) -> String {
    let system_time = SystemTime::now();
    let now: DateTime<Utc> = system_time.into();
    let rendered = html! {
      feed xmlns="http://www.w3.org/2005/Atom" {
        title { "Pat Shaughnessy" }
        id { "http://patshaughnessy.net" }
        updated { 
          (now.to_rfc3339_opts(SecondsFormat::Secs, true))
        }
        author {
          name { "Pat Shaughnessy" }
        }
        @for post in all_posts.iter().take(10) {
          entry {
            title {
              (post.title)
            }
            // Please save this in a local variable
              link href={"http://patshaughnessy.net/" (post.url)} rel="alternate" { }
              id href={"http://patshaughnessy.net/" (post.url)} rel="alternate" { }
              published { 
                (post.date.to_rfc3339_opts(SecondsFormat::Secs, true))
              }
              updated { 
                (post.date.to_rfc3339_opts(SecondsFormat::Secs, true))
              }
              category {
                @if let Some(ref t) = post.tag {
                    (t)
                } @else {
                    ""
                }
              }
              author {
                name { "Pat Shaughnessy" }
              }
              summary {
                  // TODO: Has to be escaped HTML
                  (post.summary())
              }
              content {
                  (post.content)
              }
          }
        }
      }
    };
    rendered.into_string()
}
