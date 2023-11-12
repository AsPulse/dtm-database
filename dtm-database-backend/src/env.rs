use once_cell::sync::Lazy;

const PORT_ENV: &str = "PORT";
const MODE_ENV: &str = "ENV";
const DEBUG: &str = "DEBUG";
const PRODUCTION: &str = "PRODUCTION";
pub const DEFAULT_PORT: u16 = 3001;
pub const DEFAULT_MODE: BootingMode = BootingMode::Debug;

pub static ENV: Lazy<BootingMode> = Lazy::new(|| match std::env::var(MODE_ENV) {
  Ok(value) => value.parse::<BootingMode>().unwrap(),
  Err(_) => DEFAULT_MODE,
});

pub static PORT: Lazy<u16> = Lazy::new(|| match std::env::var(PORT_ENV) {
  Ok(value) => value
    .parse::<u16>()
    .expect("PORT environment variable should be u16, between 0 and 65535"),
  Err(_) => DEFAULT_PORT,
});

#[derive(Debug, PartialEq)]
pub enum BootingMode {
  Debug,
  Production,
}

impl std::fmt::Display for BootingMode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      BootingMode::Debug => write!(f, "{}", DEBUG),
      BootingMode::Production => write!(f, "{}", PRODUCTION),
    }
  }
}

impl std::str::FromStr for BootingMode {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      DEBUG => Ok(BootingMode::Debug),
      PRODUCTION => Ok(BootingMode::Production),
      _ => Err("Unknown mode found"),
    }
  }
}
