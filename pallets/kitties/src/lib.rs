#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	// --- CONFIG ---
	#[pallet::config]
	pub trait Config: frame_system::Config {}

	// --- STORAGE ---
	#[pallet::pallet]
	pub struct Pallet<T>(_);
}
