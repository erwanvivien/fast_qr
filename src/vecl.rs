//! Contains all different levels of quality.
//! And allows to find easily max bits per version/quality pair

#![deny(unsafe_code)]
#![warn(missing_docs)]

use crate::version::Version;

#[derive(Copy, Clone)]
#[allow(dead_code)]
/// Error Correction Coding has 4 levels
pub enum ECL {
    /// Low, 7%
    L,
    /// Medium, 15%
    M,
    /// Quartile, 25%
    Q,
    /// High, 30%
    H,
}

impl std::fmt::Display for ECL {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ECL::L => write!(f, "L"),
            ECL::M => write!(f, "M"),
            ECL::Q => write!(f, "Q"),
            ECL::H => write!(f, "H"),
        }
    }
}
