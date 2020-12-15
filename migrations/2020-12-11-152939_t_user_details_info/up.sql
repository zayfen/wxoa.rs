-- Your SQL goes here
CREATE TABLE IF NOT EXISTS `t_user_details_info` (
  `f_mobile` varchar(20) NOT NULL,
  `f_name` varchar(40) DEFAULT NULL,
  `f_annual_leave_days` float(6,1) NOT NULL DEFAULT '0',
  `f_rest_days` float(6,1) NOT NULL DEFAULT '0',
  `f_datetime` varchar(30) NOT NULL,
  `f_remark` varchar(200) DEFAULT '',
  `create_at` timestamp default current_timestamp,
  `update_at` timestamp default current_timestamp,
  PRIMARY KEY (`f_mobile`,`f_datetime`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;
