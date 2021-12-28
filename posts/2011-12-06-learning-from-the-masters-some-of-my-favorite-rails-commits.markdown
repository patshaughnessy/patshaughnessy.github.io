title: "Learning from the masters: some of my favorite Rails commits"
date: 2011/12/6
tag: Ruby

<div style="float: left; padding: 7px 30px 10px 0px">
<table cellpadding="0" cellspacing="0" border="0">
  <tr><td><img src="https://patshaughnessy.net/assets/2011/12/6/bach.jpg"></td></tr>
  <tr><td align="center"><small><i>Reading the Rails source code is like looking<br/>at the score of a Bach sonata or partita</i></small></td></tr>
</table>
</div>

In the last month or two, there have been many great commits to Rails. Last week alone we saw: [0306f82 - implements automatic EXPLAIN logging for slow queries](https://github.com/rails/rails/commit/0306f82e0c3cda3aad1b45eb0c3a359c254b62cc) and [a382d60 - ActiveRecord::Relation#pluck method](https://github.com/rails/rails/commit/a382d60f6abc94b6a965525872f858e48abc00de). There were also many other useful commits that didn't get so much attention, such as [562583c](https://github.com/rails/rails/commit/562583c7667f508493ab8c5b1a4215087fafd22d), [85b64f9](https://github.com/rails/rails/commit/85b64f98d100d37b3a232c315daa10fad37dccdc), etc., etc. Hearing about all this great work inspired me to take a closer look at the Rails github repo history, to see which commits were the most interesting or creative - commits that would teach me something and help me improve as a Rails developer.

I ended up being surprised! There are plenty of examples of elegant code, cool testing techniques and great new features in Rails if you go and look for them, but what impressed me the most were the small things.DELIM Like the small, in-between notes and simple harmonies in the midst of a large symphony, the Rails commits that caught my eye were tiny changes made by the Rails core team that were easy to miss, but that showed their real passion and love for Rails.

Read on to find out more about some of my favorite Rails commits... and then let me know what you think! Which are your favorites?

## Taking the time to make small improvements

<div style="float: right; padding: 15px 0px 10px 30px">
<table cellpadding="0" cellspacing="0" border="0">
  <tr><td><img src="https://patshaughnessy.net/assets/2011/12/6/polishing.jpg"></td></tr>
  <tr><td align="center"><small><i>No, this is not DHH</i></small></td></tr>
</table>
</div>


[David Heinemeier Hansson (DHH)](http://david.heinemeierhansson.com/), as we all know, created Rails back in 2004. It’s hard to believe, but that’s almost eight years ago now! But even after all these years, DHH is still making commits to Rails code. And looking at some of his recent commits DHH seems just as interested in and passionate about Rails as he must have been back in 2004 when he created it.

One recent ActiveSupport commit from DHH that caught my eye, [9482554 - Added Array#prepend as an alias for Array#unshift and Array#append as an alias for Array](https://github.com/rails/rails/commit/9482554f31f3ac7f941e6239890c60fcc01975e1), might not seem interesting at all at first glance. Let’s take a look:

![9482554](https://patshaughnessy.net/assets/2011/12/6/9482554.png)

Here DHH has added a couple of new aliases - or alternative method names - for the commonly used Array operations of <span class="code">&lt;&lt;</span> and <span class="code">unshift</span>, which have somewhat confusing names in core Ruby. His code comments say it all: “The human way of thinking about...” He felt these aliases better reflected the way developers - humans - think about these Array operations and made the change in ActiveSupport to allow Rails apps, at least, to be more readable.

Yes, all there is to this commit are a couple calls to <span class="code">alias_method</span> and nothing more. What real value is there here? It seems like a minor detail not worth mentioning at all - in fact, if you open the [git commit page](https://github.com/rails/rails/commit/9482554f31f3ac7f941e6239890c60fcc01975e1) you’ll see a small debate about whether or not “append” and “prepend” are actually a good idea at all, since they probably belong in Rails core, and not just in ActiveSupport. Wow - DHH can’t even make a tiny change without sparking a minor debate! However, the point is: why in the world have I selected to highlight this small Rails commit among all the other thousands of commits DHH has made over the years?

Well, to me it shows his passion: 8 years later DHH still cares enough about Rails to think of, write, test and commit a change this small. In my mind DHH invented the fastest race car on the block, and he seems to be the sort of guy you would see out there on a Summer weekend afternoon shining up the chrome on his front bumper or hubcap! I ask myself: “Do I treat my code with the same amount of love and care?” “Have I gone back to an app I wrote years ago to make a change this simple, just for the sake of readability?”

The accompanying test code is also super simple - but at the same time very readable and explanatory:

![9482554 tests](https://patshaughnessy.net/assets/2011/12/6/9482554_tests.png)

 Reading this I know exactly what “append” and “prepend” do, and how to use them. No documentation needed; end of story.

## Code Gardening

<div style="float: right; padding: 15px 0px 10px 15px">
<table cellpadding="0" cellspacing="0" border="0">
  <tr><td><img src="https://patshaughnessy.net/assets/2011/12/6/weeding.jpg"></td></tr>
</table>
</div>

Another Rails commit that I came across this week is [fb6b805 - code gardening: we have assert_(nil|blank|present), more concise, with better default failure messages - let's use them](https://github.com/rails/rails/commit/fb6b80562041e8d2378cad1b51f8c234fe76fd5e), made by [Xavier Noria (@fxn)](http://hashref.com/), back in August of 2010. The commit message is certainly memorable: “code gardening...” what could Xavier be talking about? When I saw this commit message I just had to take a closer look. When I looked at the changes, at first glance they appeared to be nothing... just a few minor changes to a test file (I'm showing two here - there are more similar changes in the commit):

![fb6b805](https://patshaughnessy.net/assets/2011/12/6/fb6b805.png)

What? Yes, I know; now you’re thinking: “Pat is completely nuts.” This is a trivial change that adds no value whatsoever to Rails at all! Just last week Xavier made the fantastic change to ActiveRecord that I mentioned above, git commit [0306f82](https://github.com/rails/rails/commit/0306f82e0c3cda3aad1b45eb0c3a359c254b62cc), that implements automatic explain plans for slow queries... doesn’t this creative, useful change from Xavier deserve more attention than a very minor refactor that no one even noticed from 2010? Commit [fb6b805](https://github.com/rails/rails/commit/fb6b80562041e8d2378cad1b51f8c234fe76fd5e) does nothing more than change a few lines of test code - what’s the value in that? With this commit he hasn’t added even a single feature or bug fix to Rails. Not only that, the tests don’t run faster or work any better than they did before his change, except possibly they might display a more helpful error message.

But I found Xavier’s commit message to be noteworthy - more than that - profound. I think it’s a fascinating analogy: he views his role as a Rails core committer to be similar to that of a gardener. He has something beautiful and alive that he’s been trusted to take care of. Like a real garden, the Rails source code would become full of weeds, grow out of control, or worse yet actually die, if it weren’t for the constant attention and care he and the rest of the core team give to the Rails code base. In addition to adding new features to Rails, Xavier also takes the time to keep a close eye on the small details that most people wouldn’t even notice. Seeing this commit, I asked myself: “When was the last time I went through my test code, looking for places where things weren’t quite right, or as clean as they could be?”

Another example of Xavier serving as a “code gardener” is commit [d352e0d](https://github.com/rails/rails/commit/d352e0dff2c88bc1519ee7040b8381231b2b2fac). In this case, the commit message is again the most interesting part of the commit! Take a look:

<pre type="console">
commit d352e0dff2c88bc1519ee7040b8381231b2b2fac
Author: Xavier Noria <fxn@hashref.com>
Date:   Sat Jul 23 14:41:30 2011 +0200

    checked all .rb files in the project tree for missing magic comments, one was missing
    
    Came with this one-liner for this:
    
        find . -name '*.rb' | \
        xargs chardet | \
        grep -v ascii | \
        cut -d: -f1 -s | \
        xargs -n1 ruby -0777 -ne 'puts $FILENAME if $_ !~ /#.*coding:\s*utf/i'
    
    Welcome $_.
</pre>

If you [open the commit](https://github.com/rails/rails/commit/d352e0dff2c88bc1519ee7040b8381231b2b2fac) in github, you’ll see:

![d352e0d](https://patshaughnessy.net/assets/2011/12/6/d352e0d.png)

In this example, there aren’t even any Ruby code changes at all!  What Xavier has done here is run a search through the Rails source tree to find Ruby code files that were  missing the UTF-8 encoding comment at the top. Whatever you think of this style and the need to include this comment in every file, what impresses me about this is that Xavier, in his code gardener role, took the time to write a shell script to search for this mistake. He showed interest, concern, initiative, and passion - like a gardener would have while removing weeds from a flower bed.

If you’re interested, there’s another similar commit, [b451de0](https://github.com/rails/rails/commit/b451de0d6de4df6bc66b274cec73b919f823d5ae), made by [Santiago Pastorino](https://github.com/spastorino) also in August 2010, that removed extra, trailing whitespace. Like Xavier, he also saved the shell script he used in the commit message:

<pre type="console">
commit b451de0d6de4df6bc66b274cec73b919f823d5ae
Author: Santiago Pastorino <santiago@wyeworks.com>
Date:   Sat Aug 14 02:13:00 2010 -0300

    Deletes trailing whitespaces (over text files only find * -type f -exec sed 's/[ \t]*$//' -i {} \;)
</pre>

## Code that teaches you something

<div style="float: right; padding: 15px 0px 10px 15px">
<table cellpadding="0" cellspacing="0" border="0">
  <tr><td><img src="https://patshaughnessy.net/assets/2011/12/6/chalkboard.jpg"></td></tr>
</table>
</div>

The last Rails code commit I’ll talk about today is also in ActiveSupport, and has to do with the ActiveSupport::Concern module. For some background on what it does refer to the [Ruby on Rails API docs page](http://api.rubyonrails.org/classes/ActiveSupport/Concern.html). In a nutshell, ActiveSupport::Concern makes it easier to write a module that extends the class methods and/or the instance methods of another module. This is used extensively in ActiveRecord, among many other places in Rails, and is a great tool for gem authors to use while overriding existing behavior.

To repeat some of the info from the docs page, the way you typically use ActiveSupport::Concern would be to write a module like this:

<pre type="ruby">
module MyModule
  extend ActiveSupport::Concern
  module ClassMethods
    ...
  end
  module InstanceMethods
    ...
  end
end
</pre>

Now if you include “MyModule” in some other module, for example in “Host” to use the same name as the API docs page, like this:

<pre type="ruby">
class Host
  include MyModule
end
</pre>

...ActiveSupport::Concern will add all the methods in MyModule::ClassMethods to Host as new class methods, and the methods in MyModule::InstanceMethods to Host as new instance methods.

Now let's take a look at commit [401393b](https://github.com/rails/rails/commit/401393b6561adc1ce7101945163c9601257c057a) made by José Valim just last month:

![d352e0d](https://patshaughnessy.net/assets/2011/12/6/401393b_message.png)

I love the commit message, but what I like more is that this commit helps me be a better Rails developer. It turns out the InstanceMethods module is unnecessary. Simply adding methods to MyModule is sufficient; any methods in MyModule will be included in Host because of the call to include (class methods, however, still work the same way as before). Let’s take a look at José’s code changes:

![d352e0d](https://patshaughnessy.net/assets/2011/12/6/401393b.png)

There are more changes I’m not showing here; [refer to github](https://github.com/rails/rails/commit/401393b6561adc1ce7101945163c9601257c057a) for the complete commit. What’s interesting and helpful about this is that José deprecates the behavior related to the InstanceMethods module: that is, he leaves it in, but takes the time to add a line of code that displays this message whenever a developer uses InstanceMethods:

<pre type="console">
The InstanceMethods module inside ActiveSupport::Concern will be no longer included automatically. Please
define instance methods directly in #{base} instead.
</pre>

For me this is a masterful commit. José has:
<ul>
  <li>Found something verbose and unnecessary in a pattern Rails developers typically use,</li>
  <li>Developed a plan to remove it, and:</li>
  <li>Made a change that informs other developers what they are doing wrong and how they could improve, while retaining the existing API contract so Rails apps don't break unnecessarily.</li>
</ul>

In other words, José has written code that can teach us all something! Fantastic! On my projects, I’m content with just getting something to work; if I work hard then I can usually manage to write something in a test driven manner and be sure it works. However, I’ve never written code that would anticipate what other developers who are using my code would do wrong and teach them something! In fact, until seeing this commit I don’t think I could ever have even imagined trying to do this!

There are many other examples of this sort of deprecation message in Rails; this is just one of the latest. The Rails core team is not only concerned with their code, but our code!

## Which are your favorite Rails commits?

I hope the Rails commits I’ve highlighted here today will inspire you to be a better developer! Beyond just showing coding talent, these commits speak to a philosophy and mindset that I think we can all learn from.

However, there are thousands and thousands of commits in Rails: lots of other good examples of great code,  elegant solutions to difficult problems, sophisticated testing techniques, etc., etc. Which are you favorites? Where should we all look in Rails to learn something? Leave a comment and let all of us know!
