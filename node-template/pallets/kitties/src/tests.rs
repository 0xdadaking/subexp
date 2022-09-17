use crate::{
	mock::{Event, Kitties as KittiesModule, System, Test, *},
	Kitties, *,
};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_kitty_works() {
	new_test_ext().execute_with(|| {
		let amount = Balances::free_balance(&1);
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert!(Kitties::<Test>::contains_key(0));
		assert_eq!(Some(1), KittyOwner::<Test>::get(0));
		assert!(BelongsKitties::<Test>::contains_key(1, 0));
		assert_eq!(amount - KittyUnitPrice::get(), Balances::free_balance(&1));
		System::assert_last_event(Event::from(super::Event::KittyCreated(1, 0)));

		//create twice
		let amount = Balances::free_balance(&1);
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert!(Kitties::<Test>::contains_key(1));
		assert_eq!(Some(1), KittyOwner::<Test>::get(1));
		assert!(BelongsKitties::<Test>::contains_key(1, 0));
		assert!(BelongsKitties::<Test>::contains_key(1, 1));
		assert_eq!(amount - KittyUnitPrice::get(), Balances::free_balance(&1));
		System::assert_last_event(Event::from(super::Event::KittyCreated(1, 1)));
	});
}

#[test]
fn create_kitty_failed_on_index_overflow() {
	new_test_ext().execute_with(|| {
		NextKittyId::<Test>::put(u32::MAX);
		assert_noop!(KittiesModule::create(Origin::signed(1)), Error::<Test>::KittyIndexOverflow);
		assert_eq!(Kitties::<Test>::iter().collect::<Vec<_>>().len(), 0);
	});
}

#[test]
fn create_kitty_failed_when_not_enough_balance() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			KittiesModule::create(Origin::signed(2)),
			Error::<Test>::NotEnoughBalanceForStaking
		);
		assert_eq!(Kitties::<Test>::iter().collect::<Vec<_>>().len(), 0);
	});
}

#[test]
fn breed_kitty_works() {
	new_test_ext().execute_with(|| {
		let amount = Balances::free_balance(&1);
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_ok!(KittiesModule::breed(Origin::signed(1), 0, 1));

		assert!(Kitties::<Test>::contains_key(2));
		assert_eq!(Some(1), KittyOwner::<Test>::get(2));
		assert!(BelongsKitties::<Test>::contains_key(1, 0));
		assert!(BelongsKitties::<Test>::contains_key(1, 1));
		assert!(BelongsKitties::<Test>::contains_key(1, 2));
		assert_eq!(amount - KittyUnitPrice::get() * 3, Balances::free_balance(&1));
		System::assert_last_event(Event::from(super::Event::KittyBred(1, 2)));
	});
}

#[test]
fn breed_kitty_failed_on_index_overflow() {
	new_test_ext().execute_with(|| {
		NextKittyId::<Test>::put(u32::MAX - 2);
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_noop!(
			KittiesModule::breed(Origin::signed(1), u32::MAX - 2, u32::MAX - 1),
			Error::<Test>::KittyIndexOverflow
		);
		assert_eq!(Kitties::<Test>::iter().collect::<Vec<_>>().len(), 2);
	});
}

#[test]
fn breed_kitty_failed_when_not_enough_balance() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(3)));
		assert_ok!(KittiesModule::create(Origin::signed(3)));
		assert_noop!(
			KittiesModule::breed(Origin::signed(3), 0, 1),
			Error::<Test>::NotEnoughBalanceForStaking
		);
		assert_eq!(Kitties::<Test>::iter().collect::<Vec<_>>().len(), 2);
	});
}

#[test]
fn breed_kitty_failed_when_invalid_kitty_id() {
	new_test_ext().execute_with(|| {
		assert_noop!(KittiesModule::breed(Origin::signed(1), 0, 1), Error::<Test>::InvalidKittyId);
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_noop!(KittiesModule::breed(Origin::signed(1), 0, 1), Error::<Test>::InvalidKittyId);
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_ok!(KittiesModule::breed(Origin::signed(1), 0, 1));
	});
}

#[test]
fn breed_kitty_failed_when_same_kitty_id() {
	new_test_ext().execute_with(|| {
		assert_noop!(KittiesModule::breed(Origin::signed(1), 0, 0), Error::<Test>::SameKittyId);
	});
}

#[test]
fn transfer_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_eq!(Some(1), KittyOwner::<Test>::get(0));
		let amount_a = Balances::free_balance(&1);
		let amount_b = Balances::free_balance(&3);
		assert_ok!(KittiesModule::transfer(Origin::signed(1), 0, 3));
		assert_eq!(Some(3), KittyOwner::<Test>::get(0));

		assert_eq!(amount_b - KittyUnitPrice::get(), Balances::free_balance(&3));
		assert_eq!(amount_a + KittyUnitPrice::get(), Balances::free_balance(&1));
		System::assert_last_event(Event::from(super::Event::KittyTransferred(1, 3, 0)));
	});
}

#[test]
fn transfer_failed_when_to_self() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			KittiesModule::transfer(Origin::signed(1), 0, 1),
			Error::<Test>::DontSelfTransfer
		);
	});
}

#[test]
fn transfer_failed_when_not_owner() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_noop!(KittiesModule::transfer(Origin::signed(2), 0, 3), Error::<Test>::NotOwner);
	});
}

#[test]
fn transfer_failed_when_invalid_kitty_id() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			KittiesModule::transfer(Origin::signed(2), 0, 3),
			Error::<Test>::InvalidKittyId
		);
	});
}

#[test]
fn transfer_failed_when_not_enough_balance() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_noop!(
			KittiesModule::transfer(Origin::signed(1), 0, 2),
			Error::<Test>::NotEnoughBalanceForStaking
		);
	});
}
