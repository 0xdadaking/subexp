use crate::*;
use frame_benchmarking::{benchmarks, whitelisted_caller, account};
use frame_system::RawOrigin;

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

benchmarks! {
	create_claim {
		let d in 0 .. T::ClaimLimitSize::get();
		let claim = vec![0; d as usize];
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller.clone()), claim.clone())
	verify {
        let bounded_claim = BoundedClaim::<T>::try_from(claim.clone()).unwrap();
		assert_last_event::<T>(Event::ClaimCreated(caller, bounded_claim).into())
	}

	revoke_claim {
		let d in 0 .. T::ClaimLimitSize::get();
		let claim = vec![0; d as usize];
		let caller: T::AccountId = whitelisted_caller();
		assert!(Pallet::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(), claim.clone()).is_ok());
	}: _(RawOrigin::Signed(caller.clone()), claim.clone())
	verify {
        let bounded_claim = BoundedClaim::<T>::try_from(claim.clone()).unwrap();
		assert_last_event::<T>(Event::ClaimRevoked(caller, bounded_claim).into())
	}

	transfer_claim {
		let d in 0 .. T::ClaimLimitSize::get();
		let claim = vec![0; d as usize];
		let caller: T::AccountId = whitelisted_caller();
		let target: T::AccountId = account("target", 0, 0);
		assert!(Pallet::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(), claim.clone()).is_ok());
	}: _(RawOrigin::Signed(caller), target, claim)

	impl_benchmark_test_suite!(PoeModule, crate::mock::new_test_ext(), crate::mock::Test);
}
