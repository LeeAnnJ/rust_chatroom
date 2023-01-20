use chatroom;
-- 用户表：序号=账号，密码，名称
create table users(
	ID int not null primary key auto_increment,
    PW varchar(12) not null,
    uName varchar(12) not null
);

-- 缓存信息表：序号，发送者ID，接收者ID，信息，发送时间，是否接收
create table meslog(
	mesID int not null primary key auto_increment,
	sID int not null,
    rID int not null,
    mes text not null,
    sTime datetime not null,
    isread int default 0
);

-- 用户表插入数据 默认不可改名称，可改密码；名称不重复
insert into users(PW,uName) values('123','momo');
insert into users(PW,uName) values('456','nono');
insert into users(PW,uName) values('888','xixi');
alter table users add UNIQUE(uName);
desc users;
select * from users;

-- 信息表插入信息 产生于信息发送时 rust只需要传入sID,rID,mes 只在私发时调用；群发时，当成即时的，不保留历史信息
insert into meslog(sID,rID,mes,sTime) values(1,2,'Hello~~',NOW());
insert into meslog(sID,rID,mes,sTime) values(2,1,'Hello!!',NOW());
select * from meslog;

-- 信息表查询当前用户如 ID=1 未接收的信息
select * from meslog where rID=1;

-- 信息表删除已接收信息：相应的rID 接收到mesID为1的信息时
delete from meslog where mesID = 1;
select * from meslog;

-- 因为使用的是自增ID， 可存约43亿次的信息。

-- 好友关系表：序号，本方id，好友id 
create table  friends(
	fID int not null primary key auto_increment,
    sID int not null,
    rID int not null
);
select * from friends;

-- 新建好友关系：双向建立 
insert into friends(sID,rID) values(1,2),(2,1);

-- 查询双方是否为好友：避免重复添加等操作 查1和2，虽然记录是双向添加，查一个就行,有记录则是好友，没有就不是。
select * from friends where sID=1 AND rID=2;

-- 查询所有好友关系：只获取所有好友的id，例查询1的好友
select rID from friends where sID=1;

-- 查询所有好友关系：获取好友id和用户名
select rID as ID, uName from friends FS,users US where sID=1 and FS.rID=US.ID;

-- 删除好友关系：双向删除 如1删2
delete from friends where (sID=1 and rID=2) or (sID=2 and rID=1);

