title: "It’s time to clean up your mess: refactoring Cucumber step definitions"
date: 2011/10/7

<div style="float: left; padding: 7px 30px 10px 0px">
<table cellpadding="0" cellspacing="0" border="0">
  <tr><td><img src="http://patshaughnessy.net/assets/2011/10/7/mess.jpg"></td></tr>
  <tr><td align="center"><small><i>My step_definitions folder looks like this...</i></small></td></tr>
</table>
</div>

This week I decided to look through my features/step_definitions folder after reading [Aslak Hellesøy’s post from Wednesday](http://aslakhellesoy.com/post/11055981222/the-training-wheels-came-off) about removing web_steps.rb. I was worried that I might need to write many more custom steps since web_steps.rb will disappear the next time I upgrade Cucumber.

What I found was a mess! First of all, there were a lot of step files and no naming convention that might help me find a particular step, and each step file was a loose collection of step definitions that weren’t organized or DRY. DELIMWorse than this, many of my Cucumber steps were written with unmaintainable code that didn’t properly use the Capybara DSL.

Today I’m going to show how you can refactor and improve your Cucumber step definitions by:
<ul>
<li>Taking more advantage of the Capybara DSL, and</li>
<li> DRYing up your step code so you have fewer UI dependencies</li>
</ul>

## My dirty laundry

At risk of personal embarrassment and professional suicide, here’s an actual Cucumber step definition from a project I’m working on now:

<pre type="ruby">
Then /^the share first file button should not appear$/ do
    page.should_not have_css("input[value='right click on first file']")
end
</pre>

This step definition isn’t all that bad; at least I’ve written a custom step using business domain language instead of calling one of the steps from web_steps.rb. However, the CSS selector string I’m using here is a bit verbose and could be improved.

Here’s a worse example:

<pre type="ruby">
Then /^I should see "([^"]*)" in the second file section$/ do |text|
  page.should have_css("#file_access_container .per_file_container:nth-child(2):contains('#{text}')")
end
</pre>

Look at that crazy CSS selector string! I wrote this myself a week or two ago and even I have no idea what it really means now... clearly it needs to be refactored.

## Using Capybara finder methods and RSpec matchers

Capybara has a fairly complex and sophisticated DSL that can help you simplify or even remove some of your CSS selector strings. In my first example above, I can replace the <span class="code">page.have_css</span> call with <span class="code">page.find_button</span>:

<pre type="ruby">
button = page.find_button('right click on first file')
</pre>

Here I’m just passing the value or visible text of the button and Capybara will return the button element to me. Then I can continue to test it’s attributes or contents further. To see a list of all the available finder methods, just look at the [lib/capbyara/node/finders.rb](https://github.com/jnicklas/capybara/blob/master/lib/capybara/node/finders.rb) file in the Capybara source code. There are finder methods such as find_field, find_link, find_by_id, etc.

One catch here is that if the button doesn’t exist Capybara will raise an exception which can prevent you from testing the negative case: that the button doesn’t exist. Instead, you can use the <span class="code">first</span> method like this:

<pre type="ruby">
Then /^the share first file button should not appear$/ do
  page.first(:xpath, XPath::HTML.button('right click on first file')).should be_nil
end
</pre>

Here I’m using a gem called [XPath](https://github.com/jnicklas/xpath), also developed by [Jonas Nicklas](http://elabs.se/blog) the author of Capybara, that contains a series of XPath related utilities. If you use XPath a lot, you may want to take the time to check that gem out.

There’s a related method called <span class="code">all</span> which returns all of the matching objects found on the page; for example:

<pre type="ruby">
page.all(:xpath, XPath::HTML.button('right click on first file'))
</pre>

... would return all the buttons with this value, name or id. Then you could test things like how many there were, or iterate through them and test each one in turn.

If you’re using RSpec in your proejct, then Capybara automatically provides you with a series of matchers which are very readable and useful. For example, I can just write:

<pre type="ruby">
Then /^the share first file button should not appear$/ do
  page.should_not have_button('right click on first file')
end
</pre>

To browse through all of the available Capybara Rspec matches, just take a look at [lib/capbyara/rspec/matchers.rb](https://github.com/jnicklas/capybara/blob/master/lib/capybara/rspec/matchers.rb). Here are some examples: have_checkbox, have_field, have_table, have_selector, etc.

<span class="reminder"><img class="tip-image" src="/images/tip.png">Tip: remember to DRY up your step code</span>

Another good way to improve your step code is to move commonly used selectors into shared utility methods. That way you can DRY up the step code and reduce the number of place where you depend on CSS class names or other user interface details. In my example, I might want to refer to the “right click” button in a few different steps. In that case I could just write a new method like this:

<pre type="ruby">
def share_button
  page.first(:xpath, XPath::HTML.button('right click on first file'))
end

Then /^the share first file button should not appear$/ do
  share_button.should be_nil
end
</pre>

## Scoping your selectors using "Within"

Here’s my other ugly Cucumber step definition again:

<pre type="ruby">
Then /^I should see "([^"]*)" in the second file section$/ do |text|
  page.should have_css("#file_access_container .per_file_container:nth-child(2):contains('#{text}')")
end
</pre>

Wow - I can’t believe I wrote this; how embarrassing! I suppose I’m just copying the CSS selector info straight from the Chrome console into my Ruby step file... I need to train myself to stop and think about writing better step code.

Anyway, the real problem here is the cryptic CSS selector string. There must be a way to break this down and make it more understandable using the Capybara DSL. First let’s examine what it means in more detail:

<ul>
  <li>Look for an element with an id of “file_access_container”</li>
  <li>Then look for an element inside that element with a class of “per_file_container”</li>
  <li>Then get the 2nd child element of that inner element and test whether it contains the given text</li>
</ul>

The first thing we can do here is to separate the first part into a separate method call, using Capybara’s <span class="code">within</span> method:

<pre type="ruby">
Then /^I should see "([^"]*)" in the second file section$/ do |text|
  within('#file_access_container') do
    page.should have_css(".per_file_container:nth-child(2):contains('#{text}')")
  end
end
</pre>

This allows you to scope your CSS or XPath selectors to a specified area of the HTML page you’re testing. It’s a great way to break up complex CSS selector strings like this.

<span class="reminder"><img class="tip-image" src="/images/tip.png">Tip: remember to DRY up your step code</span>

We can DRY this up a bit more by writing a custom “within” method. If and when the HTML ID attribute changes, I’ll just need to update one line of code. It’s likely that I’m going to write a few different steps to test this content area in different ways, all in the same step file. For example, I could write a method called “within_the_file_container:”

<pre type="ruby">
def within_the_file_container(&block)
  within('#file_access_container', &block)
end
</pre>



## Custom selectors

Now my step is a lot easier to understand, but clearly still can be improved:

<pre type="ruby">
Then /^I should see "([^"]*)" in the second file section$/ do |text|
  within_the_file_container do
    page.should have_css(".per_file_container:nth-child(2):contains('#{text}')")
  end
end
</pre>

Next let’s look at a little known feature of the Capybara DSL:  the <span class="code">add_selector</span> method. This allows you to add special, custom selectors to your app that your step definitions can refer to. You can use them just as utilities for dealing with complex HTML patterns, or you can even tie them to business or domain concepts from the page you’re trying to test. For more details on <span class="code">add_selector</span> and for more examples, check out the nice article Plataforma wrote about this a few months ago: [Improving your tests with Capybara custom selectors](http://blog.plataformatec.com.br/2011/02/improving-your-tests-with-capybara-custom-selectors/).

You define a custom selector like this:

<pre type="ruby">
Capybara.add_selector(:file) do
  xpath { |num| XPath.css(".per_file_container:nth-child(#{num})") }
end
</pre>

What this means is that I’m defining a new type of Capybara selector called “file” that will apply the given number to the given XPath selector. Now I don’t need to repeat this XPath code over and over again throughout my step code. For example, now I can write:

<pre type="ruby">
Then /^I should see "([^"]*)" in the second file section$/ do |text|
  within_the_file_container do
    find(:file, 2).should have_content(text)
  end
end
</pre>

Or I can even use:


<pre type="ruby">
Then /^I should see "([^"]*)" in the second file section$/ do |text|
  within_the_file_container do
    within(:file, 2) { page.should have_content(text) }
  end
end
</pre>

Note that right now the code inside the <span class="code">add_selector</span> definitions must be expressed as XPath expressions, and not as CSS. But using the <span class="code">XPath.css</span> function like I did above we can easily convert my CSS to XPath, and then pass it along to Capybara.

A nice side benefit here is that I get a more readable error message if there’s a Cucumber failure:

<pre type="console">
{^error:And I should see "Things.txt" in the second file section
  Unable to find file 3 (Capybara::ElementNotFound)^}
</pre>

Instead of:

<pre type="console">
{^error:And I should see "Things.txt" in the second file section
  expected css ".per_file_container:nth-child(3):contains('Things.txt')" to return something
(RSpec::Expectations::ExpectationNotMetError)^}
</pre>

## Conclusion

Aslak Hellesøy’s made a lot of great points in [his article explaining why the Cucumber team removed web_steps.rb](http://aslakhellesoy.com/post/11055981222/the-training-wheels-came-off). In a nutshell the reason is that your step definitions should reflect the business domain of your application, and therefore be less dependent on the user interface details. After sorting through the dirty laundry in my step code, I’ve realized it is equally important to pay close attention to _how your step code is written_ and not just to which steps you are writing. 


One of the benefits of using TDD is that once you get to green you can easily refactor your code to DRY it up, make it more readable or just work better. Remember to refactor your Cucumber step definition code the same way you refactor your production code or specs. And take the time to learn a bit about the Capybara DSL; using it properly can make things a lot easier to read and maintain.
