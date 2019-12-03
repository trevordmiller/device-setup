# scripts

Personal scripts to automate my computer configuration.

## Usage

```sh
cd ~
mkdir repos
cd repos
git clone --recurse-submodules https://github.com/trevordmiller/scripts.git
cd scripts
./scripts/setup
```

## Contributing

### Workflow

Assuming [git](https://git-scm.com) is installed.

- Work off the `master` branch.

### Guidelines

- Use defaults as much as possible so that I can work on other computers without my configuration.
- Encapsulate all configuration so that I can reproduce my setup on other computers.
