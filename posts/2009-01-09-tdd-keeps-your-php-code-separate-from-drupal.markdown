title: TDD keeps your PHP code separate from Drupal
date: 2009/01/09
tag: Drupal

<p>Many Drupal projects that I&rsquo;ve been involved with during the past few years have all suffered from different variations on the same problem: the custom PHP code added by me and the development team I was in has ended up being highly coupled with the Drupal framework. At best, our code was hard to understand without knowing a lot about Drupal internals and could never run on its own outside of Drupal. At worst, our code changes were made right inside Drupal core functions and needed to be identified, reimplemented and merged into every newer version of Drupal whenever we decided to upgrade.</p>
<p>In hindsight, I believe that we could have avoided all of these problems if we had only used Test Driven Development (TDD). Using TDD has a lot of benefits for any software project which I won&rsquo;t go into here (see <a href="http://en.wikipedia.org/wiki/Test-driven_development#Benefits">Wikipedia</a> or <a href="http://butunclebob.com/ArticleS.UncleBob.TheThreeRulesOfTdd">Bob Martin&#x27;s explanation</a> for more information) but I believe the biggest benefit of using TDD specifically on a Drupal development project is that it makes it easy to write PHP code that is not highly coupled with Drupal, is easy to distinguish from Drupal&rsquo;s core modules, and can even run on its own outside of Drupal entirely.</p>
<p>The same is probably true while developing inside any application development framework.</p>
<p>To explore this topic in detail and find out whether I was right about this, in December I started experimenting with Drupal and TDD. After <a href="http://patshaughnessy.net/2008/12/12/writing-your-first-phpunit-test-in-drupal">I wrote my first Drupal PHPUnit test</a>, I started to <a href="http://patshaughnessy.net/2008/12/19/using-tdd-to-write-a-drupal-module">write a simple example module using TDD</a>, showing how the test first thought process can work with PHP and Drupal. Below I&rsquo;ll take a look at the finished example module to see whether the module&rsquo;s code is any less coupled to Drupal, and whether it&rsquo;s any easier to identify and maintain.</p>
<p>But first let&rsquo;s take a closer look at the 3 types of PHP code any Drupal web site will contain:</p>
<ol>
  <li><b>Drupal Code</b>: This is the PHP code you download from <a href="http://drupal.org">drupal.org</a>: files such as node.module, common.include, index.php, etc. You can assume it all works properly, was tested by the Drupal community and is used in thousands of other web sites successfully.</li>
  <li><b>Custom Code</b>: This is code you write to actually implement the special behavior your business or community needs in its web site. You need to write and maintain this, and you need to test that it works properly since no one else will use it.</li>
  <li><b>Connection Code</b>: This is code you have to write to connect your custom code to Drupal. Unfortunately, there&rsquo;s no way to magically drop your business logic into a Drupal site and see it appear in a web page, ready for users. You need to know something about how Drupal actually works, and you need to connect your Custom Code with the Drupal Code properly.</li>
</ol>
<p>A good way to understand better the difference between Drupal, Custom and Connection code is to imagine that next year you decide to upgrade your site from Drupal 6.x to Drupal 7, or that you decide to rebuild your site with Ruby on Rails or some other new technology. What you would have to do is:</p>
<ul>
  <li>Discard the Drupal Code, and replace it with the new framework.</li>
  <li>Discard your Connection Code, and write the necessary new code required to connect to the new framework.</li>
  <li>Take your Custom Code with you, possibly retaining the PHP unchanged for Drupal 7, or else rewriting it using Ruby or some other language.</li>
</ul>
<p>Clearly you need to know which code is which or else you&rsquo;ll discard code you really need! TDD can make this easy; the reason why is that using PHPUnit forces you to structure your Custom Code so that it can run outside of Drupal from the command line. Knowing that your Custom Code can run outside of Drupal inside of a unit test suite guarantees that you will be able to take the code with you and later connect it to another framework.</p>
<p>Let&rsquo;s see how my example module turned out; see <a href="http://patshaughnessy.net/2008/12/9/example-drupal-module-to-use-for-tdd-demonstration">one of my previous posts</a> for more background on what this module does and how to set it up. Here&rsquo;s the finished code:</p>
<ul>
  <li>My Custom Code: <a href="http://patshaughnessy.net/code/drupal-tdd-3/Tdd.php.txt">Tdd.php</a> and <a href="http://patshaughnessy.net/code/drupal-tdd-3/TddTests.php.txt">TddTests.php</a></li>
  <li>My Connection Code: <a href="http://patshaughnessy.net/code/drupal-tdd-3/tdd.module">tdd.module</a></li>
  <li>And the Drupal “info” file: <a href="http://patshaughnessy.net/code/drupal-tdd-3/tdd.info">tdd.info</a></li>
</ul>
<p>What I&rsquo;ve done since <a href="http://patshaughnessy.net/2008/12/19/using-tdd-to-write-a-drupal-module">my last post</a> while finishing up this simple example is to separate the Custom Code and Connection Code into two separate files. First, here&rsquo;s my finished Custom Code:</p>
<pre>&lt;?php
function tdd_search_for_titles($query, $ascending, $from, $count) {
  $titles = array();
  if ($query != NULL) {
    $sql = &quot;SELECT title FROM {node} WHERE title LIKE &#x27;%%%s%%&#x27;&quot;;
    if ($ascending) {
      $sql .= &#x27; ORDER BY title&#x27;;
    }
    else {
      $sql .= &#x27; ORDER BY title DESC&#x27;;
    }
    if ($from || $count) {
      $result = db_query_range($sql, $query, $from, $count);
    }
    else {
      $result = db_query($sql, $query);
    }
    while ($node = db_fetch_object($result)) {
      $titles[] = $node-&gt;title;
    }
  }
  return $titles;
}
?&gt;</pre>
<p>This probably could be refactored even more into 2 or 3 simpler methods, but the question here is whether this is Custom Code, or Connection Code. I know this is my Custom Code because:</p>
<ul>
  <li>This is the code that actually implements the behavior I am trying to achieve. In this simple example the code is searching for nodes by words in their titles.</li>
  <li>This code is not at all coupled to the Drupal framework. This function is easy to understand for any PHP developer, even someone who knows nothing about Drupal. The only minor exception here is that it uses 3 simple utility Drupal functions: db_query(), db_query_range() and db_fetch_object(). However, these are self-explanatory; If I were a purist, I could have used the mysql_query() function instead, and have eliminated all of my dependencies on Drupal entirely. The code also assumes the presence of the node table and the title column within it, but the same code would work on any database table containing web pages and their titles with only trivial changes. The point is that the code is easy to understand and would be easy to migrate to another framework.</li>
  <li>This code can run outside the Drupal framework. I know this is the case since I do this when I run PHPUnit on my tests in <a href="http://patshaughnessy.net/code/drupal-tdd-3/TddTests.php.txt">TddTests.php</a>:<pre>$ cd ~/htdocs/drupal3
$ phpunit TddTests modules/tdd/TddTests.php 
PHPUnit 3.2.21 by Sebastian Bergmann.
............
Time: 0 seconds
OK (12 tests)</pre></li>
</ul>
<p>The <a href="http://patshaughnessy.net/code/drupal-tdd-3/TddTests.php.txt">test code</a>, which I won&rsquo;t repeat here, is also an essential part of my Custom Code, since it is the only way I have to prove that my function is working properly. It also documents my code&rsquo;s the desired behavior, and finally will allow me to validate that the code is working if I ever move it to a newer version of Drupal or to some other technology. Also, if you take a look at <a href="http://patshaughnessy.net/code/drupal-tdd-3/TddTests.php.txt">TddTests.php</a>, you&rsquo;ll see that the test code is also not highly coupled to Drupal. There are a few references to node_save() and drupal_bootstrap() for example, but most of the test code is pure PHPUnit and has nothing to do with Drupal.</p>
<p>Let&rsquo;s take a look at my Connection Code, which is in the <a href="http://patshaughnessy.net/code/drupal-tdd-3/tdd.module">tdd.module</a> file. I won&rsquo;t repeat all of it here in this page,  but if you look at <a href="http://patshaughnessy.net/code/drupal-tdd-3/tdd.module">tdd.module</a> you can see the code does not have these 3 qualities:</p>
<ul>
  <li>This code has nothing to do with the behavior I&rsquo;m trying to implement. It simply provides URL parameters from the user&rsquo;s request to tdd_search_for_titles(), and displays the results from tdd_search_for_titles() in a web page. It also handles the complexity around the Drupal Form API.</li>
  <li>This code is very coupled to the Drupal framework. For someone who doesn&rsquo;t understand Drupal internals <a href="http://patshaughnessy.net/code/drupal-tdd-3/tdd.module">tdd.module</a> is very hard to understand &ndash; even for developers with years of PHP experience.</li>
  <li>This code could never run outside of the Drupal framework. This is because it uses functions such as drupal_get_form() and theme() that would be impossible  - and pointless - to implement outside of Drupal. More importantly, the functions in <a href="http://patshaughnessy.net/code/drupal-tdd-3/tdd.module">tdd.module</a> are called by Drupal at certain times during its processing; none of this would make any sense outside of Drupal.</li>
</ul>
<p>Here&rsquo;s the only interesting snippet from my Connection Code:</p>
<pre>...
$sortAscending = true;
if (isset($_GET[&#x27;sort&#x27;]) &amp;&amp; $_GET[&#x27;sort&#x27;] == &#x27;desc&#x27;) {
  $sortAscending = false;
}
$titles = tdd_search_for_titles($keys, $sortAscending, 0, 10);
$rows = array();
foreach ($titles as $title) {
  $rows[] = array($title);
}
...</pre>
<p>This is the actual location in the code where we &ldquo;connect&rdquo; from Drupal to my Custom Code, and vice-versa. This few lines are actually passing (connecting) the request parameters onto my Custom Code, and later saving the results in $titles which is parsed and returned to Drupal in the required format.</p>
<p>In my next post I&rsquo;ll try using TDD again to write a more complex and interesting Drupal module: one that will display monthly archive links - e.g. January 2009 (23) - for a Drupal blog, similar to what you would see in a standard <a href="http://wordpress.org/">WordPress</a> or <a href="http://b2evolution.net/">B2Evolution</a> blog site.</p>

