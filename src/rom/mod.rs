mod rom;
mod slice;
mod mapper;
mod mappers;
mod timing;

pub use self::rom::Rom;
pub use self::timing::Timing;

#[cfg(test)]
pub mod tests {
    pub use super::rom::tests::mock;
}