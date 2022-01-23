## 安装

# MacOs

``` bash
cd /tmp 
curl -s https://api.github.com/repos/cloudGrin/zg-git/releases/latest | grep browser_download_url | cut -d '"' -f 4 | grep 'apple-darwin.zip' | wget -i -
unzip zg-git*apple-darwin.zip
sudo mv ./zg-git /usr/local/bin/zgit
```


