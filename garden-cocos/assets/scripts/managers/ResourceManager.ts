import { Asset, AssetManager, JsonAsset, SpriteFrame, assetManager, resources, warn } from 'cc';
import { BUNDLE, spriteFramePath } from '../utils/AssetPaths';

/** 资源构造器(与 Cocos `AssetType<T>` 同形,避免依赖引擎内部类型导出)。 */
export type AssetCtor<T extends Asset> = new (...args: any[]) => T;

export interface LoadOptions {
    /** 目标 Bundle 名,缺省 `resources`。 */
    bundle?: string;
    /** 单次加载超时(毫秒),<= 0 表示不限时。缺省 10000。 */
    timeoutMs?: number;
    /** 失败重试次数(不含首次)。缺省 1。 */
    retries?: number;
}

export interface PreloadItem {
    path: string;
    type: AssetCtor<Asset>;
    bundle?: string;
}

export interface ResourceStats {
    /** 按路径缓存的资产条数。 */
    cachedAssets: number;
    /** 按路径持有的引用总数(含目录加载)。 */
    heldRefs: number;
    /** 已缓存的 Bundle 数(不含内置 `resources`)。 */
    cachedBundles: number;
    /** 进行中的请求数(含 Bundle)。 */
    inflightRequests: number;
}

const DEFAULT_TIMEOUT_MS = 10000;
const DEFAULT_RETRIES = 1;

interface AssetEntry {
    asset: Asset;
    refs: number;
}

interface BundleEntry {
    bundle: AssetManager.Bundle;
    /** 显式持有者计数;0 表示仅被本管理器缓存,可随时释放。 */
    refs: number;
}

/**
 * ResourceManager —— 家圃时光统一的资源加载 / 卸载入口。
 *
 * 为什么是纯类 + 单例,而不是 `Component`:
 * 资源要在首场景加载**之前**就能取(Loading 场景、启动器),`Component` 依赖节点与生命周期,
 * 用它会逼出一个「引导节点必须最先加载」的隐性顺序约束。
 *
 * 通道:`resources`(内置)与 AssetBundle 走同一个入口,玩法代码不必关心资源在哪个包。
 * 引用:加载即 `refs + 1`,**必须成对 release**;`releaseUnused()` 负责实际回收。
 * 路径:一律取自 `AssetPaths`,禁止硬编码字符串。
 */
export class ResourceManager {
    private static _instance: ResourceManager | null = null;

    static get instance(): ResourceManager {
        if (ResourceManager._instance === null) {
            ResourceManager._instance = new ResourceManager();
        }
        return ResourceManager._instance;
    }

    /** 释放全部缓存并丢弃单例(整局重启 / 单元测试用)。 */
    static reset(): void {
        if (ResourceManager._instance !== null) {
            ResourceManager._instance.clear();
            ResourceManager._instance = null;
        }
    }

    private readonly _assets = new Map<string, AssetEntry>();
    private readonly _inflight = new Map<string, Promise<Asset>>();
    /** 与 `_inflight` 同生命周期:记录合并到同一次请求上的额外调用方数量。 */
    private readonly _pendingRefs = new Map<string, number>();
    private readonly _dirAssets = new Map<string, Asset[]>();
    private readonly _bundles = new Map<string, BundleEntry>();
    private readonly _bundleInflight = new Map<string, Promise<AssetManager.Bundle>>();
    private readonly _typeIds = new WeakMap<AssetCtor<Asset>, number>();
    private _nextTypeId = 1;

    // ---------------------------------------------------------------- 加载

    /**
     * 按路径加载单个资产。同 `bundle + 类型 + 路径` 的并发请求会合并为一次 IO,
     * 每个调用方各自持有 1 个引用。
     */
    load<T extends Asset>(path: string, type: AssetCtor<T>, options: LoadOptions = {}): Promise<T> {
        const bundleName = options.bundle ?? BUNDLE.RESOURCES;
        const key = this.assetKey(bundleName, type, path);

        const cached = this._assets.get(key);
        if (cached !== undefined) {
            cached.refs += 1;
            return Promise.resolve(cached.asset as T);
        }

        const pending = this._inflight.get(key);
        if (pending !== undefined) {
            this._pendingRefs.set(key, (this._pendingRefs.get(key) ?? 0) + 1);
            return pending as Promise<T>;
        }

        const task = this.runLoad(key, bundleName, path, type, options);
        this._inflight.set(key, task);
        // 调用方拿到的是同一个 promise,这里只兜住「无人 catch」的 rejection 警告。
        void task.catch(() => undefined);
        return task;
    }

    /** 加载图片并取它的 SpriteFrame 子资产(`/spriteFrame` 后缀由 AssetPaths 统一补齐)。 */
    loadSpriteFrame(imagePath: string, bundleName: string = BUNDLE.RESOURCES): Promise<SpriteFrame> {
        return this.load(spriteFramePath(imagePath), SpriteFrame, { bundle: bundleName });
    }

    /** 加载内容数据 JSON。 */
    async loadJson<T = unknown>(path: string, bundleName: string = BUNDLE.RESOURCES): Promise<T> {
        const asset = await this.load(path, JsonAsset, { bundle: bundleName });
        return asset.json as T;
    }

    /**
     * 加载整个目录。目录内每个资产各持 1 个引用,用 {@link releaseDir} 成组释放。
     */
    async loadDir<T extends Asset>(
        dir: string,
        type: AssetCtor<T>,
        bundleName: string = BUNDLE.RESOURCES,
    ): Promise<T[]> {
        const key = this.dirKey(bundleName, type, dir);
        const cached = this._dirAssets.get(key);
        if (cached !== undefined) {
            cached.forEach((asset) => asset.addRef());
            return cached as T[];
        }

        const bundle = await this.ensureBundle(bundleName);
        const assets = await new Promise<T[]>((resolve, reject) => {
            bundle.loadDir(dir, type, (err, list) => {
                if (err !== null || list === undefined) {
                    reject(this.wrapError(err, `目录加载失败:${bundleName}/${dir}`));
                    return;
                }
                resolve(list);
            });
        });

        assets.forEach((asset) => asset.addRef());
        this._dirAssets.set(key, assets as Asset[]);
        return assets;
    }

    /**
     * 批量预加载。**会持有引用(+1)** —— 调用方负责在退出时 `release` / `releaseDir`,
     * 或者把它当作常驻资源持有到进程结束。
     */
    async preload(
        items: PreloadItem[],
        onProgress?: (finished: number, total: number) => void,
    ): Promise<void> {
        const total = items.length;
        let finished = 0;
        onProgress?.(0, total);
        await Promise.all(
            items.map(async (item) => {
                await this.load(item.path, item.type, { bundle: item.bundle });
                finished += 1;
                onProgress?.(finished, total);
            }),
        );
    }

    // ---------------------------------------------------------------- 释放

    /**
     * 释放一次加载。引用归零时对资产 `decRef()`,实际回收交给 {@link releaseUnused}。
     * @returns 本次调用是否使该资产引用归零
     */
    release(path: string, type: AssetCtor<Asset>, bundleName: string = BUNDLE.RESOURCES): boolean {
        const key = this.assetKey(bundleName, type, path);
        const entry = this._assets.get(key);
        if (entry === undefined) {
            warn(`[ResourceManager] release 未命中(未加载或已释放):${bundleName}/${path}`);
            return false;
        }
        entry.refs -= 1;
        if (entry.refs > 0) {
            return false;
        }
        this._assets.delete(key);
        entry.asset.decRef();
        return true;
    }

    /** {@link loadSpriteFrame} 的配对释放。 */
    releaseSpriteFrame(imagePath: string, bundleName: string = BUNDLE.RESOURCES): boolean {
        return this.release(spriteFramePath(imagePath), SpriteFrame, bundleName);
    }

    /** 整目录释放(与 {@link loadDir} 配对)。@returns 释放的资产数 */
    releaseDir(dir: string, type: AssetCtor<Asset>, bundleName: string = BUNDLE.RESOURCES): number {
        const key = this.dirKey(bundleName, type, dir);
        const cached = this._dirAssets.get(key);
        if (cached === undefined) {
            warn(`[ResourceManager] releaseDir 未命中:${bundleName}/${dir}`);
            return 0;
        }
        this._dirAssets.delete(key);
        cached.forEach((asset) => asset.decRef());
        return cached.length;
    }

    /** 让引擎回收引用计数归零的资产。切场景后调一次即可。 */
    releaseUnused(): void {
        assetManager.releaseUnusedAssets();
    }

    /** 加载 Bundle。返回后每个调用方各持 1 个引用,与 {@link releaseBundle} 配对。 */
    async loadBundle(name: string): Promise<AssetManager.Bundle> {
        const bundle = await this.ensureBundle(name);
        const entry = this._bundles.get(name);
        if (entry !== undefined) {
            entry.refs += 1;
        }
        return bundle;
    }

    /**
     * 释放 Bundle。引用归零时 `releaseAll()` + `decRef()` 并移出缓存。
     * @returns 本次调用是否真正卸载了该 Bundle
     */
    releaseBundle(name: string): boolean {
        if (name === BUNDLE.RESOURCES) {
            warn('[ResourceManager] resources 是内置 Bundle,不可卸载');
            return false;
        }
        const entry = this._bundles.get(name);
        if (entry === undefined) {
            warn(`[ResourceManager] releaseBundle 未命中:${name}`);
            return false;
        }
        entry.refs -= 1;
        if (entry.refs > 0) {
            return false;
        }
        entry.bundle.releaseAll();
        entry.bundle.decRef();
        this._bundles.delete(name);
        return true;
    }

    /** 释放本管理器持有的全部资产与 Bundle。退出游戏 / 重登时调用。 */
    clear(): void {
        this._assets.forEach((entry) => entry.asset.decRef());
        this._dirAssets.forEach((list) => list.forEach((asset) => asset.decRef()));
        this._bundles.forEach((entry) => {
            entry.bundle.releaseAll();
            entry.bundle.decRef();
        });
        this._assets.clear();
        this._dirAssets.clear();
        this._bundles.clear();
        this._inflight.clear();
        this._pendingRefs.clear();
        this._bundleInflight.clear();
        this.releaseUnused();
    }

    // ---------------------------------------------------------------- 查询

    isLoaded(path: string, type: AssetCtor<Asset>, bundleName: string = BUNDLE.RESOURCES): boolean {
        return this._assets.has(this.assetKey(bundleName, type, path));
    }

    /** 取已加载的资产,**不增加引用**(只读用途)。未加载返回 null。 */
    getLoaded<T extends Asset>(
        path: string,
        type: AssetCtor<T>,
        bundleName: string = BUNDLE.RESOURCES,
    ): T | null {
        const entry = this._assets.get(this.assetKey(bundleName, type, path));
        return entry === undefined ? null : (entry.asset as T);
    }

    stats(): ResourceStats {
        let heldRefs = 0;
        this._assets.forEach((entry) => (heldRefs += entry.refs));
        this._dirAssets.forEach((list) => (heldRefs += list.length));
        return {
            cachedAssets: this._assets.size,
            heldRefs,
            cachedBundles: this._bundles.size,
            inflightRequests: this._inflight.size + this._bundleInflight.size,
        };
    }

    // ---------------------------------------------------------------- 内部

    private async runLoad<T extends Asset>(
        key: string,
        bundleName: string,
        path: string,
        type: AssetCtor<T>,
        options: LoadOptions,
    ): Promise<T> {
        try {
            const bundle = await this.ensureBundle(bundleName);
            const asset = await this.loadWithRetry(bundle, path, type, options);
            asset.addRef();
            this._assets.set(key, { asset, refs: 1 + (this._pendingRefs.get(key) ?? 0) });
            return asset;
        } finally {
            this._pendingRefs.delete(key);
            this._inflight.delete(key);
        }
    }

    private async loadWithRetry<T extends Asset>(
        bundle: AssetManager.Bundle,
        path: string,
        type: AssetCtor<T>,
        options: LoadOptions,
    ): Promise<T> {
        const retries = options.retries ?? DEFAULT_RETRIES;
        let lastError: Error | null = null;
        for (let attempt = 0; attempt <= retries; attempt += 1) {
            try {
                return await this.loadRaw(bundle, path, type, options.timeoutMs ?? DEFAULT_TIMEOUT_MS);
            } catch (err) {
                lastError = this.wrapError(err, `加载失败:${bundle.name}/${path}`);
                if (attempt < retries) {
                    warn(`[ResourceManager] 第 ${attempt + 1} 次失败,重试:${bundle.name}/${path}`);
                }
            }
        }
        throw lastError ?? new Error(`[ResourceManager] 加载失败:${bundle.name}/${path}`);
    }

    private loadRaw<T extends Asset>(
        bundle: AssetManager.Bundle,
        path: string,
        type: AssetCtor<T>,
        timeoutMs: number,
    ): Promise<T> {
        return new Promise<T>((resolve, reject) => {
            let settled = false;
            let timer: ReturnType<typeof setTimeout> | null = null;

            if (timeoutMs > 0) {
                timer = setTimeout(() => {
                    if (settled) {
                        return;
                    }
                    settled = true;
                    reject(
                        new Error(`[ResourceManager] 加载超时(${timeoutMs}ms):${bundle.name}/${path}`),
                    );
                }, timeoutMs);
            }

            bundle.load(path, type, (err, asset) => {
                if (settled) {
                    return;
                }
                settled = true;
                if (timer !== null) {
                    clearTimeout(timer);
                }
                if (err !== null || asset === undefined) {
                    reject(this.wrapError(err, `加载失败:${bundle.name}/${path}`));
                    return;
                }
                resolve(asset);
            });
        });
    }

    private async ensureBundle(name: string): Promise<AssetManager.Bundle> {
        if (name === BUNDLE.RESOURCES) {
            return resources;
        }
        const cached = this._bundles.get(name);
        if (cached !== undefined) {
            return cached.bundle;
        }
        const pending = this._bundleInflight.get(name);
        if (pending !== undefined) {
            return pending;
        }

        const task = new Promise<AssetManager.Bundle>((resolve, reject) => {
            assetManager.loadBundle(name, (err, bundle) => {
                if (err !== null || bundle === undefined) {
                    reject(this.wrapError(err, `Bundle 加载失败:${name}`));
                    return;
                }
                bundle.addRef();
                resolve(bundle);
            });
        });
        this._bundleInflight.set(name, task);
        try {
            const bundle = await task;
            this._bundles.set(name, { bundle, refs: 0 });
            return bundle;
        } finally {
            this._bundleInflight.delete(name);
        }
    }

    private assetKey(bundleName: string, type: AssetCtor<Asset>, path: string): string {
        return `${bundleName}|${this.typeId(type)}|${path}`;
    }

    private dirKey(bundleName: string, type: AssetCtor<Asset>, dir: string): string {
        return `${bundleName}|${this.typeId(type)}|${dir}/`;
    }

    /** 用构造器身份做缓存键的一部分:`type.name` 会被构建期压缩改名,不能当键。 */
    private typeId(type: AssetCtor<Asset>): number {
        let id = this._typeIds.get(type);
        if (id === undefined) {
            id = this._nextTypeId;
            this._nextTypeId += 1;
            this._typeIds.set(type, id);
        }
        return id;
    }

    private wrapError(err: unknown, message: string): Error {
        if (err instanceof Error) {
            return err;
        }
        if (typeof err === 'string' && err.length > 0) {
            return new Error(`[ResourceManager] ${message} —— ${err}`);
        }
        return new Error(`[ResourceManager] ${message}`);
    }
}
