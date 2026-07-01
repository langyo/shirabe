# 打包原生库

当你发行基于 shirabe 构建的产品时，通常有两类原生文件需要随二进制一起分发：

1. **浏览器后端自身的运行时依赖。** 一份拉取到的 Chrome for Testing 构建会链接一批
   系统库（`libnss3.so`、`libdbus-1.so` 等），而一个干净的容器里往往没有它们。
2. **你自己的原生依赖** —— 你的 crate 所链接的 `.so` / `.dylib` / `.dll` 文件。

shirabe 提供一个 `shirabe::bundle` 模块来一并处理。

## 声明要携带的文件

用 `SHIRABE_BUNDLE_LIBS`（路径分隔列表：Unix 用 `:`，Windows 用 `;`）逐项列出：

```bash
SHIRABE_BUNDLE_LIBS="/opt/myapp/libfoo.so:/opt/myapp/libbar.so"
```

或写一份 `bundle.toml` 清单，再用 `SHIRABE_BUNDLE_MANIFEST` 指向它：

```toml
[[lib]]
path = "third_party/libfoo.so"
optional = true
target_os = "linux"

[[lib]]
path = "third_party/foo.dll"
```

`BundleSpec::from_env()` 会合并两个来源。

## 自动发现要携带的文件

`collect_runtime_deps(exe)` 会扫描某个二进制的共享库依赖——Linux 上用 `ldd`、
macOS 上用 `otool -L`、Windows 上做一次尽力而为的 PE 导入扫描——并返回每条记录
在案的依赖及其被解析到的位置。

## 合二为一

`BundleReport::build(&backend_exe)` 把声明清单与从解析到的后端可执行文件上发现
的依赖合并起来，`render_bundle_report(&report)` 再把它渲染成发行脚本可打印或可写
入清单的人类可读指引：

```rust
use shirabe::{BundleReport, render_bundle_report};

let report = BundleReport::build(&backend_exe);
print!("{}", render_bundle_report(&report));
```

发行脚本随后可以把每一个 `resolved` 路径（以及每一个声明过、非可选的库）`cp` 进
分发目录，得到一份在没有 Chrome 或其系统库的机器上也能运行的自包含产品。
