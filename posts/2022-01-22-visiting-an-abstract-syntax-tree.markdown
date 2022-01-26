title: "Visiting an Abstract Syntax Tree"
date: 2022/01/22
tag: Crystal

<div style="float: left; padding: 8px 30px 0px 0px; text-align: center; line-height:18px">
  <img src="https://patshaughnessy.net/assets/2022/1/22/visit-tree.jpg"><br/>
  <i>Joshua Tree National Park
  <small>(via: <a href="https://commons.wikimedia.org/wiki/File:Backpacker_at_Sunset_(22849298523).jpg">Wikimedia Commons</a>)</small>
  </i>
</div>

In my [last
post](https://patshaughnessy.net/2021/12/22/reading-code-like-a-compiler), I
explored how [Crystal](https://crystal-lang.org) parsed a simple program and
produced a data structure called an [abstract syntax
tree](https://en.wikipedia.org/wiki/Abstract_syntax_tree) (AST). But what does
Crystal do with the AST? Why bother going to such lengths to create it?

After Crystal parses my code, it repeatedly steps through all the entries or
nodes in the AST and builds up a description of the intended meaning and
behavior of my code. This process is known as _semantic analysis_. Later,
Crystal will use this description to convert my program into a machine language
executable.

But what does this description contain? What does it really mean for a compiler
to _understand_ anything? Let’s pack our bags and visit an abstract syntax tree
with Crystal to find out.

<div style="clear: both"></div>

## The Visitor Pattern

Imagine several tourists visiting a famous tree: Each of them sees the same
tree in a different way. The tree doesn’t change, but the perspective of each
person looking at it is different. They each take a different photo, or
remember different details.

In Computer Science this separation of the data structure (the tree) from the
algorithms using it (the tourists) is known as the [visitor
pattern](https://en.wikipedia.org/wiki/Visitor_pattern). This technique allows
compilers and other programs to run multiple algorithms on the same data
structure without making a mess.

The visitor pattern calls for two functions: `accept` and `visit`. First, a
node in the data structure “accepts” a visitor:

<img class="svg" width="400px" src="https://patshaughnessy.net/assets/2022/1/22/visitor1.svg">

After accepting a visitor, the node turns around and calls the `visit` method on
`Visitor`:

<img class="svg" width="400px" src="https://patshaughnessy.net/assets/2022/1/22/visitor2.svg">

The `visit` method implements whatever algorithm that visitor is interested in.

This seems kind of pointless… why use `accept` at all? We could just call
`visit` directly. The key is that, after calling the visitor and passing
itself, the node passes the visitor to each of its children, recursively:

<img class="svg" width="400px" src="https://patshaughnessy.net/assets/2022/1/22/visitor3.svg">

And then the visitor can visit each of the child nodes also. The `Visitor`
class doesn’t necessarily need to know anything about how to navigate the node
data structure. And more and more visitor classes can implement new algorithms
without changing the underlying data structure and breaking each other.

## The Visitor Pattern in the Crystal Compiler

In order to understand what my code means, Crystal reads through my program’s
AST over and over again using different visitors. Each algorithm looks for
certain syntax, records information about the types and objects my code uses or
possibly even transforms my code into a different form.

<div style="float: right; padding: 8px 0px 0px 30px; text-align: center; line-height:18px">
  <img src="https://patshaughnessy.net/assets/2022/1/22/angel-oak.jpg"><br/>
  <i>A photo I took in 2018 of <a href="https://en.wikipedia.org/wiki/Angel_Oak">Angel Oak</a>,<br/> a 400-500 year old tree in South Carolina.</i>
</div>

Crystal implements the basics of the visitor pattern in
[visitor.cr](https://github.com/crystal-lang/crystal/blob/master/src/compiler/crystal/syntax/visitor.cr#L24),
inside the superclass of all AST nodes:

<pre type="ruby">
class ASTNode
  def accept(visitor)
    if visitor.visit_any self
      if visitor.visit self
        accept_children visitor
      end
      visitor.end_visit self
      visitor.end_visit_any self
    end
  end
end
</pre>

Each subclass of `ASTNode` implements its own version of `accept_children`.

## My Tiny Crystal Program

To get a sense of how the visitor pattern works inside of Crystal, let’s look
at one line of code from my last post:

<pre type="ruby">
arr = [12345, 67890]
</pre>

As I explained last month, the Crystal parser generates this AST tree fragment:

<img class="svg" width="400px" src="https://patshaughnessy.net/assets/2022/1/22/ast1.svg">

Once the parser is finished and has created this small tree, the Crystal
compiler steps through it a number of different times, looking for classes,
variables, type declarations, etc. Each of these passes through the AST is
performed by a different visitor class: `TopLevelVisitor`,
`InstanceVarsInitializerVisitor` or `ClassVarsInitializerVisitor` among many
others.

The most important visitor class the Crystal compiler uses is called simply
`MainVisitor`. You can find the code for `MainVisitor` in
[main_visitor.cr](https://github.com/crystal-lang/crystal/blob/master/src/compiler/crystal/semantic/main_visitor.cr#L26):

<pre type="ruby">
# This is the main visitor of the program, ran after types have been declared
# and their type declarations (like `@x : Int32`) have been processed.
#
# This visits the "main" code of the program and resolves calls, instantiates
# methods and visits them, recursively, with other MainVisitors.
#
# The visitor keeps track of a method's variables (or the main program, split into
# several files, in case of top-level code). It keeps track both of the type of a
# variable at a single point (stored in @vars) and the combined type of all assignments
# to it (in @meta_vars).
#
# Call resolution logic is in `Call#recalculate`, where method lookup is done.
class MainVisitor < SemanticVisitor
</pre>

Since Crystal supports typed parameters and method overloading, the visitor
class implements a different `visit` method for each type of node that it visits,
for example:

<pre type="ruby">
class MainVisitor < SemanticVisitor
  def visit(node : Assign)
  def visit(node : Var)
  def visit(node : ArrayLiteral)
  def visit(node : NumberLiteral)

Etc…
</pre>

Now let’s look at three examples of what the `MainVisitor` class does with my
code: identifying variables, assigning types and expanding array literals. The
Crystal compiler is much too complex to describe in a single blog post, but
hopefully I can give you glimpse into the sort of work Crystal does during
semantic analysis.

## Identifying Variables

Obviously, my example code creates and initializes a variable called `arr`:

<pre type="ruby">
arr = [12345, 67890]
</pre>

But how does Crystal identify this variable’s name and value? What does it do
with `arr`?

The `MainVisitor` class starts to process my code by visiting the root node of
this branch of my AST, the `Assign` node:

<img class="svg" width="375px" src="https://patshaughnessy.net/assets/2022/1/22/visit-assign1.svg">

As you can see, earlier during the parsing phrase Crystal had saved the target
variable and value of this assign statement in the child AST nodes. The target
variable, `arr`, appears in the `Var` node, and the value to assign is an
`ArrayLiteral` node. The `MainVisitor` now knows I declared a new variable, called
`arr`, in the current lexical scope. Since my program has no classes, methods or
any other lexical scopes, Crystal saves this variable in a table of variables
for the top level program:

<img class="svg" width="300px" src="https://patshaughnessy.net/assets/2022/1/22/table.svg">

Actually, to be more accurate, there will always be many other variables in
this table along with `arr`. All Crystal programs automatically include the
standard library, so Crystal also saves all of the top level variables from the
standard library in this table.

In a more normal program, there will be many lexical scopes for different
method and class or module definitions, and `MainVisitor` will save each
variable in the corresponding table.

## Assigning Types

Probably the most important function of `MainVisitor` is to assign a type to each
value in my program. The simplest example of this is when `MainVisitor` visits a
`NumberLiteral` node:

<img class="svg" width="300px" src="https://patshaughnessy.net/assets/2022/1/22/visit-number-literal.svg">

Looking at the size of the numeric value, Crystal determines the type should be
`Int32`. Crystal then saves this type right inside of the `NumberLiteral` node:

<img class="svg" width="114px" src="https://patshaughnessy.net/assets/2022/1/22/updated-number-literal.svg">

Strictly speaking, this violates the visitor pattern because the visitors
shouldn’t be modifying the data structure they visit. But the type of each
node, the type of each programming construct in my program, is really an
integral part of that node. In this case the `MainVisitor` is really just
completing each node. It’s not changing the structure of the AST in this case…
although as we’ll see in a minute the `MainVisitor` does this for other nodes!

## Type Inference

Sometimes type values can’t be determined from the intrinsic value of an AST
node. Often the type of a node is determined by other nodes in the AST.

Recall my example line of code is:

<pre type="ruby">
arr = [12345, 67890]
</pre>

Here Crystal automatically sets the type of the arr variable to the type of the
array literal expression: `Array(Int32)`. In Computer Science, this is known as
_type inference_. Because Crystal can automatically determine the type of
`arr`, I don’t need to declare it explicitly by writing something more
complicated like this:

<pre type="ruby">
arr = uninitialized Array(Int32)
arr = [12345, 67890]
</pre>

Type inference allows me to write concise, clean code with fewer type
annotations. Most modern, statically typed languages such as Swift, Rust,
Julia, Kotlin, etc., use type inference in the same way as Crystal. Even newer
versions of Java or C++ use type inference.

The Crystal compiler implements type inference when the MainVisitor encounters
an `Assign` AST node, what we saw above.

<img class="svg" width="375px" src="https://patshaughnessy.net/assets/2022/1/22/visit-assign1.svg">

After encountering the `Assign` node, Crystal recursively processes one of the
two child nodes, the `ArrayLiteral` value, and its child nodes. When this process
finishes, Crystal knows the type of the `ArrayLiteral` node is `Array(Int32)`:

<img class="svg" width="425px" src="https://patshaughnessy.net/assets/2022/1/22/set-type.svg">

I’ll take a closer look at how Crystal processes the `ArrayLiteral` node next.
But for now, once Crystal has the type of the `ArrayLiteral` node it copies that
type over to the `Var` node and sets its type also:

<img class="svg" width="425px" src="https://patshaughnessy.net/assets/2022/1/22/set-type2.svg">

But Crystal does something else interesting here: It sets up a dependency
between the two AST nodes: it “binds” the variable to the value:

<img class="svg" width="325px" src="https://patshaughnessy.net/assets/2022/1/22/bind.svg">

This binding dependency allows Crystal to later update the type of the `arr`
variable whenever necessary. In this case the value `[12345, 67890]` will always
have the same type, but I suspect that sometimes the Crystal compiler can
update types during semantic analysis. In this way if the Crystal compiler ever
changed its mind about the type of some value, it can easy update the types of
any dependent values. I also suspect Crystal uses these type dependency
connections to produce error messages whenever you pass an incorrect type to
some function, for example. These are just guesses, however; if anyone from the
Crystal team knows exactly what these type bindings are used for let me know.

<b>Update:</b> Ary Borenszweig explained that sometimes the Crystal compiler
updates the type of variables based on how they are used. He posted an
interesting example on [The Crystal Programming Language
Forum](https://forum.crystal-lang.org/t/visiting-an-abstract-syntax-tree/4304).

## Expanding an Array Literal

So far we’ve seen Crystal set the type of the `NumberLiteral` node to `Int32`,
and we’ve seen Crystal assign `arr` a type of `Array(Int32)`. But how did Crystal
determine the type of the array literal `[12345, 67890]`?

This is where things get even more complicated. Sometimes during semantic
analysis the Crystal compiler completely rewrites parts of your code, replacing
it with something else. This happens even with my simple example. When visiting
the `ArrayLiteral` node, the `MainVisitor` expands this simple line of code into
something more complex:

<pre type="ruby">
__temp_621 = ::Array(typeof(12345, 67890)).unsafe_build(2)
__temp_622 = __temp_621.to_unsafe
__temp_622[0] = 12345
__temp_622[1] = 67890
__temp_621
</pre>

Reading this, you can see how later my compiled program will create the new
array. First Crystal creates an empty array with a capacity of 2, and an
element type of `Int32`. `typeof(12345, 67890)` returns the type (or multiple
types inside a union type) found in the given set of values, in this case just
`Int32`. Later Crystal sets the two elements in the array just by assigning
them.

Crystal achieves this by replacing part of my program’s AST with a new branch:

<img class="svg" width="375px" src="https://patshaughnessy.net/assets/2022/1/22/expanded-ast.svg">

For clarity, I’m not drawing the AST nodes for the inner assign operations,
only the first line:

<pre type="ruby">
__temp_621 = ::Array(typeof(12345, 67890)).unsafe_build(2)
</pre>

## Putting It All Together

With this new, updated AST we can see exactly how Crystal determines the type
of my variable `arr`. Starting at the root of my AST, `MainVisitor` visits all of
the AST nodes in this order in a series of recursive calls:

<img class="svg" width="114px" src="https://patshaughnessy.net/assets/2022/1/22/call-recurse.svg">

And it determines the types of each of these nodes as it returns from the
recursive calls:

<img class="svg" width="240px" src="https://patshaughnessy.net/assets/2022/1/22/return-recurse.svg">

Some interesting details here that I don’t understand completely or have space
to explain here:

* The `TypeOf` node calculates a common union type using a type formula. In this
example, it just returns `Int32` because both elements of my array, `12345` and
`67890`, are simple 32 bit integers.

* I believe the `Generic` node refers to a Crystal generic class via the `Path` node
shown above, in this example `Array(T)`. When `MainVisitor` processes the `Generic`
node, it sets `T` to the type `Int32`, arriving at the type `Array(Int32).class`.

* The `Call` node looks up the method my code is calling (`unsafe_build`) and
uses the type from that method’s return value. I didn’t have time to explore
how method lookup works in Crystal, however, so I’m not sure about this.

## Scratching the Surface

Today we looked at a tiny piece of what the Crystal compiler can do. There are
many more types of AST nodes, each of which the `MainVisitor` class handles
differently. And there are many different visitor classes also, beyond
`MainVisitor`. When analyzing a more complex program Crystal has to understand
class and module definitions, instance and class variables, type annotations,
different lexical scopes, macros, and much, much more. Crystal will need all of
this information later, during the code generation phase, the next step that
follows semantic analysis.

But I hope this article gave you a sense of what sort of work a compiler has to
do in order to understand your code. As you can see, for a statically typed
language like Crystal the compiler spends much of its time identifying all of
the types in your code, and determining which programming constructs or AST
nodes have which types.

Next time I’ll look at code generation: Now that Crystal has identified the
variables, function calls and types in my code it is ready to generate the
machine language code needed to execute my program. To do that, it will
leverage the LLVM framework.
