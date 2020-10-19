pub mod fs;
pub mod singleton;
pub use singleton::Singleton;

pub type Result<T> = std::result::Result<T, String>;
