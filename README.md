## warp-app

A web server built on hyper/warp

### Deployment

Deploy from local machine:

```bash
fly apps create --machines --name warp-app -o personal
fly ips allocate-v4 --shared -a warp-app
fly ips allocate-v6 -a warp-app
fly m run . -p 443:8080/tcp:tls:http -p 80:8080/tcp:http -r waw -a warp-app
```

### Remote Access

Connect to the remote VM:

```bash
flyctl ssh console -a warp-app
```
