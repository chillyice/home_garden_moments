# AGENTS.md — home_garden_moments(家圃时光)项目规则

> 本文件供 ZCode/opencode 跨会话加载,记录项目关键规则与导航索引。新会话自动读取;修改后对所有新会话生效。
> 分工:本文件只留「结论级规定 + 索引」。功能设计 → `docs/设计稿/植物养殖游戏设计.md`;功能现状 → `docs/需求设计说明书.md`;实现记录 → `docs/变更归档.md`;踩坑成因 → `docs/踩坑速查.md`;文档索引 → `docs/README.md`。

> **⚠ Git 规定(必须遵守)**:非人工指令,不得主动提交代码(`git commit`/`git add -A`/`git push` 一律禁止)。`git add` 只能指定具体文件路径,禁止 `git add -A`/`git add .`。

> **⚠ 敏感数据规定**:密码/密钥/凭证/token 一律不写入仓库文件。

> **⚠ 中文文件读写(硬性,PS 5.1 环境)**:含中文的源码/配置/SQL/文档一律走 Read/Write/Edit 工具,禁止 PowerShell `Get-Content`/`Set-Content`/`WriteAllText`(默认 GBK 破坏 UTF-8 中文)。脚本化改写文档保行尾:本仓 `core.autocrlf=true`,`open(p,'rb')` 读 + `split(b'\r\n')` + `b'\r\n'.join()` 写。Python 输出中文先设 `$env:PYTHONIOENCODING='utf-8'`;终端里中文路径显示 `??`/乱码只是显示问题,不代表文件损坏(成因见 `docs/踩坑速查.md`)。

## 项目概述

- **应用名**:家圃时光 · Home Garden Moments(独立游戏项目)——一圃藏四季,全家共耕耘。
- **仓库**:`home_garden_moments`(GitHub: chillyice/home_garden_moments),与 ihomy 仓平级(同在 `ihomy_games\` 目录)。
- **与 ihomy 关系**:互为引用 + 开放接口。ihomy 侧集成点(`game_info` 注册表 / `Games.vue` / `sys_home_module`)已在设计稿 §16.2/§16.5 标注为「ihomy 仓事项」,**本仓不改 ihomy 文件**。
- **当前阶段**:**设计期——无代码、无构建命令、无 CI**。技术栈待定(架构重考,含 Steam 上架考量),开发启动时定。
- **规范蓝本**:文档 / Skills / 工具链规则仿 ihomy(参考 `..\ihomy\AGENTS.md`);差异见「命名约定」。

## 工作目录与数据库

- 仓库根:`C:\Users\chill\OneDrive\WorkStation\Projects\ihomy_games\home_garden_moments`。读写前逐字核对路径。
- MySQL:本地 Docker `ihomy-mysql`(localhost:6306,root/root),**`games` 库已建**(utf8mb4/unicode_ci);业务表将来落 `games` 库,**不动 ihomy 库的 81 张表**。

## 命名约定

| 项 | 值 |
|----|----|
| 表前缀 | `garden_<module>_<entity>`(**两段式**,独立项目单游戏;不用 ihomy 的 `game_` 三段式,模块层判据见设计稿 §16.1) |
| 库名 | `games`(与 ihomy 库分离) |
| 游戏英文标识 | `garden`(路由 / game_code / i18n 命名空间 `garden.*`) |
| 显示名 | 家圃时光 |

**结构变更双文件**(仿 ihomy,实现期生效):`schema.sql`(全量)+ `migrations.sql`(幂等增量)两处同步,空库全量导入实测。

## 版本号规则(仿 ihomy)

- 形态 `V<主>.<次>`;根目录 `VERSION` 是唯一事实来源(一行纯文本、含 `V` 前缀;当前 **V0.1**,开发期占位)。
- 收尾时:更新 `VERSION` + `docs/变更归档.md` 末尾追加 `#####` 子节;发布时打 annotated tag `V<主>.<次>`。

## 文档分流

| 内容类型 | 去处 |
|---------|------|
| 功能设计(唯一设计源:原则/玩法/数值/数据模型 §16/接口 §17/落地约束 §23) | `docs/设计稿/植物养殖游戏设计.md` |
| 功能现状/规划(实现期启用) | `docs/需求设计说明书.md` |
| 实现记录(文件-改动表/决策/迁移 SQL) | `docs/变更归档.md` 末尾 `#####` 子节 |
| 踩坑(症状→真因→对策)/机制细节 | `docs/踩坑速查.md` |
| 前端 UI/交互/视觉规范(实现期启用) | `docs/UI设计提示词.md`(届时新建) |

## Skills(`.agents/skills/`,与 ihomy 同制,不入 git)

- `archive` 文档归档——已适配本仓,可用。
- `build-verify` / `deploy` / `health-check`——**占位**:技术栈与发布方式定型后按 ihomy 同名技能改写。

## 关键事实

- 设计稿是**规划稿(全文未实现)**;实现前先读其 §23 落地约束,并区分「本仓改动」与标注的「ihomy 侧集成事项」。
- 设计稿仍大量引用 ihomy 内部组件(`WeatherService`/`NotificationService`/`sys_parameter`/`family_item`/`sys_points` 等)——那是原「在 ihomy 内实现」的假设,**独立化架构定型后需逐节核对,别照抄**。
- 设计稿迁移与立项记录见 `docs/变更归档.md` V0.1。
