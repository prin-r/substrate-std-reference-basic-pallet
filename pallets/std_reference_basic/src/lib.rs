#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    dispatch::DispatchResult, ensure, pallet_prelude::*, sp_std::convert::TryInto,
    sp_std::prelude::*, sp_std::str, traits::Time,
};
use frame_system::pallet_prelude::*;
pub use pallet::*;

pub const E9: u64 = 1_000_000_000;
pub const E18: u128 = 1_000_000_000_000_000_000;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Time: Time;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    /// Storages
    #[pallet::storage]
    #[pallet::getter(fn owner)]
    pub type Owner<T: Config> = StorageValue<_, T::AccountId>;

    #[pallet::storage]
    #[pallet::getter(fn relayers)]
    pub type Relayers<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, bool>;

    #[pallet::storage]
    #[pallet::getter(fn refs)]
    pub type Refs<T> = StorageMap<_, Twox64Concat, Vec<u8>, (u64, u64, u64)>;

    /// Events
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// parameters. [new_ower, prev_owner]
        TransferOwnership(T::AccountId, T::AccountId),
        /// parameters. [some_address, is_relayer]
        SetRelayer(T::AccountId, bool),
        /// parameters. [symbol, rate, resolve_times, request_ids]
        RefDataUpdate(Vec<u8>, u64, u64, u64),
    }

    /// Errors
    #[pallet::error]
    pub enum Error<T> {
        /// Errors owner must have been set.
        OwnerNotSet,
        /// Errors sender must be the owner.
        NotAnOwner,
        /// Errors relayer must have been set.
        RelayerNotSet,
        /// Errors sender must be the relayer.
        NotARelayer,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    /// Extrinsics
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000 + T::DbWeight::get().reads(1) + T::DbWeight::get().writes(1))]
        pub fn transfer_ownership(origin: OriginFor<T>, new_owner: T::AccountId) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            let sender = ensure_signed(origin)?;

            // Query the current owner from storage.
            let owner = <Owner<T>>::get().ok_or(Error::<T>::OwnerNotSet)?;

            // Check that sender is the owner.
            ensure!(sender == owner, Error::<T>::NotAnOwner);

            // Update owner.
            <Owner<T>>::put(new_owner.clone());

            // Emit an event.
            Self::deposit_event(Event::TransferOwnership(owner, new_owner));

            // Return a successful DispatchResultWithPostInfo
            Ok(())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().reads(1) + T::DbWeight::get().writes(1))]
        pub fn set_relayer(
            origin: OriginFor<T>,
            relayer: T::AccountId,
            is_relayer: bool,
        ) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            let sender = ensure_signed(origin)?;

            // Query the current owner from storage.
            let owner = <Owner<T>>::get().ok_or(Error::<T>::OwnerNotSet)?;

            // Check that sender is the owner.
            ensure!(sender == owner, Error::<T>::NotAnOwner);

            // Set relayer.
            Relayers::<T>::insert(relayer.clone(), is_relayer);

            // Emit an event.
            Self::deposit_event(Event::SetRelayer(relayer, is_relayer));

            // Return a successful DispatchResultWithPostInfo
            Ok(())
        }

        #[pallet::weight(10_000 + T::DbWeight::get().reads(1) + T::DbWeight::get().writes(values.len() as u64))]
        pub fn relay(
            origin: OriginFor<T>,
            values: Vec<(Vec<u8>, u64, u64, u64)>,
        ) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            let sender = ensure_signed(origin)?;

            // Check that sender is one of the relayers.
            ensure!(
                <Relayers<T>>::get(sender).ok_or(Error::<T>::RelayerNotSet)?,
                Error::<T>::NotARelayer
            );

            // Set values.
            for (symbol, rate, resolve_time, request_id) in values.iter() {
                Refs::<T>::insert(symbol, (rate, resolve_time, request_id));

                // Emit an event.
                Self::deposit_event(Event::RefDataUpdate(
                    symbol.clone(),
                    *rate,
                    *resolve_time,
                    *request_id,
                ));
            }

            // Return a successful DispatchResultWithPostInfo
            Ok(())
        }
    }
}

impl<T: Config> Pallet<T> {
    pub fn get_refs(symbol: Vec<u8>) -> Option<(u64, u64, u64)> {
        Refs::<T>::get(symbol)
    }

    pub fn get_ref_data(symbol: Vec<u8>) -> Option<(u64, u64)> {
        match str::from_utf8(&symbol).ok()? {
            "USD" => {
                let now = TryInto::<u64>::try_into(T::Time::now()).ok()?;
                Some((E9, now))
            }
            _ => {
                let (rate, resolve_time, _request_id) = Self::get_refs(symbol)?;
                Some((rate, resolve_time))
            }
        }
    }

    pub fn get_reference_data(
        base_symbol: Vec<u8>,
        quote_symbol: Vec<u8>,
    ) -> Option<(u64, u64, u64)> {
        let (base_rate, base_timestamp) = Self::get_ref_data(base_symbol)?;
        let (quote_rate, quote_timestamp) = Self::get_ref_data(quote_symbol)?;
        Some((
            (((base_rate as u128) * E18) / quote_rate as u128) as u64,
            base_timestamp,
            quote_timestamp,
        ))
    }

    pub fn get_reference_data_bulk(
        base_quote_symbols: Vec<(Vec<u8>, Vec<u8>)>,
    ) -> Option<Vec<(u64, u64, u64)>> {
        base_quote_symbols
            .iter()
            .map(|(b, q)| Self::get_reference_data(b.clone(), q.clone()))
            .collect()
    }
}
