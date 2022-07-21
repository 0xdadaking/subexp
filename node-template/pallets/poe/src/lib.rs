#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	pub type BoundedClaim<T> = BoundedVec<u8, <T as Config>::ClaimLimitSize>;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		#[pallet::constant]
		type ClaimLimitSize: Get<u32> + Clone + Eq + PartialEq;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn proofs)]
	pub type Proofs<T: Config> =
		StorageMap<_, Blake2_128Concat, BoundedClaim<T>, (T::AccountId, T::BlockNumber)>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClaimCreated(T::AccountId, BoundedClaim<T>),
		ClaimRevoked(T::AccountId, BoundedClaim<T>),
		ClaimTransfered(T::AccountId, T::AccountId, BoundedClaim<T>),
	}

	#[pallet::error]
	pub enum Error<T> {
		ProofAlreadyExists,
		ClaimNotExists,
		NotClaimOwner,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn create_claim(origin: OriginFor<T>, claim: BoundedClaim<T>) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExists);

			Proofs::<T>::insert(
				&claim,
				(sender.clone(), frame_system::Pallet::<T>::block_number()),
			);

			Self::deposit_event(Event::ClaimCreated(sender, claim));

			Ok(().into())
		}

		#[pallet::weight(0)]
		pub fn revoke_claim(origin: OriginFor<T>, claim: BoundedClaim<T>) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExists)?;

			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&claim);

			Self::deposit_event(Event::ClaimRevoked(sender, claim));

			Ok(().into())
		}

		#[pallet::weight(0)]
		pub fn transfer_claim(origin: OriginFor<T>, to: T::AccountId, claim: BoundedClaim<T>) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			let (owner, block_number) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExists)?;

			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&claim);
			Proofs::<T>::insert(
				&claim,
				(to.clone(), block_number),
			);

			Self::deposit_event(Event::ClaimTransfered(sender, to, claim));

			Ok(().into())
		}
	}
}
