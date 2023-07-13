create table if not exists seller(
  id integer primary key autoincrement,
  name nvarchar(100) not null,
  stand_number integer not null,
  discount double not null,
  code nvarchar(255) not null,
  deleted boolean not null
);

create table if not exists article(
  id integer primary key autoincrement,
  title nvarchar(100) not null,
  ean nvarchar(255) not null,
  pvp double not null,
  language nvarchar(5) not null,
  stock int not null
);

create table if not exists sale(
  id integer primary key autoincrement,
  seller_id integer not null,
  total double not null,
  manual_discount double null,
  manual_customer nvarchar(255) null,
  foreign key(seller_id) references seller(id)
);

create table if not exists sale_detail(
  id integer primary key autoincrement,
  sale_id integer not null,
  article_id integer not null,
  quantity int not null,
  total_detail double not null,
  foreign key(sale_id) references sale(id),
  foreign key(article_id) references article(id)
);
