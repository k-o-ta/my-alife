[作って動かす ALife](https://www.oreilly.co.jp/books/9784873118475/)の[サポートサイト](https://github.com/alifelab/alife_book_src)にあるサンプルコードのRust実装です。  
example配下を実行してください。
~~~ShellSession
# example
cargo run --example chap02_gray_scott --release
~~~

## setup
~~~ShellSession
cargo check
~~~

## GLSLについて
* サンプルコードではOpenGL2系の記法が使われているが、gliumでは2系のGLSLが実行できなかったので、OpenGL3.1の記法に書き直しています。
* [GLSL参考1](http://nn-hokuson.hatenablog.com/entry/2016/11/07/204241)
* [GLSL参考2](http://tkengo.github.io/blog/2014/12/27/opengl-es-2-2d-knowledge-1/)

## example
![chap02_game_of_life](https://i.gyazo.com/c0bc4d8c5502722bccd33a4c599d654b.gif)

![chap05_subsumption](https://i.gyazo.com/318f9bc9e2ed40a5a8f25975134220a7.gif)

