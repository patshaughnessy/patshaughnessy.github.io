extern crate maud;
extern crate ordinal;

use self::maud::html;
use self::maud::PreEscaped;
use chrono::Datelike;
use self::ordinal::Ordinal;

use post::Post;

pub fn render(post: &Post, all_posts: &Vec<Post>, draft: bool) -> String {
    let month_string = post.date.format("%B").to_string();
    let day_string = Ordinal(post.date.day());
    let year_string = post.date.format("%Y").to_string();
    let date_string = format!("{} {} {}", month_string, day_string, year_string);
    let recent_posts = all_posts.iter()
        .filter(|p|
                (p.tag == post.tag || post.tag.is_none())
                && *p != post
        ).take(4);
    let recent_links = recent_posts.map(|p|
        (
            &p.url,
            &p.title
        )
    );
    let content = html! {

      div class="ten columns" {
        div class="container" {
          div class="row" {
            article class="post" {
              header {
                h1 { (post.title) }
                div class="metadata" {
                  (date_string)
                  (PreEscaped("&nbsp;&mdash;&nbsp;"))
                  a href="#disqus_thread" data-disqus-identifier={ "https://patshaughnessy.net/" (post.url) } {
                      (PreEscaped("&nbsp; Comments and &nbsp; Reactions"))
                  }
                }
              }
              section class="content" { (PreEscaped(&post.content)) }
              @if !draft {
                section class="comments" {
                  div id="disqus_thread" {
                    script type="text/javascript" {
                      (PreEscaped(r#"var disqus_identifier = '"#))
                      "https://patshaughnessy.net/"
                      (post.url)
                      (PreEscaped(r#"'; var disqus_shortname = 'patshaughnessy'; var disqus_title = '"#))
                      (post.title)
                      (PreEscaped(r#"';"#))
                    }
                  }
                  script type="text/javascript" src="https://disqus.com/forums/patshaughnessy/embed.js" { }
                  noscript {
                    a href="https://patshaughnessy.disqus.com/?url=ref" {
                      "View the discussion thread."
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
          @if let Some(ref t) = post.tag {
            div class="header" {
              "More on " (t)
            }
            div class="links" {
              ul {
                @for (link_url, link_title) in recent_links {
                  li {
                    a href={ "/" (link_url) } {
                      (link_title)
                    }
                  }
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


          @if !draft {
            script type="text/javascript" {
              (PreEscaped(r#"(function () {
        var s = document.createElement('script'); s.async = true;
        s.type = 'text/javascript';
        s.src = 'https://' + disqus_shortname + '.disqus.com/count.js';
        (document.getElementsByTagName('HEAD')[0] || document.getElementsByTagName('BODY')[0]).appendChild(s);
    }());"#))
            }
          }
    };
    content.into_string()
}
