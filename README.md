# noname

✨ **noname** is a CLI tool to help remove personally identifiable information from screenshots ✨

![image](https://user-images.githubusercontent.com/3993255/151440536-da12dc0d-635b-4b65-9347-8d161fd2e015.png)

**Be aware: noname is only MVP 🧨 please check ToDo section**

## Installation

#### MacOS

```
brew tap antonKalinin/noname
brew install noname
```

## ToDos

- [x] Packaging and installation instructions
- [ ] Dynamically resolve platform specific scale factor
- [ ] Providing help via --help or -h flag
- [ ] Scrolling for full size screenshots

### Deployment

To create production release:

1. Run `git checkout master && git pull`.
2. Run `cargo release --execute patch`. [create-release.yaml](.github/workflows/create-release.yaml) will create a new GitHub release draft with information about changes in the description.
3. Go to https://github.com/antonKalinin/noname/releases and publish the newly created draft release.
