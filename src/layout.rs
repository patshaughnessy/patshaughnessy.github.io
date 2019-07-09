extern crate maud;

use self::maud::html;
use self::maud::DOCTYPE;
use self::maud::PreEscaped;

pub fn render(content: String, title: &String, date_string: &String, tag: Option<&String>) -> String {
    let rendered = html! {
      (DOCTYPE)
      head {
          link rel="stylesheet" href="/assets/css/1140.css" type="text/css" media="screen";
          link rel="stylesheet" href="/assets/css/main.css" type="text/css" media="screen";
          link rel="alternate" type="application/atom+xml" title={ (title) " - Feed" } href="http://feeds2.feedburner.com/patshaughnessy";
          meta http-equiv="Content-Type" content="text/html; charset=UTF-8";
          title {
              (title) " - Pat Shaughnessy"
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
                  "blogger, rubyist, aspiring author"
                }
              }
            }
          }
        }
        div id="container" {
          div class="row" {
            div class="onecol" { }
            div class="ninecol white" {
              article class="post" {
                header {
                  h1 { (title) }
                  div class="metadata" {
                    span class="date" { (date_string) }
                    (PreEscaped("&nbsp;&mdash;&nbsp;"))
                  }
                }
                section class="content" { (PreEscaped(content)) }
              }
            }
            div class="twocol last" id="right" {
              div id="sidebar" {
                img src="/assets/images/pat.jpg" { }
                div class="header" {
                    "Subscribe"
                }
                div class="links" {
                  ul {
                    li {
                      a href="https://twitter.com/pat_shaughnessy" class="twitter-follow-button" data-show-count="false" data-show-screen-name="false" {
                        "Follow @pat_shaughnessy"
                      }
                      a href="http://feeds.feedburner.com/patshaughnessy" {
                        img src="/assets/images/feed-icon16x16B.png" { }
                      }
                      script {
                        (PreEscaped(r#"
                          !function(d,s,id){var js,fjs=d.getElementsByTagName(s)[0];if(!d.getElementById(id)){js=d.createElement(s);js.id=id;js.src="//platform.twitter.com/widgets.js";fjs.parentNode.insertBefore(js,fjs);}}(document,"script","twitter-wjs");
                        "#))
                      }
                      a href="http://twitter.com/pat_shaughnessy" {
                        "@pat_shaughnessy"
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
                @if let Some(t) = tag {
                    div class="header" {
                      "More on Rust"
                    }
                }
                div class="links" {
                  ul {
                    li {
                      a href="/2018/6/9/from-activerecord-to-diesel" {
                        "From ActiveRecord to Diesel"
                      }
                    }
                    li {
                      a href="/2018/3/15/how-rust-implements-tagged-unions" {
                        "How Rust Implements Tagged Unions"
                      }
                    }
                    li {
                      a href="/2018/1/18/learning-rust-if-let-vs--match" {
                        "Learning Rust: If Let vs. Match"
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
            div class="row" id="copyright" {
              p {
                  "Content and UI design "
                (PreEscaped("&copy;"))
                  " 2018 Pat Shaughnessy"
              }
            }
          }
        }
      }
    };
    rendered.into_string()
}
