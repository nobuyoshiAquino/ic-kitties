use super::{Error, Kitties, Kitty, KittyGender, KittyPrices};
use crate::mock::*;

use frame_support::{assert_noop, assert_ok};
use sp_core::H256;

#[test]
fn should_create_and_own_kitty() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(100)));

		let kitty =
			Kitty([59, 250, 138, 82, 209, 39, 141, 109, 163, 238, 183, 145, 235, 168, 18, 122]);

		assert_eq!(KittiesModule::kitties(100, 0), Some(kitty.clone()));
		assert_eq!(KittiesModule::next_kitty_id(), 1);

		System::assert_last_event(Event::KittiesModule(crate::Event::KittyCreated(100, 0, kitty)));
	});
}

#[test]
fn should_be_female_kitty() {
	assert_eq!(Kitty([0; 16]).gender(), KittyGender::Female);
}

#[test]
fn should_be_male_kitty() {
	assert_eq!(Kitty([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).gender(), KittyGender::Male);
}

#[test]
fn should_create_kitty_from_breeding_pair() {
	new_test_ext().execute_with(|| {
		// create kitty_0
		assert_ok!(KittiesModule::create(Origin::signed(100)));

		// create kitty_1
		MockRandom::set(H256::from([2; 32]));
		assert_ok!(KittiesModule::create(Origin::signed(100)));

		// breed kitty_0 with kitty_1
		assert_ok!(KittiesModule::breed(Origin::signed(100), 0, 1));

		let kitty =
			Kitty([187, 250, 235, 118, 211, 247, 237, 253, 187, 239, 191, 185, 239, 171, 211, 122]);

		assert_eq!(KittiesModule::kitties(100, 2), Some(kitty.clone()));
		assert_eq!(KittiesModule::next_kitty_id(), 3);

		System::assert_last_event(Event::KittiesModule(crate::Event::KittyCreatedByBreeding(
			100, 2, kitty,
		)));
	});
}

#[test]
fn should_not_breed_when_kitty_not_found() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(100)));
		assert_noop!(
			KittiesModule::breed(Origin::signed(100), 0, 11),
			Error::<Test>::InvalidKittyId
		);
	});
}

#[test]
fn should_not_breed_when_kitty_not_owned() {
	new_test_ext().execute_with(|| {
		// create kitty_0
		assert_ok!(KittiesModule::create(Origin::signed(100)));

		// create kitty_1
		MockRandom::set(H256::from([2; 32]));
		assert_ok!(KittiesModule::create(Origin::signed(100)));

		// breed kitty_0 with kitty_1
		assert_noop!(
			KittiesModule::breed(Origin::signed(101), 0, 1),
			Error::<Test>::InvalidKittyId
		);
	});
}

#[test]
fn should_tranfer_kitty() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(100)));
		assert_ok!(KittiesModule::transfer(Origin::signed(100), 200, 0));

		let kitty =
			Kitty([59, 250, 138, 82, 209, 39, 141, 109, 163, 238, 183, 145, 235, 168, 18, 122]);

		assert_eq!(KittiesModule::kitties(200, 0), Some(kitty));
		assert_eq!(KittiesModule::kitties(100, 0), None);

		System::assert_last_event(Event::KittiesModule(crate::Event::KittyTransferred(
			100, 200, 0,
		)));
	});
}

#[test]
fn should_not_transfer_when_kitty_invalid() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(100)));

		// kitty #0 not owned by user #101
		assert_noop!(
			KittiesModule::transfer(Origin::signed(101), 200, 0),
			Error::<Test>::InvalidKittyId
		);

		// kitty #1 does not exists
		assert_noop!(
			KittiesModule::transfer(Origin::signed(100), 100, 1),
			Error::<Test>::InvalidKittyId
		);
	});
}

#[test]
fn should_not_transfer_to_self() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(100)));

		// we reset events before transferring to
		// check later that the call did not emit any event.
		System::reset_events();

		assert_ok!(KittiesModule::transfer(Origin::signed(100), 100, 0));

		let kitty =
			Kitty([59, 250, 138, 82, 209, 39, 141, 109, 163, 238, 183, 145, 235, 168, 18, 122]);

		assert_eq!(KittiesModule::kitties(100, 0), Some(kitty));
		assert_eq!(System::events().len(), 0);
	});
}

#[test]
fn should_set_kitty_price() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(100)));

		// Set the price to `Some(10)`
		assert_ok!(KittiesModule::set_price(Origin::signed(100), 0, Some(10)));

		assert_eq!(KittiesModule::kitty_prices(0), Some(10));

		System::assert_last_event(Event::KittiesModule(crate::Event::KittyPriceUpdated(
			100,
			0,
			Some(10),
		)));

		// Set the price to `None` (delist)
		assert_ok!(KittiesModule::set_price(Origin::signed(100), 0, None));

		assert_eq!(KittiesModule::kitty_prices(0), None);
		assert_eq!(KittyPrices::<Test>::contains_key(0), false);

		System::assert_last_event(Event::KittiesModule(crate::Event::KittyPriceUpdated(
			100, 0, None,
		)));
	});
}

#[test]
fn should_not_set_price_when_kitty_not_owned() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(100)));

		assert_noop!(
			KittiesModule::set_price(Origin::signed(101), 0, Some(10)),
			Error::<Test>::NotOwner
		);
	});
}

#[test]
fn should_buy() {
	new_test_ext().execute_with(|| {
		// User#100 create a kitty, and then set a price for it.
		assert_ok!(KittiesModule::create(Origin::signed(100)));
		assert_ok!(KittiesModule::set_price(Origin::signed(100), 0, Some(333)));

		// User#200 bought the kitty from User#100
		assert_ok!(KittiesModule::buy(Origin::signed(200), 100, 0, 333));

		assert_eq!(KittyPrices::<Test>::contains_key(0), false);
		assert_eq!(Kitties::<Test>::contains_key(200, 0), true);

		assert_eq!(Balances::free_balance(100), 333);
		assert_eq!(Balances::free_balance(200), 167);

		System::assert_last_event(Event::KittiesModule(crate::Event::KittySold(100, 200, 0, 333)));
	});
}

#[test]
fn should_fail_buy_from_self() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(100)));
		assert_ok!(KittiesModule::set_price(Origin::signed(100), 0, Some(333)));

		assert_noop!(
			KittiesModule::buy(Origin::signed(100), 100, 0, 333),
			Error::<Test>::BuyerIsSeller
		);
	});
}

#[test]
fn should_fail_buy_when_kitty_not_listed() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(100)));

		assert_noop!(
			KittiesModule::buy(Origin::signed(200), 100, 0, 333),
			Error::<Test>::NotForSale
		);

		assert_noop!(
			KittiesModule::buy(Origin::signed(200), 100, 1, 333),
			Error::<Test>::NotForSale
		);
	});
}

#[test]
fn should_fail_buy_when_bid_price_low() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(100)));
		assert_ok!(KittiesModule::set_price(Origin::signed(100), 0, Some(333)));

		assert_noop!(
			KittiesModule::buy(Origin::signed(200), 100, 0, 300),
			Error::<Test>::BidPriceTooLow
		);
	});
}

#[test]
fn should_fail_buy_when_insufficient_balance() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(100)));
		assert_ok!(KittiesModule::set_price(Origin::signed(100), 0, Some(600)));

		assert_noop!(
			KittiesModule::buy(Origin::signed(200), 100, 0, 600),
			pallet_balances::Error::<Test, _>::InsufficientBalance
		);
	});
}
