use super::{Error, Kitty, KittyGender};
use crate::mock::*;

use frame_support::{assert_noop, assert_ok};
use sp_core::H256;

#[test]
fn should_create_and_own_kitty() {
	new_test_ext().execute_with(|| {
		assert_ok!(Kitties::create(Origin::signed(100)));

		let kitty =
			Kitty([59, 250, 138, 82, 209, 39, 141, 109, 163, 238, 183, 145, 235, 168, 18, 122]);

		assert_eq!(Kitties::kitties(100, 0), Some(kitty.clone()));
		assert_eq!(Kitties::next_kitty_id(), 1);

		System::assert_last_event(Event::Kitties(crate::Event::<Test>::KittyCreated(
			100, 0, kitty,
		)));
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
		assert_ok!(Kitties::create(Origin::signed(100)));

		// create kitty_1
		MockRandom::set(H256::from([2; 32]));
		assert_ok!(Kitties::create(Origin::signed(100)));

		// breed kitty_0 with kitty_1
		assert_ok!(Kitties::breed(Origin::signed(100), 0, 1));

		let kitty =
			Kitty([187, 250, 235, 118, 211, 247, 237, 253, 187, 239, 191, 185, 239, 171, 211, 122]);

		assert_eq!(Kitties::kitties(100, 2), Some(kitty.clone()));
		assert_eq!(Kitties::next_kitty_id(), 3);

		System::assert_last_event(Event::Kitties(crate::Event::<Test>::KittyCreatedByBreeding(
			100, 2, kitty,
		)));
	});
}

#[test]
fn should_not_breed_when_kitty_not_found() {
	new_test_ext().execute_with(|| {
		assert_ok!(Kitties::create(Origin::signed(100)));
		assert_noop!(Kitties::breed(Origin::signed(100), 0, 11), Error::<Test>::InvalidKittyId);
	});
}

#[test]
fn should_not_breed_when_kitty_not_owned() {
	new_test_ext().execute_with(|| {
		// create kitty_0
		assert_ok!(Kitties::create(Origin::signed(100)));

		// create kitty_1
		MockRandom::set(H256::from([2; 32]));
		assert_ok!(Kitties::create(Origin::signed(100)));

		// breed kitty_0 with kitty_1
		assert_noop!(Kitties::breed(Origin::signed(101), 0, 1), Error::<Test>::InvalidKittyId);
	});
}
