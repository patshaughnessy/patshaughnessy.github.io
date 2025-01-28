extern crate maud;
extern crate ordinal;

use self::maud::html;
use self::maud::PreEscaped;

pub fn render(draft: bool) -> String {
    let content = html! {

      div class="nine columns" {
        div class="container" {
          div class="row" {
            article class="book" {
              p {
                i {

                  "I've started working on a new edition of Ruby Under a
                  Microscope that covers Ruby 3.x. I'm working on this in my
                  spare time, so it will take a while to finish. Leave a comment
                  or "
                  a href="mailto:pat@patshaughnessy.net?subject=Ruby Under a Microscope Update" { "drop me a line" }
                  " and I'll email you when it's finished."
                }
              }
              header {
                h1 { "Have You Ever Wondered How Ruby Works Internally?" }
              }
              section class="content" {
                div style="float: right; margin: 5px 0 0 15px;" {
                  div id="book-tweet" {
                    a class="top-buy-now" href="http://nostarch.com/rum" { "Buy Now at NoStarch.com" }
                  }
                  a href="http://nostarch.com/rum" {
                    img id="book-cover" src="/assets/images/RUM_coverfront.png" {}
                  }
                }
                p {}
              }
              p {
                "Everyone knows that Ruby is a powerful language. Its dynamic nature allows you to concisely write the code you need to actually solve a problem instead of burying yourself under mountains of boilerplate. You don't have to fight it; you just get to enjoy the ride."
              }
              p {
                "But how is the magic created? And can you trust it? After all, MRI is written in C - a statically typed, compiled language which is pretty much the antithesis of Ruby. What "
                span { "dark voodoo" }
                " is breathed into C to enable Ruby's features - closures, metaprogramming, and so much more?"
              }
              h2 { "Find out with Science!" }
              p {
                "Ruby Under a Microscope will guide you through the internals of some of Ruby's
                most-used facets. Using experimentation, theory, and two truckloads of diagrams,
                you'll "
                span { "clearly see how Ruby is implemented" }
                "."
              }
              div class="diagram" {
                img src="/assets/images/tokenize.png" {}
                div class="caption" {
                  "Here's just one of the diagrams out of the two truckloads."
                }
                div class="caption" {
                  "Did you think that was a exaggeration? Well, it isn't!"
                }
              }
              p {
                (PreEscaped("Here's a science fact for you: before they understood the nature of combustion,
                 Enlightenment chemists believed fire resulted from the release of a negative-weight
                 particle called \"phlogiston\". Then Antoine-Laurent Lavoisier knocked that theory
                 on its derri&egrave;re by discovering oxygen and proving how combustion really works."))
              }
              p {
                "This book will offer you the same clarity. Instead of pointing at a Ruby block and
                 yelling, \"Phlogiston!\", you'll be able to lucidly explain how Ruby copies
                 variables from the stack to the heap in order to allow access to them even after
                 a function has returned."
              }
              p {
                "Have no clue what that means? Well, that's exactly why you need to read this book."
              }
              p {
                "And good news: you don't need to know a lick of C! If you can't tell your pointers
                 from your pointers to pointers, then have no fear. "
                span { "Ruby Under a Microscope is accessible to anyone" }
                " with an interest in learning how Ruby works internally."
              }
              h2 { "People Already Love It" }
              p {
                "Here's what people are already saying about "
                span { "Ruby Under a Microscope" }
                ":"
              }

              blockquote {
                div class="pic" {
                  img src="/assets/images/avatars/peter.png" {}
                }
                div class="content" {
                  "Many people have dug into the Ruby source code but few make it back out
                   and tell the tale as elegantly as Pat does in Ruby Under A Microscope! I
                   particularly love the diagrams - and there are lots of them - as they
                   make many opaque implementation topics a lot easier to understand,
                   especially when coupled with Pat's gentle narrative. This book is a
                   delight for language implementation geeks and Rubyists with a penchant
                   for digging into the guts of their tools."
                  footer {
                    (PreEscaped("&ndash;"))
                    cite {
                      "Peter Cooper ("
                      a href="https://twitter.com/peterc" {
                        "@peterc"
                      }
                      (PreEscaped(") &mdash; Editor of Ruby Inside and Ruby Weekly"))
                    }
                  }
                }
              }

              blockquote {
                div class="pic" {
                  img src="/assets/images/avatars/xavier.jpg" {}
                }
                div class="content" {
                  "going to proofread a draft of @pat_shaughnessy's \"Ruby Under a Microscope\", man this book was missing in the Ruby landscape, awesome content"
                  footer {
                    (PreEscaped("&ndash;"))
                    cite {
                      "Xavier Noria ("
                      a href="https://twitter.com/fxn" {
                        "@fxn"
                      }
                      (PreEscaped(") &mdash; Ruby Hero, Ruby on Rails Core Team Member"))
                    }
                  }
                }
              }

              blockquote {
                div class="pic" {
                  img src="/assets/images/avatars/santiago.jpg" {}
                }
                div class="content" {
                  "Pat Shaughnessy did a tremendous job writing THE book about Ruby internals. Definitely a must read, you won't find information like this anywhere else."
                  footer {
                    (PreEscaped("&ndash;"))
                    cite {
                      "Santiago Pastorino ("
                      a href="https://twitter.com/spastorino" {
                        "@spastorino"
                      }
                      (PreEscaped(") &mdash; WyeWorks Co-Founder, Ruby on Rails Core Team Member"))
                    }
                  }
                }
              }

              blockquote {
                div class="pic" {
                  img src="/assets/images/avatars/vlad.jpg" {}
                }
                div class="content" {
                  "I really enjoyed the book and now know have a far better understanding of both Ruby and CS - thanks. Your writing made very complex topics (at least for me) very accessible and I found the book hard to put down. Diagrams were awesome and already are popping in my head as I code. This is by far one of my top 3 favourite Ruby books written."
                  footer {
                    (PreEscaped("&ndash;"))
                    cite {
                      "Vlad Ivanovic ("
                      a href="https://twitter.com/vladiim" {
                        "@vladiim"
                      }
                      (PreEscaped(") &mdash; Digital Strategist @ MassMedia Sydney."))
                    }
                  }
                }
              }

              blockquote {
                div class="pic" {
                  img src="/assets/images/avatars/deryl.jpg" {}
                }
                div class="content" {
                  "While I'm not usually digging into Ruby Internals, your book was an absolute awesome read. Best $20 I've spent in ages."
                  footer {
                    (PreEscaped("&ndash;"))
                    cite {
                      "David Deryl Downey ("
                      a href="https://twitter.com/daviddwdowney" {
                        "@daviddwdowney"
                      }
                      (PreEscaped(") &mdash; Founder of CyberSpace Technologies Group"))
                    }
                  }
                }
              }

              h2 { "Table of Contents" }
              p {}
              table id="toc" {
                tr {
                  td class="main" {
                    "Foreword by Aaron Patterson"
                  }
                  td {
                    "xv"
                  }
                }
                tr {
                  td class="main" {
                    "Acknowledgments"
                  }
                  td {
                    "vi"
                  }
                }
                tr {
                  td class="main" {
                    "Introduction"
                  }
                  td {
                    "ix"
                  }
                }
                tr {
                  td class="main" {
                    "Chapter 1: Tokenization and Parsing"
                  }
                  td {
                    "3"
                  }
                }
                tr {
                  td class="main" {
                    "Chapter 2: Compilation"
                  }
                  td {
                    "31"
                  }
                }
                tr {
                  td class="main" {
                    "Chapter 3: How Ruby Executes Your Code"
                  }
                  td {
                    "55"
                  }
                }
                tr {
                  td class="main" {
                    "Chapter 4: Control Structures and Method Dispatch"
                  }
                  td {
                    "83"
                  }
                }
                tr {
                  td class="main" {
                    "Chapter 5: Objects and Classes"
                  }
                  td {
                    "105"
                  }
                }
                tr {
                  td class="main" {
                    "Chapter 6: Method Lookup and Constant Lookup"
                  }
                  td {
                    "133"
                  }
                }
                tr {
                  td class="main" {
                    "Chapter 7: The Hash Table: The Workhorse of Ruby Internals"
                  }
                  td {
                    "167"
                  }
                }
                tr {
                  td class="main" {
                    "Chapter 8: How Ruby Borrowed a Decades-Old Idea from Lisp"
                  }
                  td {
                    "191"
                  }
                  }
                tr {
                  td class="main" {
                    "Chapter 9: Metaprogramming"
                  }
                  td {
                    "219"
                  }
                }
                tr {
                  td class="main" {
                    "Chapter 10: JRuby: Ruby on the JVM"
                  }
                  td {
                    "251"
                  }
                }
                tr {
                  td class="main" {
                    "Chapter 11: Rubinius: Ruby Implemented with Ruby"
                  }
                  td {
                    "273"
                  }
                }
                tr {
                  td class="main" {
                    "Chapter 12: Garbage Collection in MRI, JRuby, and Rubinius"
                  }
                  td {
                    "295"
                  }
                }
                tr {
                  td class="main" {
                    "Index"
                  }
                  td {
                    "327"
                  }
                }
              }

              h2 { "About Pat" }
              div id="mugshot" {
                img src="/assets/images/pat2.jpeg" {}
              }

              p {
                "Pat Shaughnessy writes a blog about Ruby development here on this web site, "
                a href="https://patshaughnessy.net" {
                  "patshaughnessy.net"
                }
                ". Pat's articles and presentations have been featured multiple times on the Ruby
                Weekly newsletter, the Ruby5 podcast and the Ruby Show."
              }
              p {
                "When he's not at the keyboard, Pat enjoys spending time with his wife and
                two kids. Pat is also a fluent Spanish speaker and travels frequently to
                Spain to visit his wife's family."
              }

              @if !draft {
                section class="comments" {
                  div id="disqus_thread" {
                    script type="text/javascript" {
                      (PreEscaped(r#"var disqus_identifier = '"#))
                      "https://patshaughnessy.net/ruby-under-a-microscope"
                      (PreEscaped(r#"'; var disqus_shortname = 'patshaughnessy';"#))
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

