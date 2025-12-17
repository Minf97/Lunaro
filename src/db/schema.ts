import { integer, pgTable, text, timestamp, varchar } from 'drizzle-orm/pg-core';

export const usersTable = pgTable('users', {
	id: integer().primaryKey().generatedAlwaysAsIdentity(),
	name: varchar({ length: 255 }).notNull(),
	age: integer().notNull(),
	email: varchar({ length: 255 }).notNull().unique()
});

export const picturesTable = pgTable('pictures', {
	id: integer().primaryKey().generatedAlwaysAsIdentity(),
	// Pexels 元数据
	pexelsId: integer('pexels_id').notNull().unique(), // Pexels 图片 ID（防重复）
	photographer: varchar({ length: 255 }), // 摄影师
	photographerUrl: text('photographer_url'), // 摄影师主页,
	title: varchar({ length: 255 }).notNull(),
	// 文件信息
	localPath: text('local_path'), // TODO: 本地图片路径?
	originalUrl: text('original_url').notNull(), // 原始图片 URL
	thumbnailUrl: text('thumbnail_url').notNull(), // 缩略图 URL
	// 图片尺寸
	width: integer('width').notNull(), // 图片宽度
	height: integer('height').notNull(), // 图片高度
	createdAt: timestamp('created_at', { mode: 'date' }).notNull().defaultNow()
});
