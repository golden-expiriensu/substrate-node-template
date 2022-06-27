#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);
    
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ClaimCreated(T::AccountId, Vec<u8>),
        ClaimRevoked(T::AccountId, Vec<u8>)
    }

    #[pallet::error]
    pub enum Error<T> {
        ProofAlreadyClaimed,
        ProofDoesNotExist,
        NotProofOwner
    }

    #[pallet::storage]
    pub(super) type Proofs<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber), ValueQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(1_000)]
        pub fn claim_proog(
            origin: OriginFor<T>,
            proof: Vec<u8>
        ) -> DispatchResult {
            let sender = ensure_signed(origin);

            ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);

            let current_block = <frame_support::Pallets<T>>::block_number();

            Proofs::<T>::insert(&proof, (&sender, current_block));

            Self::deposit_event(Events::ClaimCreated(sender, proof));

            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn revoke_proof(
            origin: OriginFor<T>,
            proof: Vec<u8>
        ) -> DispatchResult {
            let sender = ensure_signed(origin);

            ensure(Proofs::<T>::contains_key(&proof), Error::<T>::ProofDoesNotExist);
            
            (owner, ) = Proofs::<T>::get(&proof);

            ensure(owner == sender, Error::<T>::NotProofOwner);

            Proofs::<T>::remove(&proof);

            Self::deposit_event(Events::ClaimRevoked(sender, proof));

            Ok(())
        }
    }
}