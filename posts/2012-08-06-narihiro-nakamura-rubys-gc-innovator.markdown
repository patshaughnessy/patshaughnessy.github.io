title: "Narihiro Nakamura: Ruby’s GC Innovator"
date: 2012/8/6
tag: Interviews

<div style="float: left; padding: 7px 30px 10px 0px">
<table cellpadding="0" cellspacing="0" border="0">
  <tr><td><a href="http://www.narihiro.info/index.en.html"><img src="http://patshaughnessy.net/assets/2012/8/6/narihiro-nakamura.jpeg"></a></td></tr>
  <tr><td align="center"><small><i><a href="http://www.narihiro.info/index.en.html">Narihiro Nakamura</a> has made a number of key<br/>improvements to Ruby’s GC algorithm</i></small></td></tr>
</table>
</div>

I first came across Narihiro Nakamura’s name while researching an [article about garbage collection](http://patshaughnessy.net/2012/3/23/why-you-should-be-excited-about-garbage-collection-in-ruby-2-0) I wrote back in March. He had just committed a large code change to the upcoming MRI Ruby 2.0 release enabling a new garbage collection technique called “bitmap marking,” which promises to speed up your apps by improving the way Ruby processes work with shared memory. Later I watched a video of a great presentation Narihiro did at RubyConf Argentina 2011 called [Parallel worlds of CRuby's GC](http://vimeo.com/38994805) about a garbage collection technique called “parallel marking.” Then I noticed on [his web site](http://www.narihiro.info/index.en.html) that Narihiro had committed various other GC related code changes to Ruby over the past few years. It was clear that garbage collection has been an ongoing passion for Narihiro – a passion that benefits all of us!

This month I decided to interview Narihiro for RubySource – I was curious to learn more about him and his work. Because of the language barrier we conversed via email, unlike my other RubySource interviews, but I’ve included his original Japanese answers here for those of you who understand Japanese. [Read the interview on RubySource.com](http://rubysource.com/narihiro-nakamura-rubys-gc-innovator) to learn more about Narihiro, and to get a sense of all the improvements Narihiro and the core team have made to GC over the past few years.
