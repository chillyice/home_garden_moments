# Cocos Creator Web 开发指南

> 本文档记录家圃时光项目的 Cocos Creator Web 开发工作流、配置、示例和优化指南。

## 一、项目配置（project.json）

### 1.1 基础配置

```json
{
  "name": "home-garden-moments",
  "version": "1.0.0",
  "description": "家圃时光 · Home Garden Moments",
  "author": "chillyice",
  "engine": "cocos-creator-3.8",
  "target": {
    "platform": "web-mobile",
    "buildPath": "build/web"
  },
  "settings": {
    "compatibility": {
      "requireStrict": false
    },
    "groupList": [
      "default",
      "ui",
      "audio",
      "sprite",
      "prefab"
    ]
  },
  "renderMode": 1,
  "webGL": {
    "extensions": [
      "OES_texture_float",
      "OES_texture_float_linear",
      "OES_standard_derivatives",
      "OES_element_index_uint"
    ]
  }
}
```

### 1.2 TypeScript 配置（tsconfig.json）

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "ES2020",
    "moduleResolution": "node",
    "allowSyntheticDefaultImports": true,
    "esModuleInterop": true,
    "strict": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noImplicitAny": true,
    "strictNullChecks": true,
    "strictFunctionTypes": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noImplicitReturns": true,
    "noFallthroughCasesInSwitch": true,
    "sourceMap": true
  },
  "include": [
    "assets/**/*"
  ],
  "exclude": [
    "build/**/*",
    "temp/**/*"
  ]
}
```

---

## 二、基础 UI 组件示例

### 2.1 按钮脚本（ButtonHandler.ts）

```typescript
import { _decorator, Component, Button, Node, EventKeyboard, KeyCode, input, InputEventType } from 'cc';

const { ccclass, property } = _decorator;

@ccclass('ButtonHandler')
export class ButtonHandler extends Component {
    @property(Button)
    private button: Button = null;

    @property(Node)
    private targetNode: Node = null;

    @property(Node)
    private targetNode2: Node = null;

    start() {
        if (this.button) {
            this.button.node.on(Button.EventType.CLICK, this.onButtonClick, this);
        }
    }

    onDestroy() {
        if (this.button) {
            this.button.node.off(Button.EventType.CLICK, this.onButtonClick, this);
        }
    }

    onButtonClick() {
        console.log('Button clicked!');

        // 切换目标节点显示/隐藏
        if (this.targetNode) {
            this.targetNode.active = !this.targetNode.active;
        }

        // 切换另一个目标节点显示/隐藏
        if (this.targetNode2) {
            this.targetNode2.active = !this.targetNode2.active;
        }
    }
}
```

### 2.2 场景管理器（SceneManager.ts）

```typescript
import { _decorator, Component, Scene, director, SceneAsset, Prefab, instantiate } from 'cc';

const { ccclass, property } = _decorator;

@ccclass('SceneManager')
export class SceneManager extends Component {
    @property(SceneAsset)
    private menuScene: SceneAsset = null;

    @property(SceneAsset)
    private gameScene: SceneAsset = null;

    @property(Prefab)
    private playerPrefab: Prefab = null;

    start() {
        // 加载菜单场景
        director.loadScene(this.menuScene.name);
    }

    /**
     * 切换到游戏场景
     */
    loadGameScene() {
        director.loadScene(this.gameScene.name);
    }

    /**
     * 切换到菜单场景
     */
    loadMenuScene() {
        director.loadScene(this.menuScene.name);
    }

    /**
     * 加载预制体并实例化
     */
    instantiatePlayer() {
        if (this.playerPrefab) {
            const player = instantiate(this.playerPrefab);
            director.getScene()?.addChild(player);
        }
    }
}
```

### 2.3 角色控制器（CharacterController.ts）

```typescript
import { _decorator, Component, RigidBody2D, input, InputEventType, Vec3, Vec2, EPhysics2DDrawFlags } from 'cc';

const { ccclass, property } = _decorator;

@ccclass('CharacterController')
export class CharacterController extends Component {
    @property(RigidBody2D)
    private rb: RigidBody2D = null;

    @property
    private moveSpeed = 5;

    @property
    private jumpForce = 10;

    private isGrounded = false;
    private velocity = new Vec2(0, 0);

    start() {
        this.node.on(InputEventType.KEY_DOWN, this.onKeyDown, this);
        this.node.on(InputEventType.KEY_UP, this.onKeyUp, this);
    }

    onDestroy() {
        this.node.off(InputEventType.KEY_DOWN, this.onKeyDown, this);
        this.node.off(InputEventType.KEY_UP, this.onKeyUp, this);
    }

    onKeyDown(event: any) {
        switch (event.keyCode) {
            case KeyCode.KEY_W:
                this.velocity.y = 1;
                break;
            case KeyCode.KEY_S:
                this.velocity.y = -1;
                break;
            case KeyCode.KEY_A:
                this.velocity.x = -1;
                break;
            case KeyCode.KEY_D:
                this.velocity.x = 1;
                break;
            case KeyCode.KEY_SPACE:
                if (this.isGrounded) {
                    this.jump();
                }
                break;
        }
    }

    onKeyUp(event: any) {
        switch (event.keyCode) {
            case KeyCode.KEY_W:
                if (this.velocity.y > 0) {
                    this.velocity.y = 0;
                }
                break;
            case KeyCode.KEY_S:
                if (this.velocity.y < 0) {
                    this.velocity.y = 0;
                }
                break;
            case KeyCode.KEY_A:
                if (this.velocity.x < 0) {
                    this.velocity.x = 0;
                }
                break;
            case KeyCode.KEY_D:
                if (this.velocity.x > 0) {
                    this.velocity.x = 0;
                }
                break;
        }
    }

    jump() {
        this.isGrounded = false;
        this.velocity.y = this.jumpForce;
    }

    update(deltaTime: number) {
        // 更新速度
        this.velocity.x *= 0.9; // 摩擦力
        this.velocity.y *= 0.9; // 重力

        // 应用速度
        this.rb.linearVelocity = new Vec3(this.velocity.x * this.moveSpeed, this.velocity.y * this.moveSpeed, 0);
    }

    onCollisionEnter(event: any) {
        const contact = event.contact;
        for (let i = 0; i < contact.count; i++) {
            const contactPoint = contact.getWorldPoint(i);
            const collision = contact.colliderB || contact.colliderA;
            if (collision.name === 'Ground') {
                this.isGrounded = true;
            }
        }
    }
}
```

### 2.4 动画控制器（AnimationController.ts）

```typescript
import { _decorator, Component, Animation, input, InputEventType } from 'cc';

const { ccclass, property } = _decorator;

@ccclass('AnimationController')
export class AnimationController extends Component {
    @property(Animation)
    private animation: Animation = null;

    private currentState = 'Idle';

    start() {
        this.node.on(InputEventType.KEY_DOWN, this.onKeyDown, this);
    }

    onDestroy() {
        this.node.off(InputEventType.KEY_DOWN, this.onKeyDown, this);
    }

    onKeyDown(event: any) {
        switch (event.keyCode) {
            case KeyCode.KEY_W: // W key
                this.changeAnimation('Run');
                break;
            case KeyCode.KEY_S: // S key
                this.changeAnimation('Idle');
                break;
        }
    }

    changeAnimation(newState: string) {
        if (this.currentState === newState) return;

        if (this.animation) {
            this.animation.play(newState);
            this.currentState = newState;
        }
    }
}
```

---

## 三、Web 部署工作流

### 3.1 本地开发

#### 3.1.1 启动本地服务器

```bash
# 进入项目根目录
cd C:\Users\chill\OneDrive\WorkStation\Projects\ihomy_games\home_garden_moments

# 使用 npx serve 启动本地服务器
npx serve build/web -p 8000
```

#### 3.1.2 访问游戏

打开浏览器访问：`http://localhost:8000`

---

### 3.2 测试阶段

#### 3.2.1 多浏览器测试

- **Chrome**：`chrome://version/`
- **Firefox**：`about:support`
- **Safari**：`Safari > 关于本机`
- **微信浏览器**：微信内置浏览器

#### 3.2.2 多设备测试

- **PC**：Windows 10/11、macOS
- **移动端**：Android、iOS
- **平板**：iPad、Android 平板

#### 3.2.3 性能测试

使用 Chrome DevTools 的 Performance 面板：

1. 打开 DevTools（F12）
2. 进入 Performance 面板
3. 点击 Record 开始录制
4. 运行游戏
5. 点击 Stop 停止录制
6. 分析性能报告

---

### 3.3 部署阶段

#### 3.3.1 部署到 Vercel（推荐）

**步骤**：

1. **安装 Vercel CLI**：

```bash
npm install -g vercel
```

2. **登录 Vercel**：

```bash
vercel login
```

3. **部署项目**：

```bash
cd build/web
vercel
```

4. **配置域名（可选）**：

```bash
vercel domains add your-domain.com
```

5. **设置环境变量（可选）**：

```bash
vercel env add API_KEY
```

**Vercel 配置文件（vercel.json）**：

```json
{
  "version": 2,
  "builds": [
    {
      "src": "index.html",
      "use": "@vercel/static"
    }
  ],
  "routes": [
    {
      "src": "/(.*)",
      "dest": "/index.html"
    }
  ]
}
```

---

#### 3.3.2 部署到 Netlify

**步骤**：

1. **安装 Netlify CLI**：

```bash
npm install -g netlify-cli
```

2. **登录 Netlify**：

```bash
netlify login
```

3. **部署项目**：

```bash
cd build/web
netlify deploy --prod
```

4. **配置构建命令（可选）**：

在 Netlify 网页控制台配置：

- **Build command**：`cd build/web`
- **Publish directory**：`/`

**Netlify 配置文件（netlify.toml）**：

```toml
[build]
  command = "cd build/web"
  publish = "/"

[[redirects]]
  from = "/*"
  to = "/index.html"
  status = 200
```

---

#### 3.3.3 部署到 GitHub Pages

**步骤**：

1. **创建 GitHub 仓库**：

```bash
git init
git add .
git commit -m "Initial commit"
git remote add origin https://github.com/chillyice/home-garden-moments.git
git branch -M main
git push -u origin main
```

2. **启用 GitHub Pages**：

- 进入仓库 Settings
- 点击 Pages
- 选择 `main` 分支
- 点击 Save

3. **等待部署完成**：

访问：`https://chillyice.github.io/home-garden-moments/`

---

## 四、性能优化指南

### 4.1 渲染模式优化

| 模式 | 特点 | 适用场景 | 性能 |
|------|------|---------|------|
| **Canvas 模式** | 兼容性好，性能一般 | 低端设备 | ⭐⭐ |
| **WebGL 模式** | 性能好，效果强 | 大多数设备 | ⭐⭐⭐⭐ |
| **WebGL2 模式** | 性能更强，支持更多特性 | 高端设备 | ⭐⭐⭐⭐⭐ |

**配置方式**（`project.json`）：

```json
{
  "renderMode": 1,  // 0=Canvas, 1=WebGL, 2=WebGL2
  "webGL": {
    "extensions": [
      "OES_texture_float",
      "OES_texture_float_linear",
      "OES_standard_derivatives",
      "OES_element_index_uint"
    ]
  }
}
```

---

### 4.2 资源优化

#### 4.2.1 图集打包

**步骤**：

1. 在 Cocos Creator 编辑器中，选中多个精灵
2. 右键 → SpriteFrame → Create Sprite Atlas
3. 命名图集（如 `ui_atlas`）
4. 在 `assets/resources/sprites/ui_atlas` 中找到生成的图集

**优势**：
- 减少纹理切换
- 减少 Draw Call
- 提升性能

---

#### 4.2.2 压缩纹理

**步骤**：

1. 在 Cocos Creator 编辑器中，选中纹理资源
2. 右键 → Compress Texture
3. 选择压缩格式（如 ASTC、ETC2）
4. 点击 Compress

**压缩格式选择**：

| 格式 | 设备支持 | 压缩率 | 质量 |
|------|---------|--------|------|
| **ASTC** | Android、iOS | 高 | 高 |
| **ETC2** | Android、iOS | 中 | 中 |
| **PVRTC** | iOS | 高 | 高 |
| **DXT** | PC | 中 | 中 |

---

#### 4.2.3 精灵图集

**步骤**：

1. 在 Cocos Creator 编辑器中，选中多个精灵
2. 右键 → SpriteFrame → Create Sprite Atlas
3. 命名图集（如 `ui_atlas`）
4. 在 `assets/resources/sprites/ui_atlas` 中找到生成的图集

**优势**：
- 减少 Draw Call
- 提升性能
- 方便管理

---

#### 4.2.4 资源预加载

```typescript
import { _decorator, resources, AssetManager } from 'cc';

const { ccclass, property } = _decorator;

@ccclass('ResourceManager')
export class ResourceManager extends Component {
    private assetManager: AssetManager = resources;

    /**
     * 预加载资源
     */
    async preloadAssets() {
        // 预加载 UI 资源
        await this.loadPrefab('ui/button');
        await this.loadPrefab('ui/panel');

        // 预加载音频资源
        await this.loadAudio('audio/bgm');
        await this.loadAudio('audio/sfx');

        console.log('所有资源加载完成！');
    }

    /**
     * 加载预制体
     */
    async loadPrefab(path: string): Promise<any> {
        return new Promise((resolve, reject) => {
            this.assetManager.loadDir(path, (err, assets) => {
                if (err) {
                    reject(err);
                } else {
                    resolve(assets);
                }
            });
        });
    }

    /**
     * 加载音频
     */
    async loadAudio(path: string): Promise<any> {
        return new Promise((resolve, reject) => {
            this.assetManager.loadDir(path, (err, assets) => {
                if (err) {
                    reject(err);
                } else {
                    resolve(assets);
                }
            });
        });
    }
}
```

---

### 4.3 代码优化

#### 4.3.1 对象池优化

**示例**：

```typescript
import { _decorator, Node, Prefab, instantiate } from 'cc';

const { ccclass, property } = _decorator;

@ccclass('ObjectPool')
export class ObjectPool {
    private pool: Node[] = [];
    private prefab: Prefab;

    constructor(prefab: Prefab) {
        this.prefab = prefab;
    }

    /**
     * 从池中获取对象
     */
    get(): Node | null {
        let node: Node | null = null;

        if (this.pool.length > 0) {
            node = this.pool.pop();
        } else {
            node = instantiate(this.prefab);
        }

        return node;
    }

    /**
     * 回收对象到池中
     */
    put(node: Node) {
        node.active = false;
        this.pool.push(node);
    }
}
```

---

#### 4.3.2 减少 GC（垃圾回收）

**避免**：

```typescript
// ❌ 不好的做法：在循环中创建对象
for (let i = 0; i < 10000; i++) {
    const obj = new GameObject();
    // ...
}
```

**推荐**：

```typescript
// ✅ 好的做法：使用对象池
const pool = [];

for (let i = 0; i < 10000; i++) {
    let obj = pool.pop();
    if (!obj) {
        obj = new GameObject();
    }
    // ...
    pool.push(obj);
}
```

---

#### 4.3.3 减少重复计算

```typescript
// ❌ 不好的做法：在 update 中重复计算
update(deltaTime: number) {
    const distance = Math.sqrt(this.x * this.x + this.y * this.y);
    const angle = Math.atan2(this.y, this.x);
    // ...
}

// ✅ 好的做法：缓存计算结果
private _distance = 0;
private _angle = 0;

update(deltaTime: number) {
    this._distance = Math.sqrt(this.x * this.x + this.y * this.y);
    this._angle = Math.atan2(this.y, this.x);
    // ...
}
```

---

## 五、常见问题

### 5.1 游戏无法启动

**原因**：资源加载失败

**解决方案**：

1. 检查 `build/web` 目录是否存在
2. 检查 `index.html` 是否存在
3. 检查浏览器控制台是否有错误信息

---

### 5.2 性能卡顿

**原因**：Draw Call 过多、资源未压缩、代码未优化

**解决方案**：

1. 使用图集打包
2. 压缩纹理
3. 使用对象池
4. 减少 GC

---

### 5.3 移动端无法访问

**原因**：移动端浏览器不支持 WebGL

**解决方案**：

1. 检查浏览器版本
2. 使用 Canvas 模式（`project.json` 中设置 `"renderMode": 0`）
3. 优化代码性能

---

## 六、相关资源

- **Cocos Creator 官方文档**：https://docs.cocos.com/creator/manual/zh/
- **Cocos Creator 社区**：https://forum.cocos.org/
- **Cocos Creator GitHub**：https://github.com/cocos-creator/engine
- **Web 部署工具**：
  - Vercel：https://vercel.com/
  - Netlify：https://www.netlify.com/
  - GitHub Pages：https://pages.github.com/
