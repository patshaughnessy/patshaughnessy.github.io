title: "Using transactions in a separate database with Drupal PHPUnit tests"
date: 2009/01/19
url: /drupal-test-database
tag: Drupal

<p>
  Update April 2009:<br/>
  I&rsquo;ve started working with <a href="http://markbennett.ca">Mark Bennett</a> on phpunit_setup.inc and moved it up to a <a href="http://github.com/patshaughnessy/drupal_tdd">github repository</a>. Mark has adapted it to work with Drupal 5, and we&rsquo;re working on a number of other related ideas as well. I&rsquo;ll post updates here about our progress.</p>
<p>This month I&rsquo;ve been experimenting with using testing ideas from Ruby on Rails while developing a Drupal module. To read more, see:</p>
<ul>
  <li><a href="https://patshaughnessy.net/2009/1/19/using-mysql-transactions-with-drupal-unit-tests">Using MySQL transactions with Drupal unit tests</a></li>
  <li><a href="https://patshaughnessy.net/2009/1/16/using-a-test-database-with-drupal-unit-tests">Using a test database with Drupal unit tests</a></li>
</ul>
<p>If you want to try this out yourself, follow these instructions:</p>
<ol>
  <li><p>Edit settings.php and use an array of two values for $db_url:</p>
    <pre>$db_url["default"] = 'mysql://user:password@localhost/drupal;
$db_url["test"] = 'mysql://user:password@localhost/drupal_test';</pre></li>
  <li><p>Create a new test database in MySQL:</p><pre>CREATE DATABASE drupal_test DEFAULT CHARACTER SET utf8
                            COLLATE utf8_unicode_ci;</pre></li>
  <li>Download and save <a href="https://patshaughnessy.net/assets/code/drupal-tdd-4/phpunit_setup.inc">phpunit_setup.inc</a> somewhere in your Drupal application; for example in the &ldquo;includes&rdquo; folder.</li>
  <li>Include <a href="https://patshaughnessy.net/assets/code/drupal-tdd-4/phpunit_setup.inc">phpunit_setup.inc</a> at the top of each of your PHPUnit test classes. See one of the two articles above for example PHPUnit tests.</li>
  <li><p>Execute your PHPUnit test class from the root folder of your Drupal app:</p><pre>$ cd /path/to/your/drupal-site
$ phpunit YourClass modules/your_module/YourClassFileName.php 
PHPUnit 3.2.21 by Sebastian Bergmann.
..
Time: 0 seconds
OK (2 tests)</pre></li>
</ol>
<p>For more information about how to run PHPUnit with Drupal, see: <a href="https://patshaughnessy.net/2008/12/12/writing-your-first-phpunit-test-in-drupal">Writing your first PHPUnit test in Drupal</a>.</p>
