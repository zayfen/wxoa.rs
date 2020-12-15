
-- CREATE DATABASE `bottle_zoa` /*!40100 DEFAULT CHARACTER SET utf8 */;
-- bottle_zoa.t_user_info definition

CREATE TABLE `t_user_info` (
  `f_open_id` varchar(30) NOT NULL,
  `f_mobile` varchar(20) NOT NULL,
  `f_state` int(8) NOT NULL COMMENT '状态 0 未开始注册， 1： 注册流程中,等待手机号，2： 注册流程中，等待验证码， 3： 注册成功',
  `f_remark` varchar(200) DEFAULT NULL,
  `create_at` timestamp default current_timestamp,
  `update_at` timestamp default current_timestamp on update,
  PRIMARY KEY (`f_open_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

-- bottle_zoa.t_user_details_info definition

CREATE TABLE IF NOT EXISTS `t_user_details_info` (
  `f_mobile` varchar(20) NOT NULL,
  `f_name` varchar(40) DEFAULT NULL,
  `f_annual_leave_days` float(6,1) NOT NULL DEFAULT '0',
  `f_rest_days` float(6,1) DEFAULT NOT NULL '0',
  `f_datetime` varchar(30) NOT NULL,
  `f_remark` varchar(200) DEFAULT NULL,
  `create_at` timestamp default current_timestamp,
  `update_at` timestamp default current_timestamp on update,
  PRIMARY KEY (`f_mobile`,`f_datetime`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;
