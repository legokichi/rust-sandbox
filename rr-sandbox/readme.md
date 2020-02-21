## rr 入門

- using rust with rr - https://gist.github.com/spacejam/15f27007c0b1bcc1d6b4c9169b18868c
- /proc/sys/kernel/perf_event_paranoid - https://access.redhat.com/documentation/ja-jp/red_hat_enterprise_linux/7/html/7.3_release_notes/known_issues_compiler_and_tools
- https://github.com/mozilla/rr/wiki/Using-rr-in-an-IDE
- https://bitshifter.github.io/rr+rust/index.html

```
echo 1 | sudo tee /proc/sys/kernel/perf_event_paranoid
sudo apt install rr
```

* `rr record /path/to/executable`
* `rr replay -d rust-gdb`

## GDB(+RR) 入門

- https://qiita.com/miyagaw61/items/4a4514e2de0b458c2589
- https://darkdust.net/files/GDB%20Cheat%20Sheet.pdf
- https://github.com/cyrus-and/gdb-dashboard
- https://sourceware.org/gdb/onlinedocs/gdb/index.html#Top
* https://sourceware.org/gdb/onlinedocs/gdb/Rust.html#Rust

```
wget -P ~ https://git.io/.gdbinit
```

基本

* `run [args]` - `Start it from the beginning? (y or n)`
* `kill` - `Kill the program being debugged? (y or n)`
* `quit`

ブレイクポイントの管理

* `break <where>` - ソースコードの n 行目にブレークポイントを入れる
* `info breakpoints` - ブレークポイントの一覧
* `delete n` - ブレークポイント番号 n を消す
* `clear` - 全部消す
* `enable n` - ブレークポイント番号 n を有効
* `continue n` 無効

ウオッチポイントの管理

* `watch <where>`
* `info watchpoints`

&lt;where&gt; に指定できるもの

* function_name
* line_number
* file:line_number

(スタック)フレームの管理

* `backtrace`
    * `where` - bt と同じ
    * `backtrace full` - 呼び出しスタックを表示し、各フレームのローカル変数も出力します。
* `info locals` - 局所変数
* `info args` - 関数の引数
* `whatis <variable_name>` - 変数の型を表示

プログラム変更

* `set var <variable_name>=<value>` - 変数をセット
* `return <expression>` - 現在の関数から強制リターン

ステッピングの管理

* `s` - `step` - ステップ（呼び出された関数に入る）
    * `rs` - `reverse-step` - 逆のステップ
* `n` - `next` - 次（呼び出された関数には入りません）	
    * `rn` - `reverse-next` - 次を逆に
* `c` - `continue` - 次のブレークポイントまで続行
    * `rc` - `reverse-continue`
* `si` - `stepi` - 1マシン命令を実行
    * `rsi` - `reverse-stepi`
* `ni` - `nexti` - 1マシン命令を実行、関数呼び出しは無視
    * `rni` - `reverse-nexti`
* `f` - `finish` - 現在の関数のリターンまで続行
    * `rf` - `reverse-finish`

ソースコード

* `list` - 現在のポイントの周辺のソースコードを表示
* `disassemble (<where>)` - 現在または指定箇所のメモリ配置を表示

アセンブリ

* `layout asm` - 全体をページャで表示
* `info sharedlibrary` - ロード済み共有ライブラリの一覧

シグナル

* `info signals` - シグナルのハンドリング状況
    * `info handle` 同上

スレッド

* `info threads`


## 実行バイナリ解析入門

基本

* `cargo nm -- --demangle target/debug/rr-sandbox | less`
* `strings target/debug/rr-sandbox | less`
* `cargo objdump -- -disassemble target/debug/rr-sandbox | less`

cargo asm

* `cargo asm rr_sandbox::id --rust --build-type debug`
* `cargo llvm-ir rr_sandbox::main --rust --build-type debug`

共有ライブラリ

* `ldd target/debug/rr-sandbox` - 共有ライブラリの依存関係
* `find ./ -name "*.so*" -print | xargs readelf -d` - 共有ライブラリのシンボルテーブルを探す
* `LD_DEBUG=all <コマンドを実行>` - 共有ライブラリの読み込みに関して大量のデバッグログが吐かれる
* `env | grep LD` - パスを調べる
* `readelf -a hogehoge.so | grep SONAME` - SONAME を調べる
* `find build/ -name '*so' | xargs ldd | less` - 依存関係を調べる

strace

* `strace -tt -T -f target/debug/rr-sandbox` - システムコールの呼び出しをトレース

readelf

* `readelf -a /path/to/executable | less` - elf 全部表示
* `readelf -s /path/to/executable` - シンボルテーブル一覧
* `readelf -h /path/to/executable` - ELF ファイルヘッダ

```console
$ readelf --file-header target/debug/rr-sandbox
ELF ヘッダ:
  マジック:   7f 45 4c 46 02 01 01 00 00 00 00 00 00 00 00 00 
  クラス:                            ELF64
  データ:                            2 の補数、リトルエンディアン
  バージョン:                        1 (current)
  OS/ABI:                            UNIX - System V
  ABI バージョン:                    0
  型:                                DYN (共有オブジェクトファイル)
  マシン:                            Advanced Micro Devices X86-64
  バージョン:                        0x1
  エントリポイントアドレス:               0x3ca0
  プログラムヘッダ始点:          64 (バイト)
  セクションヘッダ始点:          2759808 (バイト)
  フラグ:                            0x0
  このヘッダのサイズ:                64 (バイト)
  プログラムヘッダサイズ:            56 (バイト)
  プログラムヘッダ数:                10
  セクションヘッダ:                  64 (バイト)
  セクションヘッダサイズ:            43
  セクションヘッダ文字列表索引:      42
```

```console
$ readelf -l target/debug/rr-sandbox

Elf ファイルタイプは DYN (共有オブジェクトファイル) です
エントリポイント 0x3ca0
There are 10 program headers, starting at offset 64

プログラムヘッダ:
  タイプ        オフセット          仮想Addr           物理Addr
                 ファイルサイズ        メモリサイズ         フラグ 整列
  PHDR           0x0000000000000040 0x0000000000000040 0x0000000000000040
                 0x0000000000000230 0x0000000000000230  R      0x8
  INTERP         0x0000000000000270 0x0000000000000270 0x0000000000000270
                 0x000000000000001c 0x000000000000001c  R      0x1
      [Requesting program interpreter: /lib64/ld-linux-x86-64.so.2]
  LOAD           0x0000000000000000 0x0000000000000000 0x0000000000000000
                 0x000000000002f8a4 0x000000000002f8a4  R E    0x200000
  LOAD           0x0000000000030680 0x0000000000230680 0x0000000000230680
                 0x00000000000019b8 0x0000000000001b28  RW     0x200000
  DYNAMIC        0x00000000000317c8 0x00000000002317c8 0x00000000002317c8
                 0x0000000000000240 0x0000000000000240  RW     0x8
  NOTE           0x000000000000028c 0x000000000000028c 0x000000000000028c
                 0x0000000000000044 0x0000000000000044  R      0x4
  TLS            0x0000000000030680 0x0000000000230680 0x0000000000230680
                 0x0000000000000000 0x0000000000000098  R      0x20
  GNU_EH_FRAME   0x0000000000029954 0x0000000000029954 0x0000000000029954
                 0x0000000000000b2c 0x0000000000000b2c  R      0x4
  GNU_STACK      0x0000000000000000 0x0000000000000000 0x0000000000000000
                 0x0000000000000000 0x0000000000000000  RW     0x10
  GNU_RELRO      0x0000000000030680 0x0000000000230680 0x0000000000230680
                 0x0000000000001980 0x0000000000001980  R      0x1

 セグメントマッピングへのセクション:
  セグメントセクション...
   00     
   01     .interp 
   02     .interp .note.ABI-tag .note.gnu.build-id .gnu.hash .dynsym .dynstr .gnu.version .gnu.version_r .rela.dyn .rela.plt .init .plt .plt.got .text .fini .rodata .debug_gdb_scripts .eh_frame_hdr .eh_frame .gcc_except_table 
   03     .init_array .fini_array .data.rel.ro .dynamic .got .data .bss 
   04     .dynamic 
   05     .note.ABI-tag .note.gnu.build-id 
   06     .tbss 
   07     .eh_frame_hdr 
   08     
   09     .init_array .fini_array .data.rel.ro .dynamic .got 
```



