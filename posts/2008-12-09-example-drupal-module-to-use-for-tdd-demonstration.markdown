title: Example Drupal module to use for TDD demonstration
date: 2008/12/09
tag: Drupal

<p>To illustrate how to use Test Driven Development while developing a Drupal module, we first need an example module to write: Let's assume we want a page in our Drupal admin console that lists all of nodes in the database, similar to the Administer-&gt;Content Management-&gt;Content page. It will also filter and sort the list, but instead of filtering on the status and type of nodes, it will filter on words contained in the node title.</p>
<p>The first thing I did was to write the module as quickly as possible as I normally would have without using TDD. Here&rsquo;s the completed module: <a href="http://patshaughnessy.net/code/demo.module">demo.module</a>, and <a href="http://patshaughnessy.net/code/demo.info">demo.info</a>. If you want to follow along this demonstration just install a fresh copy of the latest version of Drupal 6.x (Drupal 6.6 when I wrote this); be sure to create and use a new MySQL database to use in these examples so your existing work won&rsquo;t get in the way. Then download the demo.module and demo.info files into a new "demo" module folder and enable the demo module in your admin console (Administer-&gt;Site building-&gt;Modules). It will be called &quot;Demo&quot; and will appear at the bottom of the list of modules in a section called &quot;TDD Demo Modules.&quot;</p>
<p>Once the Demo module is enabled, look for a new page in your admin console at Administer-&gt;Content management-&gt;Demo Page. It will initially display a list of all the nodes in your database, and also allow you to enter a keyword to show only the nodes whose title contains the given word.</p>
<p>If you are using a new empty database, create a couple of pages (Create Content->Page) to have some data to display here. I created two pages with these titles (body text doesn't matter):
<ul>
  <li>This is my first page</li>
  <li>My second page!</li>
</ul>
</p>
<p>A quick review of the module code will reveal nothing special. Demo.module is very simple&hellip; it has some boilerplate code (demo_help and demo_menu), and then displays the new &ldquo;Demo Page&rdquo; with the demo_page_view function:</p>
<pre>function demo_page_view($keys = NULL)
{
  $header = array(
    array('data' => t('Page Title'), 'field' => 'title', 'sort' => 'asc')
  );
  $sql = "SELECT * FROM {node} WHERE title LIKE '%%%s%%'";
  $sql .= tablesort_sql($header);
  $result = pager_query($sql, 50, 0 , NULL, $keys);
  $rows = array();
  while ($data = db_fetch_object($result)) {
    $rows[] = array(check_plain($data->title));
  }
  if (empty($rows)) {
    $rows[] = array(array('data' => 'No pages match the given pattern.'));
  }
  $output = drupal_get_form('demo_pattern_form', $keys);
  $output .= "Pages matching this pattern:";
  $output .= theme('table', $header, $rows);
  $output .= theme('pager', NULL, 50, 0);
  return $output;
}</pre>
<p>This is really all there is to demo.module &ndash; this function generates a SQL statement using tablesort_sql, then selects all the matching nodes from the database using pager_query and builds an array containing the matches. Then it returns the HTML for a table containing the matching records using the currently selected theme. Later in the module file there are more functions that handle navigation and processing for the &ldquo;demo_pattern_form.&rdquo;</p>
<p>This code is so simple it&rsquo;s hard to imagine that anything could be wrong with it: in just a few lines it gets the data we need and also displays it. It&rsquo;s also very typical Drupal module code. If you look around the Drupal code base or at code from 3rd party modules you will see a lot of code that looks just like this. It uses common Drupal functions like &ldquo;theme,&rdquo; &ldquo;pager_query&rdquo; and &ldquo;db_fetch_object.&rdquo;</p>
<p>But in my opinion there are a two things wrong with this example module:
<ul>
  <li>There are no tests &ndash; if I needed to change something about the behavior of demo.module I would have no way to know whether or not I broke something after making a code change, other than by opening a browser and testing manually.</li>
<li>It&rsquo;s very hard to separate the actual business logic or custom behavior that demo.module provides from the boilerplate code required to run inside of Drupal.</li>
</ul></p>
<p>A veteran Drupal developer would disagree with the second point, of course: obviously the SQL statement used in demo_page_view really is the only custom behavior here, and all of the other calls to Drupal functions like drupal_get_form, table_sort_sql, etc., are used to help display our data in a Drupal page.
But imagine a PHP developer who wasn&rsquo;t yet very familiar with Drupal looking at this module file for the first time: how would she or he have any idea what this module did? Or where to begin to change it&rsquo;s behavior if necessary? This is another important use of tests: as living documentation for what code does and how it should work.</p>
<p>This is a silly example, but imagine if this module did something sophisticated and important to your business. Then imagine if your business decided to upgrade from one version of Drupal to another &ndash; or imagine that your business decided to use DJango, Joomla or some other CMS system&hellip; or to implement it using custom Java or Ruby code. Then the question would become: what part of the module&rsquo;s code is ours that we want to keep? And what part of the module is simply needed to coexist with Drupal? Right now it&rsquo;s not so easy to tell.</p>
<p>I believe that using TDD while writing a Drupal module will not only provide the normal benefits of testing: emergent design, living documentation and simply more robust code, but will also lead to isolating your business&rsquo;s code from the framework in a natural way. By writing unit tests first, you will have to think about what you are trying to test: your business logic only and not the Drupal framework itself.</p>
<p>In my next post we&rsquo;ll get started by installing PHPUnit and writing our first PHPUnit unit test with Drupal.</p>
