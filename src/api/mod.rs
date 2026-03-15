#[cfg(feature = "std")]
mod client_std;

#[cfg(feature = "no_std")]
mod client_no_std;

#[cfg(feature = "std")]
pub use client_std::CarrisClient;

#[cfg(feature = "no_std")]
pub use client_no_std::CarrisClient;
