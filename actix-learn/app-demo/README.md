# app demo 
一个前后端分离的项目示例，包括：账户管理、论坛模块、聊天室模块、IM模块。

后端功能：
1. 静态文件部分。
   1. 前端静态文件
      1. 需要加文件服务的错误处理，如果请求文件不存在，则转向默认页面
   2. 用户上传静态文件
2. restful api
   1. 用户注册
   2. 用户登录
   3. 获取话题
3. 数据库
   1. leveldb/rocksdb
   2. mongodb
   3. sqlite
   4. mysql
4. 后台管理
5. 错误处理

# 依赖
## sqllite
### Install SQLite

```sh
# on OpenSUSE
sudo zypper install sqlite3-devel libsqlite3-0 sqlite3

# on Ubuntu
sudo apt-get install libsqlite3-dev sqlite3

# on Fedora
sudo dnf install libsqlite3x-devel sqlite3x

# on macOS (using homebrew)
brew install sqlite3
```

### Initialize SQLite Database

```sh
cd examples/diesel
cargo install diesel_cli --no-default-features --features sqlite

echo "DATABASE_URL=test.db" > .env
diesel migration run
```

There will now be a database file at `./test.db`.
