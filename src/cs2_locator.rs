use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use bitness::{Bitness, BitnessError};
use winreg::enums::HKEY_LOCAL_MACHINE;
use winreg::RegKey;

pub(crate) struct Cs2Locator;

impl<'s> Cs2Locator {
    pub fn locate() -> Result<String, LocateError> {
        let hkey = RegKey::predef(HKEY_LOCAL_MACHINE);
        let subkey = hkey.open_subkey(Self::get_registry_path()?)?;
        Ok(subkey.get_value("installpath")?)
    }

    pub fn get_autoexec_path(game_path: String) -> String
    {
        format!("{}\\game\\csgo\\cfg\\autoexec.cfg", game_path)
    }

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
    GetRegistryPathError(GetRegistryPathError),
    IoError(std::io::Error)
}

impl LocateError {
    fn message(&self) -> String {
        match self {
            LocateError::GetRegistryPathError(err) => err.to_string(),
            LocateError::IoError(err) => err.to_string(),
        }
    }
}

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
pub(crate) enum GetRegistryPathError {
    Unknown,
    BitnessError(BitnessError)
}

impl From<BitnessError> for GetRegistryPathError {
    fn from(value: BitnessError) -> Self {
        Self::BitnessError(value)
    }
}

impl GetRegistryPathError {
    fn message(&self) -> String {
        match self {
            GetRegistryPathError::Unknown => "Unknown OSArchitecture".to_string(),
            GetRegistryPathError::BitnessError(err) => err.to_string()
        }
    }
}

impl Debug for GetRegistryPathError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Display for GetRegistryPathError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Error for GetRegistryPathError {

}