extern crate maud;

use self::maud::html;

use post::Post;
use post_link::PostLink;

pub fn render(all_posts: &Vec<Post>) -> String {
    let content = html! {

      div class="ten columns" {
        div class="container" {
          div class="row" {
            article class="post" {
              header {
                h1 { "All Articles" }
              }
              section class="content" {
                table id="archive-table" {
                  @for link in PostLink::all_from(&all_posts) {
                    tr {
                      td id="date" {
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
          }
        }
      }

      div class="two columns" {
        div id="sidebar" {
          img src="/assets/images/pat.jpg" { }
          div class="header" {
              "Subscribe"
          }
          div class="links" {
            ul {
              li {
                a id="feed" href="http://feeds.feedburner.com/patshaughnessy" {
                  img src="/assets/images/feed-icon16x16B.png" { }
                }
                a href="http://twitter.com/pat_shaughnessy" {
                  img width="20" height="20" src="/assets/images/twitter.svg" { }
                }
              }
            }
          }
          div class="header" {
              "Buy my book"
          }
          div class="links" {
            ul {
              li {
                a href="/ruby-under-a-microscope" {
                  img src="/assets/images/book-cover.png" { }
                }
              }
              li id="eBook" {
                a href="/ruby-under-a-microscope" {
                  "Ruby Under a Microscope"
                }
              }
            }
          }
          div class="header" {
            "Popular"
          }
          div class="links" {
            ul {
              li {
                a href="/2016/11/26/learning-to-read-x86-assembly-language" {
                  "Learning to Read x86 Assembly Language"
                }
              }
              li {
                a href="/2012/1/4/never-create-ruby-strings-longer-than-23-characters" {
                  "Never create Ruby strings longer than 23 characters"
                }
              }
              li {
                a href="/2012/3/23/why-you-should-be-excited-about-garbage-collection-in-ruby-2-0" {
                  "Why You Should Be Excited About Garbage Collection in Ruby 2.0"
                }
              }
              li {
                a href="/2013/4/3/ruby-2-0-works-hard-so-you-can-be-lazy" {
                  "Ruby 2.0 Works Hard So You Can Be Lazy"
                }
              }
            }
          }
          div class="header" {
            a href="/" {
              "More..."
            }
          }

        }
      }

    };
    content.into_string()
}
