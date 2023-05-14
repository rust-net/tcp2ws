# /etc/nginx/nginx.conf

```
http {
    server {
        server_name example.com;
        
        location / {
            proxy_http_version 1.1;
            proxy_set_header Upgrade $http_upgrade;
            proxy_set_header Connection "Upgrade";
            proxy_set_header Host $host:$server_port;

            if ($http_upgrade = "websocket") {
                proxy_pass http://127.0.0.1:8080;
            }
        }
    }
}
```