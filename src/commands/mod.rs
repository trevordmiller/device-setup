mod documents;
mod executables;
mod exercises;
mod feeds;
mod git;
mod homebrew;
mod notes;
mod paths;
mod processes;
mod website;

pub fn setup() {
    homebrew::install_package("vim");
    paths::create_dir(&paths::vim_plugins());
    git::clone(
        &paths::vim_plugins(),
        "https://github.com/octref/RootIgnore",
    );
    homebrew::install_package("node");
}

pub fn upgrade() {
    executables::update("brew");
    homebrew::upgrade_package("vim");
    git::pull_all(&paths::vim_plugins());
    homebrew::upgrade_package("git");
    homebrew::upgrade_package("rustup-init");
    executables::update("rustup");
    homebrew::upgrade_package("node");
}

pub fn end() {
    git::check_all(&paths::repos());
    processes::stop("vim");
    processes::stop("rls");
    processes::stop("node");
    homebrew::clean_artifacts();
}

pub fn study() {
    feeds::next();
    exercises::next();
    documents::next();
    notes::next();
}

pub fn release() {
    website::clean();
    website::build();
    website::configure();
    website::deploy();
}
