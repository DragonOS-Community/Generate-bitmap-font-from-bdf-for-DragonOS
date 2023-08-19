# Generate bitmap font from bdf for DragonOS

输入bdf文件，为DragonOS生成位图字体。
---

## 用法

### 生成字体

```
cargo run <bdf路径> <输出路径>
```

请注意，默认只生成ASCII字符，如果需要生成更多字符，请修改`src/main.rs`中的`splits`变量，根据程序输出的keys，调整splits的元组范围。

### 使用字体

将生成的字体放入dragonos的textui的font目录下，并仿照spleen.rs的格式，添加新的字体。
请注意，您需要正确设置Mapping（与split对应）,这样才能正确显示字符

## Contributing

欢迎提交PR,如果您有任何问题，请提交issue。
