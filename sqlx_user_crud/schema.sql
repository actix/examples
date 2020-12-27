DROP TABLE IF EXISTS `users_to_groups`;
DROP TABLE IF EXISTS `groups`;
DROP TABLE IF EXISTS `users`;

CREATE TABLE IF NOT EXISTS users 
(
	id VARCHAR(48) NOT NULL UNIQUE,
	name VARCHAR(64) NOT NULL UNIQUE,
	email VARCHAR(256) NOT NULL UNIQUE,
	PRIMARY KEY (id)
);
            
CREATE TABLE IF NOT EXISTS `groups`
(
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(64) NOT NULL UNIQUE,
    PRIMARY KEY(id)
);
            
CREATE TABLE IF NOT EXISTS `users_to_groups`
(
    `user_id` VARCHAR(48) NOT NULL,
    `group_id` BIGINT UNSIGNED NOT NULL,
    FOREIGN KEY (`user_id`) REFERENCES `users`(`id`),
    FOREIGN KEY (`group_id`) REFERENCES `groups`(`id`)
);