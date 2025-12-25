# pi-pico-2-w

## debug probe の更新

- https://www.raspberrypi.com/documentation/microcontrollers/debug-probe.html
- https://github.com/raspberrypi/debugprobe/releases/tag/debugprobe-v2.2.3

```
$ lsusb -v -d 2e8a:000c | grep bcdDevice             
  bcdDevice            2.23
```

## 接続確認

PC に Pi Pico 2 W と Raspberry Pi Debug Probe を接続した状態で、以下を確認します。

```sh
lsusb
```

表示例:

```text
Bus 003 Device 005: ID 2e8a:000c Raspberry Pi Debug Probe (CMSIS-DAP)
Bus 003 Device 008: ID 2e8a:000f Raspberry Pi RP2350 Boot
```

メモ:
- `RP2350 Boot` は BOOTSEL モードで認識されている状態です。
- BOOTSEL を解除して再接続するとランモードの表示になります。
