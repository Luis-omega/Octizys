use crate::common_configuration::CommonConfiguration;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Version {
    major: u16,
    minor: u16,
    patch: u16,
}

#[derive(Debug)]
pub enum ExternalTarget {
    Python(Version),
}

/// They are use to debug the compiler except for the
/// [Typed] Target. It is used to check the code
#[derive(Debug)]
pub enum InternalTarget {
    ///output the core code to file
    CoreOutput(Version),
    /// runs all up to the core transformation but don't produce anything
    Core(Version),
    ///Just check the types in the [Sast] (sugared abstract tree).
    Typed(Version),
    ///run all the way up to the optimizer
    ///then don't produce anything.
    Debug(Version),
}

#[derive(Debug)]
pub enum Target {
    Internal(InternalTarget),
    External(ExternalTarget),
}

#[derive(Debug)]
pub struct LibraryConfiguration {
    //Those can be files or directories
    pub paths_to_compile: Vec<PathBuf>,
    pub output_folder: PathBuf,
}

#[derive(Debug)]
pub struct ExecutableConfiguration {
    pub path_to_file: PathBuf,
    pub file_output_name: Option<String>,
    pub output_folder: PathBuf,
}

#[derive(Debug)]
pub enum CompilationKind {
    Library(LibraryConfiguration),
    Executable(ExecutableConfiguration),
}

#[derive(Debug)]
/// This separation is more meaningful for package managers.
/// A package manager would distinguish if the source is a
/// local package or something that they may need to retrieve.
/// They can ignore this separation and just provide all as
/// [external] or [local].
pub struct DependencyPaths {
    external: Vec<PathBuf>,
    local: Vec<PathBuf>,
}

#[derive(Debug)]
pub struct CompilerConfiguration {
    pub common: CommonConfiguration,
    /// Basically, we want to produced a lib or a executable?
    pub kind: CompilationKind,
    /// This can be empty, it means we do nothing!
    pub targets: Vec<Target>,
    pub dependency_paths: DependencyPaths,
}
