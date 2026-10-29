# AGENTS.md — home_garden_moments(家圃时光)项目规则

> 本文件供 ZCode/opencode 跨会话加载,记录项目关键规则与导航索引。新会话自动读取;修改后对所有新会话生效。
> 分工:本文件只留「结论级规定 + 索引」。功能设计 → `docs/设计稿/植物养殖游戏设计.md`;功能现状 → `docs/需求设计说明书.md`;实现记录 → `docs/变更归档.md`;踩坑成因 → `docs/踩坑速查.md`;文档索引 → `docs/README.md`。

> **⚠ Git 规定(必须遵守)**:非人工指令,不得主动提交代码(`git commit`/`git add -A`/`git push` 一律禁止)。`git add` 只能指定具体文件路径,禁止 `git add -A`/`git add .`。

> **⚠ 敏感数据规定**:密码/密钥/凭证/token 一律不写入仓库文件。

> **⚠ 中文文件读写(硬性,PS 5.1 环境)**:含中文的源码/配置/SQL/文档一律走 Read/Write/Edit 工具,禁止 PowerShell `Get-Content`/`Set-Content`/`WriteAllText`(默认 GBK 破坏 UTF-8 中文)。脚本化改写文档保行尾:本仓 `core.autocrlf=true`,`open(p,'rb')` 读 + `split(b'\r\n')` + `b'\r\n'.join()` 写。Python 输出中文先设 `$env:PYTHONIOENCODING='utf-8'`;终端里中文路径显示 `??`/乱码只是显示问题,不代表文件损坏(成因见 `docs/踩坑速查.md`)。

## 项目概述

- **应用名**:家圃时光 · Home Garden Moments(独立游戏项目)——一圃藏四季,全家共耕耘。
- **仓库**:`home_garden_moments`(GitHub: chillyice/home_garden_moments)。**实际布局**:ihomy 在 `Projects\ihomy`,本仓在 `Projects\ihomy_games\home_garden_moments`——两者是 `Projects\` 下的**兄弟目录**,ihomy **不在** `ihomy_games\` 下(相对本仓的 ihomy 路径是 `..\..\ihomy`)。
- **与 ihomy 关系**:互为引用 + 开放接口。ihomy 侧集成点(`game_info` 注册表 / `Games.vue` / `sys_home_module`)已在设计稿 §16.2/§16.5 标注为「ihomy 仓事项」,**本仓不改 ihomy 文件**。
- **当前阶段**:**实现期(客户端骨架已搭)**。**首发口径(2026-10-13 定):只做 ihomy 内嵌**(Cocos **web-mobile** 构建),以设计稿 §21 的 **P1 全量**为界;Solo / P2P / standalone(P6)与 Steam / native 构建**不在首发**。技术栈:**客户端 Cocos Creator 3.8**(`garden-cocos/`)+ 服务端 Java 同栈;**核心的接入方式**已定(Web 走 `wasm-bindgen`,native 走 **Cocos 原生插件**,**不走 `napi`、不引 C++**),但**核心用什么语言写仍未裁**(设计稿 §32 说 Rust、§28.1/§28.5 说 Java jar,两者互斥——见 `docs/需求设计说明书.md` §9 首条,P1 第一阻塞项)。**Steam/Steamworks 全部由 Cocos 侧承担**,核心侧不写 Steam API。`garden-web`(Vue 3)/`garden-native` 是上一轮脚手架,已不在技术栈内(去留待清理)。核心须为无依赖纯函数模块,被 ihomy / standalone 两个 provider 共用(设计稿 §28.5)。
- **规范蓝本**:文档 / Skills / 工具链规则仿 ihomy(参考 `..\..\ihomy\AGENTS.md`);差异见「命名约定」。

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

- 形态 `V<主>.<次>`;根目录 `VERSION` 是唯一事实来源(一行纯文本、含 `V` 前缀;当前 **V0.4**,开发期占位)。
- **⚠ 版本号暂停迭代(2026-10-13 指示)**:`VERSION` 不再往上进位;对外首发版本按 `1.0.0` 登记(需求说明书 §1.4)。此前的收尾规则(更新 `VERSION` + `docs/变更归档.md` 末尾追加 `#####` 子节)中,**变更归档照常追加**,`VERSION` 改动暂停。发布时打 annotated tag 的规则不变。

## 运行模式(一核三端,设计稿 §24–§30)

- **三种运行模式**:单人 Solo(本地权威)/ 多人 P2P(房主权威 + op-log)/ 服务器(同一条 `/api/garden` 协议,两个 provider:`IHOMY` / `STANDALONE`)。**同一核心 `garden-core`(无 IO 纯函数)+ 同一份存档 `GardenSave v1`(`*.garden.json`)**。
- **世界 World 泛化家庭**:`garden_world`(**第 11 张表**,`world_id`=ULID)是存档 / 房间 / 结算票据的统一标识;`kind`=`FAMILY`/`COOP`/`SOLO`;各表 `family_id` 列语义 = 世界作用域 id(ihomy 模式下与 `family_id` 恒等)。
- **⚠ 积分只在 `platform=IHOMY` 产出**;单人 / P2P 无积分(op-log 结算网关为候选未排期),UI 明示「不可兑换」。
- **⚠ 存档迁移有信任分级**:`S→C 新世界`可「认领」(时间戳必须净化)、`S→C 已有世界`**禁止回灌**、`P→C` 只走 op-log 重放。互通的是「世界」,不是「任意修改后的状态」。
- **⚠ 核心零平台依赖 + 确定性**:`garden-core` 不得 import ihomy 服务,禁 `Math.random` / 直接取 `now()` / 浮点比较;三端重放同一 op-log 必须状态哈希一致(golden 测试)。
- **⚠ 家庭级设置落 `garden_world.settings`(JSON)**,不用 `sys_parameter`(该表是**全局表**、无 `family_id`、`value` 上限 500 字符)。
- **标识**:六处 `user_id` 已改 `player_uid`(`CHAR(26)`);ihomy 绑定列 `sys_user_id` **只加在 `garden_member`**。
- **勘误**:积分真实落表是 `family_points_record`(`PointsService` 按 `user_id` 求和),**不是 `sys_points`**;设计稿 §30.2 是已核实的 ihomy 接口签名清单。

## 文档分流

| 内容类型 | 去处 |
|---------|------|
| 功能设计(唯一设计源:原则/玩法/数值/数据模型 §16/接口 §17/落地约束 §23/运行模式与互通 §24–§30) | `docs/设计稿/植物养殖游戏设计.md` |
| 需求基线(**要动代码先读这份**:范围与首发口径 / FR 编号 + 验收标准 / 数据 / 接口 / 非功能 / 首发验收清单 / 未决项 / 分期) | `docs/需求设计说明书.md` |
| 客户端资源(组织结构 / 命名规范 / 加载卸载契约) | `docs/CocosCreator资源管理规范.md` |
| 实现记录(文件-改动表/决策/迁移 SQL) | `docs/变更归档.md` 末尾 `#####` 子节 |
| 踩坑(症状→真因→对策)/机制细节 | `docs/踩坑速查.md` |
| 前端 UI/交互/视觉规范(实现期启用) | `docs/UI设计提示词.md`(届时新建) |

## Skills(`.agents/skills/`,与 ihomy 同制,不入 git)

- `archive` 文档归档——已适配本仓,可用。
- `build-verify` / `deploy` / `health-check`——**占位**:技术栈与发布方式定型后按 ihomy 同名技能改写。
- `asset-audit` 资源规范巡检 / `balance-check` 数值配置校验 / `new-crop` 新增作物全链路 / `golden-test` 核心确定性重放(核心落地后启用)——游戏开发向,均已适配本仓。

## 关键事实

- 设计稿是**规划稿(全文未实现)**;实现前先读其 §23 落地约束与 §24–§30 运行模式,并区分「本仓改动」与标注的「ihomy 侧集成事项」。
- **⚠ 核心语言未裁,是 P1 第一阻塞项**:设计稿 §32(Rust)与 §28.1/§28.5(Java jar 被两个 provider 共用)+ §30.1(Java 接口签名)互相矛盾,而首发是服务器权威模式、首发不需要客户端侧核心 ⇒ 首发核心天然在 Java 服务端。选项与代价见 `docs/需求设计说明书.md` §9 首条。**未裁之前不要动 `garden-core` 的语言选型**(`garden-core/` 现有的 Rust 代码是上一轮脚手架产物)。
- 设计稿引用 ihomy 内部组件(`WeatherService`/`NotificationService`/`family_item`/`ParameterService` 等)之处,**以 §30.2 的已核实签名为准**——该节同时是勘误:积分表是 `family_points_record`,不是 `sys_points`。
- 设计稿迁移与立项记录见 `docs/变更归档.md` V0.1;**运行模式与数据互通设计见 V0.2**;**客户端资源规范与 Cocos 工程骨架见 V0.3**;**引擎口径落定(设计稿改写为 Cocos)与场域命名勘误见 V0.4**。
- **模式层成本集中在「第一次写核心时有没有把 IO 隔开」**:`garden-core` 一旦 import 了 `WeatherService`,三端就永远只剩一个端。P1 必须守住 §24.3 的分层。
- **客户端资源口径见 `docs/CocosCreator资源管理规范.md`**(实现:`garden-cocos/assets/scripts/utils/AssetPaths.ts` + `managers/ResourceManager.ts`)。三条结论级规定:命名两条轨道(资产小写 snake_case + 11 个类前缀;代码/场景 PascalCase 与类名一致)、**「代码按字符串路径取」的资源必须落 `assets/resources/` 或 AssetBundle**(其余位置运行期无法按路径加载)、**加载即引用 +1、必须成对 release**。
- **设计稿已按 Cocos 口径改写(2026-10-13)**:§31–§37 的「**不需要游戏引擎** / 纯 Web + Tauri」**已作废**,客户端统一 **Cocos Creator 3.8**(§37.1/§37.2);§18.1 美术方案、§3 性能基线、§5.3.1 变体渲染手段、§15.3 拍照晒图、§18 / §18.2、§24.3 分层示意、§16.5 的 V1→V2 对照行同步改写。**遗留三项待定**:Rust 在 Cocos native 构建里的绑定方式、图标来源、作物本体用贴图还是 Cocos `Graphics`。
- **⚠ 场域一律用 `venue`(不是 `field`)**:数据层 code 取设计稿 §16.4 的 `WINDOWSILL/PLANTER/TERRARIUM/BACKYARD/FARM/GREENHOUSE`;资源侧前缀 `bg_venue_<小写>`、目录 `sprites/venues/`、数据文件 `content/venues.json`。**《CocosCreator 项目细化检查清单》不是设计源**(其 §2.2.1 目录树、§2.2.1/§2.5.1 的示例码有多处臆造与错误),实现以设计稿为准。
