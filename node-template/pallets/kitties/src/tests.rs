#![cfg(test)]

use crate::{mock::*, pallet::Error, *};
use frame_support::{assert_noop, assert_ok};

fn assert_ownership(owner: u64, kitty_id: [u8; 16]) {
	let kitty = Kitties::<Test>::get(kitty_id).unwrap();
	assert_eq!(kitty.owner, owner);

	for (check_owner, owned) in KittiesOwned::<Test>::iter() {
		if owner == check_owner {
			assert!(owned.contains(&kitty_id));
		} else {
			assert!(!owned.contains(&kitty_id));
		}
	}
}

#[test]
fn should_build_genesis_kitties() {
	new_test_ext(vec![
		(1, *b"6834567890123456", Gender::Female),
		(2, *b"593456789012345a", Gender::Male),
	])
	.execute_with(|| {
		assert_eq!(CountForKitties::<Test>::get(), 2);

		let kitties_owned_by_1 = KittiesOwned::<Test>::get(1);
		assert_eq!(kitties_owned_by_1.len(), 1);

		let kitties_owned_by_2 = KittiesOwned::<Test>::get(2);
		assert_eq!(kitties_owned_by_2.len(), 1);

		let kitty_1 = kitties_owned_by_1[0];
		assert_ownership(1, kitty_1);

		let kitty_2 = kitties_owned_by_2[0];
		assert_ownership(2, kitty_2);
	});
}

#[test]
fn create_kitty_should_work() {
	new_test_ext(vec![]).execute_with(|| {
		assert_ok!(SubstrateKitties::create_kitty(Origin::signed(10)));

		assert_eq!(CountForKitties::<Test>::get(), 1);

		let kitties_owned = KittiesOwned::<Test>::get(10);
		assert_eq!(kitties_owned.len(), 1);
		let id = kitties_owned.last().unwrap();
		assert_ownership(10, *id);

		frame_system::Pallet::<Test>::set_extrinsic_index(1);
		assert_ok!(SubstrateKitties::create_kitty(Origin::signed(10)));
	});
}

#[test]
fn create_kitty_fails() {
	new_test_ext(vec![]).execute_with(|| {
		for _i in 0..<Test as Config>::MaxKittiesOwned::get() {
			assert_ok!(SubstrateKitties::create_kitty(Origin::signed(10)));
			System::set_block_number(System::block_number() + 1);
		}

		assert_noop!(
			SubstrateKitties::create_kitty(Origin::signed(10)),
			Error::<Test>::TooManyOwned
		);

		let id = [0u8; 16];

		assert_ok!(SubstrateKitties::mint(&1, id, Gender::Male));

		assert_noop!(SubstrateKitties::mint(&1, id, Gender::Male), Error::<Test>::DuplicateKitty);
	});
}

#[test]
fn transfer_kitty_should_work() {
	new_test_ext(vec![]).execute_with(|| {
		assert_ok!(SubstrateKitties::create_kitty(Origin::signed(10)));
		let id = KittiesOwned::<Test>::get(10)[0];

		assert_ok!(SubstrateKitties::transfer(Origin::signed(10), 3, id));

		assert_eq!(KittiesOwned::<Test>::get(10).len(), 0);

		assert_eq!(KittiesOwned::<Test>::get(3).len(), 1);
		assert_ownership(3, id);
	});
}

#[test]
fn transfer_kitty_should_fail() {
	new_test_ext(vec![
		(1, *b"5634567890123456", Gender::Female),
		(2, *b"673456789012345a", Gender::Male),
	])
	.execute_with(|| {
		let dna = KittiesOwned::<Test>::get(1)[0];

		assert_noop!(
			SubstrateKitties::transfer(Origin::signed(9), 2, dna),
			Error::<Test>::NotOwner
		);

		assert_noop!(
			SubstrateKitties::transfer(Origin::signed(1), 1, dna),
			Error::<Test>::TransferToSelf
		);

		let random_id = [0u8; 16];

		assert_noop!(
			SubstrateKitties::transfer(Origin::signed(2), 1, random_id),
			Error::<Test>::NoKitty
		);

		for _i in 0..<Test as Config>::MaxKittiesOwned::get() {
			assert_ok!(SubstrateKitties::create_kitty(Origin::signed(10)));
			System::set_block_number(System::block_number() + 1);
		}

		assert_noop!(
			SubstrateKitties::transfer(Origin::signed(1), 10, dna),
			Error::<Test>::TooManyOwned
		);
	});
}

#[test]
fn breed_kitty_works() {
	new_test_ext(vec![(2, *b"123456789012345a", Gender::Male)]).execute_with(|| {
		let mom = [0u8; 16];
		assert_ok!(SubstrateKitties::mint(&1, mom, Gender::Female));

		let dad = [1u8; 16];
		assert_ok!(SubstrateKitties::mint(&1, dad, Gender::Male));

		assert_ok!(SubstrateKitties::breed_kitty(Origin::signed(1), mom, dad));

		let new_dna = KittiesOwned::<Test>::get(1)[2];
		for &i in new_dna.iter() {
			assert!(i == 0u8 || i == 1u8)
		}

		assert_noop!(
			SubstrateKitties::breed_kitty(Origin::signed(1), mom, mom),
			Error::<Test>::CantBreed
		);

		let kitty_1 = KittiesOwned::<Test>::get(1)[0];

		let kitty_2 = KittiesOwned::<Test>::get(2)[0];
		assert_noop!(
			SubstrateKitties::breed_kitty(Origin::signed(1), kitty_1, kitty_2),
			Error::<Test>::NotOwner
		);
	});
}

#[test]
fn breed_kitty_fails() {
	new_test_ext(vec![]).execute_with(|| {
		let kitty_1 = [1u8; 16];
		let kitty_2 = [3u8; 16];

		assert_ok!(SubstrateKitties::mint(&3, kitty_1, Gender::Female));
		assert_ok!(SubstrateKitties::mint(&3, kitty_2, Gender::Female));

		let kitty_3 = [4u8; 16];
		assert_ok!(SubstrateKitties::mint(&3, kitty_3, Gender::Male));

		assert_noop!(
			SubstrateKitties::breed_kitty(Origin::signed(3), kitty_1, kitty_2),
			Error::<Test>::CantBreed
		);

		for _i in 0..<Test as Config>::MaxKittiesOwned::get() - 3 {
			assert_ok!(SubstrateKitties::create_kitty(Origin::signed(3)));
			System::set_block_number(System::block_number() + 1);
		}

		assert_noop!(
			SubstrateKitties::breed_kitty(Origin::signed(3), kitty_1, kitty_3),
			Error::<Test>::TooManyOwned
		);
	});
}

#[test]
fn dna_helpers_work_as_expected() {
	new_test_ext(vec![]).execute_with(|| {
		let dna_1 = [1u8; 16];
		let dna_2 = [2u8; 16];

		let (dna, _) = SubstrateKitties::breed_dna(&dna_1, &dna_2);

		for &i in dna.iter() {
			assert!(i == 1u8 || i == 2u8)
		}

		let (random_dna_1, _) = SubstrateKitties::gen_dna();
		frame_system::Pallet::<Test>::set_extrinsic_index(1);
		let (random_dna_2, _) = SubstrateKitties::gen_dna();
		assert_ne!(random_dna_1, random_dna_2);
	});
}

#[test]
fn buy_kitty_works() {
	new_test_ext(vec![
		(1, *b"1234567890123456", Gender::Female),
		(2, *b"123456789012345a", Gender::Male),
		(3, *b"1234567890123451", Gender::Male),
	])
	.execute_with(|| {
		let id = KittiesOwned::<Test>::get(2)[0];
		let set_price = 4;
		let balance_1_before = Balances::free_balance(&1);
		let balance_2_before = Balances::free_balance(&2);

		assert_ok!(SubstrateKitties::set_price(Origin::signed(2), id, Some(set_price)));

		let limit_price = 6;
		assert_ok!(SubstrateKitties::buy_kitty(Origin::signed(1), id, limit_price));

		let balance_1_after = Balances::free_balance(&1);
		let balance_2_after = Balances::free_balance(&2);

		assert_eq!(balance_1_before - set_price, balance_1_after);
		assert_eq!(balance_2_before + set_price, balance_2_after);

		assert_noop!(
			SubstrateKitties::buy_kitty(Origin::signed(3), id, set_price),
			Error::<Test>::NotForSale
		);
	});
}

#[test]
fn buy_kitty_fails() {
	new_test_ext(vec![
		(1, *b"1234567890123456", Gender::Female),
		(2, *b"123456789012345a", Gender::Male),
		(10, *b"1234567890123410", Gender::Male),
	])
	.execute_with(|| {
		let id = KittiesOwned::<Test>::get(1)[0];
		assert_noop!(
			SubstrateKitties::buy_kitty(Origin::signed(2), id, 2),
			Error::<Test>::NotForSale
		);

		let id = KittiesOwned::<Test>::get(2)[0];
		let set_price = 4;
		assert_ok!(SubstrateKitties::set_price(Origin::signed(2), id, Some(set_price)));

		assert_noop!(
			SubstrateKitties::buy_kitty(Origin::signed(10), id, set_price / 2),
			Error::<Test>::BidPriceTooLow
		);

		let balance_of_account_10 = Balances::free_balance(&10);

		assert_ok!(SubstrateKitties::set_price(
			Origin::signed(2),
			id,
			Some(balance_of_account_10 * 10)
		));

		assert_noop!(
			SubstrateKitties::buy_kitty(Origin::signed(10), id, balance_of_account_10 * 10),
			pallet_balances::Error::<Test>::InsufficientBalance
		);
	});
}

#[test]
fn set_price_works() {
	new_test_ext(vec![
		(1, *b"1234567890123456", Gender::Female),
		(2, *b"123456789012345a", Gender::Male),
	])
	.execute_with(|| {
		let id = KittiesOwned::<Test>::get(2)[0];
		let set_price = 4;
		assert_ok!(SubstrateKitties::set_price(Origin::signed(2), id, Some(set_price)));

		assert_noop!(
			SubstrateKitties::set_price(Origin::signed(1), id, Some(set_price)),
			Error::<Test>::NotOwner
		);

		let non_dna = [2u8; 16];
		assert_noop!(
			SubstrateKitties::set_price(Origin::signed(1), non_dna, Some(set_price)),
			Error::<Test>::NoKitty
		);
	});
}
