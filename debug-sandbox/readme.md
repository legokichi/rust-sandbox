* lldb を使おう - https://speakerdeck.com/orgachem/debugging-knowhow-that-improved-our-development-velocity-to-170-percent?slide=99

## vscode-lldb
* 大本営 - https://github.com/vadimcn/vscode-lldb
* 大本営マニュアル - https://github.com/vadimcn/vscode-lldb/blob/master/MANUAL.md
* Debugging Rust programs with lldb on MacOS - https://bryce.fisher-fleig.org/blog/debugging-rust-programs-with-lldb/index.html
* https://www.reddit.com/r/rust/comments/6q2j6q/can_you_call_debug_print_using_lldb/

### コアダンプのロード
* https://github.com/vadimcn/vscode-lldb/blob/master/MANUAL.md#loading-a-core-dump

### 実行
```
expression [関数]
```

変数名を入れると print と同じ動作っぽい。
ジェネリック関数やトレイトオブジェクト関数は使えない

### 変数一覧の表示

vscode-lldb のサイドパネルについてるので使わない

```
frame variable
```

### 変数の表示

```
print [変数名]
```

### 現在の実行コンテキスト（関数）の表示

```
list 
```

main 関数のときは main 関数の中のどこかが表示されるっぽい

