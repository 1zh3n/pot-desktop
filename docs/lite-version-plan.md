# Pot 精简版（Linux 自用）功能裁剪方案（草案）

本文档从「产品功能」和「实现方式」两个角度，整理一个面向 Linux、个人自用的精简版 Pot 方案，方便后续按文档逐项修改、确认。

> 目标画像：  
> 只在 Linux 上使用，用来快速划词翻译、截图翻译 / OCR。  
> 只保留少量翻译引擎；不需要 TTS、收藏、备份、插件等重型能力。

---

## 一、功能保留 / 删除总览

### 1. 建议保留的核心功能

- 划词翻译（选中文本 + 快捷键 → 弹出翻译窗口）
- 输入翻译（手动打开翻译窗口，输入或粘贴文本）
- 截图识别（截图 → OCR → 文本）
- 截图翻译（截图 → OCR → 翻译窗口）
- 少量翻译服务（例如保留 2–3 个：DeepL / Google / 有道）
- 必要设置：
  - 快捷键设置
  - 翻译服务的 API Key / 域名配置
  - 语言 / 主题 / 字体基础设置

### 2. 建议裁剪/关闭的功能点（产品视角）

下面列出「不需要」或「优先砍掉」的功能点（可作为 checklist）：

1. **TTS 朗读**
   - 不需要对文字进行语音朗读。
   - 不再展示「播放 / 喇叭」按钮，不配置 TTS 服务。

2. **收藏 / 单词本集成**
   - 不需要与 Anki / 欧路词典（Eudic）等软件联动。
   - 不展示「添加到 Anki / 欧路」按钮。

3. **插件系统**
   - 不需要通过插件扩展新的翻译 / OCR / TTS / 收藏服务。
   - 不需要安装插件、运行插件二进制等操作。

4. **备份与同步**
   - 不需要 WebDAV / 阿里云 OSS / 本地文件备份配置。
   - 配置迁移直接靠手动拷贝 `config.json` 即可。

5. **在线更新 / 更新窗口**
   - 不需要应用内部检查更新与自动升级（Linux 下自己编译或通过包管理器升级）。
   - 不需要更新窗口和更新日志展示。

6. **大多数额外翻译 / OCR 服务**
   - 精简翻译服务列表，只保留常用的 2–3 个。
   - 精简 OCR 服务，只保留 1–2 个（例如系统 OCR + Tesseract）。

7. **部分设置页面**
   - 不需要的页面：TTS 设置、收藏服务设置、备份设置、插件管理、部分高级服务管理页面。
   - 保留下来的设置侧边栏项：通用 / 服务 / 快捷键 / 关于（历史视情况而定）。

> 后续可以在这个列表上直接打勾：  
> \- [x] TTS  
> \- [x] 收藏 / 单词本  
> \- [x] 插件  
> \- [x] 备份 / 同步  
> \- [x] 在线更新  
> \- [x] 多余翻译服务  
> \- [x] 多余 OCR 服务  
> \- [x] 部分设置页面

### 3. OCR 精简决策（已确认）

- 保留的 OCR 服务（本地）：
  - `system`：系统内置 OCR（Windows/macOS 原生接口，Linux 仅在可用平台下生效）
  - `tesseract`：本地 Tesseract 引擎 OCR
- 移除的 OCR 服务：
  - `baidu_ocr`
  - `baidu_accurate_ocr`
  - `baidu_img_ocr`
  - `iflytek_ocr`
  - `iflytek_intsig_ocr`
  - `iflytek_latex_ocr`
  - `qrcode`
  - `simple_latex_ocr`
  - `tencent_ocr`
  - `tencent_accurate_ocr`
  - `tencent_img_ocr`
  - `volcengine_ocr`
  - `volcengine_multi_lang_ocr`

### 4. 翻译服务精简决策（已确认）

- 当前精简版实际保留的翻译服务：
  - 通用翻译与词典：
    - `youdao`
    - `google`
    - `deepl`
  - 其他服务可以在前端/配置中移除或物理删除，不再参与构建。

---

## 二、前端（React）侧的裁剪建议

### 1. 翻译窗口（Translate）精简

文件位置：`src/window/Translate/index.jsx:66-228`

- 当前行为：
  - 从配置读取 4 类服务列表：
    - `translate_service_instance_list`
    - `recognize_service_instance_list`
    - `tts_service_instance_list`
    - `collection_service_instance_list`
  - 将这些服务实例的配置整合到 `serviceInstanceConfigMap`。
  - 通过拖拽排序翻译服务卡片，并将结果回写到配置。

- 精简版做法：
  - 删除与下列 key 相关的逻辑：
    - `tts_service_instance_list`
    - `collection_service_instance_list`
  - 在 `translate_service_instance_list` 初始值中，只保留常用的少数服务，例如：
    - `deepl`
    - `google`
    - `youdao`
  - 如果不再使用识别服务内嵌在翻译窗口中，也可以考虑简化 `recognize_service_instance_list`（视实际需求）。

### 2. 设置窗口（Config）页面精简

目录：`src/window/Config/pages/` & 路由：`src/window/Config/routes/index.jsx`

- 当前设置页包括：
  - `General`（通用）
  - `Translate` / `Recognize` / `Tts` / `Collection`（服务）
  - `Service`（插件与实例管理）
  - `Hotkey`（快捷键）
  - `History`（历史）
  - `Backup`（备份）
  - `About`（关于）

- 精简版建议保留：
  - `General`（基础通用设置）
  - `Service` 中的翻译 / OCR 子部分
  - `Hotkey`
  - `About`
  - `History`（可选，看你是否需要历史记录）

- 建议删除/不挂载的页面：
  - `Tts` 相关页
  - `Collection` 相关页
  - `Backup/*`
  - `Service` 目录下的 `Tts` / `Collection` / `PluginConfig` / `SelectPluginModal` 等

### 3. App 入口中的窗口精简

文件：`src/App.jsx:19-25, 116`

- 当前 `windowMap` 包括：
  - `translate`
  - `screenshot`
  - `recognize`
  - `config`
  - `updater`

- 精简版：
  - 如果不再需要更新窗口：
    - 从 `windowMap` 中移除 `updater`。
    - 删除 `src/window/Updater` 目录。

### 4. TTS / 收藏相关组件的移除

- 在翻译结果的卡片 UI（`TargetArea` 等组件）里：
  - 删除 TTS 播放按钮及对 `tts` 服务的调用。
  - 删除「收藏到 Anki / 欧路」按钮及对 `collection` 服务的调用。
- 删除不再使用的服务目录：
  - `src/services/tts/*`
  - `src/services/collection/*`
  - `src/services/translate/` 下你决定不用的具体服务。

---

## 三、后端（Tauri / Rust）侧的裁剪建议

### 1. 主入口模块精简

文件：`src-tauri/src/main.rs:4-16, 44-160`

- 可考虑保留的模块：
  - `config`（配置）
  - `hotkey`（全局快捷键）
  - `window`（窗口和翻译/OCR 调度）
  - `screenshot`（截图）
  - `system_ocr`（系统 OCR，如需要）
  - `clipboard`（如需要剪贴板监控）

- 可考虑删除/关闭的模块：
  - `backup`（备份）
  - `server`（内置 HTTP 服务器）
  - `updater`（更新）
  - `lang_detect`（本地语言检测，根据需求决定）
  - 插件相关命令（`install_plugin` / `run_binary`）

### 2. `invoke_handler` 中精简命令

文件：`src-tauri/src/main.rs:130-151`

- 精简思路：
  - 仅保留前端还会调用的命令，例如：
    - 配置：`reload_store`
    - 文本 & 截图：`get_text` / `cut_image` / `get_base64` / `copy_img` / `screenshot`
    - OCR / 翻译相关：`system_ocr` / `lang_detect`（如保留）
    - 窗口 & 快捷键：`open_devtools` / `register_shortcut_by_frontend` / `update_tray`
  - 移除不再需要的命令：
    - 插件：`install_plugin` / `run_binary`
    - 备份：`webdav` / `aliyun` / `local` 等（取决于最终是否保留备份功能）
    - 更新：`updater_window` 及 `check_update` 等

### 3. 配置模块中的服务列表裁剪（已完成）

文件：`src-tauri/src/config.rs:29-83`

- 当前实现：
  - 仅保留 `builtin_recognize_list` 和 `builtin_translate_list` 两个列表，对应 OCR 与翻译。
  - 不再存在 `builtin_tts_list` / `builtin_collection_list`。
  - `check_service_available` 只校验翻译和 OCR 服务，确保配置中不会出现已删除的服务 key。

### 4. 插件与外部二进制（已移除）

文件：`src-tauri/src/cmd.rs`

- 当前实现：
  - `install_plugin`、`run_binary` 已删除。
  - 仅保留与文本/截图/代理/字体/DevTools 相关的命令。
  - `Cargo.toml` 中仅为插件与备份引入的依赖（如 `zip`、`walkdir`、`reqwest_dav` 等）已移除。

### 5. 备份 / 更新 / 语言检测等模块

- `backup.rs`：已删除，对应的 `webdav` / `local` / `aliyun` 命令也从 `invoke_handler` 中移除。
- `updater.rs` 与 `window.rs` 中的 `updater_window`：已删除，前端不再存在更新窗口。
- `lang_detect.rs`：仍保留本地语言检测能力，可通过配置开关决定是否启用。

---

## 四、配置与构建方面的简化

### 1. Tauri 配置权限收紧

文件：`src-tauri/tauri.conf.json:1-133`

- `allowlist` 中：
  - 仅启用实际需要的模块（`window` / `clipboard` / `globalShortcut` / `notification` / `path` 等）。
  - 若不再使用 `shell` / `http` / `fs` 等能力，可以关闭或收紧 scope。

- `updater`：
  - 如不需要在线更新，可将 `active` 设为 `false` 或直接删掉整个 `updater` 节。

### 2. Linux 专用资源

- 如果不会再为 macOS / Windows 构建：
  - 删除 `tauri.macos.conf.json`、`tauri.windows.conf.json`、`icons_mac/` 等资源。
  - 在打包配置中主要关注 `deb` / `rpm` 等 Linux 目标。

### 3. 依赖清理（Rust & 前端）

- Rust：
  - 删除模块后，清理 `Cargo.toml` 中不再使用的 crate（插件、备份、server、更新等相关）。
- 前端：
  - 删除 Updater / TTS / 收藏 / 插件后，清理 `package.json` 中只被这些模块使用的依赖。

---

## 五、实施建议

- 建议在单独的分支（例如 `lite`）上做精简：
  - 第一步：按「一、2」中的功能点列表决定最终要删除的项，在列表上打勾确认。
  - 第二步：按「二」中的前端裁剪建议，先删 UI 与服务模块。
  - 第三步：按「三」中的后端裁剪建议，删命令与模块，并清理 `invoke_handler`。
  - 第四步：按「四」精简 Tauri 配置和依赖。
- 完成后，在 Linux 环境跑一遍构建与基本功能测试（划词翻译 / 截图翻译 / 设置 / 快捷键），再视需要进一步精简。

> 本文档目前是草案，后续你确认具体不要的功能后，可以在对应条目前打勾，并逐步在代码中落实。*** End Patch*** 
