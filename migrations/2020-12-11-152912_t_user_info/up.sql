-- Your SQL goes here
CREATE TABLE IF NOT EXISTS `t_user_info` (
  `f_open_id` varchar(30) NOT NULL,
  `f_mobile` varchar(20) NOT NULL,
  `f_state` int(8) NOT NULL COMMENT '状态 0 未开始注册， 1： 注册流程中,等待手机号，2： 注册流程中，等待验证码， 3： 注册成功',
  `f_remark` varchar(200) DEFAULT NULL,
  `create_at` timestamp default current_timestamp,
  `update_at` timestamp default current_timestamp,
  primary key (`f_open_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;
