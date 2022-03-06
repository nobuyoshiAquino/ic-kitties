#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
	pallet_prelude::{DispatchError, RuntimeDebug},
	traits::Randomness,
	Parameter,
};
use scale_info::TypeInfo;
use sp_io::hashing::blake2_128;
use sp_runtime::{
	traits::{AtLeast32BitUnsigned, Bounded, CheckedAdd, One},
	ArithmeticError,
};

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	// --- CONFIG ---
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type KittyIndex: AtLeast32BitUnsigned + Bounded + Copy + Default + MaxEncodedLen + Parameter;
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
	}

	// --- STORAGE ---
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Stores an index value used as an identifier for the new kitties.
	#[pallet::storage]
	#[pallet::getter(fn next_kitty_id)]
	pub type NextKittyId<T: Config> = StorageValue<_, T::KittyIndex, ValueQuery>;

	/// Stores all the kitties.
	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub type Kitties<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		T::KittyIndex,
		Kitty,
		OptionQuery,
	>;

	// --- EVENTS ---
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A kitty is created. \[owner, kitty_id, kitty\]
		KittyCreated(T::AccountId, T::KittyIndex, Kitty),
	}

	// --- CALLS ---
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new kitty
		#[pallet::weight(1000)]
		pub fn create(origin: OriginFor<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let kitty_id = Self::get_kitty_id()?;
			let dna = Self::generate_kitty_dna(&sender);

			let kitty = Kitty(dna);
			Kitties::<T>::insert(&sender, kitty_id, &kitty);

			Self::deposit_event(Event::KittyCreated(sender, kitty_id, kitty));

			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	fn get_kitty_id() -> Result<T::KittyIndex, DispatchError> {
		NextKittyId::<T>::try_mutate(|next_id| -> Result<T::KittyIndex, DispatchError> {
			let id = *next_id;
			*next_id = next_id.checked_add(&One::one()).ok_or(ArithmeticError::Overflow)?;
			Ok(id)
		})
	}

	fn generate_kitty_dna(sender: &T::AccountId) -> [u8; 16] {
		let payload =
			(T::Randomness::random_seed().0, &sender, <frame_system::Pallet<T>>::extrinsic_index());
		payload.using_encoded(blake2_128)
	}
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub struct Kitty(pub [u8; 16]);
