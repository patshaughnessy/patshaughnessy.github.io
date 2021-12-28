title: "Show some love for prepared statements in Rails 3.1"
date: 2011/10/22
tag: Ruby

<div style="float: left; padding: 7px 30px 10px 0px">
<table cellpadding="0" cellspacing="0" border="0">
  <tr><td><img src="https://patshaughnessy.net/assets/2011/10/22/heart.jpg"></td></tr>
  <tr><td align="center"><small><i>@Tenderlove and the rest of the Rails core team<br/> deserve some love for speeding up your app!</i></small></td></tr>
</table>
</div>

We’ve heard a lot about many of the great new features in Rails 3.1: the asset pipeline, Coffee Script, HTTP streaming and on and on. But if you’re still using ActiveRecord with a traditional SQL database, like me, then you’re probably using one of Rails 3.1’s most powerful new features without even realizing it: Prepared Statements.

Database servers such as Postgres and Oracle for years have allowed client applications to preprocess and cache specific SQL statement patterns ahead of time, later allowing the query results to be returned even faster.DELIM These “prepared statements” can be a great way to speed up frequently used SQL queries. However, until now the Rails framework never supported using them.

With Rails 3.1, [Aaron Patterson (@Tenderlove)](http://tenderlovemaking.com/) and the rest of the Rails core team managed to refactor ActiveRecord to create and cache prepared statements automatically without changing the existing ActiveRecord API. That’s a really amazing achievement, and means that many of us will get a significant performance boost just by upgrading to Rails 3.1... without writing even a single line of code!

Today I’m going to take a look at how ActiveRecord 3.1 implements prepared statements, and write a simple Ruby script that will display a log message each time your application takes advantage of them - so you’ll know how much love you need to show @Tenderlove and the rest of the Rails core team!

## Taking a closer look at a Rails 3.1 log file

If you’ve already upgraded your app to Rails 3.1, you might have noticed some subtle changes in the log file on the lines showing the SQL statements your app executes. For example, suppose I have a Rails 3.0 or Rails 2.x app with a “Person” model:

<pre type="ruby">
class Person < ActiveRecord::Base
end
</pre>

If I open a console and set the ActiveRecord log output to STDOUT, then I can see the SQL statement ActiveRecord uses to load a single person record:

<pre type="console">
$ rails c
Loading development environment (Rails 3.0.10)
ruby-1.8.7-p352 :001 > ActiveRecord::Base.logger = Logger.new(STDOUT)
ruby-1.8.7-p352 :002 > Person.find 1
  Person Load (0.1ms)  SELECT "people".* FROM "people" WHERE "people"."id" = 1 LIMIT 1
 => #&lt;Person id: 1, name: "one", age: 23, created_at...
</pre>

No surprise here: ActiveRecord has constructed a simple SELECT statement that finds a record in the “people” table with the primary key “id” set to 1.

Now after I upgrade my sample app to Rails 3.1 and try the same simple query in the console, I’ll get:

<pre type="console">
$ rails c
Loading development environment (Rails 3.1.1)
ruby-1.8.7-p352 :001 > Person.find 1
  Person Load (5.6ms)  SELECT "people".* FROM "people" WHERE "people"."id" = ? LIMIT 1  [["id", 1]]
 => #&lt;Person id: 1, name: "one", age: 23, created_at...
</pre>

Do you see the subtle difference? ActiveRecord 3.0 generated a select statement that contained a where clause: <span class="code">WHERE people.id = 1</span>, but ActiveRecord 3.1 generated a slightly different where clause: <span class="code">WHERE people.id = ?</span> followed by: <span class="code">[["id", 1]]</span>. Why is there a difference? What does the <span class="code">[["id", 1]]</span> notation mean? Why does any of this matter at all?

Note you won't see this change in the log file if you're using MySQL; more on that later...

## What are prepared statements?

I won’t take the time here today to thoroughly explain RDBMS prepared statements since there’s a wealth of information out there on the Internet. Prepared statements have been around for a long time, and actually the fact that until now Rails didn’t support them caused widespread complaints from developers coming from Java or .NET. It was just another excuse for considering Rails not “Enterprise ready.”

If you’re not familiar at all with prepared statements, then a good way to start learning would be to take the time to watch [@Tenderlove’s keynote address at RailsConf 2011](http://www.youtube.com/watch?v=kWOAHIpmLAI). He does a great job explaining the fundamental idea behind prepared statements and how much of a performance improvement we can expect for each RDBMS server, Postgres, MySQL, SQLite, etc.

For now, I’ll just show this simple diagram:

![prepared statement](https://patshaughnessy.net/assets/2011/10/22/prepared%20statement.png)

The basic idea behind prepared statements and the log message above is that the SQL statement itself is compiled once and cached for future use. This is the left portion of the log message: <span class="code">SELECT ...etc... LIMIT 1</span>. Notice that the actual value of the id column is replaced with “?”. By caching the SQL statement the DB server only needs to perform the work of compiling the SQL string and building up an execution plan once.

Later when your application needs to use the prepared statement, it just needs to provide the actual, desired value for “id.” This is known as a “bind variable.” The value <span class="code">[[“id”, 1]]</span> at the end of the log message indicates that for this SQL call the application is passing the value 1 for the bind variable “id.”

## How are prepared statements implemented in ActiveRecord 3.1?

It turns out that in Rails 3.1 prepared statement support is implemented in the ActiveRecord connection adapters, not in ActiveRecord itself. This is because each RDBMS server implements prepared statements differently, using a slightly different API. Let’s take a look at how ActiveRecord connects to SQLite, for example:

![sqlite3 stack](https://patshaughnessy.net/assets/2011/10/22/sqlite3%20stack.png)

The magic for prepared statements happens inside the SQLiteAdapter class. The “StatementPool” inner class inside SQLiteAdapter is a simple hash-based cache implementation that holds on to all of the prepared statements returned by the SQLite database server. Each time your Rails 3.1 application executes a SQL statement the SQLiteAdapter class first checks whether there already is a prepared statement generated for that SQL statement and uses it if there is. If there isn’t a cached statement, it sends the SQL string along to SQLite to be processed and converted into a new prepared statement. Here’s the algorithm:

![flowchart](https://patshaughnessy.net/assets/2011/10/22/flowchart.png)

## How much is your Rails 3.1 app taking advantage of prepared statements?

OK - now let’s find out how much love you need to send to the Ruby core team for all of this great work! We’ll do that by displaying a message in the Rails log file each time your application uses a cached prepared statement from the StatementPool class.

First, add this code to one of your Rails 3.1 apps in a new file called config/initializers/love_meter.rb. This only works for the SQLiteAdapter so you'll have to reconfigure your developement environment to use SQLite for this test.

<pre type="ruby">
ActiveSupport.on_load :active_record do

  class ActiveRecord::ConnectionAdapters::SQLiteAdapter::StatementPool
    def [](key)
      info = cache[key]
      unless info.nil?
        puts "SEND SOME LOVE TO THE RAILS CORE TEAM FOR SPEEDING UP YOUR APP!"
        puts "Using cached prepared statement for #{key}"
      end
      info
    end
  end

end
</pre>

What it does is monkey patch the StatementPool class to generate some additional log file output by overriding the <span class="code">def []</span> method. This is the method that the SQLiteAdapter uses to look for a prepared statement in StatementPool.

The value of “key” here is the actual SQL string your application is about to execute; for example: <span class="code">SELECT "people".* FROM "people" WHERE "people"."id" = ? LIMIT 1</span>.

The value of <span class="code">cache[key]</span> - what StatementPool saves in its hash - is an object representing the prepared statement object from the database.

The code checks to see if the value of <span class="code">cache[key]</span> is not nil, and if so displays a message. Now if we repeat our console exercise from above:

<pre type="console">
$ rails c
Loading development environment (Rails 3.1.1)
ruby-1.8.7-p352 :001 > Person.find 3
  Person Load (3.4ms)  SELECT "people".* FROM "people" WHERE "people"."id" = ? LIMIT 1  [["id", 3]]
 => #&lt;Person id: 3, name: "three", age: 43, created_at...
</pre>

The first time we execute a query for a person, the StatementPool class will have an empty cache, and the SQLiteAdapter class will have to send the SQL statement to the database to be compiled and processed - to be “prepared.” But during this call SQLiteAdapter saves the new prepared statement into the StatementPool cache.

Now if we load another person record:

<pre type="console">
ruby-1.8.7-p352 :002 > Person.find 1
SEND SOME LOVE TO THE RAILS CORE TEAM FOR SPEEDING UP YOUR APP!
Using cached prepared statement for SELECT  "people".* FROM "people"  WHERE "people"."id" = ? LIMIT 1
  Person Load (0.4ms)  SELECT "people".* FROM "people" WHERE "people"."id" = ? LIMIT 1  [["id", 1]]
 => #&lt;Person id: 1, name: "one", age: 23, created_at...
</pre>

... you can see that we now reuse the cached prepared statement from StatementPool and save some database execution time. Notice this was true even though I was loading a different person record, not id=3, but id=1. If you add this code to one of your Rails 3.1 apps and take a look at the log file, you’ll find out how much love you need to send to the Rails core team!

If there's any interest I'll package up this test script as a gem and support Postgres, Oracle etc., as well as SQLite. Then if you drop the gem into any Rails app it could produce a more helpful and complete report on prepared statement usage.

## No MySQL support

If you watch @Tenderlove’s presentation or if you just read the code inside the different ActiveRecord adapter classes, you’ll notice that for MySQL there’s no implementation of prepared statements in Rails (at least using the newer mysql2 gem and adapter). However, don’t blame the Rails team; the reason it’s not supported is that actually MySQL slows down when you start using prepared statements, and it turns out to be faster not to use them at all. So send that love to the Rails team anyway... they’re helping us out again by avoiding this performance bottleneck!
