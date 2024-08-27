-- phpMyAdmin SQL Dump
-- version 5.2.1
-- https://www.phpmyadmin.net/
--
-- 主机： db
-- 生成日期： 2024-08-27 05:33:44
-- 服务器版本： 8.3.0
-- PHP 版本： 8.2.15

SET SQL_MODE = "NO_AUTO_VALUE_ON_ZERO";
START TRANSACTION;
SET time_zone = "+00:00";


/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;

--
-- 数据库： `pond_roots`
--

-- --------------------------------------------------------

--
-- 表的结构 `collocation`
--

CREATE TABLE `collocation` (
  `id` int NOT NULL,
  `word_id` int DEFAULT NULL COMMENT '对应的单词id',
  `orig_collocation` text CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci COMMENT '词组搭配',
  `interpretation_collocation` text CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '词组搭配释义',
  `create_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `update_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

-- --------------------------------------------------------

--
-- 表的结构 `example`
--

CREATE TABLE `example` (
  `id` int NOT NULL,
  `word_id` int DEFAULT NULL COMMENT '对应的单词id',
  `orig_example` text CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci COMMENT '例句',
  `interpretation_example` text CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '例句翻译',
  `create_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `update_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

-- --------------------------------------------------------

--
-- 表的结构 `pronunciation`
--

CREATE TABLE `pronunciation` (
  `id` int NOT NULL,
  `word_id` int DEFAULT NULL COMMENT '对应的单词id',
  `source_url` text CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci COMMENT '读音对应的资源',
  `region_type` int DEFAULT NULL COMMENT '读音类型（英音0，美音1）',
  `create_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `update_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

-- --------------------------------------------------------

--
-- 表的结构 `roots_affixes`
--

CREATE TABLE `roots_affixes` (
  `id` int NOT NULL,
  `derive_from` int DEFAULT NULL COMMENT '词根词缀的来源',
  `interpretation` text CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci COMMENT '词根词缀的释义',
  `type` int NOT NULL COMMENT '类型（词根0，前缀1，中缀2，后缀3）',
  `create_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `update_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

-- --------------------------------------------------------

--
-- 表的结构 `roots_affixes_form`
--

CREATE TABLE `roots_affixes_form` (
  `id` int NOT NULL,
  `roots_affixes_id` int DEFAULT NULL COMMENT '对应的词根词缀id',
  `form` text CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci COMMENT '词根词缀的形式',
  `create_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `update_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

-- --------------------------------------------------------

--
-- 表的结构 `word`
--

CREATE TABLE `word` (
  `id` int NOT NULL,
  `british_word` varchar(256) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '英式拼写',
  `american_word` varchar(256) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '美式拼写',
  `british_soundmark` varchar(256) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '英式音标',
  `american_soundmark` varchar(256) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL COMMENT '美式音标',
  `comment` text CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci COMMENT '单词注释',
  `notes` text CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci COMMENT '单词笔记',
  `create_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `update_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

-- --------------------------------------------------------

--
-- 表的结构 `word_definition`
--

CREATE TABLE `word_definition` (
  `id` int NOT NULL,
  `word_id` int DEFAULT NULL COMMENT '对应的单词id',
  `en_definition` text CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci COMMENT '英文定义',
  `zh_definition` text CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '中文定义',
  `part_of_speech` int NOT NULL COMMENT '词性',
  `create_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `update_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

-- --------------------------------------------------------

--
-- 表的结构 `word_roots_affixes`
--

CREATE TABLE `word_roots_affixes` (
  `id` int NOT NULL,
  `roots_affixes_id` int DEFAULT NULL COMMENT '词根词缀id',
  `word_id` int DEFAULT NULL COMMENT '单词id',
  `order_no` int DEFAULT NULL COMMENT '排序（用于确定组成单词的词根词缀的先后顺序）',
  `create_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `update_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

--
-- 转储表的索引
--

--
-- 表的索引 `collocation`
--
ALTER TABLE `collocation`
  ADD PRIMARY KEY (`id`),
  ADD KEY `word_id` (`word_id`);

--
-- 表的索引 `example`
--
ALTER TABLE `example`
  ADD PRIMARY KEY (`id`),
  ADD KEY `word_id` (`word_id`);

--
-- 表的索引 `pronunciation`
--
ALTER TABLE `pronunciation`
  ADD PRIMARY KEY (`id`),
  ADD KEY `word_id` (`word_id`);

--
-- 表的索引 `roots_affixes`
--
ALTER TABLE `roots_affixes`
  ADD PRIMARY KEY (`id`);

--
-- 表的索引 `roots_affixes_form`
--
ALTER TABLE `roots_affixes_form`
  ADD PRIMARY KEY (`id`),
  ADD KEY `root_id` (`roots_affixes_id`);

--
-- 表的索引 `word`
--
ALTER TABLE `word`
  ADD PRIMARY KEY (`id`);

--
-- 表的索引 `word_definition`
--
ALTER TABLE `word_definition`
  ADD PRIMARY KEY (`id`),
  ADD KEY `word_id` (`word_id`);

--
-- 表的索引 `word_roots_affixes`
--
ALTER TABLE `word_roots_affixes`
  ADD PRIMARY KEY (`id`),
  ADD KEY `roots_affixes_id` (`roots_affixes_id`),
  ADD KEY `word_id` (`word_id`);

--
-- 在导出的表使用AUTO_INCREMENT
--

--
-- 使用表AUTO_INCREMENT `collocation`
--
ALTER TABLE `collocation`
  MODIFY `id` int NOT NULL AUTO_INCREMENT;

--
-- 使用表AUTO_INCREMENT `example`
--
ALTER TABLE `example`
  MODIFY `id` int NOT NULL AUTO_INCREMENT;

--
-- 使用表AUTO_INCREMENT `pronunciation`
--
ALTER TABLE `pronunciation`
  MODIFY `id` int NOT NULL AUTO_INCREMENT;

--
-- 使用表AUTO_INCREMENT `roots_affixes`
--
ALTER TABLE `roots_affixes`
  MODIFY `id` int NOT NULL AUTO_INCREMENT;

--
-- 使用表AUTO_INCREMENT `roots_affixes_form`
--
ALTER TABLE `roots_affixes_form`
  MODIFY `id` int NOT NULL AUTO_INCREMENT;

--
-- 使用表AUTO_INCREMENT `word`
--
ALTER TABLE `word`
  MODIFY `id` int NOT NULL AUTO_INCREMENT;

--
-- 使用表AUTO_INCREMENT `word_definition`
--
ALTER TABLE `word_definition`
  MODIFY `id` int NOT NULL AUTO_INCREMENT;

--
-- 使用表AUTO_INCREMENT `word_roots_affixes`
--
ALTER TABLE `word_roots_affixes`
  MODIFY `id` int NOT NULL AUTO_INCREMENT;

--
-- 限制导出的表
--

--
-- 限制表 `collocation`
--
ALTER TABLE `collocation`
  ADD CONSTRAINT `collocation_ibfk_1` FOREIGN KEY (`word_id`) REFERENCES `word` (`id`);

--
-- 限制表 `example`
--
ALTER TABLE `example`
  ADD CONSTRAINT `example_ibfk_1` FOREIGN KEY (`word_id`) REFERENCES `word` (`id`);

--
-- 限制表 `pronunciation`
--
ALTER TABLE `pronunciation`
  ADD CONSTRAINT `pronunciation_ibfk_1` FOREIGN KEY (`word_id`) REFERENCES `word` (`id`);

--
-- 限制表 `roots_affixes_form`
--
ALTER TABLE `roots_affixes_form`
  ADD CONSTRAINT `roots_affixes_form_ibfk_1` FOREIGN KEY (`roots_affixes_id`) REFERENCES `roots_affixes` (`id`);

--
-- 限制表 `word_definition`
--
ALTER TABLE `word_definition`
  ADD CONSTRAINT `word_definition_ibfk_1` FOREIGN KEY (`word_id`) REFERENCES `word` (`id`);

--
-- 限制表 `word_roots_affixes`
--
ALTER TABLE `word_roots_affixes`
  ADD CONSTRAINT `word_roots_affixes_ibfk_1` FOREIGN KEY (`roots_affixes_id`) REFERENCES `roots_affixes` (`id`),
  ADD CONSTRAINT `word_roots_affixes_ibfk_2` FOREIGN KEY (`word_id`) REFERENCES `word` (`id`);
COMMIT;

/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
