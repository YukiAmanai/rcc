# rcc

#### これは何か？
自作CコンパイラをRustで実装したものです
最終的にはペアレントホスト(ターゲット言語がCのためセルフホストができない。そのためここではペアレントホストと定義している)できるCコンパイラにしていきます

**機能としては以下のものがあります**
- 四則演算(+と-の単項演算子でも演算可能です)
- 比較演算子

**なぜ開発しようと思ったのか？**
- 低レイヤープログラミングに興味を持ち、特にCコンパイラ開発に興味を持ったから
- 1から低レイヤーの領域でスクラッチ開発の経験がなかったから

**なぜRustで開発しているのか？**

- RustでCコンパイラ開発している人々が少なく、RustでCコンパイラ開発するアーリーアダプターになりたかったから
- 低レイヤープログラミングに適した言語であり、C、C++言語よりも興味があったから
- Rustほんのちょっとできるようになりたいから

**今後する追加機能**
- 関数とローカル変数
- 複数文字のローカル変数
- return文  
- 制御構文
- ブロック
- 関数の呼び出しの対応
- 関数の定義の対応
- ポインタ追加
