# Cocos Creator 资源管理规范 — 家圃时光

> **定位**:客户端(Cocos Creator 3.8)的**资源组织结构、命名规范、加载与卸载契约**的唯一口径。落地《CocosCreator项目细化检查清单》§2.2(资源管理·高优先级)与 §五第 1 项。
> **上游**:玩法与数值以 `设计稿/植物养殖游戏设计.md` 为准(作物码 §5.2、变体短码 §5.3.1、场域 §3、六个阶段 §5.1);本规范只管辖「这些代码落到资源目录与文件名上叫什么」。

**配套实现**:`garden-cocos/assets/scripts/utils/AssetPaths.ts`(路径登记表)、`garden-cocos/assets/scripts/managers/ResourceManager.ts`(加载管理器)。

---

## 〇、口径声明（Cocos 已定为客户端引擎）

本规范按 **2026-10-13 的决定**落到 **Cocos Creator 3.8(客户端)+ Rust `garden-core`(核心)**。

**《设计稿》已按同一口径改写**(同日,见其文末增补与 §37):§31–§37 里「不需要游戏引擎 / 纯 Web + Tauri」的结论**已作废**,§18.1 美术方案、§3 性能基线、§5.3.1 变体渲染手段、§15.3 拍照晒图、§18 / §18.2 客户端体验、§24.3 分层示意、§16.5 的 V1→V2 对照行均已同步为 Cocos。

| 设计稿出处 | 改写后口径 |
|---|---|
| §18.1 美术方案 | 极简参数化资产(形状 × 参数),资源清单 = 86 张成熟态 + 1 张通用芽 + 3 张变体遮罩 + 6 张场域背景 |
| §18 客户端体验 | 地块卡 = Prefab 实例化 + 对象池;变体 = 遮罩贴图 + Sprite 颜色 tint |
| §31.1 平台表 | Browser = web-mobile 构建;Desktop / Mobile = native 构建(Steam / iOS / Android) |
| §32.2 原生绑定 | ⚠ 保留为**待重定**:原稿的 `napi` 宿主是 Node/Tauri,不适用 Cocos native |
| §37 引擎选择 | **Cocos Creator 3.8(已采用)** |

**仍未定的三项**(会影响本规范,已列入 §七):Rust 核心在 Cocos native 构建里的绑定方式、图标来源、作物本体用贴图还是 Cocos `Graphics`。

**不受影响的部分**:`garden-core` 的纯函数、`content/*.json` 数据、§16 的十一张表、§17 接口、§24–§30 运行模式——资源层变更不碰这些。

---

## 一、前置约束(Cocos Creator 3.8 的三条硬规则)

目录为什么这样切,全部由这三条推出:

1. **`assets/` 是唯一纳入版本管理的资源根。** 每个资产配一个同名 `.meta`(内含 UUID 与导入设置)。**`.meta` 由编辑器生成,禁止手工创建或编辑**——手写 UUID 会让引用错乱,且编辑器打开时会覆盖。
2. **只有两处能在运行时按路径加载**:① `assets/resources/**`(内置 Bundle,包名恒为 `resources`);② 被**配置为 AssetBundle 的目录**(`assetManager.loadBundle(name)`)。`assets/` 下其余资产只能靠编辑器里的引用(节点上拖)拿到,**`resources.load('scenes/Garden')` 这种写法必然失败**。
   → 推论:**凡是「代码按字符串路径取」的资源,必须落在 `resources/` 或某个 Bundle 内。**
3. **编辑器资源库与构建产物一律不入库**:`library/`、`temp/`、`profiles/`、`local/`、`build/`、`native/`。见 §五。

> 附带一条工程事实:本机**尚未安装 Cocos Creator 编辑器**。`garden-cocos/` 下的 `assets/` 树、`package.json`、`tsconfig.json` 已按 3.8 的形态手写就位,但 `.meta`、`settings/`、`library/` 只能由编辑器首次打开时生成;首次打开后编辑器可能规范化 `package.json` 与 `tsconfig.json`(例如 tsconfig 改为 `extends ./temp/tsconfig.cocos.json`),属预期行为。

---

## 二、资源组织结构

### 2.1 工程根

```
garden-cocos/                       # Cocos Creator 3.8 工程根(客户端)
├── assets/                         # ← 唯一入库的资源与脚本目录
├── settings/                       # 工程设置    ← 编辑器生成,不手工维护
├── library/  temp/  profiles/  local/   # 导入缓存 / 临时 / 本地偏好   ← 不入库
├── build/  native/                 # 构建输出 / 生成的平台工程          ← 不入库
├── package.json                    # 工程描述(name/uuid/引擎版本)
├── tsconfig.json                   # strict 全开(与仓库既有口径一致)
└── .gitignore
```

### 2.2 `assets/` 目录树(实际已创建)

```
assets/
├── resources/                      # 内置 Bundle(name 恒为 'resources'),启动即进首包
│   ├── ui/
│   │   ├── button/                 # ui_button_*
│   │   ├── panel/                  # ui_panel_*
│   │   └── popup/                  # ui_popup_*
│   ├── sprites/
│   │   ├── crops/                  # 常驻作物图:crop_sprout(通用芽)
│   │   ├── venues/                 # 场域背景六张:bg_venue_*
│   │   ├── plots/                  # 地块 / 土壤 / 设施:plot_*
│   │   ├── icons/                  # 图标:ico_*
│   │   └── ui/                     # 通用 UI 底图
│   ├── fx/                         # 特效与叠加贴图:fx_variant_*(变体遮罩)
│   ├── audio/
│   │   ├── bgm/                    # bgm_*
│   │   └── sfx/                    # sfx_*
│   ├── prefab/
│   │   ├── plot/                   # pf_plot_*
│   │   ├── crop/                   # pf_crop_*
│   │   ├── hud/                    # pf_hud_*
│   │   └── popup/                  # pf_popup_*
│   ├── fonts/                      # font_*
│   ├── atlas/                      # atlas_*
│   └── content/                    # ← 与仓库根 content/ 同步的拷贝(只读,见 §2.5)
│       └── i18n/                   # 文案 zh / en
├── bundles/                        # AssetBundle(编辑器里逐个「配置为 Bundle」)
│   ├── crops/                      # 包名 crops:86 种作物的成熟态贴图
│   │   └── sprites/crops/          #   crop_<crop_code>(每作物 1 张)
│   └── audio/                      # 包名 audio:背景音乐与音效
│       └── audio/
│           ├── bgm/
│           └── sfx/
├── scenes/                         # Loading.scene / MainMenu.scene / Garden.scene …
└── scripts/
    ├── core/                       # garden-core 的 wasm 绑定封装(仅适配层,无玩法逻辑)
    ├── managers/                   # ResourceManager 等
    ├── ui/                         # 视图与组件
    └── utils/                      # AssetPaths 等
```

> **「包内目录与 `resources/` 同构」是一条刻意的规则。** 代价是 `bundles/audio/` 下会出现同名子目录 `audio/`(即 `bundles/audio/audio/bgm/`),读起来略绕;收益是**同一路径字符串在 `resources` 与任意 Bundle 间可直接平移**——将来某个资源因为体积从 `resources` 挪进分包(或反向),玩法代码零改动。这个项目里「资源放包内还是包外」会随体积验收反复调整,所以值得。

### 2.3 关键目录职责

| 目录 | 放什么 | 不入什么 |
|---|---|---|
| `resources/ui/**` | 需按路径动态取的 UI 图(弹窗底、按钮态) | 只在场景里静态引用的装饰图 |
| `resources/sprites/venues/` | 六个场域的背景(**设计稿 §18.1 称其为唯一新增美术资产**) | 作物贴图 |
| `resources/sprites/crops/` | 仅 `crop_sprout`(通用芽,86 种共用) | 逐作物成熟态(在 `crops` 包) |
| `resources/content/` | `crops.json` / `venues.json` / `recipes.json` / `i18n/*.json` | 任何手改(见 §2.5) |
| `bundles/crops/**` | 86 张作物成熟态 | 地块 / UI |
| `bundles/audio/**` | bgm 与 sfx | — |
| `scenes/` | 场景文件(编辑器引用装配) | — |
| `scripts/core/` | `garden-core` wasm 绑定 | 玩法逻辑(玩法逻辑属核心) |

### 2.4 分包(AssetBundle)划分

| Bundle | 落地目录 | 内容 | 加载时机 |
|---|---|---|---|
| `resources`(内置) | `assets/resources/` | 首屏必需:UI、HUD、图标、字体、`content/*.json`、通用芽 | 启动 |
| `crops` | `assets/bundles/crops/` | 86 张作物成熟态 | 进入花园 / 打开图鉴时 |
| `audio` | `assets/bundles/audio/` | bgm / sfx | 首次播放前 |

划分理由:`resources` 里的一切都会进首包。86 张作物贴图放进去会把首屏下载撑大,而它们在「Loading → 主菜单」阶段一张都用不到;按包切开,只有真正进花园的玩家付这份流量。

> ⚠ **编辑器动作(`.meta` 无法手写,必须人工在编辑器里做一次)**
> 1. 右键 `assets/bundles/crops` → 「配置为 Bundle」,Bundle 名填 `crops`(`audio` 同理)。
> 2. Bundle 名必须与 `AssetPaths.BUNDLE` 里的字符串**逐字一致**,否则 `loadBundle` 静默走不通。
> 3. 「优先级 / 压缩类型」按 3.8 默认即可;`crops` 建议勾选合并依赖以减少请求数。

### 2.5 内容数据(`content/`)的唯一事实源

设计稿 §24.3 规定 `garden-core` 吃 `content/*.json`,而 Cocos 客户端也要吃同一份。**唯一事实源是仓库根 `content/`**(与 `garden-core` 平级),`garden-cocos/assets/resources/content/` 是**构建期拷贝**:

- **规则**:`assets/resources/content/**` 视为只读产物,**禁止直接在客户端侧改数据**;改数据只改仓库根 `content/`。
- **为什么不能把它整个搬进 Cocos 工程**:`garden-core` 是独立 crate,读配置需要编译期或运行期可见的路径;把数据放进客户端目录会让核心反过来依赖客户端,违反 §24.3「核心零 IO、零平台依赖」。
- **同步方式待定**:侵入期补一个拷贝脚本(或构建前置步骤),现阶段两份都不存在。见 §七。

---

## 三、资源命名规范

### 3.1 总则(四条,无例外)

1. **字符集**:仅 `a-z`、`0-9`、`_`;分段分隔符**只用下划线**。禁止中文、空格、大写、`-`、`@`、括号。文件扩展名由编辑器/构建决定,不写进名字。
2. **两条轨道,不混用**:
   - **资产轨道**(图片 / 音频 / 预制体 / 图集 / 字体):`<类前缀>_<域>_<对象>[_<限定>][_<变体>]`,**全小写 snake_case**。
   - **代码轨道**(`.ts` / `.scene`):**PascalCase**,与 `@ccclass('X')` 或导出类名逐字一致(编辑器按文件名注册组件与场景)。
3. **类前缀必须是 §3.2 表内的 11 个之一**,不得新造;要用新前缀,先改本表再落文件。
4. **长度**:建议 ≤ 40 字符,限定段 ≤ 2 段;完整路径(含 Bundle 名)≤ 100 字符。

### 3.2 类前缀表

| 前缀 | 用途 | 正例 |
|---|---|---|
| `ui_` | UI 组件图(按钮 / 面板 / 弹窗 / 进度条) | `ui_button_start.png`、`ui_panel_settings.png` |
| `ico_` | 图标 | `ico_water.png`、`ico_season_spring.png` |
| `bg_` | 背景(场域背景固定 `bg_venue_<场域小写>`) | `bg_venue_backyard.png` |
| `crop_` | 作物贴图 | `crop_rose.png`、`crop_sprout.png` |
| `plot_` | 地块 / 土壤 / 设施 | `plot_pot_01.png`、`plot_soil_loam.png` |
| `fx_` | 特效、粒子、**变体叠加遮罩** | `fx_water_drop.png`、`fx_variant_white_edge.png` |
| `pf_` | 预制体(`.prefab`) | `pf_plot_card.prefab`、`pf_popup_confirm.prefab` |
| `sfx_` | 音效 | `sfx_watering.mp3` |
| `bgm_` | 背景音乐 | `bgm_home_day.mp3` |
| `font_` | 字体 | `font_ui_zh.ttf`、`font_num.ttf` |
| `atlas_` | 图集 / 自动图集配置 | `atlas_ui_common` |

> **内容 JSON 是唯一例外,不走前缀**:固定 `content/<域>.json`,域取值以设计稿实际引用为准——`crops` / `venues` / `unlock` / `recipes` / `achievements` / `events` / `companions`;文案在 `content/i18n/<lang>.json`。
> 理由:该文件由 `garden-core` 与客户端**共用**,且 `content/crops.json` 这个名字已在设计稿 §5.2 写死(「每行即 `content/crops.json` 一条记录」),不能为了统一前缀去改它。

### 3.3 分类型细则

| 类别 | 规则 | 正例 |
|---|---|---|
| UI 组件图 | `ui_<部件>_<语义>` | `ui_button_start.png`、`ui_popup_confirm_bg.png` |
| 图标 | `ico_<语义>[_<修饰>]` | `ico_water.png`、`ico_season_winter.png` |
| 场域背景 | `bg_venue_<场域 code 小写>`(恰好六张) | `bg_venue_greenhouse.png` |
| 作物贴图 | `crop_<crop_code 小写>`;阶段覆盖 `crop_<code>_s<N>`;变体遮罩 `fx_variant_<叶纹>` | `crop_strawberry.png`、`crop_rose_s3.png` |
| 地块 / 土壤 | `plot_<类型>[_<序号>]` | `plot_pot_01.png`、`plot_soil_field.png` |
| 预制体 | `pf_<玩域>_<对象>` | `pf_plot_card.prefab`、`pf_hud_top_bar.prefab` |
| 音效 | `sfx_<动作>[_<对象>]` | `sfx_watering.mp3`、`sfx_harvest_coin.mp3` |
| 背景音乐 | `bgm_<场景>_<时段>` | `bgm_home_day.mp3`、`bgm_garden_night.mp3` |
| 字体 | `font_<用途>_<语种>` | `font_ui_zh.ttf`、`font_num.ttf` |
| 图集 | `atlas_<域>` | `atlas_ui_common` |
| 场景 | PascalCase,按用途 | `Loading.scene`、`MainMenu.scene`、`Garden.scene` |
| 脚本 | PascalCase,与导出类名一致 | `ResourceManager.ts` → `@ccclass('ResourceManager')`;`AssetPaths.ts`(纯常量模块,无 ccclass) |

### 3.4 作物与场域:数据 code ↔ 资源文件名的映射

数据层与文件层之间只允许这一种转换(已写死在 `AssetPaths` 里,不靠人记):

| 层 | 形态 | 例 |
|---|---|---|
| `crop_code`(数据 / 协议 / 存档) | UPPER_SNAKE | `SNAKE_PLANT`、`RADISH`、`STRAWBERRY` |
| 文件名段(资源) | 小写 snake_case | `snake_plant`、`radish`、`strawberry` |
| 生长阶段 | 默认**不进文件名**(由参数/缩放推导);确有独立贴图时才加 `_s<N>`,`N ∈ 1..6` | `crop_strawberry.png`、`crop_rose_s3.png` |
| 变体短码(§5.3.1) | `<花型>-<叶纹>-<颜色>`,大写三段 | `D-MAG-PINK`(重瓣 / 金边 / 粉) |
| 变体落图(默认) | **不逐作物出图**,叠一张通用叶纹遮罩 | `fx_variant_white_edge.png` 等三张,86 种复用 |
| 变体落图(个别作物要独立图时) | 小写并把 `-` 换 `_` | `crop_rose_d_mag_pink.png` |

- **为什么默认只出成熟态一张**:设计稿 §18.1 的成本表已判定「作物 × 6 阶段全画 = 516 张 ❌ / 只画成熟态 = 86 张 ✅」,阶段差异靠缩放与参数插值。资源侧照此执行:86 张成熟态 + 1 张通用芽 + 3 张变体遮罩。
- 变体短码的**完整取值表**以设计稿 §5.3.2 的 `trait_pool` 为准(该节已自标「P1 开工时必须补齐到 86 种」);本规范只定「三段顺序 + 缺位留空」,**不自造取值**。
- 未出变体时**不加任何段**,不要写 `_normal` / `_default`。

**场域 code——直接取设计稿 §16.4 `garden_plot.venue` 列的枚举(不自造)**:

| 场域(设计稿 §3) | code(`garden_plot.venue`) | 背景资源 |
|---|---|---|
| 室内窗台(含 +3 大花盆) | `WINDOWSILL` | `bg_venue_windowsill.png` |
| 花箱 | `PLANTER` | `bg_venue_planter.png` |
| 生态缸 | `TERRARIUM` | `bg_venue_terrarium.png` |
| 后院(含深土区) | `BACKYARD` | `bg_venue_backyard.png` |
| 农场(含农田设施与扩充) | `FARM` | `bg_venue_farm.png` |
| 温室大棚 | `GREENHOUSE` | `bg_venue_greenhouse.png` |

> 数据层 code 是**大写**(`venue VARCHAR(20)`),进文件名转小写(§3.1 的两条轨道),与作物码的处理一致。**四个附属阶段**(+3 大盆 / +2 深土垄 / +2 经济垄 / +1 垄)不单独建 code,沿用所属场域的背景与命名——与设计稿「附属阶段跟着场域走」一致。

> ⚠ **勘误(本规范初稿的错误)**:初稿把场域写成 `field` + `window_sill/flower_box/tank` 一套自造小写码(依据是《清单》§2.5.1 的存档示例 `plot_type: "window_sill"`)。**那是错的**——设计稿 §16.4 的列名是 `venue`,枚举是 `WINDOWSILL/PLANTER/TERRARIUM/BACKYARD/FARM/GREENHOUSE`。已按设计稿改正:目录 `sprites/fields/` → `sprites/venues/`,前缀 `bg_field_` → `bg_venue_`,`AssetPaths` 的 `FIELD` → `VENUE`。**《清单》不是设计源,以设计稿为准。**

### 3.5 图集

- **手动图集**:资产放 `resources/atlas/`,命名 `atlas_<域>`(如 `atlas_ui_common`);图集内 SpriteFrame 名沿用各自资产名,不重复加前缀。
- **自动图集(Auto Atlas)**:配置资产命名 `atlas_<目标目录名>`,置于目标目录的**父目录**,打包范围在编辑器里勾选。具体放置层级与 3.8 的行为差异以编辑器实测为准(见 §七)。
- **准入判据**:同一界面里会同时出现、且单个尺寸 ≤ 128px 的散图才进图集;场域背景(整屏)与作物成熟态(单张展示)不进图集。

### 3.6 反例对照(对《清单》§2.2.1 原文的勘误)

| 《清单》原文 | 问题 | 应为 |
|---|---|---|
| `prefab_player.ts` | ① 预制体不是 `.ts`,是 `.prefab`;② 家圃时光**没有玩家角色**(没有可操作角色,是网格经营) | `pf_plot_card.prefab` |
| `crop_carrot_stage1.png` | ① 阶段段写成 `stage1`,而 Stage 段规范是 `_s1`;② 作物码大小写两套(`carrot` 小写,但设计稿 §5.2 的 `crop_code` 是 `RADISH`/`TULIP` 这类大写) | `crop_radish.png`(仅成熟态) |
| `sfx_watering.mp3` | ✅ 合规 | 登记为规范正例 |
| `ui_button_start.png` | ✅ 合规 | 登记为规范正例 |
| 目录 `prefab/player/` | 无「玩家」概念 | `prefab/{plot,crop,hud,popup}/` |
| 目录 `assets/fonts/` | 字体要按语言/主题动态取,放 `assets/` 顶层**无法按路径加载**(违反 §一 规则 2) | `resources/fonts/` |

---

## 四、资源加载管理器(`ResourceManager`)

实现:`garden-cocos/assets/scripts/managers/ResourceManager.ts`(已通过 `tsc --noEmit`,strict 全开)。

### 4.1 设计取舍

| 决策 | 选择 | 理由 |
|---|---|---|
| 形态 | **纯类 + 单例**,不是 `Component` | 资源要在首场景加载**之前**就能取(Loading 场景、启动器);`Component` 依赖节点与生命周期,会逼出一个「引导节点必须最先加载」的隐性顺序约束 |
| 通道 | `resources` 与 AssetBundle **同一个入口** | 玩法代码不必关心资源在哪个包;资源换包不改编排代码 |
| 引用 | **加载即 `refs + 1`,必须成对 release** | 显式优于自动;实际回收交给 `releaseUnused()` |
| 去重 | 同 `bundle + 类型 + 路径` 的并发请求**合并为一次 IO** | 图鉴页会并发拉同一批静帧;不加锁会重复解码同一张图 |
| 失败 | 超时(默认 10s)+ 重试(默认 1 次),**失败不静默** | 资源缺了要立刻炸出来,不能给玩家留白 |
| 路径 | 一律来自 `AssetPaths` | 命名规范靠代码强制,不靠自觉;同时消灭 `/spriteFrame` 这类易漏后缀 |
| 缓存键 | `bundle + 构造器身份 + 路径` | 构建期会压缩改名,`type.name` 不能当键 |

### 4.2 两条通道

| 通道 | 对应目录 | 取法 |
|---|---|---|
| 内置 Bundle | `assets/resources/**` | 缺省,`{ bundle: BUNDLE.RESOURCES }` 可省略 |
| 自定义 Bundle | `assets/bundles/<name>/**` | `load` 时传 `{ bundle: BUNDLE.CROPS }`,或先 `loadBundle` |

两者路径规则完全一致(§2.2 的同构规则),唯一差别是 `resources` 不可卸载。

> ⚠ **SpriteFrame 是图片的子资产**:Cocos 里 `resources.load('a/b')` 拿到的是 `ImageAsset`,要当精灵用必须取 `a/b/spriteFrame`。这是新人最常踩的一脚,本管理器用 `loadSpriteFrame()` / `AssetPaths.spriteFramePath()` 统一补齐。

### 4.3 API

| 方法 | 说明 |
|---|---|
| `ResourceManager.instance` | 单例入口 |
| `load<T>(path, type, options?)` | 加载单个资产;并发去重;每调用方各持 1 引用 |
| `loadSpriteFrame(imagePath, bundle?)` | 加载图片并取 `SpriteFrame`(自动补 `/spriteFrame`) |
| `loadJson<T>(path, bundle?)` | 取内容 JSON 的 `json` 字段 |
| `loadDir<T>(dir, type, bundle?)` | 整目录加载;每资产 1 引用 |
| `preload(items, onProgress?)` | 批量预加载,**持有引用** |
| `release(path, type, bundle?)` | 释放一次加载;返回是否归零 |
| `releaseSpriteFrame(imagePath, bundle?)` | `loadSpriteFrame` 的配对释放 |
| `releaseDir(dir, type, bundle?)` | 整目录释放;返回释放数量 |
| `releaseUnused()` | 让引擎回收引用为零的资产(切场景后调一次) |
| `loadBundle(name)` / `releaseBundle(name)` | Bundle 的加载 / 卸载(引用计数) |
| `clear()` | 释放本管理器持有的全部 |
| `isLoaded` / `getLoaded` | 查询(`getLoaded` **不加引用**) |
| `stats()` | 调试:缓存条数 / 引用总数 / 在途请求 |

### 4.4 引用计数契约(必须遵守)

```
load()            → refs +1        release()        → refs -1,归零时 decRef()
loadDir()         → 每个资产 +1     releaseDir()     → 整目录 decRef()
loadBundle()      → bundle refs+1  releaseBundle()  → 归零时 releaseAll() + decRef() + 出缓存
preload()         → 等同 load()(持有引用)
clear()           → 释放本管理器持有的全部,并 releaseUnused()
```

- **规则:谁 `load` 谁 `release`。**
- **跨场景常驻的资源**(UI 图集、字体、`content/constants.json`)在**启动器/游戏入口**里加载并持有到进程结束,不要散在各场景的 `onLoad` 里各加载一份。
- **切场景后调一次 `releaseUnused()`**,让上一场景的残留资产真正释放。

### 4.5 生命周期与预加载时机

| 时机 | 加载什么 | 释放 |
|---|---|---|
| 启动(Loading 场景) | `resources` 内 UI / 字体 / `content/venues.json` | 进程结束 `clear()` |
| 进入花园场景 | `content/crops.json`;**不**预载作物贴图 | 离开花园且未进图鉴则 `release` |
| 打开图鉴 / 进入种植 | `loadBundle('crops')` + `loadDir('sprites/crops')` | 关闭时 `releaseDir` + `releaseBundle('crops')` |
| 首次播放音频 | `loadBundle('audio')` | 设置页关闭音效时 `releaseDir` / `releaseBundle` |

> 预加载的取舍原则:**按「玩家一定会看到」预载,按「可能不看」懒载**。作物贴图属于后者(86 张,而玩家一次只看几个阶段)。

### 4.6 失败、超时与重试

- 默认超时 10s、重试 1 次;可用 `options.timeoutMs` / `options.retries` 逐次覆盖(`timeoutMs <= 0` 为不限时)。
- 错误一律抛出 `Error`,消息含 `bundle/路径` 与原因;重试时 `console` 输出 `warn`,最终失败要由调用方决定是「占位图」还是「中断流程」,**不做静默降级**。
- 资源缺失在开发期应当**直接暴露**;上线后再按 §七的兜底策略处理。

### 4.7 用法示例

```ts
import { JsonAsset, SpriteFrame } from 'cc';
import { ResourceManager } from '../managers/ResourceManager';
import {
    BUNDLE, CONTENT_FILE, CROP_SPROUT, DIR, VENUE, contentPath, cropSpritePath, sfxPath,
    venueBackgroundPath,
} from '../utils/AssetPaths';

const rm = ResourceManager.instance;

// 1) 启动预载(持有引用,常驻到进程结束)
await rm.preload([
    { path: contentPath(CONTENT_FILE.VENUES), type: JsonAsset },
    { path: venueBackgroundPath(VENUE.BACKYARD), type: SpriteFrame },
]);

// 2) 取一张作物成熟态(`/spriteFrame` 后缀由管理器补齐)
const frame = await rm.loadSpriteFrame(cropSpritePath('STRAWBERRY'));
sprite.spriteFrame = frame;
// …用完之后
rm.releaseSpriteFrame(cropSpritePath('STRAWBERRY'));

// 3) 通用芽:所有作物共用一张,常驻
const sprout = await rm.loadSpriteFrame(CROP_SPROUT);

// 4) 内容数据
const crops = await rm.loadJson<{ code: string }[]>(contentPath('crops'));

// 5) 分包:进图鉴时载入,关闭时卸载
await rm.loadBundle(BUNDLE.CROPS);
const atlas = await rm.loadDir<SpriteFrame>(DIR.SPRITE_CROP, SpriteFrame, BUNDLE.CROPS);
atlas.forEach((f) => { /* 填图鉴格子 */ });
rm.releaseDir(DIR.SPRITE_CROP, SpriteFrame, BUNDLE.CROPS);
rm.releaseBundle(BUNDLE.CROPS);

// 6) 切场景后抖掉残留
rm.releaseUnused();
console.log(rm.stats());
```

### 4.8 内存与验收

- **验收口径**:首次进主菜单后 `stats().heldRefs` 应等于「常驻资源数」;进出图鉴一轮后 `heldRefs` 回到该值(说明分包与 `releaseDir` 配对正确)。
- **首屏预算**:`resources` 内的资产总量即首包下载量。任何新增资源先进 `resources`,超过阈值(建议 2MB)再挪进分包——同构规则保证挪包不改代码。
- **内存排查**:`stats()` + Chrome DevTools Memory 快照比对 `SpriteFrame` 实例数,定位「加载了没释放」的场景。

---

## 五、入库与忽略规则

**入库**:`assets/**`(含 `.meta`)、`package.json`、`tsconfig.json`、`.gitignore`。
**不入库**(`garden-cocos/.gitignore` 已配):`library/`、`temp/`、`profiles/`、`local/`、`build/`、`native/`、`node_modules/`。

- `.meta` **必须入库**:它是 UUID 与导入设置的载体,不入库会导致每次 clone 后编辑器重新生成 UUID、场景与预制体引用全部断链。
- 空目录用 `.gitkeep` 占位(git 不跟踪空目录);点文件编辑器资源库不导入,不会变成资产。
- 二进制美术资源的原始工程文件(PSD/AI/ASE)建议放仓库外的资源仓,只入库导出的 WebP 等成品。

---

## 六、落地检查清单

- [ ] 用 Cocos Creator 3.8 打开 `garden-cocos/`,确认 `assets/` 树被正确导入、`.meta` 已生成。
- [ ] 右键 `bundles/crops`、`bundles/audio` → 配置为 Bundle,名字与 `AssetPaths.BUNDLE` 逐字一致。
- [ ] `tsconfig.json` 的 strict 全开是否被编辑器覆盖成 `extends ./temp/tsconfig.cocos.json` 形态;若被覆盖,确认 strict 项仍在。
- [ ] 六张场域背景先落地(设计稿 §18.1 的实施顺序第 1 步:先定视觉语言)。
- [ ] `content/` 唯一事实源与客户端拷贝的同步方式定稿(§七第 1 项)。
- [x] 《设计稿》§18.1 / §31–§37 的口径已改写为 Cocos(2026-10-13,见设计稿文末增补)。

---

## 七、待定项(实现前必须定)

1. **`content/` 同步方式**:仓库根 `content/` → `assets/resources/content/` 是走脚本拷贝、构建前置步骤,还是让 Cocos 侧只保留一份并把根目录软链过去。**未定前两份都别建**。
2. **`content/venues.json` 的内容**:场域 code 已按设计稿 §16.4 定为 `WINDOWSILL/PLANTER/TERRARIUM/BACKYARD/FARM/GREENHOUSE`(§3.4),不再是待定项;但该 JSON 的**网格规格字段名**与 6 张背景的最终视觉仍未定。
3. **作物本体用贴图还是 Cocos `Graphics`**:设计稿 §18.1 已把这条列为待定。选贴图 → 按 §3.4 的 86+1+3 张执行;选 `Graphics`(在引擎里画几何)→ `resources/sprites/crops/` 与 `bundles/crops/` 的位图归零,§2.2 / §2.4 的作物部分作废。变体遮罩同理:默认 3 张通用遮罩,若改逐作物出图则资源量 86 → 86×N。
4. **自动图集在 3.8 的实际行为**(放置层级、是否与 Bundle 冲突):需在编辑器里实测一次再写死。
5. **`garden-web`(Vue 3)的去留**:客户端已定 Cocos(设计稿 §37.1),`garden-web` 与 `garden-native` 是上一轮的脚手架,需确认删除还是保留作参考。这决定 `docs/UI设计提示词.md` 该写哪一套。
