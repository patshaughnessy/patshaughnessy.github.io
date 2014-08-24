title: "The technology you never knew you were using to test your Rails site"
date: 2011/10/1

<div style="float: left; padding: 7px 30px 10px 0px">
<table cellpadding="0" cellspacing="0" border="0">
  <tr><td><img src="http://patshaughnessy.net/assets/2011/10/1/libxml2.gif"></td></tr>
  <tr><td align="center"><small><i>Libxml2 powers your BDD/integration tests<br/>
that use the default Capybara driver</i></small></td></tr>
</table>
</div>

If you’re a Rails developer using Behavior Driven Development, then you’re probably using either Cucumber features or RSpec integration tests with the [Capybara gem](https://github.com/jnicklas/capybara) to simulate what a user would do with a real browser. You probably know that by default Capybara uses a driver based on Rack to connect to your Rails app and make simulated HTTP requests, all within the same process using only Ruby.

DELIM
<br/>

But what you probably didn’t know is that indirectly Capybara uses a large, complex open source C library called [Libxml2](http://xmlsoft.org) to parse the HTML returned by your site, and to run assertions against it. It’s interesting to me that each time I run a Cucumber BDD test on a Ruby on Rails web site, the HTML I’m trying to inspect and test is passed into a C library... in other words, that my Cucumber tests are actually implemented by C code I never even knew I had on my machine.

## An example Rails BDD/integration test

Let’s start by taking a look at a typical Cucumber feature:

<pre type="console">
{^rx:Scenario:^} Viewing the home page
    {^kw:Given^} I am on the home page
     {^cl:Then^} I should see "This is a simple web page."
</pre>

This tests one of the web pages in a Rails app, the “home page,” and checks whether the text “This is a simple web page.” appears on it. You could just as easily have written this by calling Capybara directly from RSpec:

<pre type="ruby">
describe 'the text on the home page' do
  it 'should explain what to do next' do
    visit '/'
    page.should have_content('This is a simple web page.')
  end
end
</pre>

In either case there are two steps to the test:
<ul>
  <li>Load the home page using a simulated HTTP request via Rack, and</li>
  <li>Assert that “This is a simple web page” appears on that page, i.e. in the HTML response returned by your Rails web site.</li>
</ul>

In a future blog post I may look at step 1 in more detail: how does Capybara simulate the HTTP request via Rack? But for today I’ll just assume that Capybara somehow calls the target Rails application and saves the response away in an internal variable. Instead, let’s take a closer look at step 2: how does Capybara assert something about your web site’s page, the home page in this example? In other words, how does the call to <span class="code">page.should have_content(...)</span> work?

## Capybara, Nokogiri and Libxml2

It turns out that Capybara does this by parsing the HTML returned by the home page request using a gem called [Nokogiri](http://nokogiri.org/), which, in turn, uses the Libxml2 C library to perform the actual HTML parsing work. Here’s a diagram summarizing all of this:

![capybara, nokogiri and libxml2](http://patshaughnessy.net/assets/2011/10/1/capybara-libxml.png)

All of the code above the dashed line is Ruby, while the code below the line is C. I show Nokogiri straddling the dashed line, since it actually uses both C and Ruby code. Nokogiri contains “C extension code;” you can find this inside the gem’s “ext” folder. For Nokogiri you can [look here on github](https://github.com/tenderlove/nokogiri/tree/master/ext/nokogiri) to see its C extension code. C extension code allows a gem author to write functions in C that can be called by their Ruby code. C extension code, therefore, often plays the role of a bridge between Ruby and other C code in the application. When you install a Ruby gem containing C extension code, you’ll see the message “Compiling native extensions” appear - this means that Rubygems has called your C compiler to produce the native binaries needed to run the code on your platform.

Nokogiri is a high level, elegant, Ruby abstraction layer on the Libxml2 library, and provides Ruby clients with a simple way to parse XML, HTML and also to use XPath operations to search through the DOM (Document Object Model) returned by Libxml2. Nokogiri uses its C extension code to call the Libxml2 C API functions, and convert data between the format Ruby uses and the format the C Libxml2 API requires.

## Proving that Cucumber BDD tests use the Libxml2 C library

Next, let’s take a look at exactly how Nokogiri is calling Libxml2, and prove that this C library is being used in my example Cucumber test. Here’s the documentation for C Libxml2 API call used by Nokogiri to parse HTML (from: [http://xmlsoft.org/html/libxml-HTMLparser.html#htmlReadMemory](http://xmlsoft.org/html/libxml-HTMLparser.html#htmlReadMemory)):

<b>Function: htmlReadMemory</b>
<pre type="console">
htmlDocPtr	htmlReadMemory (const char * buffer, 
      int size, 
      const char * URL, 
      const char * encoding, 
      int options)
</pre>
parse an XML in-memory document and build a tree.<br/>
<i>buffer</i>: a pointer to a char array<br/>
<i>size</i>: the size of the array<br/>
<i>URL</i>: the base URL to use for the document<br/>
<i>encoding</i>: the document encoding, or NULL<br/>
<i>options</i>: a combination of htmlParserOption(s)<br/>
<i>Returns</i>: the resulting document tree<br/>

This C function is what actually parses the HTML text, and is located inside the Libxml2 library. You can see its declaration inside the HTMLparser.h header file, which on my machine is located in /usr/include/libxml2/libxml/HTMLparser.h. Depending on how you installed Libxml2 (homebrew, macports, from source) this will be located in a different location.

This is called from the Nokogiri Ruby gem’s C extension code, in the [ext/nokogiri/html_document.c](https://github.com/tenderlove/nokogiri/blob/master/ext/nokogiri/html_document.c#L98) file:

<pre type="c">
/*
 * call-seq:
 *  read_memory(string, url, encoding, options)
 *
 * Read the HTML document contained in +string+ with given +url+, +encoding+,
 * and +options+.  See Nokogiri::HTML.parse
 */
static VALUE read_memory( VALUE klass,
                          VALUE string,
                          VALUE url,
                          VALUE encoding,
                          VALUE options )
{
  const char * c_buffer = StringValuePtr(string);
  const char * c_url    = NIL_P(url)      ? NULL : StringValuePtr(url);
  const char * c_enc    = NIL_P(encoding) ? NULL : StringValuePtr(encoding);
  int len               = (int)RSTRING_LEN(string);
  VALUE error_list      = rb_ary_new();
  VALUE document;
  htmlDocPtr doc;

  /* Added by Pat: */
  fprintf(stderr, "DEBUG: the HTML passed from Capybara and Nokogiri to Libxml2 is:\n");
  fprintf(stderr, c_buffer);
  fprintf(stderr, "DEBUG: end.\n");

  xmlResetLastError();
  xmlSetStructuredErrorFunc((void *)error_list, Nokogiri_error_array_pusher);

  doc = htmlReadMemory(c_buffer, len, c_url, c_enc, (int)NUM2INT(options));
  xmlSetStructuredErrorFunc(NULL, NULL);

  if(doc == NULL) {
    xmlErrorPtr error;

    xmlFreeDoc(doc);

    error = xmlGetLastError();
    if(error)
      rb_exc_raise(Nokogiri_wrap_xml_syntax_error((VALUE)NULL, error));
    else
      rb_raise(rb_eRuntimeError, "Could not parse document");

    return Qnil;
  }

  document = Nokogiri_wrap_xml_document(klass, doc);
  rb_iv_set(document, "@errors", error_list);
  return document;
}
</pre>

I added the three fprintf lines at the top of the C function which will print out the HTML that is passed in from Capybara to Nokogiri on its way to Libxml2. Now if we manually recompile the C extension code in Nokogiri:

<pre type="console">
$ cd ~/.rvm/gems/ruby-1.8.7-p352/gems/nokogiri-1.5.0/ext/nokogiri
$ make
$ make install
/usr/bin/install -c -m 0755 nokogiri.bundle
/Users/pat/.rvm/gems/ruby-1.8.7-p352/gems/nokogiri-1.5.0/lib/nokogiri
</pre>

...and run the Cucumber test back in my Ruby app, we’ll see the HTML on it’s way to the Libxml2 library!

<pre type="console">
$ bundle exec cucumber features/view_web_page.feature
Using the default profile...
Feature: View the home page
  As a web visitor
  I want to be able to view the home page
  In order to find out what else is on this web site

  Scenario: Viewing the home page                  # features/view_web_page.feature:6
    {^s:Given I am on the home page^}                    # features/step_definitions/web_steps.rb:44
DEBUG: the HTML passed from Capybara and Nokogiri to Libxml2 is:
<!DOCTYPE html>
<html>
<head>
  <title>SimpleWebApp</title>
  <link href="/assets/application.css" media="screen" rel="stylesheet" type="text/css" />
  <script src="/assets/application.js" type="text/javascript"></script>
  
</head>
<body>

This is a simple web page.


</body>
</html>
DEBUG: end.
    {^s:Then I should see "This is a simple web page."^} # features/step_definitions/web_steps.rb:105

1 scenario ({^s:1 passed^})
2 steps ({^s:2 passed^})
0m0.217s
</pre>

Who knew so much C code was required by just a simple, elementary Cucumber integration test? I suppose this is one of the reasons for the success of the Ruby platform: it takes advantage of lower level native C libraries when necessary, but allows most developers to enjoy the elegance and succinctness of the Ruby language most of the time, without worrying about the low level details.
