アウトライン
* エラーライブラリ
    * https://docs.rs/error-chain/
        * https://brson.github.io/2016/11/30/starting-with-error-chain
        * https://www.reddit.com/r/rust/comments/5ftgv5/starting_a_new_rust_project_right_with_errorchain/ - 
    * https://docs.rs/failure
        * https://boats.gitlab.io/failure/
        * https://boats.gitlab.io/blog/post/2017-11-16-announcing-failure/
        * https://www.reddit.com/r/rust/comments/7dg95u/announcing_failure/
        * https://amp.reddit.com/r/rust/comments/7h8v1z/errorchain_and_failure/
        * https://rust-lang-nursery.github.io/failure/guidance.html
    * std::error::Error
        * https://github.com/rust-lang/rfcs/blob/master/text/2504-fix-error.md
* エラーにおける関心
    * https://rust-jp.slack.com/archives/C8FLSR5F1/p1556674891068900?thread_ts=1556670148.065800


# `std::error::Error` を楽に実装したい
* Error を楽に実装したい
        * 旧来の `std::error::Error` (std::io::Error の場合)
        * `error_chain::example_generated::inner::ErrorKind`
        * `failure_derive` および [An Error and ErrorKind pair](https://boats.gitlab.io/failure/error-errorkind.html)
        * 新生 `std::error::Error` +  err-derive および `failure::ErrorEx`

# main で ? がつかえる
* impl Termination が条件
* Result って -> TRPL
* ? って -> 2018
* Termination って？ -> RFC

`fn main() -> Result<impl Termination, impl Debug>` の話

https://doc.rust-lang.org/stable/edition-guide/rust-2018/error-handling-and-panics/question-mark-in-main-and-tests.html

https://github.com/rust-lang/rfcs/blob/master/text/1937-ques-in-main.md

? 演算子
https://doc.rust-lang.org/stable/edition-guide/rust-2018/error-handling-and-panics/the-question-mark-operator-for-easier-error-handling.html



# エラーを合成したい
* TypeScript でいう `type Error = E1 | E2 | E2;` みたいなやつ
* `struct Error` & `enum ErrorKind` 派
    * error-chain, failure(inner: Context使用)
* `enum Error` & `struct E1` 派
    * failure, err-derive

* Error を合成したい
        * 旧来の `std::error::Error` (Box<dyn Error>)
        * failure::Error
        * 新生 `failure::DefaultError`

* ErrorKind したい、エラーを合成したい
        * 旧来の `std::error::Error` (std::io::Error の場合)
        * `error_chain!` および `error_chain::example_generated::inner::ErrorKind`
        * `derive(Fail)` および [An Error and ErrorKind pair](https://boats.gitlab.io/failure/error-errorkind.html)
        * 新生 `std::error::Error` + err-derive
        * 新生 `failure::ErrorEx`

エラー型の設計が難しすぎるので内部表現を完全に隠蔽して誤魔化すことにしたが、結局内部表現をどうするかで悩んでいる……救いなんてなかった

マジで Box<(dyn std::error::Error + 'static)> にしてやろうか

https://mobile.twitter.com/lo48576/status/1099254637067038720


# `Box<dyn Error>`
* `fn main() -> Result<(), Box<dyn Error>>` はできない
    * `impl Error for Box<dyn Error>` がない
* `failure::Error`


impl From<String> for Box<dyn std::error::Error> などされているため、雑コードでは結構便利
failure::Error 使えよという話は置いといて

https://mobile.twitter.com/lo48576/status/1071758674714972160


If you've been frustrated that `Box<dyn Error>` doesn't actually implement `Error`, there's a working solution: (link: https://github.com/rust-lang/rust/pull/58974) github.com/rust-lang/rust…

https://mobile.twitter.com/seanmonstar/status/1106582851456122880

https://github.com/rust-lang/rust/pull/58974

Box&lt;Error&gt; は Error を実装していないので不便

https://github.com/rust-lang/rust/pull/58859

エラー型を内包した強いenum作ろうとするよりも、素直に `Box<dyn Error>` かfailure::Error使えばいいじゃんという気持ちに傾きつつある

https://mobile.twitter.com/ubnt_intrepid/status/1079643741218992128

# donwcast したい
https://rust-lang-nursery.github.io/api-guidelines/interoperability.html#error-types-are-meaningful-and-well-behaved-c-good-err

* downcast したい
        * https://rust-jp.slack.com/archives/C0DJCNRPC/p1541050702002200
    * https://rust-jp.slack.com/messages/C8FLSGCBH/convo/C0DJCNRPC-1544455406.037800/
    * https://rust-jp.slack.com/archives/C0DJCNRPC/p1541050702002200


# Backtrace 欲しい
* error-chain
* std::error::Backtrace (RFC)

* backtrace したい
        * `error_chain::Backtrace`
        * `failure::Backtrace`
        * `std::error::Backtrace`

# cause スタックを楽に積みたい

* cause スタックを楽に積みたい
    * 旧来の `std::error::Error`
    * `error_chain::ChainedError::chain_err` - https://docs.rs/error-chain/0.12.1/error_chain/trait.ChainedError.html#tymethod.chain_err
    * `failure::Fail::context`
    * 新生 `std::error::Error`
    * 新生 `failure::ErrorEx`

# cause スタックを iterable にしたい

* cause スタックを楽に辿りたい
    * 旧来の `std::error::Error`
    * `error_chain::example_generated::Error::iter`
    * `failure::Fail::iter_causes` - https://docs.rs/failure/0.1.5/failure/struct.Context.html
    * 新生 `std::error::Error`
    * 新生 `failure::ErrorEx`

# 新生 std::error::Error

このうち1. の式では、二項演算子の + はどの単項演算子よりも弱い。2. は1. と一貫している。また、既に3. の構文はそれにあわせて優先度が下げられている。 (RFC0438)
たとえば、 &Error + Send と書くことはできず、 &(Error + Send) とする必要がある。

一方、4. のdynと5. のimplでは今のところ、 + は貪欲に読まれる(つまり最も優先される)。しかしこれは1-3と一貫性がないだけではなくて構文上も奇妙なことになる。
そこでdynとimplに関しても + の優先順位を下げるというのがこのRFCの主眼である。

https://mobile.twitter.com/qnighy/status/949648143477440514

ところが仮に + の優先順位を下げるにしても、2つほど判断しなければいけないことがある。
まず1つは、dynとimpl自体は+とどう作用するか。これに関して、
・dynやimplは+より弱く、 &(dyn Error + Send) と書く。(案2)
・dynやimplも+より強く、 &dyn (Error + Send) と書く。(案3)
がある。
https://mobile.twitter.com/qnighy/status/949648146455396352


もう1つは、仮に案2を採用するとして、関数定義で
fn f() -> impl Error + Send { .. }
のように書いた場合に括弧を要求するか。

単項演算子がないので要求しなくても問題ないが、一方で
Fn() -> (impl Error + Send)
では括弧がいるので、これとのアナロジーで括弧を要求したい。
https://mobile.twitter.com/qnighy/status/949648148439318531

基本的には案2が議論の出発点だけど、 &(dyn Error + Send) とか Fn() -> (impl Error + Send) のような位置に括弧が必要なのはどうもあまり嬉しくないというところまでは割と合意できてるっぽい。じゃあ結局どうするか？というと難しそうだけど……

https://mobile.twitter.com/qnighy/status/949648150851067910



今日エラー処理とfailureについて色々書いたけど、実はfailureの作者がstd::error::Errorの改善を提案していた (link: https://github.com/rust-lang/rfcs/pull/2504) github.com/rust-lang/rfcs…
基本的には「どうしてそうなった」という自明な修正を2つ入れるだけっぽいけど、これに応じてfailureがFail: StdError になる可能性が言及されている。
https://mobile.twitter.com/qnighy/status/1021021205166297090

1. バックトレースを保持できるようにする。
2. causeが&Errorを返していたのを、&(Error + 'static)にする。(downcastできるようにするため)
   ただし互換性を保ちつつ移行するため別のメソッド名にする。
&Errorは&'a (dyn Error + 'a)と解釈されるんだけど、昔はError: 'staticだったので&'a (dyn Error + 'static)と解釈されていたらしい。
Error: 'staticを外すときに&(Error + 'static)に書き換えるべきだったんだけどそれを忘れてたのが原因っぽい(悲しいね)





# ここがややこしいRustのエラー
* error-chain 非推奨
* backtrace 欲しい 
# ここがダメだよ Rust のエラー
* 実装するのが面倒 (derive マクロなし、Debug, Display, Error を手で impl)

# どれを使えばいい？

* いますぐリッチに -> failure
* std::error::Error トレイトを楽に実装したい -> err-derive

# error-chain とはなんだったのか？

error-chainは他のエラーをErrorKindで束ねる機能とと.chain_errでError::causeのスタックを積む機能とbacktraceを取得する機能が混じっており混沌

failureはErrorトレイトの気に入らない点を置き換えるFailトレイトを導入するのとerror-chainのErrorKind定義のマクロが可読性最悪だったのをderiveマクロにしたのと.chain_errの代わりに.contextでスタックを積めるようにしたのと `Box<dyn Fail>` じみたError構造体を導入したのでますます混沌としている

err-deriveはbacktraceがstdに入るのでfailureのderiveマクロで簡単にErrorトレイトを実装できるようにしただけで.chai_err()とかは一切生やさないやつ
(link: https://github.com/rust-lang-nursery/failure/issues/287) 
FailトレイトはErrorExtに名前換えしてstd::error::Errorを継承するようだ

(link: https://github.com/rust-lang-nursery/failure/issues/181#issuecomment-377398482) github.com/rust-lang-nurs…
error-chainはErrorKindを主眼においてるがfailureはそうではない

https://mobile.twitter.com/ygkm911/status/1145291000367108096


# 図式の世界
# error-chain 可換図
# failure 可換図
# std::error::Error 可換図
# ユーザ←アプリ→ライブラリ→std

# failure と std::error::Error
Rustはまずfailureを標準に入れろ(もしもう入ってたら許して欲しい，まだ標準に入ってない頃のRustしか知らないので…)
std::errorを使わずにfailureを使えってそこそこ見るんだけど、やっぱりfailureのほうがいいの・・・？
なんか新しいの (link: https://twitter.com/madmenino/status/1131255975661277184) twitter.com/madmenino/stat… が生えたらしいですが，少なくとも
- 複数のエラーの合成が容易(例外設計をコードに落としやすい)
- とりあえずfailure::Errorで大体のエラーを取れるので例外設計を真面目にやってない場合も手早く動くものが作れる
の2点で良かったのは確かです
ほむほむ　　新しいのも気になるけど、新しいものの良さを知る上でもとりあえず次Rustのプログラム書くときにはfailureを使ってみますぽよぽよ
とりあえずfailure::Errorだけでも「楽」ではあるんですよね(およそ任意のErrorをfailure::Errorでとれるので，?演算子がメチャクチャ雑に使えるようになる)
動くようになってから例外設計を持ち込めるのでプロトタイピングから本番までスムーズに持っていける気がする 多分このへんはTSでanyだらけのコードを書いた後にちょっとずつ型をつけていく作業に近い
なるほよ　　サクッと調べたらstd::errorがなぜだめでfailureがいいかって記事が日本語でたくさん出てきたので完全に自分の調査不足ですた・・・ちゃんと勉強して出直してきますっ＞＜
たぶん「現行の標準の範囲内だとderiveしにくいせいで複数のErrorを統合しにくい」みたいなの(真面目に例外設計しても実装に落とすときにクッソ面倒になるやつ)を解決しようというのが件のerr-deriveで，failureはそれに加えて「とりあえずErrorがなんでも入るやつ」も入ってる感じ
ふむふむ、なんとなくわかった・・・かも   調べると、failureの前はerror-chainが推奨されてたっぽいし、std::errorがstableであるとはいえこの辺まだまだ使いやすいものを模索中という感じなのかな
あとfaliureはスタックトレースを持てるのでデバッグに便利
実際のところerr-deriveってfailureと比べて何が嬉しいんですかね 小さいこと？
failure::Failは実験的なものでその知見がErrorに反映される予定((link: https://github.com/rust-lang/rfcs/blob/master/text/2504-fix-error.md) github.com/rust-lang/rfcs…)なので便利だけど将来的にはError使っといたほうがいいよ的なことをerr-deriveの作者は言ってる
あー，なので `failure::Fail` ではなく `std::error::Error` が使えるけど `failure-derive` のようにderiveできるライブラリがあれば，将来的には実験のために作ったfailureの旨味を全部吸い上げた状態でfailureそのものは排除できる，と… ありがとうございます

* failure は fix-error が収まるまで 0.1 のまま


### TPPL
* TRPL - https://doc.rust-lang.org/1.30.0/book/first-edition/error-handling.html - https://github.com/rust-lang/book
* TRPL - https://doc.rust-lang.org/book/ch09-00-error-handling.html - https://github.com/rust-lang/book
* https://doc.rust-lang.org/1.30.0/book/first-edition/error-handling.html

https://boats.gitlab.io/failure/error-msg.html
https://github.com/rust-lang-nursery/api-guidelines/search?q=error&unscoped_q=error
https://github.com/rust-lang-nursery/edition-guide/search?utf8=%E2%9C%93&q=error&type=
https://github.com/rust-lang-nursery/rust-cookbook/search?p=2&q=error&type=Issues
https://github.com/rust-lang/book/search?q=error&unscoped_q=error
https://github.com/rust-unofficial/patterns/search?q=error&unscoped_q=error
https://github.com/rust-lang/rust-by-example/search?q=error&unscoped_q=error
https://doc.rust-lang.org/1.0.0-beta.5/book/error-handling.html