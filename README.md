这是一个用Rust中的warp框架中使用diesel的示例

# diesel是Rust的ORM(对象关系映射器)和查询构建器
# diesel为PostgreSQL、Mysql及SQLite提供了开箱即用的支持

diesel数据库迁移使用说明
diesel是Rust的ORM(对象关系映射器)和查询构建器
diesel为PostgreSQL、Mysql及SQLite提供了开箱即用的支持
diesel-cli命令行工具（创建、迁移）：
安装diesel-cli工具：cargo install diesel_cli --no-default-features --features postgres

在cargo项目根目录下添加.env文件,加下如下条进行postgres数据库连接配置：
postgres数据库：
DATABASE_URL=postgres://postgres:llxxs@127.0.0.1:5432/linksnap
mysql数据库：
DATABASE_URL=mysql://[user[:password]@]host/database_name

在Cargo.toml中添加依赖项：
diesel = { version="1.4.6",features=["extras","postgres","r2d2"] }
dotenv = "0.15.0"

运行"diesel setup"命令生成"migrations"目录与"diesel.toml"文件并且会创建数据库：
elapse@elapse-PC:/luck/Language/Rust/warp-wiki$ diesel setup
Creating migrations directory at: /luck/Language/Rust/warp-wiki/migrations
Creating database: warpwiki
elapse@elapse-PC:/luck/Language/Rust/warp-wiki$

创建admins表迁移，运行创建表迁移命令（diesel migration generate 表名）：
elapse@elapse-PC:/luck/Language/Rust/warp-wiki$ diesel migration generate admins
Creating migrations/2021-05-13-071702_admins/up.sql
Creating migrations/2021-05-13-071702_admins/down.sql 
elapse@elapse-PC:/luck/Language/Rust/warp-wiki$ 
命令运行后会生成两个空的迁移文件up.sql和down.sql,
迁移文件只是普通的SQL,接着在up.sql上面添加CREATE TABLE,同时在down.sql添加相应的DROP TABLE

执行表迁移命令（diesel migration run）：
elapse@elapse-PC:/luck/Language/Rust/warp-wiki$ diesel migration run
Running migration 2021-05-13-071702_admins
elapse@elapse-PC:/luck/Language/Rust/warp-wiki$
命令执行完后，会在数据库中生成表，同时在项目中生成src/schema.rs文件。


迁移时执行：diesel setup


                      170 0505 0718
17005058750	 福建 福州  170 0505 0817  170 0505 0813
                       130 4808 0609  17005050813 17005050813



