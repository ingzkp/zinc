//!
//! The Zargo project manager `proof-check` subcommand.
//!

use std::convert::TryFrom;
use std::path::PathBuf;

use failure::Fail;
use structopt::StructOpt;

use crate::arguments::command::IExecutable;
use crate::directory::build::Directory as BuildDirectory;
use crate::directory::build::Error as BuildDirectoryError;
use crate::directory::data::Directory as DataDirectory;
use crate::directory::data::Error as DataDirectoryError;
use crate::directory::source::Directory as SourceDirectory;
use crate::executable::compiler::Compiler;
use crate::executable::compiler::Error as CompilerError;
use crate::executable::virtual_machine::Error as VirtualMachineError;
use crate::executable::virtual_machine::VirtualMachine;
use crate::manifest::Error as ManifestError;
use crate::manifest::Manifest;

///
/// The Zargo project manager `proof-check` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(
    about = "Runs the full project building, running, trusted setup, proving & verifying sequence"
)]
pub struct Command {
    /// The logging level value, which helps the logger to set the logging level.
    #[structopt(
        short = "v",
        parse(from_occurrences),
        help = "Shows verbose logs, use multiple times for more verbosity"
    )]
    pub verbosity: usize,

    /// The path to the Zargo project manifest file.
    #[structopt(
        long = "manifest-path",
        help = "Path to Zargo.toml",
        default_value = "./Zargo.toml"
    )]
    pub manifest_path: PathBuf,

    /// The path to the binary bytecode file.
    #[structopt(
        long = "binary",
        help = "Path to the bytecode file",
        default_value = "./build/main.znb"
    )]
    pub binary_path: PathBuf,

    /// The path to the witness JSON file.
    #[structopt(
        long = "witness",
        help = "Path to the witness JSON file",
        default_value = "./data/main_witness.json"
    )]
    pub witness_path: PathBuf,

    /// The path to the public data JSON file.
    #[structopt(
        long = "public-data",
        help = "Path to the public data JSON file",
        default_value = "./data/main_public_data.json"
    )]
    pub public_data_path: PathBuf,

    /// The path to the proving key file.
    #[structopt(
        long = "proving-key",
        help = "Path to the proving key file",
        default_value = "./data/proving_key"
    )]
    pub proving_key_path: PathBuf,

    /// The path to the verifying key file.
    #[structopt(
        long = "verifying-key",
        help = "Path to the verifying key file",
        default_value = "./data/verifying_key.txt"
    )]
    pub verifying_key_path: PathBuf,
}

///
/// The Zargo project manager `proof-check` subcommand error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The manifest file error.
    #[fail(display = "manifest file {}", _0)]
    ManifestFile(ManifestError),
    /// The project binary build directory error.
    #[fail(display = "build directory {}", _0)]
    BuildDirectory(BuildDirectoryError),
    /// The project template, keys, and other auxiliary data directory error.
    #[fail(display = "data directory {}", _0)]
    DataDirectory(DataDirectoryError),
    /// The compiler process error.
    #[fail(display = "compiler {}", _0)]
    Compiler(CompilerError),
    /// The virtual machine `run` process error.
    #[fail(display = "virtual machine 'run' {}", _0)]
    VirtualMachineRun(VirtualMachineError),
    /// The virtual machine `setup` process error.
    #[fail(display = "virtual machine 'setup' {}", _0)]
    VirtualMachineSetup(VirtualMachineError),
    /// The virtual machine `proof-check` process error.
    #[fail(display = "virtual machine 'prove & verify' {}", _0)]
    VirtualMachineProveAndVerify(VirtualMachineError),
}

impl IExecutable for Command {
    type Error = Error;

    fn execute(self) -> Result<(), Self::Error> {
        let _manifest = Manifest::try_from(&self.manifest_path).map_err(Error::ManifestFile)?;

        let mut manifest_path = self.manifest_path.clone();
        if manifest_path.is_file() {
            manifest_path.pop();
        }

        let source_directory_path = SourceDirectory::path(&manifest_path);
        let build_directory_path = BuildDirectory::path(&manifest_path);
        let data_directory_path = DataDirectory::path(&manifest_path);

        BuildDirectory::create(&manifest_path).map_err(Error::BuildDirectory)?;
        DataDirectory::create(&manifest_path).map_err(Error::DataDirectory)?;

        Compiler::build(
            self.verbosity,
            &data_directory_path,
            &build_directory_path,
            &source_directory_path,
            false,
        )
        .map_err(Error::Compiler)?;

        VirtualMachine::run(
            self.verbosity,
            &self.binary_path,
            &self.witness_path,
            &self.public_data_path,
        )
        .map_err(Error::VirtualMachineRun)?;

        VirtualMachine::setup(
            self.verbosity,
            &self.binary_path,
            &self.proving_key_path,
            &self.verifying_key_path,
        )
        .map_err(Error::VirtualMachineSetup)?;

        VirtualMachine::prove_and_verify(
            self.verbosity,
            &self.binary_path,
            &self.witness_path,
            &self.public_data_path,
            &self.proving_key_path,
            &self.verifying_key_path,
        )
        .map_err(Error::VirtualMachineProveAndVerify)?;

        Ok(())
    }
}
