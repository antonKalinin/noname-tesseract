# noname

✨ **noname** is a CLI tool to remove personally identifiable information from screenshots ✨

![image](https://user-images.githubusercontent.com/3993255/151440536-da12dc0d-635b-4b65-9347-8d161fd2e015.png)

### This is MVP 🧨

## Installation

#### MacOS

```
brew tap antonKalinin/noname
brew install noname
```

## ToDos

- [ ] Resolve platform specific scale factor
- [ ] Providing help via --help or -h flag
- [ ] Scrolling for full screen screenshots
- [x] Packaging and installation instructions
- [ ] Entity name recognition with BERT via --suggest or -s flag

## Deployment

To create production release:

1. Run `git checkout master && git pull`.
1. Run `cargo release --execute patch`. [create-release.yaml](.github/workflows/create-release.yaml) will create a new GitHub release draft with information about new PRs in the description.
1. Go to https://github.com/antonKalinin/noname/releases and publish the newly created draft release.
