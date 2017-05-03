# script-project

Used to start a new project

## Usage

```
script-project {project_type} {name} {description}
```

- `{project_type}` can be `library` or `web-app`
- `{name}` should be a kebab-cased name like `hello-world`
- `{description}` should be a short description in quotes like `"A server to say Hello World"`

## Examples

```
script-project library example-library "An example library scaffold using script-project"
```

- Repo: [github.com/trevordmiller/example-library](https://github.com/trevordmiller/example-library)
- npm package: [npmjs.com/package/example-library](https://www.npmjs.com/package/example-library)

```
script-project web-app example-web-app "An example web app scaffold using script-project"
```

- Repo: [github.com/trevordmiller/example-web-app](https://github.com/trevordmiller/example-web-app)
- now deployment: [example-web-app-mhvtitozah.now.sh/](https://example-web-app-mhvtitozah.now.sh/)
