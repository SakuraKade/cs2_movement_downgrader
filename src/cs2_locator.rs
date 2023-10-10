use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use walkdir::WalkDir;

#[cfg(target_os = "windows")]
use bitness::{Bitness, BitnessError};
#[cfg(target_os = "windows")]
use winreg::enums::HKEY_LOCAL_MACHINE;
#[cfg(target_os = "windows")]
use winreg::RegKey;

pub(crate) struct Cs2Locator;

impl<'s> Cs2Locator {
    #[cfg(target_os = "windows")]
    pub fn locate() -> Result<String, LocateError> {
        let key = RegKey::predef(HKEY_LOCAL_MACHINE);
        let sub_key = key.open_subkey(Self::get_registry_path()?)?;
        Ok(sub_key.get_value("installpath")?)
    }

    #[cfg(target_os = "linux")]
    pub fn locate() -> Result<String, LocateError> {
        println!("Searching for cs2, This will take a while . . .");

        match Self::search_filesystem() {
            None => {
                Err(LocateError::Cs2NotFound)
            }
            Some(cs2_dir) => {
                Ok(cs2_dir)
            }
        }
    }

    #[cfg(target_os = "linux")]
    fn search_filesystem() -> Option<String> {
        // Walk the whole filesystem to find cs2
        // Dumb approach but should work
        let mut cs2_directory: Option<String> = None;
        for entry in WalkDir::new("/") {
            if entry.is_err() {
                continue
            }

            let entry = entry.ok();
            if entry.is_none() {
                continue
            }

            let entry = entry.unwrap();

            if entry.path().is_dir() {
                continue
            }

            let file_name = entry.file_name().to_string_lossy().to_string();

            match entry.path().parent() {
                None => continue, // Why would it be installed at root?
                Some(parent) => {

                    // Is it valid?
                    if !Self::is_target_valid(parent.to_string_lossy().to_string(), &file_name) {
                        continue;
                    }

                    // This must be it
                    cs2_directory = Some(parent.to_string_lossy().to_string());
                },
            }
        }

        cs2_directory
    }

    #[cfg(target_os = "linux")]
    fn is_target_valid(directory: String, file_name: &String) -> bool {
        const TARGET_FILE_NAME: &str = "cs2.sh";
        const TARGET_DIRECTORY_NAME: &str = "game";

        // Wrong file
        if file_name != TARGET_FILE_NAME {
            return false;
        }

        let dir_name = directory.split("/").last();
        if dir_name.is_none() {
            return false;
        }

        let parent_dir_name = dir_name.unwrap();

        // Not the dir we are looking for
        if parent_dir_name != TARGET_DIRECTORY_NAME {
            return false;
        }

        true
    }

    pub fn get_autoexec_path(game_path: String) -> String {
        Cs2Locator::get_auto_exec_path(game_path)
    }

    #[cfg(target_os = "windows")]
    fn get_auto_exec_path(game_path: String) -> String {
        format!("{}\\game\\csgo\\cfg\\autoexec.cfg", game_path)
    }

    #[cfg(target_os = "linux")]
    fn get_auto_exec_path(game_path: String) -> String {
        format!("{}/csgo/cfg/autoexec.cfg", game_path)
    }

    #[cfg(target_os = "windows")]
    fn get_registry_path() -> Result<&'s str, GetRegistryPathError> {
        match bitness::os_bitness()? {
            Bitness::X86_32 => Ok("SOFTWARE\\Valve\\cs2"),
            Bitness::X86_64 => Ok("SOFTWARE\\WOW6432Node\\Valve\\cs2"),
            Bitness::Unknown => Err(GetRegistryPathError::Unknown),
        }
    }
}

//
// LocateError
//

pub(crate) enum LocateError {
    #[cfg(target_os = "windows")]
    GetRegistryPathError(GetRegistryPathError),
    IoError(std::io::Error),
    Cs2NotFound
}

impl LocateError {
    fn message(&self) -> String {
        match self {
            #[cfg(target_os = "windows")]
            LocateError::GetRegistryPathError(err) => err.to_string(),
            LocateError::IoError(err) => err.to_string(),
            LocateError::Cs2NotFound => String::from("CS2NotFound"),
        }
    }
}

#[cfg(target_os = "windows")]
impl From<GetRegistryPathError> for LocateError {
    fn from(value: GetRegistryPathError) -> Self {
        Self::GetRegistryPathError(value)
    }
}

impl From<std::io::Error> for LocateError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl Debug for LocateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Display for LocateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Error for LocateError {

}

//
// GetRegistryPathError
//
#[cfg(target_os = "windows")]
pub(crate) enum GetRegistryPathError {
    Unknown,
    BitnessError(BitnessError)
}

#[cfg(target_os = "windows")]
impl From<BitnessError> for GetRegistryPathError {
    fn from(value: BitnessError) -> Self {
        Self::BitnessError(value)
    }
}

#[cfg(target_os = "windows")]
impl GetRegistryPathError {
    fn message(&self) -> String {
        match self {
            GetRegistryPathError::Unknown => "Unknown OSArchitecture".to_string(),
            GetRegistryPathError::BitnessError(err) => err.to_string()
        }
    }
}

#[cfg(target_os = "windows")]
impl Debug for GetRegistryPathError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

#[cfg(target_os = "windows")]
impl Display for GetRegistryPathError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

#[cfg(target_os = "windows")]
impl Error for GetRegistryPathError {

}
