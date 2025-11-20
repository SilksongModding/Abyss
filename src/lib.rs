pub mod detector;
pub mod provider;
pub mod bepinex;

// Re-export useful types at crate root if desired
pub use detector::Detector;
pub use provider::RealSteamProvider;
