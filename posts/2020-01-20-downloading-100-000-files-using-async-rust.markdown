title: "Downloading 100,000 Files Using Async Rust"
date: 2020/1/20
tag: Rust

<div style="float: left; padding: 8px 30px 20px 0px; text-align: center; line-height:18px">
  <img src="http://patshaughnessy.net/assets/2020/1/20/traffic-light.jpg"><br/>
  <i>Rust's new async/await feature makes it <br/>
easy to stop and start asynchronous tasks</i><br/>
  <small>(from: <a href="https://commons.wikimedia.org/wiki/File:Red_and_green_traffic_signals,_Stamford_Road,_Singapore_-_20111210.jpg">Wikimedia Commons</a>)</small></i> 
</div>

Imagine if you had a text file containing thousands of URLs:

<pre>
$ cat urls.txt
https://example.com/1.html
https://example.com/2.html
https://example.com/3.html

etc...

https://example.com/99999.html
https://example.com/100000.html
</pre>

…and you needed to download all of those HTML pages efficiently. How would you
do it? Maybe a shell script using <span class="code">xargs</span> and <span
class="code">curl</span>? Maybe a simple Golang program? Go’s powerful
concurrency features would work well for this.

<div style="clear: both"></div>

Instead, I decided to try to use Rust. I’ve read a lot about safe concurrency
in Rust, but I’ve never tried it. I also wanted to learn what Rust’s new
“async/await” feature was all about. This seemed like the perfect task for
asynchronous Rust code.

*TL/DR*: [Here’s the
code](https://gist.github.com/patshaughnessy/27b1611e2c912346b929df97998d488d).
The rest of this post will explain how it works.

## Getting Started With Reqwest

There are many different Rust HTTP clients to choose from, and [apparently some
controversy](https://medium.com/@shnatsel/smoke-testing-rust-http-clients-b8f2ee5db4e6)
about which works best. Because I’m a Rust newbie, I decided simply to pick the most
popular: [reqwest](https://github.com/seanmonstar/reqwest). Request is a high
level, easy to use HTTP client, written by [Sean
McArthur](https://seanmonstar.com/). He just updated it to work with Tokio,
Rust’s new async/await engine, so this is the perfect time to try using it.
Here’s the example from the readme:

<pre type="rust">
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
</pre>

This version downloads some JSON and parses it. Notice the new <span
class="code">async</span> and <span class="code">await</span> keywords. The
main function is <span class="code">async</span> - this means that the function
becomes part of a large state machine run by Tokio. When you mark a function
asynchronous, you can then call <span class="code">await</span> inside it,
which will pause that function temporarily, allowing other asynchronous
functions to run on the same thread.

I decided to modify this to print out the number of bytes downloaded instead; you could
easily change it to save the data to a file or do whatever you want.

<pre type="rust">
let path = "https://httpbin.org/ip";
match reqwest::get(path).await {
    Ok(resp) => {
        match resp.text().await {
            Ok(text) => {
                println!("RESPONSE: {} bytes from {}", text.len(), path);
            }
            Err(_) => println!("ERROR reading {}", path),
        }
    }
    Err(_) => println!("ERROR downloading {}", path),
}
Ok(())
</pre>

This is a two step process:

* First I call <span class="code">get(path)</span> to send the HTTP GET request. Then I use <span class="code">await</span> to wait for
  the request to finish and return a result.

* Second, if the request was successful, I call <span
  class="code">resp.text()</span> to get the contents of the response body. And
  I wait again while that is loaded.

I handle the errors explicitly and always return a unit result <span
class="code">Ok(())</span> because that makes the code below simpler
when I start downloading more than one page concurrently.

Visually, I can draw the <span class="code">get</span> and <span
class="code">text</span> calls like this:

<img src="http://patshaughnessy.net/assets/2020/1/20/get-and-text.png">

First I call <span class="code">get</span> and wait, then I call <span
class="code">text</span> and wait.

But what is asynchronous about this? This reads like normal, single threaded
code. I do one thing, then I do another.

## Sending 3 Concurrent Requests

The magic happens when I have more than one request I want to make in parallel. Let’s use three hard coded path strings:

<pre type="rust">
let paths = vec![
    "https://example.com/1.html".to_string(),
    "https://example.com/2.html".to_string(),
    "https://example.com/3.html".to_string(),
];
</pre>

To download the 3 HTML files in parallel, I spawn three Tokio “tasks” and wait
for them all to complete. (This requires adding the futures crate to
Cargo.toml, which implements <span class="code">join_all</span>.)

<pre type="rust">
// Iterate over the paths.
let mut tasks: Vec<JoinHandle<Result<(), ()>>>= vec![];
for path in paths {

    // Copy each path into a new string
    // that can be consumed/captured by the task closure
    let path = path.clone();

    // Create a Tokio task for each path
    tasks.push(tokio::spawn(async move {
        match reqwest::get(&path).await {
            Ok(resp) => {
                match resp.text().await {
                    Ok(text) => {
                        println!("RESPONSE: {} bytes from {}", text.len(), path);
                    }
                    Err(_) => println!("ERROR reading {}", path),
                }
            }
            Err(_) => println!("ERROR downloading {}", path),
        }
        Ok(())
    }));
}

// Wait for them all to finish
println!("Started {} tasks. Waiting...", tasks.len());
join_all(tasks).await;
</pre>

Each Tokio task is a closure passed to the <span
class="code">tokio::spawn</span> function, marked <span class="code">async
move</span>. I create a copy of each path, using <span
class="code">path.clone()</span>, so the closure has its own copy of the path
string with its own lifetime.

The complex type annotation on the <span class="code">tasks</span> array
indicates what each call to <span class="code">spawn</span> returns: a <span
class="code">JoinHandle</span> enclosing a <span class="code">Result</span>. To
keep things simple, I handle all errors in the closure and just return <span
class="code">Ok(())</span>.  This means each <span
class="code">JoinHandle</span> contains a trivial result: <span
class="code">Result<(), ()></span>. I could have written the closure to return
some value and/or some error value instead.

After the loop is finished and all three tasks have been spawned, I call <span
class="code">join_all(tasks).await</span> to wait for them all to finish.

## Asynchronous vs Multithreaded

<div style="float: right; padding: 8px 0px 20px 30px; text-align: center; line-height:18px">
  <img src="http://patshaughnessy.net/assets/2020/1/20/traffic-light2.jpg"><br/>
  <small>(from: <a href="https://commons.wikimedia.org/wiki/File:Traffic_lights,_Zl%C3%ADn.JPG">Wikimedia Commons</a>)</small></i> 
</div>

At first glance, it looks like this code is spawning three different threads. I
even call a spawn function. A multithreaded download might look like this:

<img src="http://patshaughnessy.net/assets/2020/1/20/multithreaded.png">

We have 3 paths, so we have 3 threads. Each thread calls <span class="code">get</span> and waits, and
then calls <span class="code">text</span> and waits.

However, Rust’s Tokio engine doesn’t work that way. Instead of launching an
entirely new thread for each task, it runs all three tasks on the same thread.
I imagine three tasks running on one thread like this:

<img src="http://patshaughnessy.net/assets/2020/1/20/one-thread.png">

Each time I call <span class="code">await</span>, Rust stops one task and
starts another using the same thread. In fact, depending on how long it takes
for each task to complete, they might be run in a different order:

<img src="http://patshaughnessy.net/assets/2020/1/20/different-order.png">

There’s no way to predict ahead of time what order the tasks will run it.
That’s why I needed to copy each path string above; each task needs it own copy
of the string with its own independent lifetime because it might be run at any
time.

The only guarantee I have is that the <span class="code">join_all</span> call
at the bottom will block until all of the tasks have finished; that is, until
all of the futures I pushed onto the tasks array have completed.

## Sending 100,000 Concurrent Requests

I can scale this up to 100,000 requests by reading the URLs in from a file instead:

<pre type="rust">
fn read_lines(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(
        reader.lines().filter_map(Result::ok).collect()
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

	let paths: Vec<String> = read_lines("urls.txt")?;

etc...
</pre>

When I tried this out for the first time I was excited: How long would it take
to download 100,000 HTML pages simultaneously like this? Would it be 100,000x
faster than downloading one file? I typed <span class="code">cargo run
--release</span> to build my code in release mode and get the best possible
performance out of Rust. Asynchronous code, zero cost abstractions, no garbage
collector, this was going to be great!

Of course, it didn’t work.

What happened? The problem is the web server can't handle so many concurrent network
connections. Using my thread/task diagram, launching all 100,000 tasks might
look like this:

<img src="http://patshaughnessy.net/assets/2020/1/20/simultaneous.png">

I spawn 100,000 tasks all on to the same thread, and Tokio starts executing
them all. Each time my code above calls <span
class="code">get(&path).await</span>, Tokio pauses that task and starts
another, which calls <span class="code">get(&path).await</span> again, opening
yet another HTTP request. My laptop quickly runs out of network resources and
these tasks start to fail.

## Sending a Buffered, Concurrent Stream of 100,000 Requests

Instead, I need to limit the number of concurrent Tokio tasks - the number of
concurrent HTTP requests. I need the diagram to look something like this:

<img src="http://patshaughnessy.net/assets/2020/1/20/buffered.png">

After the first 8 tasks are started, the first 8 blue boxes on the left, Tokio
waits for at least one of them to complete before starting a 9th task. I
indicate this with the “max concurrency” arrow.

Once one of the first 8 calls to <span class="code">reqwest::get</span>
completes, Tokio is free to run a 9th task. The first "pop from buffer" arrow.
And once that 9th task or any other task completes, Tokio starts a 10th task,
etc., in this manner processing all 100,000 tasks 8 at a time.

To achieve this, I can use <span class="code">StreamExt</span> trait’s <a
href="https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.5/futures/stream/trait.StreamExt.html#method.buffer_unordered"><span
class="code">buffer_unordered</span></a> function:

<pre type="rust">
let fetches = futures::stream::iter(
    paths.into_iter().map(|path| {
        async move {
            match reqwest::get(&path).await {
                Ok(resp) => {
                    match resp.text().await {
                        Ok(text) => {
                            println!("RESPONSE: {} bytes from {}", text.len(), path);
                        }
                        Err(_) => println!("ERROR reading {}", path),
                    }
                }
                Err(_) => println!("ERROR downloading {}", path),
            }
        }
})
).buffer_unordered(8).collect::<Vec<()>>();
println!("Waiting...");
fetches.await;
</pre>

First I create an iterator which maps all of the paths to my closures, and passes
them to <span class="code">futures::stream::iter</span>.
This will create a list of futures, each one executing my closure.

At the bottom I call <span class="code">buffer_unordered</span> and pass in 8.  The
code in <span class="code">buffer_unordered</span> will execute up to 8 futures
from the stream concurrently, and then start to buffer the remaining futures.
As each task completes, each HTTP request in my example, <span
class="code">buffer_unordered</span> will pull another task out of its buffer
and execute it.

This code will slowly but steadily iterate over the 100,000 URLs, downloading
them in parallel. Experimenting with this, it doesn’t seem to matter very much
exactly what level of concurrency I pick. I found the best performance when I
picked a concurrency of 50. Using 50 concurrent Tokio tasks, it took about 30
minutes to download all one hundred thousand HTML files.

However, none of that matters. I’m not measuring the performance of Rust, Tokio
or Reqwest. These numbers have more to do with the web server and network
connection I’m using. The real performance here was my own developer
performance: With just a few lines of code I was able to write an asynchronous
I/O program that can scale as much as I would like. The <span
class="code">async</span> and <span class="code">await</span> keywords make
this code easy to write and easy to read.
