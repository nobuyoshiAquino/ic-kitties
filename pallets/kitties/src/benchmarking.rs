use super::*;

use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	create {
		let caller = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))

	breed {
		let caller = whitelisted_caller();

		// kitty_mom_dna = [0, 0, ..., 0]
		let mut kitty = Kitty(Default::default());
		let kitty_mom_id = Pallet::<T>::get_kitty_id()?;

		pallet::Kitties::<T>::insert(&caller, kitty_mom_id, kitty.clone());

		// kitty_dad_dna = [1, 0, ..., 0]
		kitty.0[0] = 1;
		let kitty_dad_id = Pallet::<T>::get_kitty_id()?;

		pallet::Kitties::<T>::insert(&caller, kitty_dad_id, kitty);

	}: _(RawOrigin::Signed(caller), kitty_mom_id, kitty_dad_id)

	transfer {
		let caller = whitelisted_caller();
		let to = account("to", 0, 0);

		let kitty_id = Pallet::<T>::get_kitty_id()?;
		pallet::Kitties::<T>::insert(&caller, kitty_id, Kitty(Default::default()));

	}: _(RawOrigin::Signed(caller), to, kitty_id)

	set_price {
		let caller = whitelisted_caller();

		let kitty_id = Pallet::<T>::get_kitty_id()?;
		pallet::Kitties::<T>::insert(&caller, kitty_id, Kitty(Default::default()));

	}: _(RawOrigin::Signed(caller), kitty_id, Some(100u32.into()))

	buy {
		let caller = whitelisted_caller();
		let seller: T::AccountId = account("seller", 0, 0);

		let _ = T::Currency::make_free_balance_be(&caller, 1000u32.into());

		let kitty_id = Pallet::<T>::get_kitty_id()?;
		pallet::Kitties::<T>::insert(&seller, kitty_id, Kitty(Default::default()));

		Pallet::<T>::set_price(RawOrigin::Signed(seller.clone()).into(), kitty_id, Some(500u32.into()))?;
	}: _(RawOrigin::Signed(caller), seller, kitty_id, 500u32.into())
}

impl_benchmark_test_suite!(Pallet, crate::tests::new_test_ext(), crate::tests::Tests,);
