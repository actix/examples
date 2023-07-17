CREATE TABLE `teller_details` (
  `id` int NOT NULL AUTO_INCREMENT,
  `teller_name` varchar(100) DEFAULT '',
  `branch_name` varchar(30) DEFAULT '',
  `date_added` datetime DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8mb4;
