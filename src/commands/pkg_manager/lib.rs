#[allow(dead_code)]
pub enum Manager {
    Paru,
    Yay,
    Pacman,
    Emerge,
    Zypper,
    Dnf,
    Nix,
    Apt,
    Winget,
    Scoop,
    Brew,
    Pkg, //FreeBSD & DragonFly
    PkgAdd, //OpenBSD
    PkgSrc, //NetBSD
}

impl Manager {
    pub fn req_sudo(&self) -> bool {
        ! matches!(self, Manager::Paru | Manager::Yay)
    }
}
use std::fmt;
impl fmt::Display for Manager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Manager::Paru => "paru",
            Manager::Yay => "yay",
            Manager::Pacman => "pacman",
            Manager::Emerge => "emerge",
            Manager::Zypper => "zypper",
            Manager::Dnf => "dnf",
            Manager::Nix => "nix",
            Manager::Apt => "apt",
            Manager::Winget => "winget",
            Manager::Scoop => "scoop",
            Manager::Brew => "brew",
            Manager::Pkg => "pkg",
            Manager::PkgAdd => "pkg_add",
            Manager::PkgSrc => "pkgsrc",
        };
        write!(f, "{s}")
    }
}
