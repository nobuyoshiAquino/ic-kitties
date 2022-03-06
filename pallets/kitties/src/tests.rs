use super::Kitty;
use crate::mock::*;

use frame_support::assert_ok;

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
