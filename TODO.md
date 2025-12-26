1. 本地 pgsql + drizzle 实现本地服务端? ✅
2. 本地 eslint 没配置好，比如每次保存自动格式化代码✅
3. 多元参考壁纸库，看侧边栏 or 菜单栏是怎么实现的，参考 UI
4. tauri 的控制台如何打开？✅
   前端：command + shift + i
   后端 rust： `log::info!("hello world");` `log::error!("hello error");`
5. 给个 Tab，分为静态,动态,自定义
6. 要优化壁纸的体验

- 可预览
- 懒加载

7. 设置中心

- 打开壁纸目录
- 一键清除缓存
- 快捷键设置
- 多语言
- 主题色

8. 快捷键

- ⌘ + H 历史记录
- ⌘ + S 设置中心
- ⌘ + R 刷新App（重新加载壁纸列表）?
- ⌘ + B 侧边栏

9. Search组件 + 联想功能 前后端
10. 相似推荐 这个功能是很重要的 能很大程度给用户提供便利，并增长留存时间. 但这个是怎么做的呢？给所有图片打 tag？
11. 数据源的话，可以尝试暂时用 pixabay（搜索时给每次都添加一个 wallpaper 关键字，搜索结果还不错）我去 pixabay 的 API 还要申请

# Plan

Lunaro 壁纸软件数据库设计实现计划

项目概述

设计并实现一个完整的 Mac
壁纸管理应用的数据库架构，支持静态/动态壁纸、分类标签、收藏轮播、历史记录等功能。

用户需求确认

- ✅ 支持静态和动态壁纸
- ✅ 分类和标签系统（风景、动漫、极简等）
- ✅ 收藏和喜欢功能
- ✅ 自动切换/轮播壁纸
- ✅ 下载和浏览历史追踪
- ✅ 本地文件系统存储（数据库存元数据）
- ✅ 第三方 API 作为壁纸来源（Unsplash、Pexels）

---

数据库架构设计

核心表结构（共 8 张表）

1.  pictures - 壁纸主表

存储所有壁纸的核心信息和元数据。

关键字段：

- id - 主键（自增）
- title, description - 基本信息
- type - 壁纸类型（静态/动态/live）枚举
- localPath - 本地文件路径（必需）
- thumbnailPath - 缩略图路径
- originalUrl - API 原始 URL
- fileSize, mimeType, width, height, aspectRatio - 文件属性
- source - 来源（unsplash/pexels/local/ai_generated）枚举
- sourceId, sourceAuthor, sourceAuthorUrl, sourceLicense - 来源元数据
- dominantColor, colorPalette - 颜色信息（用于搜索/过滤）
- categoryId - 分类外键
- status - 状态（active/archived/deleted）枚举
- duration, fps - 动态壁纸专用字段
- aiModel, aiPrompt, aiMetadata - AI 生成预留字段
- viewCount, downloadCount, favoriteCount - 统计数据
- createdAt, updatedAt, lastViewedAt - 时间戳

索引：

- category_id, type, source, status, created_at DESC, dominant_color

2.  categories - 分类表

壁纸分类管理（风景、动漫、极简等）。

关键字段：

- id - 主键
- name, slug - 名称和 URL 友好标识（唯一）
- description - 描述
- icon, coverImagePath - 视觉元素
- parentId - 支持嵌套分类（自引用外键）
- sortOrder - 显示排序
- isActive - 激活状态
- pictureCount - 壁纸数量（反范式化，性能优化）

3.  tags - 标签表

灵活的标签系统，支持多标签。

关键字段：

- id - 主键
- name, slug - 标签名称（唯一）
- color - 标签颜色（UI 显示）
- pictureCount - 使用次数统计

4.  picture_tags - 壁纸-标签关联表

多对多关系，一张壁纸可以有多个标签。

关键字段：

- id - 主键
- pictureId, tagId - 外键
- 唯一约束：(pictureId, tagId)

5.  favorites - 收藏表

用户收藏的壁纸。

关键字段：

- id - 主键
- pictureId - 壁纸外键
- note - 个人备注
- 唯一约束：pictureId（每张壁纸只能收藏一次）

6.  download_history - 下载历史

追踪壁纸下载记录。

关键字段：

- id - 主键
- pictureId - 壁纸外键
- downloadPath - 下载保存路径
- status - 下载状态（completed/failed/cancelled）
- errorMessage - 错误信息
- downloadDuration, downloadSpeed - 下载性能数据

7.  view_history - 浏览历史

追踪用户浏览行为。

关键字段：

- id - 主键
- pictureId - 壁纸外键
- viewDuration - 浏览时长（秒）
- viewSource - 浏览来源（gallery/search/rotation）
- deviceInfo - 设备信息（JSON）

8.  wallpaper_rotation - 轮播配置表

自动切换壁纸的配置。

关键字段：

- id - 主键
- name - 配置名称（如 "自然风光"、"暗色主题"）
- isActive - 是否启用（全局只能有一个激活）
- interval - 切换间隔（每分钟/每小时/每天等）枚举
- mode - 轮播模式（顺序/随机/洗牌/仅收藏）枚举
- categoryIds, tagIds - 过滤条件（JSON 数组）
- wallpaperType - 壁纸类型过滤
- onlyFavorites - 是否仅限收藏
- minWidth, minHeight - 分辨率过滤
- excludePictureIds - 排除列表
- currentPictureId - 当前壁纸
- lastRotationAt, nextRotationAt - 轮播时间追踪
- rotationHistory - 最近轮播历史（JSON 数组）

---

关键设计决策

1.  静态 vs 动态壁纸统一存储

- 方案： 使用单一 pictures 表，通过 type 字段区分
- 原因： 共享大部分字段（标题、分类、标签、收藏等），避免重复代码
- 动态壁纸特殊字段： duration, fps, mimeType (video/mp4)

2.  本地文件系统存储策略

/wallpapers/
├── static/ # 静态壁纸
│ ├── landscape/ # 按比例分类
│ ├── portrait/
│ └── square/
├── dynamic/ # 动态壁纸
│ ├── videos/
│ └── live/
├── thumbnails/ # 缩略图（提升加载性能）
│ ├── static/
│ └── dynamic/
└── temp/ # 临时下载目录

文件命名规范：
{source}_{sourceId}_{timestamp}\_{width}x{height}.{ext}
示例：unsplash_abc123_1702745600_1920x1080.jpg

3.  第三方 API 元数据存储

- 存储原始 URL、作者信息、许可证
- sourceId 用于去重（防止重复下载）
- 颜色信息（dominantColor, colorPalette）支持按颜色搜索

4.  性能优化策略

- 反范式化： pictureCount 字段避免昂贵的 COUNT 查询
- 统计字段： viewCount, downloadCount, favoriteCount 实时更新
- 索引策略： 在常用查询字段和外键上建立索引
- 缩略图： 列表视图使用缩略图，提升加载速度

5.  扩展性预留

- AI 生成字段： aiModel, aiPrompt, aiMetadata
- 多用户支持： 保留 usersTable，favorites/history 表可添加 userId
- JSONB 字段： 灵活存储未来需要的元数据

6.  软删除机制

- 使用 status 枚举（active/archived/deleted）
- 保留历史数据和统计信息
- 支持恢复操作

---

实现步骤

Phase 1: 数据库 Schema 定义 ⭐ 核心

文件： /Users/mac/Desktop/code/Lunaro/src/db/schema.ts

1.  导入 Drizzle ORM 类型：

- 添加 serial, text, timestamp, boolean, jsonb, pgEnum, unique

2.  定义枚举类型（5 个）：

- wallpaperTypeEnum: static, dynamic, live
- wallpaperSourceEnum: unsplash, pexels, wallhaven, local, ai_generated
- wallpaperStatusEnum: active, archived, deleted
- wallpaperRotationEnum: 时间间隔选项
- rotationModeEnum: sequential, random, shuffle, favorites_only

3.  定义表结构（8 张表）：

- picturesTable - 完整的字段定义
- categoriesTable - 支持嵌套分类
- tagsTable - 简单标签表
- pictureTagsTable - 多对多关联，带唯一约束
- favoritesTable - 收藏记录，带唯一约束
- downloadHistoryTable - 下载历史
- viewHistoryTable - 浏览历史
- wallpaperRotationTable - 轮播配置

4.  导出 TypeScript 类型：
    export type Picture = typeof picturesTable.$inferSelect;
 export type NewPicture = typeof picturesTable.$inferInsert;
    // ... 其他表的类型

Phase 2: 数据库连接和工具层

文件： /Users/mac/Desktop/code/Lunaro/src/db/index.ts（新建）

1.  创建数据库连接实例：
    import { drizzle } from 'drizzle-orm/node-postgres';
    import \* as schema from './schema';
    export const db = drizzle(process.env.DATABASE_URL!, { schema });
2.  重新导出 schema 供其他模块使用

Phase 3: 查询层（CRUD 操作）

文件： /Users/mac/Desktop/code/Lunaro/src/db/queries.ts（新建）

1.  Pictures 查询：

- createPicture() - 创建壁纸记录
- getPictureById() - 根据 ID 获取（包含关联数据）
- getPicturesByCategory() - 分类筛选
- searchPicturesByTags() - 标签搜索
- incrementViewCount() - 更新浏览统计

2.  Favorites 查询：

- addToFavorites() - 添加收藏（同时更新 favoriteCount）
- removeFromFavorites() - 取消收藏
- getAllFavorites() - 获取所有收藏

3.  Rotation 查询：

- getActiveRotation() - 获取激活的轮播配置
- getNextRotationPicture() - 根据配置获取下一张壁纸

4.  Categories & Tags 查询：

- getAllCategories(), createCategory()
- getAllTags(), createTag()
- addTagToPicture(), removeTagFromPicture()

Phase 4: 数据库迁移

执行命令：

# 1. 推送 schema 到数据库（自动创建表）

npm run db:push

# 2. 或者生成迁移文件（更正式的方式）

npx drizzle-kit generate

# 3. 验证表结构（可选，使用 Drizzle Studio）

npm run db:studio

Phase 5: Tauri 后端集成

文件： /Users/mac/Desktop/code/Lunaro/src-tauri/src/commands/wallpaper.rs（新建）

实现 Tauri 命令：

1.  set_wallpaper(path: String) - macOS 系统 API 设置壁纸
2.  get_wallpaper_dir() - 获取应用壁纸存储目录
3.  download_wallpaper(url: String, filename: String) - HTTP 下载壁纸

修改文件： /Users/mac/Desktop/code/Lunaro/src-tauri/src/lib.rs

- 导入 wallpaper 模块
- 注册命令到 tauri::generate_handler![]

修改文件： /Users/mac/Desktop/code/Lunaro/src-tauri/Cargo.toml

- 添加依赖：HTTP 客户端（如 reqwest）

Phase 6: 外部 API 服务层

文件： /Users/mac/Desktop/code/Lunaro/src/lib/services/unsplash.ts（新建）
文件： /Users/mac/Desktop/code/Lunaro/src/lib/services/pexels.ts（新建）

实现功能：

1.  API 密钥管理
2.  搜索壁纸接口
3.  下载壁纸并保存到本地
4.  提取元数据并存入数据库

Phase 7: 前端 Store 层

文件： /Users/mac/Desktop/code/Lunaro/src/lib/stores/wallpaper.svelte.ts（新建）

使用 Svelte 5 Runes 创建响应式 Store：
export class WallpaperStore {
private \_state = $state({
currentWallpaper: null as Picture | null,
categories: [] as Category[],
favorites: [] as Picture[],
// ...
});

async setWallpaper(picture: Picture) { /_ ... _/ }
async addToFavorites(pictureId: number) { /_ ... _/ }
// ...
}

Phase 8: UI 组件（后续）

- 壁纸画廊组件
- 分类/标签筛选器
- 收藏列表
- 轮播配置面板

---

需要修改/创建的文件清单

修改现有文件

1.  /Users/mac/Desktop/code/Lunaro/src/db/schema.ts

- 替换为完整的 8 表 schema
- 添加枚举定义
- 添加类型导出

2.  /Users/mac/Desktop/code/Lunaro/src-tauri/src/lib.rs

- 注册新的 wallpaper 命令

3.  /Users/mac/Desktop/code/Lunaro/src-tauri/Cargo.toml

- 添加 HTTP 客户端依赖

新建文件

4.  /Users/mac/Desktop/code/Lunaro/src/db/index.ts

- 数据库连接实例

5.  /Users/mac/Desktop/code/Lunaro/src/db/queries.ts

- CRUD 操作和复杂查询

6.  /Users/mac/Desktop/code/Lunaro/src-tauri/src/commands/wallpaper.rs

- Tauri 命令实现

7.  /Users/mac/Desktop/code/Lunaro/src/lib/services/unsplash.ts

- Unsplash API 集成

8.  /Users/mac/Desktop/code/Lunaro/src/lib/services/pexels.ts

- Pexels API 集成

9.  /Users/mac/Desktop/code/Lunaro/src/lib/stores/wallpaper.svelte.ts

- 前端状态管理

10. /Users/mac/Desktop/code/Lunaro/src/lib/types/wallpaper.ts（可选）

- 前端类型定义重新导出

---

初始数据建议

实现后可以手动或通过脚本插入初始数据：

初始分类（categories）

- 风景 (Landscape)
- 动漫 (Anime)
- 极简 (Minimalist)
- 抽象 (Abstract)
- 自然 (Nature)
- 城市 (Urban)
- 太空 (Space)

初始标签（tags）

- 蓝色、绿色、暖色、冷色（颜色系）
- 4K、8K（分辨率）
- 夜景、日出、日落（时间）
- 山脉、海洋、森林（自然元素）

---

数据库索引策略（迁移后执行）

-- Pictures 表核心索引
CREATE INDEX idx_pictures_category_status ON pictures(category_id, status);
CREATE INDEX idx_pictures_type_status ON pictures(type, status);
CREATE INDEX idx_pictures_created_at_desc ON pictures(created_at DESC);
CREATE INDEX idx_pictures_dominant_color ON pictures(dominant_color) WHERE dominant_color
IS NOT NULL;

-- 关联表索引
CREATE INDEX idx_picture_tags_tag_picture ON picture_tags(tag_id, picture_id);
CREATE INDEX idx_favorites_created_at_desc ON favorites(created_at DESC);

-- 历史表索引
CREATE INDEX idx_download_created_at_desc ON download_history(created_at DESC);
CREATE INDEX idx_view_created_at_desc ON view_history(created_at DESC);

---

注意事项

1.  环境变量配置

确保 .env 文件正确配置：
DATABASE_URL=postgres://postgres:54916@localhost:5432/postgres

2.  macOS 壁纸设置权限

Tauri 应用需要申请文件系统访问权限（在 tauri.conf.json 配置）。

3.  API 密钥管理

- Unsplash: https://unsplash.com/developers
- Pexels: https://www.pexels.com/api/
- 使用环境变量存储 API 密钥，不要提交到 Git

4.  文件大小管理

- 动态壁纸可能很大（视频文件），考虑：
  - 文件大小限制
  - 磁盘空间监控
  - 自动清理策略

5.  并发下载控制

实现下载队列，避免同时下载过多壁纸导致带宽耗尽。

---

后续扩展方向

1.  AI 壁纸生成

- 集成 DALL-E、Stable Diffusion API
- 使用预留的 aiModel, aiPrompt, aiMetadata 字段

2.  多用户支持

- 在 favorites/history 表添加 userId 外键
- 实现用户登录/同步功能

3.  云同步

- 使用 Firebase/Supabase 同步配置和收藏
- 跨设备壁纸库共享

4.  社区功能

- 用户上传分享壁纸
- 点赞评论系统

5.  高级筛选

- 按颜色搜索（使用 dominantColor 字段）
- 按分辨率、宽高比筛选
- AI 相似度搜索

---

估计的实现难度

| 阶段                     | 难度   | 说明                      |
| ------------------------ | ------ | ------------------------- |
| Phase 1-2: Schema & DB   | ⭐⭐   | 主要是表设计和 ORM 配置   |
| Phase 3: Queries         | ⭐⭐   | CRUD 操作相对直接         |
| Phase 4: Migration       | ⭐     | 一条命令完成              |
| Phase 5: Tauri Backend   | ⭐⭐⭐ | macOS 系统 API 调用有难度 |
| Phase 6: API Integration | ⭐⭐   | HTTP 请求和错误处理       |
| Phase 7-8: Frontend      | ⭐⭐⭐ | Svelte 5 + 复杂的 UI 交互 |

建议优先级： Phase 1-4（数据库层）→ Phase 6（API 集成）→ Phase 5（Tauri 后端）→ Phase
7-8（前端）

---

总结

这是一个完整的、可扩展的壁纸管理应用数据库设计方案，核心特点：

✅ 功能完整 - 涵盖所有用户需求（分类、标签、收藏、轮播、历史）
✅ 类型安全 - 使用 Drizzle ORM 和 TypeScript 枚举
✅ 性能优化 - 反范式化、索引策略、缩略图
✅ 可扩展 - 预留 AI 生成、多用户、云同步字段
✅ 架构清晰 - 前后端分离，Tauri + Svelte 5

下一步：执行 Phase 1（Schema 定义）和 Phase 4（数据库迁移），建立数据库基础。
