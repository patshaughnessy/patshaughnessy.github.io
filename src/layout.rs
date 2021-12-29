extern crate maud;

use self::maud::html;
use self::maud::DOCTYPE;
use self::maud::PreEscaped;

pub mod post;
pub mod home_page;
pub mod rss;

pub fn render(content: String, title: Option<&String>) -> String {
    let rendered = html! {
      (DOCTYPE)
      html lang="en" {
        head {
          meta charset="utf-8";
          title {
            @if let Some(t) = title{
              (t) " - Pat Shaughnessy"
            } @else {
              "Pat Shaughnessy"
            }
          }
          meta name="description" content="";
          meta name="author" content="";
          meta name="viewport" content="width=device-width, initial-scale=1";
          link rel="stylesheet" href="/assets/css/normalize.css";
          link rel="stylesheet" href="/assets/css/skeleton.css";
          link rel="alternate" type="application/atom+xml" title={ "Pat Shaughnessy - Feed" } href="http://feeds2.feedburner.com/patshaughnessy";
          link rel="icon" type="image/png" href="images/favicon.png";
        }
        body {
          div id="banner" {
            a href="/" {
              span id="title" {
                "Pat Shaughnessy"
              }
              span id="tagline" {
                " blogger, rubyist, aspiring author"
              }
            }
          }
          div class="container" {
            div class="row" {
              div class="one-half column" style="margin-top: 35px" {
                (PreEscaped(content))
                div class="row" id="copyright" {
                  p {
                    "Content and UI design "
                    (PreEscaped("&copy;"))
                    " 2022 Pat Shaughnessy"
                  }
                }
              }
            }
          }
        }
      }
    };
    rendered.into_string()
}
