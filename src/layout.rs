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
      html {
        head {
          link rel="stylesheet" href="/assets/css/1140.css" type="text/css" media="screen";
          link rel="stylesheet" href="/assets/css/main.css" type="text/css" media="screen";
          link rel="alternate" type="application/atom+xml" title={ "Pat Shaughnessy - Feed" } href="http://feeds2.feedburner.com/patshaughnessy";
          meta http-equiv="Content-Type" content="text/html; charset=UTF-8";
          title {
            @if let Some(t) = title{
              (t) " - Pat Shaughnessy"
            } @else {
              "Pat Shaughnessy"
            }
          }
        }
        body {
          div id="banner" {
            div class="row" {
              div class="onecol" { }
              div class="elevencol last" {
                a href="/" {
                  span id="title" {
                    "Pat Shaughnessy"
                  }
                  span id="tagline" {
                    " blogger, rubyist, aspiring author"
                  }
                }
              }
            }
          }
          div id="container" {
            (PreEscaped(content))
            div class="row" id="copyright" {
              p {
                "Content and UI design "
                (PreEscaped("&copy;"))
                " 2019 Pat Shaughnessy"
              }
            }
          }
        }
      }
    };
    rendered.into_string()
}
