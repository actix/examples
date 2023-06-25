CREATE TABLE `bank_details` (
  `id` int NOT NULL AUTO_INCREMENT,
  `bank_name` varchar(30) DEFAULT '',
  `country` varchar(30) DEFAULT '',
  `date_added` datetime DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4;
