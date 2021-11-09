# Paste2VRChat CLI

## How to use

### RegisterUrlScheme

```shell
p2vrc_cli.exe --register
```

```js
window.open(`p2vrc://paste?text=${encodeURIComponent(location.href)}`);
```

### Paste text into VRChat

```shell
p2vrc_cli.exe https://www.youtube.com/watch?v=lFNWUL9fqG8
```

Or

```shell
p2vrc_cli.exe --open-url "p2vrc://paste?text=https%3A%2F%2Fwww.youtube.com%2Fwatch%3Fv%3DlFNWUL9fqG8"
```

## How to build

1. [Install Nim](https://nim-lang.org/install.html)
2. Build

```shell
git clone https://github.com/project-vrcat/paste2vrchat
cd paste2vrchat
nimble install --depsOnly -y
nim c -d:release -d:strip --opt: size --outdir: bin cli/*.nim
```

3. Check your bin directory
