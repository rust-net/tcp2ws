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
```

now, you can use `ssh root@client -p 2222` which should be the same as `ssh root@server`


# development

```
RUST_LOG=debug cargo run -- server
```
