CREATE TABLE `branch_details` (
  `id` int NOT NULL AUTO_INCREMENT,
  `branch_name` varchar(30) DEFAULT '',
  `location` varchar(30) DEFAULT '',
  `date_added` datetime DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4;
