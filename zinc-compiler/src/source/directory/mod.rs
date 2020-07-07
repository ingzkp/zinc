//!
//! The source code directory.
//!

pub mod error;
pub mod string;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

use crate::error::Error as CompilerError;
use crate::generator::bytecode::Bytecode;
use crate::generator::program::Program;
use crate::semantic::analyzer::entry::Analyzer as EntryAnalyzer;
use crate::source::error::Error as SourceError;
use crate::source::file::File;
use crate::source::Source;
use crate::source::SourceString;

use self::error::Error;
use self::string::String as DirectoryString;

///
/// The Zinc source code directory, which consists of its path, root module (usually `mod.zn`),
/// and dependency modules.
///
#[derive(Debug, Clone)]
pub struct Directory {
    pub path: PathBuf,
    pub name: String,
    pub entry: File,
    pub dependencies: HashMap<String, Source>,
}

impl Directory {
    ///
    /// Initializes an application directory from string data.
    ///
    pub fn try_from_string(
        directory: DirectoryString,
        is_entry: bool,
    ) -> Result<Self, SourceError> {
        let path = PathBuf::from(directory.path);

        let name = path
            .file_stem()
            .ok_or(Error::StemNotFound)
            .map_err(SourceError::Directory)?
            .to_string_lossy()
            .to_string();

        let mut entry = None;
        let mut dependencies = HashMap::new();

        for (name, module) in directory.modules.into_iter() {
            match module {
                SourceString::File(file) => {
                    if is_entry && file.is_module_entry() {
                        return Err(SourceError::Directory(Error::ModuleEntryInRoot));
                    }

                    if !is_entry && file.is_application_entry() {
                        return Err(SourceError::Directory(Error::ApplicationEntryBeyondRoot));
                    }

                    let file = File::try_from_string(file)?;

                    if file.is_entry() {
                        entry = Some(file);
                    } else {
                        dependencies.insert(name, Source::File(file));
                    }
                }
                SourceString::Directory(directory) => {
                    let directory = Directory::try_from_string(directory, false)?;

                    dependencies.insert(name, Source::Directory(directory));
                }
            }
        }

        match entry {
            Some(entry) => Ok(Self {
                path,
                name,
                entry,
                dependencies,
            }),
            None if is_entry => Err(SourceError::Directory(Error::ApplicationEntryNotFound)),
            None => Err(SourceError::Directory(Error::ModuleEntryNotFound)),
        }
    }

    ///
    /// Initializes an application module from a hard disk directory.
    ///
    pub fn try_from_path(path: &PathBuf, is_entry: bool) -> Result<Self, SourceError> {
        let directory = fs::read_dir(path)
            .map_err(Error::Reading)
            .map_err(SourceError::Directory)?;

        let name = path
            .file_stem()
            .ok_or(Error::StemNotFound)
            .map_err(SourceError::Directory)?
            .to_string_lossy()
            .to_string();

        let mut entry = None;
        let mut modules = HashMap::new();

        for directory_entry in directory.into_iter() {
            let directory_entry = directory_entry
                .map_err(Error::DirectoryEntry)
                .map_err(SourceError::Directory)?;
            let path = directory_entry.path();
            let module = Source::try_from_path(&path)?;
            let name = module.name().to_owned();

            match module {
                Source::File(file) => {
                    if is_entry && file.is_module_entry() {
                        return Err(SourceError::Directory(Error::ModuleEntryInRoot));
                    }

                    if !is_entry && file.is_application_entry() {
                        return Err(SourceError::Directory(Error::ApplicationEntryBeyondRoot));
                    }

                    if file.is_entry() {
                        entry = Some(file);
                    } else {
                        modules.insert(name, Source::File(file));
                    }
                }
                Source::Directory(directory) => {
                    modules.insert(name, Source::Directory(directory));
                }
            }
        }

        match entry {
            Some(entry) => Ok(Self {
                path: path.to_owned(),
                name,
                entry,
                dependencies: modules,
            }),
            None if is_entry => Err(SourceError::Directory(Error::ApplicationEntryNotFound)),
            None => Err(SourceError::Directory(Error::ModuleEntryNotFound)),
        }
    }

    ///
    /// Gets all the intermediate represenation scattered around the application scope tree and
    /// writes it to the bytecode.
    ///
    pub fn compile(self, name: String) -> Result<Rc<RefCell<Bytecode>>, SourceError> {
        let scope = EntryAnalyzer::define(Source::Directory(self))
            .map_err(CompilerError::Semantic)
            .map_err(|error| error.format())
            .map_err(SourceError::Compiling)?;

        let bytecode = Bytecode::new(name).wrap();
        Program::new(scope.borrow().get_intermediate()).write_all_to_bytecode(bytecode.clone());

        Ok(bytecode)
    }

    ///
    /// Initialized a test module directory.
    ///
    pub fn test(
        code: &str,
        path: PathBuf,
        file_index: usize,
        dependencies: HashMap<String, Source>,
    ) -> Result<Self, CompilerError> {
        Ok(Self {
            path: path.clone(),
            name: "test".to_owned(),
            entry: File::test(code, path, file_index)?,
            dependencies,
        })
    }
}
