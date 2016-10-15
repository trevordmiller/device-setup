# projects

## Init

```
script-project {project_type} {name} "{description}"
```

- `{project_type}` can be `app` or `package`
- `app` is currently a React app setup (with `create-react-app` as the base)
- `package` is currently an npm package

### Example

`script-project package cool-thing "A cool thing"`

---

## Releases

```
script-release {release_type}
```

- `{release_type}` can be any [npm version semver type](https://docs.npmjs.com/cli/version)

### Example

`script-release minor`
