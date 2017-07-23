# create-js-project

Used to start a new project

## Usage

```
create-js-project {project_type} {name} {description}
```

- `{project_type}` can be `web-app`, `library`, or `service`
- `{name}` should be a kebab-cased name like `hello-world`
- `{description}` should be a short description in quotes like `"A web app to say Hello World"`

## Examples

```
create-js-project library example-library "An example library scaffold using create-js-project"
```

- Repo: [github.com/trevordmiller/example-library](https://github.com/trevordmiller/example-library)
- Release: [npmjs.com/package/example-library](https://www.npmjs.com/package/example-library)

```
create-js-project web-app example-web-app "An example web app scaffold using create-js-project"
```

- Repo: [github.com/trevordmiller/example-web-app](https://github.com/trevordmiller/example-web-app)
- Release: [example-web-app.now.sh/](https://example-web-app.now.sh/)

```
create-js-project service example-service "An example service scaffold using create-js-project"
```

- Repo: [github.com/trevordmiller/example-service](https://github.com/trevordmiller/example-service)
- Release: [example-service.now.sh/](https://example-service.now.sh/)
