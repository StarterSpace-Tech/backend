USE defaultdb;
CREATE TABLE `badges`(
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
    `name` VARCHAR(255) NOT NULL UNIQUE,
    `description` TEXT NOT NULL,
    `points` BIGINT NOT NULL,
    `category` BIGINT UNSIGNED NOT NULL
);
-- ALTER TABLE
--     `badges` ADD PRIMARY KEY `badges_id_primary`(`id`);
-- ALTER TABLE
--     `badges` ADD UNIQUE `badges_name_unique`(`name`);
CREATE TABLE `badge_categories`(
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
    `name` VARCHAR(255) NOT NULL UNIQUE
);
-- ALTER TABLE
--     `badge_categories` ADD PRIMARY KEY `badge_categories_id_primary`(`id`);
-- ALTER TABLE
--     `badge_categories` ADD UNIQUE `badge_categories_name_unique`(`name`);
CREATE TABLE `labels`(
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
    `name` VARCHAR(255) NOT NULL UNIQUE
);
-- ALTER TABLE
--     `labels` ADD PRIMARY KEY `labels_id_primary`(`id`);
-- ALTER TABLE
--     `labels` ADD UNIQUE `labels_name_unique`(`name`);
CREATE TABLE `teams`(
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
    `rank` INT UNSIGNED NULL UNIQUE,
    `score` BIGINT UNSIGNED NOT NULL,
    `stage` INT UNSIGNED NOT NULL,
    `name` VARCHAR(255) NOT NULL UNIQUE,
    `logo_url` VARCHAR(255) NULL,
    `banner_url` VARCHAR(255) NULL,
    `description` TEXT NOT NULL,
    `creation_date` DATE NOT NULL,
    `location` VARCHAR(255) NOT NULL
);
-- ALTER TABLE
--     `teams` ADD PRIMARY KEY `teams_id_primary`(`id`);
-- ALTER TABLE
--     `teams` ADD UNIQUE `teams_rank_unique`(`rank`);
-- ALTER TABLE
--     `teams` ADD UNIQUE `teams_name_unique`(`name`);
CREATE TABLE `persons`(
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
    `team` BIGINT UNSIGNED NOT NULL,
    `name` VARCHAR(255) NOT NULL,
    `career` VARCHAR(255) NOT NULL,
    `graduation_date` DATE NOT NULL,
    `picture_url` VARCHAR(255) NULL,
    `portafolio_url` VARCHAR(255) NULL
);
-- ALTER TABLE
--     `persons` ADD PRIMARY KEY `persons_id_primary`(`id`);
CREATE TABLE `label_ownerships`(
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
    `team_id` BIGINT UNSIGNED NOT NULL,
    `label_id` BIGINT UNSIGNED NOT NULL
);
CREATE UNIQUE INDEX `index` ON label_ownerships (`team_id`, `label_id`);
CREATE TABLE `badge_ownerships`(
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY ,
    `team_id` BIGINT UNSIGNED NOT NULL,
    `badge_id` BIGINT UNSIGNED NOT NULL,
    `acquisition_date` DATE NOT NULL
);
CREATE UNIQUE INDEX `index` ON badge_ownerships (`team_id`, `badge_id`);
ALTER TABLE
    `label_ownerships` ADD CONSTRAINT `label_ownerships_team_id_foreign` FOREIGN KEY(`team_id`) REFERENCES `teams`(`id`);
ALTER TABLE
    `badge_ownerships` ADD CONSTRAINT `badge_ownerships_team_id_foreign` FOREIGN KEY(`team_id`) REFERENCES `teams`(`id`);
ALTER TABLE
    `persons` ADD CONSTRAINT `persons_team_foreign` FOREIGN KEY(`team`) REFERENCES `teams`(`id`);
ALTER TABLE
    `badge_ownerships` ADD CONSTRAINT `badge_ownerships_badge_id_foreign` FOREIGN KEY(`badge_id`) REFERENCES `badges`(`id`);
ALTER TABLE
    `label_ownerships` ADD CONSTRAINT `label_ownerships_label_id_foreign` FOREIGN KEY(`label_id`) REFERENCES `labels`(`id`);
ALTER TABLE
    `badges` ADD CONSTRAINT `badges_category_foreign` FOREIGN KEY(`category`) REFERENCES `badge_categories`(`id`);