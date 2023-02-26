CREATE TABLE `teams`(
    `id` BIGINT UNSIGNED NOT NULL ,
    `rank` INT UNSIGNED NOT NULL,
    `score` BIGINT UNSIGNED NOT NULL,
    `stage` INT UNSIGNED NOT NULL,
    `name` VARCHAR(255) NOT NULL,
    `logo_url` VARCHAR(255) NOT NULL,
    `banner_url` VARCHAR(255) NULL,
    `descriptioin` TEXT NOT NULL,
    `creation_date` DATE NOT NULL,
    `location` VARCHAR(255) NOT NULL
);
ALTER TABLE
    `teams` ADD PRIMARY KEY `teams_id_primary`(`id`);
CREATE TABLE `persons`(
    `id` BIGINT UNSIGNED NOT NULL ,
    `name` VARCHAR(255) NOT NULL,
    `career` VARCHAR(255) NOT NULL,
    `graduation_date` DATE NOT NULL,
    `picture_url` VARCHAR(255) NOT NULL,
    `portafolio_url` VARCHAR(255) NULL
);
ALTER TABLE
    `persons` ADD UNIQUE `persons_id_unique`(`id`);
CREATE TABLE `badges`(
    `id` BIGINT UNSIGNED NOT NULL ,
    `name` VARCHAR(255) NOT NULL,
    `description` TEXT NOT NULL,
    `points` BIGINT NOT NULL,
    `category` BIGINT NOT NULL
);
ALTER TABLE
    `badges` ADD PRIMARY KEY `badges_id_primary`(`id`);
ALTER TABLE
    `badges` ADD UNIQUE `badges_category_unique`(`category`);
CREATE TABLE `badge_ownerhips`(
    `team_id` BIGINT NOT NULL,
    `badge_id` BIGINT NOT NULL
);
ALTER TABLE
    `badge_ownerhips` ADD UNIQUE `badge_ownerhips_team_id_unique`(`team_id`);
ALTER TABLE
    `badge_ownerhips` ADD UNIQUE `badge_ownerhips_badge_id_unique`(`badge_id`);
CREATE TABLE `labels`(
    `id` BIGINT UNSIGNED NOT NULL ,
    `name` VARCHAR(255) NOT NULL,
    `description` TEXT NOT NULL
);
ALTER TABLE
    `labels` ADD PRIMARY KEY `labels_id_primary`(`id`);
CREATE TABLE `label_ownership`(
    `team_id` BIGINT NOT NULL,
    `label_id` BIGINT NOT NULL
);
ALTER TABLE
    `label_ownership` ADD UNIQUE `label_ownership_team_id_unique`(`team_id`);
ALTER TABLE
    `label_ownership` ADD UNIQUE `label_ownership_label_id_unique`(`label_id`);
CREATE TABLE `badge_categories`(
    `id` BIGINT UNSIGNED NOT NULL ,
    `name` VARCHAR(255) NOT NULL,
    `text` TEXT NOT NULL
);
ALTER TABLE
    `badge_categories` ADD PRIMARY KEY `badge_categories_id_primary`(`id`);
ALTER TABLE
    `badges` ADD CONSTRAINT `badges_category_foreign` FOREIGN KEY(`category`) REFERENCES `badge_categories`(`id`);
ALTER TABLE
    `persons` ADD CONSTRAINT `persons_id_foreign` FOREIGN KEY(`id`) REFERENCES `teams`(`id`);
ALTER TABLE
    `label_ownership` ADD CONSTRAINT `label_ownership_team_id_foreign` FOREIGN KEY(`team_id`) REFERENCES `teams`(`id`);
ALTER TABLE
    `label_ownership` ADD CONSTRAINT `label_ownership_label_id_foreign` FOREIGN KEY(`label_id`) REFERENCES `labels`(`id`);
ALTER TABLE
    `badge_ownerhips` ADD CONSTRAINT `badge_ownerhips_team_id_foreign` FOREIGN KEY(`team_id`) REFERENCES `teams`(`id`);
ALTER TABLE
    `badge_ownerhips` ADD CONSTRAINT `badge_ownerhips_badge_id_foreign` FOREIGN KEY(`badge_id`) REFERENCES `badges`(`id`);