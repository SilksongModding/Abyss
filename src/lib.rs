pub mod bepinex;
pub mod detector;
pub mod installer;
pub mod provider;

// Re-export useful types at crate root if desired
pub use detector::Detector;
pub use provider::RealSteamProvider;
