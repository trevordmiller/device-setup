mod documents;
mod executables;
mod exercises;
mod feeds;
mod git;
mod homebrew;
mod notes;
mod paths;
mod processes;
mod public;

pub fn setup() {
    git::configure("user.name", "Trevor D. Miller");
    git::configure(
        "user.email",
        "5497885+trevordmiller@users.noreply.github.com",
    );
    homebrew::install_package("vim");
    homebrew::install_package("node");
    paths::create_dir(&paths::vim_plugins());
    git::clone(
        &paths::vim_plugins(),
        "https://github.com/octref/RootIgnore",
    );
    git::clone(&paths::vim_plugins(), "https://github.com/tpope/vim-sleuth");
    git::clone(
        &paths::vim_plugins(),
        "https://github.com/dense-analysis/ale",
    );
}

pub fn clean() {
    git::check_all(&paths::repos());
    processes::stop("vim");
    processes::stop("rls");
    processes::stop("node");
    executables::upgrade("brew");
    executables::upgrade("rustup");
    homebrew::upgrade_package("vim");
    homebrew::upgrade_package("git");
    homebrew::upgrade_package("rustup-init");
    homebrew::upgrade_package("node");
    homebrew::clean_artifacts();
    git::pull_all(&paths::vim_plugins());
}

pub fn study() {
    feeds::next();
    exercises::next();
    documents::next();
    notes::next();
}

pub fn generate() {
    public::clean();
    public::build();
    public::configure();
}
