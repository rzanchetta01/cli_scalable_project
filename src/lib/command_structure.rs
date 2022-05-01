use std::fmt;

pub enum DebugCommannds {
    _DebugMode,
    _Exit,
}

pub enum GarbageCleanerCommands {
    _Garbage
}

pub enum InicialComands {
    _Yal,
    _Version,
    _Help,
}

pub enum FileSistemCommands {
    _Read,
    _Search,
    _Dntread,
}

pub enum CalculatorCommands {
    _Math,
}

impl fmt::Display for GarbageCleanerCommands {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GarbageCleanerCommands::_Garbage => write!(f , "-garbage"),
        }
    }
}

impl fmt::Display for InicialComands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InicialComands::_Yal => write!(f, "yal"),
            InicialComands::_Version => write!(f, "-version"),
            InicialComands::_Help => write!(f, "-help"),
        }
    }
}

impl fmt::Display for FileSistemCommands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileSistemCommands::_Read => write!(f, "-read"),
            FileSistemCommands::_Search => write!(f, "-search"),
            FileSistemCommands::_Dntread => write!(f, "-dntread"),
        }
    }
}

impl fmt::Display for CalculatorCommands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalculatorCommands::_Math => write!(f, "-math"),
        }
    }
}

impl fmt::Display for DebugCommannds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DebugCommannds::_DebugMode => write!(f, "-debugMode"),
            DebugCommannds::_Exit => write!(f, "-exit"),
        }
    }
}
