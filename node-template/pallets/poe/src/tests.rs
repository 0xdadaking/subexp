use crate::{mock::*, BoundedClaim, Error, Proofs};
use frame_support::{assert_noop, assert_ok};

fn pop_event() -> Event {
	System::events()
		.pop()
		.expect("Expected at least one Registered to be found")
		.event
}

#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![1, 2, 3, 4, 5];
		assert_ok!(Poe::create_claim(Origin::signed(1), claim.clone()));

		let bounded_claim = BoundedClaim::<Test>::try_from(claim.clone()).unwrap();
		assert_eq!(
			Proofs::<Test>::get(&bounded_claim),
			Some((1, frame_system::Pallet::<Test>::block_number()))
		);

		assert_eq!(Event::from(super::Event::ClaimCreated(1, bounded_claim)), pop_event());
	});
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![1, 2, 3, 4, 5];
		assert_ok!(Poe::create_claim(Origin::signed(1), claim.clone()));
		assert_noop!(
			Poe::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ProofAlreadyExists
		);
	});
}

#[test]
fn create_claim_failed_when_exceed_claim_size() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Poe::create_claim(Origin::signed(1), vec![1, 2, 3, 4, 5, 6]),
			Error::<Test>::ClaimTooLong
		);
	});
}

#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![1, 2, 3, 4, 5];
		assert_ok!(Poe::create_claim(Origin::signed(1), claim.clone()));
		assert_ok!(Poe::revoke_claim(Origin::signed(1), claim.clone()));
		let bounded_claim = BoundedClaim::<Test>::try_from(claim.clone()).unwrap();
		assert_eq!(Proofs::<Test>::get(&bounded_claim), None);

		assert_eq!(Event::from(super::Event::ClaimRevoked(1, bounded_claim)), pop_event());
	});
}

#[test]
fn revoke_claim_failed_when_claim_not_exists() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Poe::revoke_claim(Origin::signed(1), vec![1, 3, 9]),
			Error::<Test>::ClaimNotExists
		);
	});
}

#[test]
fn revoke_claim_failed_when_exceed_claim_size() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Poe::revoke_claim(Origin::signed(1), vec![1, 2, 3, 4, 5, 6]),
			Error::<Test>::ClaimTooLong
		);
	});
}

#[test]
fn revoke_claim_failed_when_not_claim_owner() {
	new_test_ext().execute_with(|| {
		let claim = vec![1, 2, 3, 4, 5];
		assert_ok!(Poe::create_claim(Origin::signed(1), claim.clone()));
		assert_noop!(
			Poe::revoke_claim(Origin::signed(2), claim.clone()),
			Error::<Test>::NotClaimOwner
		);
	});
}

#[test]
fn transfer_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![1, 2, 3, 4, 5];
		assert_ok!(Poe::create_claim(Origin::signed(1), claim.clone()));
		assert_ok!(Poe::transfer_claim(Origin::signed(1), 2, claim.clone()));
		let bounded_claim = BoundedClaim::<Test>::try_from(claim.clone()).unwrap();
		assert_eq!(
			Proofs::<Test>::get(&bounded_claim),
			Some((2, frame_system::Pallet::<Test>::block_number()))
		);

		assert_eq!(Event::from(super::Event::ClaimTransferred(1, 2, bounded_claim)), pop_event());
	});
}

#[test]
fn transfer_claim_failed_when_claim_not_exists() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Poe::transfer_claim(Origin::signed(1), 2, vec![1, 3, 9]),
			Error::<Test>::ClaimNotExists
		);
	});
}

#[test]
fn transfer_claim_failed_when_exceed_claim_size() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Poe::transfer_claim(Origin::signed(1), 2, vec![1, 2, 3, 4, 5, 6]),
			Error::<Test>::ClaimTooLong
		);
	});
}

#[test]
fn transfer_claim_failed_when_not_claim_owner() {
	new_test_ext().execute_with(|| {
		let claim = vec![1, 2, 3, 4, 5];
		assert_ok!(Poe::create_claim(Origin::signed(1), claim.clone()));
		assert_noop!(
			Poe::transfer_claim(Origin::signed(2), 1, claim.clone()),
			Error::<Test>::NotClaimOwner
		);
	});
}
