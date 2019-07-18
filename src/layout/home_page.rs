extern crate maud;

use self::maud::html;
use self::maud::PreEscaped;

use post::Post;
use post_link::PostLink;

pub fn render(all_posts: &Vec<Post>) -> String {
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
            @for link in PostLink::all_from(&all_posts) {
              tr {
                td align="right" {
                  @if let Some(str) = link.date_string {
                    (str)
                  } else {
                    ""
                  }
                }
                td {
                  a href=(link.url) {
                    (link.title)
                  }
                }
              }
            }
          }
        }
      }
    };
    content.into_string()
}
