#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		pallet_prelude::*,
		traits::{Currency, Randomness, ReservableCurrency},
	};
	use frame_system::pallet_prelude::*;
	use sp_io::hashing::blake2_128;
	use sp_runtime::traits::{AtLeast32BitUnsigned, Bounded};

	type Dna = [u8; 16];
	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
	pub struct Kitty(pub Dna);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
		type KittyIndex: Parameter + AtLeast32BitUnsigned + Default + Copy + Bounded + MaxEncodedLen;
		#[pallet::constant]
		type KittyUnitPrice: Get<BalanceOf<Self>>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn next_kitty_id)]
	pub type NextKittyId<T: Config> = StorageValue<_, T::KittyIndex>;

	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub type Kitties<T: Config> = StorageMap<_, Blake2_128Concat, T::KittyIndex, Kitty>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_owner)]
	pub type KittyOwner<T: Config> = StorageMap<_, Blake2_128Concat, T::KittyIndex, T::AccountId>;

	//TODO! Are there other better solutions?
	#[pallet::storage]
	pub type BelongsKitties<T: Config> =
		StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Identity, T::KittyIndex, ()>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		KittyCreated(T::AccountId, T::KittyIndex),
		KittyBred(T::AccountId, T::KittyIndex),
		KittyTransferred(T::AccountId, T::AccountId, T::KittyIndex),
	}

	#[pallet::error]
	pub enum Error<T> {
		InvalidKittyId,
		SameKittyId,
		NotOwner,
		KittyIndexOverflow,
		NotEnoughBalanceForStaking,
		DontSelfTransfer,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn create(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let event_emitter = |owner: &T::AccountId, kitty_id: &T::KittyIndex| {
				Event::KittyCreated(owner.clone(), kitty_id.clone())
			};
			Self::new_kitty(&who, Self::gen_dna(&who), Some(event_emitter))?;
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn breed(
			origin: OriginFor<T>,
			kitty_id1: T::KittyIndex,
			kitty_id2: T::KittyIndex,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(kitty_id1 != kitty_id2, Error::<T>::SameKittyId);
			let kitty1 = Self::get_kitty(kitty_id1).map_err(|_| Error::<T>::InvalidKittyId)?;
			let kitty2 = Self::get_kitty(kitty_id2).map_err(|_| Error::<T>::InvalidKittyId)?;

			let event_emitter = |owner: &T::AccountId, kitty_id: &T::KittyIndex| {
				Event::KittyBred(owner.clone(), kitty_id.clone())
			};
			Self::new_kitty(
				&who,
				Self::gen_dna_for_breed(&who, &kitty1.0, &kitty2.0),
				Some(event_emitter),
			)?;
			Ok(())
		}

		#[pallet::weight(10000)]
		pub fn transfer(
			origin: OriginFor<T>,
			kitty_id: T::KittyIndex,
			new_owner: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(who != new_owner, Error::<T>::DontSelfTransfer);
			ensure!(Kitties::<T>::contains_key(kitty_id), Error::<T>::InvalidKittyId);
			ensure!(Self::kitty_owner(kitty_id) == Some(who.clone()), Error::<T>::NotOwner);

			T::Currency::reserve(&new_owner, T::KittyUnitPrice::get())
				.map_err(|_| Error::<T>::NotEnoughBalanceForStaking)?;
			T::Currency::unreserve(&who, T::KittyUnitPrice::get());

			KittyOwner::<T>::insert(kitty_id, &new_owner);
			BelongsKitties::<T>::insert(&new_owner, kitty_id, ());
			BelongsKitties::<T>::remove(&who, kitty_id);

			Self::deposit_event(Event::KittyTransferred(who, new_owner, kitty_id));
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn gen_dna(owner: &T::AccountId) -> Dna {
			let payload = (
				T::Randomness::random_seed(),
				&owner,
				<frame_system::Pallet<T>>::extrinsic_index(),
			);
			payload.using_encoded(blake2_128)
		}

		fn gen_dna_for_breed(owner: &T::AccountId, dna1: &Dna, dna2: &Dna) -> Dna {
			let selector = Self::gen_dna(owner);
			let mut data = [0u8; 16];
			for i in 0..dna1.len() {
				data[i] = (dna1[i] & selector[i]) | (dna2[i] & !selector[i]);
			}
			data
		}

		fn get_kitty(kitty_id: T::KittyIndex) -> Result<Kitty, ()> {
			match Self::kitties(kitty_id) {
				Some(kitty) => Ok(kitty),
				None => Err(()),
			}
		}

		fn new_kitty<F>(owner: &T::AccountId, dna: Dna, event_to_emit: Option<F>) -> DispatchResult
		where
			F: Fn(&T::AccountId, &T::KittyIndex) -> Event<T>, //TODO! to emit kitty reference
		{
			let kitty_id = match Self::next_kitty_id() {
				Some(id) => {
					ensure!(id != T::KittyIndex::max_value(), Error::<T>::KittyIndexOverflow);
					id
				},
				None => 0u32.into(),
			};

			let unit_price = T::KittyUnitPrice::get();
			T::Currency::reserve(&owner, unit_price)
				.map_err(|_| Error::<T>::NotEnoughBalanceForStaking)?;

			let kitty = Kitty(dna);

			Kitties::<T>::insert(kitty_id, &kitty);
			KittyOwner::<T>::insert(kitty_id, owner);
			BelongsKitties::<T>::insert(owner, kitty_id, ());
			NextKittyId::<T>::put(kitty_id + 1u32.into());

			if let Some(event_emitter) = event_to_emit {
				Self::deposit_event(event_emitter(owner, &kitty_id));
			}
			Ok(())
		}
	}
}
