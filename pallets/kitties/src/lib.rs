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
		/// A kitty is created from a breeding pair. \[owner, kitty_id, kitty\]
		KittyCreatedByBreeding(T::AccountId, T::KittyIndex, Kitty),
	}

	// --- ERRORS ---
	#[pallet::error]
	pub enum Error<T> {
		InvalidKittyId,
		SameGender,
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

		/// Breed kitties to create a new kitty
		#[pallet::weight(1000)]
		pub fn breed(
			origin: OriginFor<T>,
			kitty1_id: T::KittyIndex,
			kitty2_id: T::KittyIndex,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let kitty1 = Self::kitties(&sender, kitty1_id).ok_or(Error::<T>::InvalidKittyId)?;
			let kitty2 = Self::kitties(&sender, kitty2_id).ok_or(Error::<T>::InvalidKittyId)?;

			ensure!(kitty1.gender() != kitty2.gender(), Error::<T>::SameGender);

			let kitty_id = Self::get_kitty_id()?;

			let dna = Self::combine_kitties_dna(&sender, kitty1.dna(), kitty2.dna());
			let kitty = Kitty(dna);

			Kitties::<T>::insert(&sender, kitty_id, &kitty);

			Self::deposit_event(Event::KittyCreatedByBreeding(sender, kitty_id, kitty));

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

	fn generate_kitty_dna(sender: &T::AccountId) -> Dna {
		let payload =
			(T::Randomness::random_seed().0, &sender, <frame_system::Pallet<T>>::extrinsic_index());
		payload.using_encoded(blake2_128)
	}

	fn combine_kitties_dna(sender: &T::AccountId, kitty1_dna: Dna, kitty2_dna: Dna) -> Dna {
		let mut dna = Self::generate_kitty_dna(&sender);

		for i in 0..dna.len() {
			dna[i] = (!dna[i] & kitty1_dna[i]) | (dna[i] & kitty2_dna[i]);
		}

		dna
	}
}

type Dna = [u8; 16];

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub struct Kitty(pub Dna);

#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq, RuntimeDebug)]
pub enum KittyGender {
	Female,
	Male,
}

impl Kitty {
	pub fn dna(&self) -> Dna {
		self.0
	}

	pub fn gender(&self) -> KittyGender {
		if self.0[0] % 2 == 0 {
			KittyGender::Female
		} else {
			KittyGender::Male
		}
	}
}
