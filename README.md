# noname

✨ **noname** is a CLI tool that makes it easy to remove PII from screenshots without breaking a flow ✨

![image](https://user-images.githubusercontent.com/3993255/151440536-da12dc0d-635b-4b65-9347-8d161fd2e015.png)

### Status

MVP

### ToDos

- [ ] Resolve platform specific scale factor
- [ ] Providing help via --help or -h flag
- [ ] Scrolling for full screen screenshots
- [ ] Packaging and installation instructions
- [ ] Entity name recognition with BERT via --suggest or -s flag

## Deployment

To create production release:

1. Run `git checkout master && git pull`.
1. Run `yarn version`.
1. Run `git push --follow-tags`. [create-release.yaml](.github/workflows/create-release.yaml) will create a new GitHub release draft with information about new PRs in the description.
1. Go to https://github.com/creditornot/liveops-events/releases and publish the newly created draft release.
