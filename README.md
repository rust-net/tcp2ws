# tcp2ws
```
client <------ tcp ------> server
to:
client <---tcp2ws---> websocket <---tcp2ws---> server
```

server:
```
tcp2ws server -l 8080 -p 22
```

client:
```
tcp2ws -l 2222 -w ws://server:8080
tcp2ws -l 2222 -w ws://server:8080 -udp server:1234  # UDP forward
```

now, you can use `ssh root@client -p 2222` which should be the same as `ssh root@server`

and we have a Web GUI client in [web-client](https://github.com/develon2015/tcp2ws/tree/web-client) branch.

![image](https://user-images.githubusercontent.com/27133157/233881091-887a7fc9-b21d-43ff-9838-437a4f0c5e46.png)

# development

```
RUST_LOG=debug cargo run -- server
```
