# keyboard-layer-cursor

QMKなどの自作キーボードファームウェアのraw
hid機能を利用して、現在のレイヤーの状態をカーソル付近に表示するためのWindowsソフトです

## 準備

Usage Pageが0xFF60, Usage
Idが0x61であるデバイスからHIDデータを受信します。そのデータが0x01から始まる場合、その次のバイトをレイヤとして認識します。

- QMKの例

```c
layer_state_t layer_state_set_user(layer_state_t state) {
  uint8_t data[2];
  data[0] = 0x01;
  data[1] = get_highest_layer(state);
  raw_hid_send(data, 2)

  return state;
}
```

## 使い方

Rustが必要です。

```
cargo run
```

で起動します。
