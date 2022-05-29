v2up servers
----
list servers and pick outbound servers

v2up start
----
start v2ray core and v2up worker

v2up stop
----
stop v2ray core and v2up worker

v2up mode
----
switch mode global/pac/manual


v2up update --subscribe=
v2up config
v2up logs --tail
v2up upgrade

# subscriptions
v2up subscripitions add url
v2up subscripitions remove 

~/.v2up/config.json 
```yaml
v2ray:
    path:
    version:
log:
    location:
subscriptions:
    - name:
      url:
      last
```

~/.v2up/v2ray-core/ver/
~/.v2up/servers.json
~/.v2up/v2ray.log