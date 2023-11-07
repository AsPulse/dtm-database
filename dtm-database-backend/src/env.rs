pub const PORT_ENV: &str = "PORT";
pub const MODE_ENV: &str = "ENV";
const DEBUG: &str = "DEBUG";
const PRODUCTION: &str = "PRODUCTION";
pub const DEFAULT_PORT: u16 = 3001;
pub const DEFAULT_MODE: BootingModes = BootingModes::Debug;

#[derive(Debug, PartialEq)]
pub enum BootingModes {
  Debug,
  Production,
}

impl std::fmt::Display for BootingModes {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      BootingModes::Debug => write!(f, "{}", DEBUG),
      BootingModes::Production => write!(f, "{}", PRODUCTION),
    }
  }
}

impl std::str::FromStr for BootingModes {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      DEBUG => Ok(BootingModes::Debug),
      PRODUCTION => Ok(BootingModes::Production),
      _ => Err("Unknown mode found"),
    }
  }
}

/// A booting mode getter for this server which is hyper.
/// this function is not pure, and write out message to stdout.
pub fn get_booting_mode(mode_env: &str) -> BootingModes {
  match std::env::var(mode_env) {
    Ok(value) => value.parse::<BootingModes>().unwrap(),
    Err(_) => DEFAULT_MODE,
  }
}

/// A port getter for this server which is hyper.
/// this function is not pure, and write out message to stdout.
pub fn get_port(port_env: &str) -> u16 {
  match std::env::var(port_env) {
    Ok(value) => value
      .parse::<u16>()
      .expect("PORT environment variable should be u16, between 0 and 65535"),
    Err(_) => DEFAULT_PORT,
  }
}

#[cfg(test)]
mod test {
  use crate::env::{
    get_booting_mode, get_port, BootingModes, DEFAULT_MODE, DEFAULT_PORT, MODE_ENV, PORT_ENV,
  };

  /// get_port function's test. this test check whether get_port function's return value is equivalent DEFAULT_PORT or not.
  #[test]
  fn test_get_default_port() {
    let port: u16 = get_port(PORT_ENV);
    assert_eq!(port, DEFAULT_PORT);
  }

  #[test]
  fn test_get_default_mode() {
    let booting_mode: BootingModes = get_booting_mode(MODE_ENV);
    assert_eq!(booting_mode, DEFAULT_MODE);
  }
}
