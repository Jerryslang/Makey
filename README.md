# Makey
super simple toml based make system programmed in rust

### Instructions

projects that use makey should have a `makey.conf` file in the project root

you can see an example config below

### Example Config

```toml
[main]
flags=['hello', 'world']
command=['echo']
execute='{command} {flags}'

[test]
flags=['this', 'is', 'a', 'test']
command=['echo']
execute='{command} {flags}'
```

with this example we can run:

`makey` to call `[main]` which will run `echo hello world`

`makey test` to call `[test]` which will run `echo this is a test`
