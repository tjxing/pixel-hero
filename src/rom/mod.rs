mod rom;
mod slice;
mod mapper;
mod mappers;

pub use self::rom::{Rom, Timing};

#[cfg(test)]
pub mod tests {
    pub use super::rom::tests::mock;
}