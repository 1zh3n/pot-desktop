# pot-desktop 项目结构说明

本文档梳理 `pot-desktop` 基于 Tauri 的桌面翻译应用的整体结构，重点说明应用入口、前后端主要模块以及各自职责，便于后续阅读和扩展代码。

## 一、整体架构

- **技术栈**
  - 桌面容器：Tauri（Rust 后端 + Web 前端）
  - 前端：React + React Router（`BrowserRouter`），UI 使用 NextUI，主题使用 `next-themes`
  - 状态与配置：`tauri-plugin-store-api`（前端） + `tauri-plugin-store`（后端）
  - 多语言：`react-i18next`，配置在 `src/i18n` 目录

- **目录总览**
  - 前端主应用：`src/`
  - Tauri 后端：`src-tauri/`
  - 静态资源：`public/`、`asset/`
  - 项目元信息：`tauri.conf.json`、`package.json`、各平台 Tauri 配置等

## 二、前端入口与核心流程

### 1. Web 入口

- **入口文件**：`src/main.jsx:1-28`
  - 初始化配置存储：`initStore()`（`src/utils/store.js:8-16`）
  - 初始化运行环境（操作系统信息、应用版本等）：`initEnv()`（`src/utils/env.js:9-13`）
  - 创建 React 根节点并渲染：
    - 外层包裹 `NextUIProvider` 和 `NextThemesProvider`，用于 UI 组件和主题
    - 内部渲染核心组件 `App`
  - 生产模式下禁止右键菜单以避免调试入口：`import.meta.env.PROD` 判断

### 2. App 组件与窗口分发

- **核心组件**：`src/App.jsx:19-25, 27-37, 116`
  - 通过 `windowMap` 将 Tauri 窗口标签映射到不同 React 窗口组件：
    - `translate` → 翻译窗口 `src/window/Translate/index.jsx`
    - `screenshot` → 截图窗口 `src/window/Screenshot/index.jsx`
    - `recognize` → OCR 识别窗口 `src/window/Recognize/index.jsx`
    - `config` → 设置窗口 `src/window/Config/index.jsx`
    - `updater` → 更新窗口 `src/window/Updater/index.jsx`
  - 使用 `BrowserRouter` 包裹当前窗口组件（虽然主要是单页面，多用于内部路由支持）
  - 从配置中读取并应用：
    - `app_theme`：应用主题（浅色/深色/跟随系统）
    - `app_language`：界面语言，通过 `i18n.changeLanguage` 生效
    - 字体与字号：`app_font`、`app_fallback_font`、`app_font_size`，直接设置 `document.documentElement.style`
  - 键盘事件管理：
    - 限制 `Ctrl` 快捷键，避免误操作（复制/粘贴等除外）
    - `Esc` 关闭当前窗口（`appWindow.close`）
    - 开发模式下允许 `F12` 打开/关闭 DevTools（调用后端 `open_devtools` 命令）

### 3. 配置与运行环境

- **配置存储封装**：`src/utils/store.js:1-16`
  - 使用 `tauri-plugin-store-api` 操作位于应用配置目录的 `config.json`
  - 通过 `watch` 监听配置文件变更，并调用后端命令 `reload_store` 让 Rust 端重新加载配置

- **环境信息封装**：`src/utils/env.js:1-13`
  - 导出 `osType`、`arch`、`osVersion`、`appVersion`
  - 在启动时由 `initEnv()` 异步获取 Tauri 提供的系统信息与应用版本

## 三、前端主要功能模块

### 1. 窗口模块（window）

位于 `src/window/`，对应后端创建的不同 Tauri 窗口：

- **Translate 翻译窗口**：`src/window/Translate/index.jsx:1-345`
  - 负责主翻译 UI：
    - `SourceArea`：源文本输入区域
    - `LanguageArea`：源/目标语言选择
    - `TargetArea`：翻译结果展示，每个目标服务一个卡片
  - 通过配置管理翻译相关行为：
    - `translate_close_on_blur`：是否失焦自动关闭
    - `translate_always_on_top`：是否默认置顶
    - `translate_window_position`、`translate_remember_window_size`：窗口位置和大小的记忆与持久化
  - 服务实例管理：
    - 通过 `useConfig('translate_service_list'...)` 读取翻译服务列表，仅保留少量内置服务（如有道 / Google / DeepL）。
    - 通过 `DragDropContext` 支持翻译服务卡片拖拽排序，修改后写回配置。
  - 窗口行为：
    - 监听 Tauri 的 `blur`/`focus`/`move`/`resize` 事件，在合适时自动关闭或保存位置/尺寸
    - 通过 `BsPinFill` 控制是否置顶及是否关闭失焦监听

- **Recognize OCR 识别窗口**：`src/window/Recognize/index.jsx`
  - 包括：
    - `ImageArea`：展示截取的图片
    - `TextArea`：展示识别出的文字
    - `ControlArea`：选择识别服务、触发识别、复制结果等
  - 与后端 `system_ocr`、`screenshot` 等命令配合使用，实现「截图 → OCR → 文本」流程

- **Screenshot 截图窗口**：`src/window/Screenshot/index.jsx`
  - 用于非 macOS 下的区域截图：
    - 接收后端 `screenshot` 命令生成的整屏截图（缓存文件）
    - 在前端显示屏幕图像，用户框选区域后通过事件回传坐标给后端裁剪
  - 截图完成后，会发出 `success` 事件，被后端监听以继续后续 OCR/翻译流程

- **Config 设置窗口**：`src/window/Config/index.jsx` 与子路由
  - 结构：
    - 侧边栏：`src/window/Config/components/SideBar/index.jsx`
    - 路由分发：`src/window/Config/routes/index.jsx`
  - 设置页（`src/window/Config/pages`）在精简版中主要包括：
    - `General`：通用设置（语言、主题、启动行为等）
    - `Translate` / `Recognize`：翻译与 OCR 服务启用、排序、默认服务等
    - `Service`：服务实例配置
    - `Hotkey`：全局快捷键配置（与后端 `hotkey.rs` 对应）
    - `History`：翻译/识别历史记录（可选）
    - `About`：版本/日志目录/开源地址等信息

### 2. 通用组件与 Hooks

- **窗口控制按钮**：`src/components/WindowControl/index.jsx`
  - 封装最小化、最大化/还原、关闭按钮
  - 使用 Tauri `appWindow` 控制原生窗口状态

- **Hooks**：`src/hooks/`
  - `useConfig.jsx`：封装对 `store`（配置）的读写，提供 React Hook 样式的配置项访问
  - `useSyncAtom.jsx` / `useGetState.jsx`：用于在多个组件间同步状态
  - `useToastStyle.jsx`：统一 Toast 提示样式
  - `useVoice.jsx`：与 TTS 服务集成的语音播放相关逻辑
  - `hooks/index.jsx`：导出聚合入口，方便统一引入

### 3. 外部服务集成模块（services）

位于 `src/services/`，按照服务类别划分子目录，每个具体服务通常包含：

- `Config.jsx`：服务配置界面（API Key、域名、开关等）
- `index.jsx`：服务调用逻辑（请求参数构造、HTTP 请求、结果解析）
- `info.ts`：服务元数据定义（名称、图标、支持语言等）

主要类别（精简版）：

- **翻译服务**：`src/services/translate/`
  - 仅保留少量常用服务，例如：`youdao`、`google`、`deepl` 等。

- **OCR 识别服务**：`src/services/recognize/`
  - 精简为系统 OCR 与 Tesseract 等少数本地/系统服务。

在精简版中不再启用：

- `src/services/tts/`：TTS 语音朗读功能。
- `src/services/collection/`：与 Anki、欧路词典等收藏集成功能。

保留的服务在配置界面（`Config` 窗口）中统一管理，在翻译窗口中按配置顺序展示。

## 四、后端（Tauri/Rust）入口与模块

### 1. Tauri 入口

- **配置文件**：`src-tauri/tauri.conf.json:1-133`
  - `build`：开发/构建命令（`pnpm dev` / `pnpm build`）、前端资源路径
  - `package`：应用名称、版本
  - `tauri.allowlist`：允许前端访问的 API（shell、window、clipboard、globalShortcut、http、fs 等）
  - `tauri.bundle`：打包配置（图标、依赖等）
  - `tauri.windows`：内置 `daemon` 隐藏窗口（加载 `daemon.html`）
  - `systemTray`：系统托盘图标配置

- **入口函数**：`src-tauri/src/main.rs:38-145`
  - `tauri::Builder::default()`：
    - 注册插件：单实例、日志、开机自启、SQL、配置存储、文件监控等
    - 创建系统托盘：`system_tray`，托盘逻辑在 `tray.rs`
  - `setup` 回调：
    - 初始化全局 `APP` 句柄，供其他模块使用。
    - 调用 `init_config` 初始化配置存储（`src-tauri/src/config.rs:11-27`）。
    - 首次启动时打开配置窗口：`config_window()`（`src-tauri/src/window.rs`）。
    - 管理全局状态：`StringWrapper`（用于临时存储待翻译文本）、`ClipboardMonitorEnableWrapper` 等。
    - 注册全局快捷键：`register_shortcut("all")`（`src-tauri/src/hotkey.rs`）。
    - 根据配置初始化代理、语言检测（可选）、剪贴板监控等。
  - `invoke_handler`：
    - 精简版仅暴露当前前端需要的命令，例如：
      - 配置：`reload_store`
      - 文本 & 截图：`get_text` / `cut_image` / `get_base64` / `copy_img` / `screenshot`
      - OCR / 语言检测：`system_ocr` / `lang_detect`
      - 窗口 & 快捷键：`open_devtools` / `register_shortcut_by_frontend` / `update_tray`
      - 其他：`set_proxy` / `unset_proxy` / `font_list`
  - `on_system_tray_event`：
    - 托盘事件回调委托给 `tray_event_handler`（`src-tauri/src/tray.rs`）
  - `run`：
    - 拦截 `ExitRequested` 事件，调用 `api.prevent_exit()`，实现「关闭窗口不退出程序」的常驻驻留行为

### 2. 配置与服务可用性校验

- **配置存储封装**：`src-tauri/src/config.rs:11-27, 80-132`
  - `init_config`：在应用配置目录创建/加载 `config.json`，封装为 `StoreWrapper` 注入 Tauri 状态。
  - `get`/`set`：提供对配置的读取与写入，前端通过 `tauri-plugin-store-api` 间接使用。

- **服务可用性检测**：`src-tauri/src/config.rs:29-83`
  - 定义精简后的内置服务列表（仅翻译与 OCR 部分）。
  - 读取配置中的服务列表，并与当前内置服务比对。
  - 若列表中包含已被移除的服务，将其从配置中自动删除。

### 3. 窗口与业务逻辑模块

- **window.rs**：`src-tauri/src/window.rs`
  - 负责所有前端窗口的创建与复用：
    - `get_daemon_window`：获取/创建隐藏的 `daemon` 窗口，用于获取监视器信息等
    - `build_window`：统一的窗口构建函数，根据鼠标所在屏幕创建窗口、设置无边框/阴影/透明等
    - `config_window`：配置窗口
    - `translate_window`：翻译窗口，支持记忆上次窗口大小/位置
    - `selection_translate` / `input_translate` / `text_translate` / `image_translate`：不同翻译触发方式，通过修改全局 `StringWrapper` 状态并向前端发出 `new_text` 事件
    - `recognize_window`：OCR 识别窗口
    - `screenshot_window`：全屏截图窗口（非 macOS）
    - `ocr_recognize` / `ocr_translate`：完整的截图→识别→显示/翻译流程控制

- **hotkey.rs**：`src-tauri/src/hotkey.rs:1-71, 73-98`
  - 提供全局快捷键注册逻辑：
    - `register_shortcut("all")`：读取配置中各类快捷键设置并注册
    - 支持前端通过 `register_shortcut_by_frontend` 直接更新快捷键
  - 快捷键回调会调用 `selection_translate`、`input_translate`、`ocr_recognize`、`ocr_translate` 等函数

- **cmd.rs**：`src-tauri/src/cmd.rs`
  - 向前端暴露通用命令：
    - 文本/截图剪裁获取：`get_text`、`cut_image`、`get_base64`、`copy_img`
    - 网络代理管理：`set_proxy` / `unset_proxy`
    - 字体列表获取：`font_list`
    - DevTools 控制：`open_devtools`

- **screenshot.rs**：`src-tauri/src/screenshot.rs:3-30`
  - 根据传入坐标找到对应屏幕，截取整屏图像并保存到缓存目录
  - 与前端截图窗口联动完成区域截图

- **其他模块（概览）**
  - `clipboard.rs`：剪贴板监控与文本同步（结合 `ClipboardMonitorEnableWrapper`）。
  - `system_ocr.rs`：调用系统原生 OCR 能力（Windows/macOS）。
  - `lang_detect.rs`：本地语言检测引擎初始化与调用（可选）。
  - `tray.rs`：系统托盘菜单与点击事件处理。

## 五、典型调用链示例

### 1. 选中文本 → 快捷键翻译

1. 用户在系统中选中文本，按下配置好的「划词翻译」快捷键
2. 后端 `hotkey.rs:46-51` 调用 `selection_translate`（`window.rs:226-239`）
3. 后端将选中文本写入 `StringWrapper`，创建/聚焦 `translate` 窗口，并向前端发送 `new_text` 事件
4. 前端翻译窗口监听到事件，读取 `get_text` 命令返回的内容，发起各翻译服务请求并展示结果

### 2. 截图 → OCR 识别 → 显示结果

1. 用户按下「截图识别」快捷键
2. 后端 `ocr_recognize`（`window.rs:335-367`）创建截图窗口：
   - macOS 使用系统 `screencapture` 命令
   - 其他平台使用前端 `Screenshot` 窗口 + `screenshot` 命令协作
3. 截图完成后，识别窗口 `recognize_window` 被打开，并通过事件通知前端加载最新截图
4. 前端在 `Recognize` 窗口中选择识别服务并展示识别结果

---

以上是 `pot-desktop` 项目中与入口和主要功能模块相关的结构梳理。如果你后续希望我进一步细化某个模块（例如只看翻译服务、插件系统或备份模块的详细设计），可以直接指出具体子模块名称。

