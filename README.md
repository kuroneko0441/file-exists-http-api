# File Exists HTTP API

HTTP API that responds 200 if the file exists

```sh
curl --head http://localhost:3000/etc/hosts # 200 OK
curl --head http://localhost:3000/etc/host  # 404 Not Found
curl http://localhost:3000/etc/hosts        # 405 Method Not Allowed
```
