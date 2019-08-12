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
    let now = now.to_rfc3339_opts(SecondsFormat::Secs, true);
    let rendered = html! {
      feed xmlns="http://www.w3.org/2005/Atom" {
        title { "Pat Shaughnessy" }
        id { "http://patshaughnessy.net" }
        updated { 
          (now)
        }
        author {
          name { "Pat Shaughnessy" }
        }
        @for post in all_posts.iter().take(10) {
          @let absolute_url = format!("http://patshaughnessy.net/{}", post.url);
          @let date = post.date.to_rfc3339_opts(SecondsFormat::Secs, true);
          entry {
            title {
              (post.title)
            }
            link href={ (absolute_url) } rel="alternate" { }
            id href={ (absolute_url) } rel="alternate" { }
            published { 
              (date)
            }
            updated { 
              (date)
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
            summary type="html" {
                (post.summary())
            }
            content type="html" {
                (post.content)
            }
          }
        }
      }
    };
    rendered.into_string()
}
