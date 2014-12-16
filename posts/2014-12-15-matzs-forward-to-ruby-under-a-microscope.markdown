title: "Matz’s Foreword to Ruby Under a Microscope"
date: 2014/12/15
url: /2014/12/15/matzs-forward-to-ruby-under-a-microscope

<div style="float: left; padding: 7px 30px 0px 0px; text-align: center;">
  <img src="http://patshaughnessy.net/assets/2014/12/15/rumja_cover.png"><br/>
  <i><a href="http://shop.ohmsha.co.jp/shopdetail/000000004065/">A Japanese translation</a> of Ruby Under a<br/>Microscope was published last month.</i>
</div>

Matz wrote a new foreword to [Ruby Under a
Microscope](http://patshaughnessy.net/ruby-under-a-microscope), which was
included in the Japanese version of the book published just last month. Today
I’d like to share a translation so everyone has a chance to read it. The
Japanese edition also includes a new appendix written by Koichi Sasada about
YARV, Ruby’s virtual machine. I’ll post a translation of that tomorrow.

I love Matz’s sentiment about inspiring someone to work on Ruby… I hope that
might happen too. Thank you for writing this, Matz! You’ve already inspired so
many of us for years with your personality, your philosophy and with the
beautiful programming language you created.

Japanese-language edition copyright &copy; 2014 by Ohmsha, Ltd. Reproduced with
permission of the copyright owner.

<div style="clear: left"></div>

<div class="jp">
<h2>日本語版序文</h2>
<p class="jp">
私が小学生のときに読んで記憶に残っている一冊として、マーク・トウェインによる「アーサー王とあった男」<sup>*1</sup>というSF作品があります。1880年代のアメリカ人がアーサー王時代にタイムスリップしてしまうものの、現代（出版当時）の知識を活用して活躍するというストーリーです。5世紀に電話や自転車、銃などの知識を持ち込めばさぞかし無敵でしょう。しかし、仮に21世紀の我々が5世紀にタイムスリップしたとして、我々の持っている知識をどれだけ活用することができるでしょう。自転車くらいならともかく、何もないところからコンピュータやネットワークを構築することなどできそうもありません。現代のテクノロジーは個人レベルで再現するには高度化しすぎています。普段はなにげなく使っている技術でも中身までは理解していないものです。
</p>
</div>

<h2>Foreword for Japanese Edition</h2>
<p>The science fiction novel <cite class="book">A Connecticut Yankee in King Arthur's
Court</cite><sup>*1</sup> by Mark Twain is one of the books I still remember reading from my
elementary school days. It is the story of an American living in the 1880s who accidentally
time travels to King Arthur era England and nonetheless survives, taking
advantage of his knowledge from the modern (1880) era. Surely you would be very
powerful in the 5th century if you had knowledge of telephones, bicycles, and
guns.  But if we time travelled from the 21st century to the 5th century, how
much of our knowledge could we utilize?  Bicycles are okay, but how about
computers? It seems almost impossible to build computers and networks from
scratch ourselves.  Modern technology products are too advanced for individuals
to reproduce. We don't know how technologies work even when we use them in our
everyday lives.</p>

<div class="jp">
<p>
私たちが普段使っているRubyもそのような現代テクノロジーのひとつです。Rubyを便利に使っていても、その中がどうなっていて、どのように実行されているのか、あるいはRubyのような言語をどうすれば再現できるのか正確な知識を持っている人はほとんどいないでしょう。本書はそのような謎に包まれている「Rubyの中身」を明らかにしてくれる一冊です。
</p>
<p>
本書はRubyのソフトウェア構造から、オブジェクトシステムの構成、性能を向上させるための工夫まで解説されています。さらにCRubyだけでなく、JRubyやRubiniusについてまでカバーしています。このような知識を学べる書籍はなかなかありません。日本には類書として「Rubyソースコード完全解説」<sup>*2</sup>がありますが、入手困難になって久しいですし、対象のRubyバージョンも1.7と古いので、YARVのような新しい技術については当然解説されていません。このような書物の登場は、Rubyの内部知識の一般化に貢献すると信じます。
</p>
</div>

Ruby is one such technology. Even though we use it every day, not many of us
seem to know what it looks like on the inside, how it runs internally, or how
one could recreate such a programming language. This book sheds light on this and
reveals the mystery of Ruby internals.

This book explains the software architecture of Ruby, the structure of its
object system, and tips for performance improvement. In addition to that, it
covers not only CRuby but also JRuby and Rubinius as well. I know of few books
where you can find this type of knowledge. Though we have the <cite
class="book">Ruby Hacking Guide</cite><sup>*2</sup> in Japan, it's been
difficult to obtain a copy for a long time. It explains a version of Ruby as
old as 1.7 and naturally does not cover newer technologies like YARV. I believe
RUM will contribute to a wider understanding of Ruby internals.

<div class="jp">
<p>
本書の知識をもとにして、もしかすると本書の読者のうちの誰か、もしかするとあなたがRubyの開発に関わるようになるかもしれませんし、そうなれば我々は大歓迎します。あるいは、次世代の言語処理系を開発するきっかけになるかもしれません。そのような未来が見たいものです。
</p>
<p>
<br/>
2014年10月、松江にて<br/>
まつもと ゆきひろ
</p>
<br/>
<hr align="left"/>
<small>*1 マーク・トウェーン作、亀山龍樹訳「アーサー王とあった男」（岩崎書店、1971）<br/>
*2 青木峰郎著「Rubyソースコード完全解説」（インプレス、2002）</small>
</div>

In the future someone inspired by this book may join the development of Ruby.
It may be you. We will definitely welcome that. Or, he/she may begin creating a
next generation programming language. I hope to see that happen.

In Matsue, October 2014<br/>
Yukihiro Matsumoto
<br/>
<hr align="left"/>
<small>\*1 Twain, Mark, A Connecticut Yankee in King Arthur’s Court. (Kameyama Nagarjuna translation, Iwasaki Bookstore, 1971)<br/>
\*2 Aoki Minero al., Ruby Hacking Guide. (Impress, 2002)</small>
