title: "One Chapter From My Upcoming eBook: Ruby Under a Microscope"
date: 2012/5/9

<div style="float: left; padding: 7px 30px 10px 0px">
<table cellpadding="0" cellspacing="0" border="0">
  <tr><td><img src="http://patshaughnessy.net/assets/2012/1/4/microscope.jpg"></td></tr>
  <tr><td align="center"><small><i>In my eBook I’ll explore Ruby internals<br/>using an experimental approach.</i></small></td></tr>
</table>
</div>

<b>Update:</b> I just setup an [email sign up form](http://patshaughnessy.net/ruby-under-a-microscope) - if you give me your email address I'll send you one - count ’em - one email messsage when the eBook is finished.

You may have noticed I haven’t been blogging here for a while; instead I’ve been working on a new eBook about Ruby language internals. I find this topic fascinating! In fact many of my blog posts this year have been about how Ruby, JRuby and Rubinius actually work on the inside, “under the hood.” Instead of continuing to blog about it, I’ve decided to write an entire book which I plan to release this Fall. For now, I’m calling the book “<u>Ruby Under a Microscope - Learning Ruby Internals Through Experiment</u>.”

Today I’m excited to release a preview of one chapter of the eBook called “How Hashes Scale From One To One Million Elements.” Here’s a link to it:

<a href="http://patshaughnessy.net/Ruby-Under-a-Microscope-Rough-Draft-May.pdf"><img src="http://patshaughnessy.net/assets/2012/5/9/pdficon_large.png"/></a>&nbsp;<a href="http://patshaughnessy.net/Ruby-Under-a-Microscope-Rough-Draft-May.pdf">Ruby Under A Microscope</a>

I might release another chapter in the next couple of months, or at least some excerpts from the book as blog posts. Ultimately I might charge some money for the finished version. To hear when the book is finished and available, just follow me on Twitter and/or subscribe to my RSS feed.

It was a blast to write and I hope it will be a relatively quick, fun read for you. I realize it’s very long - at 41 pages you’ll probably have to put it on your “read-it-later” list. But if you’re interested in the topic of Ruby internals I’d love to know what you think about either the chapter’s writing style or content.
If you take the time to download and read it, here’s what you’ll find inside:

## Table of Contents

<ul>
  <li>Introduction</li>
  <ul>
    <li>Why bother to study Ruby internals?</a</li>
    <li>My approach in this book: theory and experiment</a></li>
  </ul>
  <li>How Hashes Scale From One To One Million Elements</a></li>
  <ul>
    <li>Theory: Hash tables in Ruby</a></li>
    <li>Experiment 1: Retrieving a value from hashes of varying sizes</a></li>
    <li>Theory: How hash tables expand to accommodate more values</a></li>
    <li>Experiment 2: Inserting one new element into hashes of varying sizes</a></li>
    <li>Theory: Why Hashes will be faster in Ruby 2.0</a></li>
    <li>Experiment 3: Inserting one new element into hashes of varying sizes, for Ruby 2.0</a></li>
    <li>Theory: How Ruby implements hash functions</a></li>
    <li>Experiment 4: Using objects as keys in a hash</a></li>
    <li>Theory: How Ruby saves order information in hashes</a></li>
    <li>Experiment 5: Iterating over elements inserted into a Hash</a></li>
    <li>Alternate theories: Hashes in JRuby</a></li>
    <li>Alternate theories: Hashes in Rubinius</a></li>
    <li>Conclusion</a></li>
    <li>Appendix: Experiment Code</a></li>
  </ul>
</ul>

Many thanks a lot to everyone who read the “rough, rough draft” of this, and helped me refine my thinking and improve the text: Alex, Daniel, Rashmi, Peter, Mike, Jonathan and Amit.
