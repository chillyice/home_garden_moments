/**
 * AssetPaths —— 资源路径登记表。
 *
 * 玩法代码只准引用本文件的常量与构造函数,禁止硬编码路径字符串:
 * 命名规范靠这里强制,不靠自觉。规则见 `docs/CocosCreator资源管理规范.md` 第三章。
 *
 * 约定:
 *  - 构造函数返回的是**相对各自 Bundle 根**的路径,不含 `resources/` 前缀,不含扩展名;
 *  - 每个 Bundle 内部目录与 `resources/` 同构,因此同一路径字符串可在包间平移;
 *  - 数据层 code(crop_code)用 UPPER_SNAKE,进入文件名时统一转小写 snake_case。
 */

/** Bundle 名。必须与编辑器「配置为 Bundle」时填的名字逐字一致。 */
export const BUNDLE = {
    /** 内置 Bundle,`assets/resources/` 的包名恒为 'resources'。 */
    RESOURCES: 'resources',
    /** 作物成熟态贴图(每作物 1 张)。 */
    CROPS: 'crops',
    /** 背景音乐与音效。 */
    AUDIO: 'audio',
} as const;

export type BundleName = (typeof BUNDLE)[keyof typeof BUNDLE];

/** 目录常量(相对 Bundle 根)。 */
export const DIR = {
    UI: 'ui',
    UI_BUTTON: 'ui/button',
    UI_PANEL: 'ui/panel',
    UI_POPUP: 'ui/popup',
    SPRITE_CROP: 'sprites/crops',
    SPRITE_VENUE: 'sprites/venues',
    SPRITE_PLOT: 'sprites/plots',
    SPRITE_ICON: 'sprites/icons',
    SPRITE_UI: 'sprites/ui',
    FX: 'fx',
    AUDIO_BGM: 'audio/bgm',
    AUDIO_SFX: 'audio/sfx',
    PREFAB_PLOT: 'prefab/plot',
    PREFAB_CROP: 'prefab/crop',
    PREFAB_HUD: 'prefab/hud',
    PREFAB_POPUP: 'prefab/popup',
    FONT: 'fonts',
    ATLAS: 'atlas',
    CONTENT: 'content',
    I18N: 'content/i18n',
} as const;

/**
 * 场域 code —— 直接取自设计稿 §16.4 的 `garden_plot.venue` 列口径(不自造):
 * `WINDOWSILL / PLANTER / TERRARIUM / BACKYARD / FARM / GREENHOUSE`。
 * 四个附属阶段(+3 大盆 / +2 深土垄 / +2 经济垄 / +1 垄)沿用所属场域,不单独建 code;
 * 进入文件名时统一转小写(§3.1 的两条轨道)。
 */
export const VENUE = {
    WINDOWSILL: 'WINDOWSILL',
    PLANTER: 'PLANTER',
    TERRARIUM: 'TERRARIUM',
    BACKYARD: 'BACKYARD',
    FARM: 'FARM',
    GREENHOUSE: 'GREENHOUSE',
} as const;

export type VenueCode = (typeof VENUE)[keyof typeof VENUE];

/** 内容数据文件名(不含扩展名),与仓库根 `content/` 同名,并与设计稿各处引用一致。 */
export const CONTENT_FILE = {
    CROPS: 'crops',
    VENUES: 'venues',
    UNLOCK: 'unlock',
    RECIPES: 'recipes',
    ACHIEVEMENTS: 'achievements',
    EVENTS: 'events',
    COMPANIONS: 'companions',
} as const;

/** 生长阶段取值范围(设计稿 §5.1:6 个阶段由同一组参数插值)。 */
export const CROP_STAGE_MIN = 1;
export const CROP_STAGE_MAX = 6;

/** 常驻的通用芽贴图(86 种作物共用;成熟态之外的阶段默认由缩放/参数推导)。 */
export const CROP_SPROUT = 'sprites/crops/crop_sprout';

const SNAKE_RE = /^[a-z][a-z0-9_]*$/;

function assertSnake(value: string, label: string): string {
    if (!SNAKE_RE.test(value)) {
        throw new Error(`[AssetPaths] ${label} 必须是全小写 snake_case(仅 a-z/0-9/_),收到:"${value}"`);
    }
    return value;
}

function assertStage(stage: number): number {
    if (!Number.isInteger(stage) || stage < CROP_STAGE_MIN || stage > CROP_STAGE_MAX) {
        throw new Error(
            `[AssetPaths] 生长阶段必须是 ${CROP_STAGE_MIN}~${CROP_STAGE_MAX} 的整数,收到:${stage}`,
        );
    }
    return stage;
}

/** `SNAKE_PLANT` → `snake_plant`;变体短码 `D-MAG-PINK` → `d_mag_pink`。 */
export function toSnake(code: string): string {
    return code.trim().toLowerCase().replace(/-/g, '_');
}

/**
 * 作物贴图。
 * 默认返回**成熟态**一张(`crop_<code>`);仅当该阶段确实有独立贴图时才传 `stage`
 * (`crop_<code>_s<N>`)。变体不走逐作物出图,见 {@link cropVariantOverlayPath}。
 */
export function cropSpritePath(cropCode: string, stage?: number): string {
    const base = `crop_${assertSnake(toSnake(cropCode), 'cropCode')}`;
    if (stage === undefined) {
        return `${DIR.SPRITE_CROP}/${base}`;
    }
    return `${DIR.SPRITE_CROP}/${base}_s${assertStage(stage)}`;
}

/** 通用变体遮罩(叶纹:白边 / 斑叶 / 白化),86 种作物复用。 */
export function cropVariantOverlayPath(foliage: string): string {
    return `${DIR.FX}/fx_variant_${assertSnake(toSnake(foliage), 'foliage')}`;
}

/** 场域背景(六张之一)。`WINDOWSILL` → `bg_venue_windowsill`。 */
export function venueBackgroundPath(venue: string): string {
    return `${DIR.SPRITE_VENUE}/bg_venue_${assertSnake(toSnake(venue), 'venue')}`;
}

export function uiButtonPath(name: string): string {
    return `${DIR.UI_BUTTON}/ui_button_${assertSnake(name, 'button name')}`;
}

export function uiPanelPath(name: string): string {
    return `${DIR.UI_PANEL}/ui_panel_${assertSnake(name, 'panel name')}`;
}

export function uiPopupPath(name: string): string {
    return `${DIR.UI_POPUP}/ui_popup_${assertSnake(name, 'popup name')}`;
}

export function iconPath(name: string): string {
    return `${DIR.SPRITE_ICON}/ico_${assertSnake(name, 'icon name')}`;
}

/** 音效。注意:落在 `audio` Bundle 里,加载时需带 `{ bundle: BUNDLE.AUDIO }`。 */
export function sfxPath(name: string): string {
    return `${DIR.AUDIO_SFX}/sfx_${assertSnake(name, 'sfx name')}`;
}

/** 背景音乐。 */
export function bgmPath(name: string): string {
    return `${DIR.AUDIO_BGM}/bgm_${assertSnake(name, 'bgm name')}`;
}

export function plotPrefabPath(kind: string): string {
    return `${DIR.PREFAB_PLOT}/pf_plot_${assertSnake(kind, 'plot prefab kind')}`;
}

export function cropPrefabPath(kind: string): string {
    return `${DIR.PREFAB_CROP}/pf_crop_${assertSnake(kind, 'crop prefab kind')}`;
}

export function hudPrefabPath(kind: string): string {
    return `${DIR.PREFAB_HUD}/pf_hud_${assertSnake(kind, 'hud prefab kind')}`;
}

export function popupPrefabPath(name: string): string {
    return `${DIR.PREFAB_POPUP}/pf_popup_${assertSnake(name, 'popup prefab name')}`;
}

export function fontPath(name: string): string {
    return `${DIR.FONT}/font_${assertSnake(name, 'font name')}`;
}

export function atlasPath(name: string): string {
    return `${DIR.ATLAS}/atlas_${assertSnake(name, 'atlas name')}`;
}

/** 内容数据 JSON(不含扩展名)。 */
export function contentPath(file: string): string {
    return `${DIR.CONTENT}/${assertSnake(file, 'content file')}`;
}

/** 文案 JSON(不含扩展名)。 */
export function i18nPath(lang: string): string {
    return `${DIR.I18N}/${assertSnake(lang, 'language code')}`;
}

/**
 * 图片的 SpriteFrame 子资产路径。
 * Cocos 中图片不是直接可用的 SpriteFrame:`SpriteFrame` 是图片资产的子资产,
 * 动态加载时必须带 `/spriteFrame` 后缀。统一由这里补齐,避免各处漏写。
 */
export function spriteFramePath(imagePath: string): string {
    return `${imagePath}/spriteFrame`;
}
