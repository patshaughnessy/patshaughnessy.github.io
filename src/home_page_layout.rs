extern crate maud;

use self::maud::html;
use self::maud::PreEscaped;

use chrono::NaiveDate;
use chrono::Datelike;

use post::Post;
use super::layout;

struct MonthAndPost {
    month: Option<String>,
    post: Post
}

pub fn render(all_posts: &Vec<Post>) -> String {
    let mut last_date_var: Option<NaiveDate> = None;
    let months_and_posts = all_posts.iter().map(|p| {
        let month_and_post = match last_date_var {
            Some(last_date) if last_date.year() == p.date.year() && last_date.month() == p.date.month() =>
                MonthAndPost {
                    month: None,
                    post: p.clone()
                },
            _ => MonthAndPost {
                    month: Some(p.month_name()),
                    post: p.clone()
                }
        };
        last_date_var = Some(p.date);
        month_and_post
    });
    let content = html! {
      div class="row" {
        div class="onecol" { }
        div class="tencol white" {
          div id="about1" class="about" {
            div id="about_photo" {
              img src="/assets/images/pat2.jpg" { }
            }
            p {
              b { "Blogger" }
              " I love diving into the details of a technology, learning how it works, and then explaining it in simple terms that everyone can understand."
            }
            p {
              b { "Rubyist" }
              " I’ve written a few gems that you can find "
              a href="https://github.com/patshaughnessy" {
                "on my github profile"
              }
              "."
            }
            p {
              b { "Aspiring Author" }
              "  In 2014 I published a book called "
              a href="/ruby-under-a-microscope" {
                "Ruby Under a Microscope"
              }
              "."
            }
          }
          div id="about2" class="about" {
            p {
              b { "SitePoint" }
              " I also write for"
              a href="http://www.sitepoint.com/author/pshaughnessy" {
                " SitePoint.com"
              }
              ";  recently I've been having fun interviewing famous Rubyists and open source developers."
            }
            div id="about2-subscribe" {
              div class="header" {
                b { "Subscribe" }
                (PreEscaped("&nbsp;"))
                a href="http://feeds.feedburner.com/patshaughnessy" {
                  img src="/assets/images/feed-icon16x16.png" { }
                }
              }
              div class="links" {
                ul {
                  li {
                    a href="https://twitter.com/pat_shaughnessy" class="twitter-follow-button" data-show-count="false" data-show-screen-name="false" {
                      "Follow @pat_shaughnessy"
                    }
                    script {
                      (PreEscaped(r#"!function(d,s,id){var js,fjs=d.getElementsByTagName(s)[0];if(!d.getElementById(id)){js=d.createElement(s);js.id=id;js.src="//platform.twitter.com/widgets.js";fjs.parentNode.insertBefore(js,fjs);}}(document,"script","twitter-wjs");"#))
                    }
                    a href="http://twitter.com/pat_shaughnessy" {
                      "@pat_shaughnessy"
                    }
                  }
                }
              }
            }
            div id="about2-book" {
              div class="header" {
                b {
                  "Buy my book:"
                }
              }
              div id="about2-book-icon" {
                a href="/ruby-under-a-microscope" {
                  img src="/assets/images/book-cover.png" { }
                }
              }
              div class="header" {
                b {
                  a href="/ruby-under-a-microscope" {
                    "Ruby Under a Microscope"
                  }
                }
              }
            }
          }
          div style="clear: left" { }
          div id="all-articles" {
            h2 { "All Articles" }
          }
          table id="archive-table" {
            @for month_and_post in months_and_posts {
              tr {
                td align="right" {
                  @if let Some(m) = month_and_post.month {
                    (m)
                  } else {
                    ""
                  }
                }
                td {
                  a href=(month_and_post.post.url) {
                    (month_and_post.post.title)
                  }
                }
              }
            }
          }
        }
      }
    };
    layout::render(content.into_string(), None)
}
