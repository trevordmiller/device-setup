mod documents;
mod exercises;
mod feeds;
mod git;
mod homebrew;
mod notes;
mod paths;
mod processes;
mod public;
mod rust;

pub fn setup() {
    homebrew::install_package("git");
    paths::create_dir(&paths::repos());
    git::configure("user.name", "Trevor D. Miller");
    git::configure(
        "user.email",
        "5497885+trevordmiller@users.noreply.github.com",
    );
    git::clone(
        &paths::repos(),
        "https://github.com/trevordmiller/trevordmiller",
    );

    homebrew::install_package("vim");
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

    homebrew::install_package("rustup-init");

    homebrew::install_package("node");
}

pub fn clean() {
    git::check_all(&paths::repos());
    homebrew::upgrade_package("git");

    processes::stop("vim");
    homebrew::upgrade_package("vim");
    git::pull_all(&paths::vim_plugins());

    processes::stop("rls");
    homebrew::upgrade_package("rustup-init");
    rust::upgrade_self();
    rust::upgrade_executable("trevordmiller");

    processes::stop("node");
    homebrew::upgrade_package("node");

    homebrew::upgrade_self();
    homebrew::clean_artifacts();
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
