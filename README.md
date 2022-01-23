## 简介

> 为了保持git提交合并的清晰明了，原merge.ff 设置后会对git pull, 造成影响，所以开发了这个命令。

只是对原来的git merge 命令，进行参数补齐，当不传入 --no-ff、--ff、--ff-only 时，默认 zgit merge 使用 --no-ff，即 不使用 fast forward 模式
## 安装

- MacOs

``` bash
cd /tmp 
curl -s https://api.github.com/repos/cloudGrin/zg-git/releases/latest | grep browser_download_url | cut -d '"' -f 4 | grep 'apple-darwin.zip' | wget -i -
unzip zg-git*apple-darwin.zip
sudo mv ./zg-git /usr/local/bin/zgit
```

### 使用

和git使用一致

```bash
zgit log

zgit status

zgit merge master
```