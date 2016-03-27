# funzzy
The configurable watcher.
Run configured commands for differents directories using semantic yaml.

Example:
```yaml
# watch.yaml
# list here all the events and the commands that it should execute

- name: run my tests
  when:
    change: '**myproject/tests/**'
    run: make test

- name: compile my sass
  when:
    change: '**myproject/src/static/**'
    run: compass

```

## Installing
Make sure you have installed the follow dependecies:
- Rust
```bash
make build
```

## Running
```bash
funzzy watch
```

# Licence
This project was made under MIT Licence.
