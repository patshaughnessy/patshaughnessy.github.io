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
          article class="post" {
            header {
              h1 { (post.title) }
              div class="metadata" {
                span class="date" { (date_string) }
                (PreEscaped("&nbsp;&mdash;&nbsp;"))
                    a href="#disqus_thread" data-disqus-identifier={ "https://patshaughnessy.net/" (post.url) } class="date" {
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
