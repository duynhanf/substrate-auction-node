#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage

	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type HighestPrice<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn highest_bidder)]
	pub type HighestBidder<T: Config> = StorageValue<_, T::AccountId>;

	#[pallet::storage]
	#[pallet::getter(fn is_started)]
	pub type IsStarted<T> = StorageValue<_, bool>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		// MyStorageStored(u32, T::AccountId),
		AuctionStarted(u32, T::AccountId),
		HighestPriceUpdated(u32, T::AccountId),
		AuctionEnded(u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// Lỗi này trả ra khi có người nào đó đã trả cao hơn
		PaidLower,
		/// aaaa
		AuctionNotStartedOrEnded,
		//
		AuctionStarted,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// /// An example dispatchable that takes a singles value as a parameter, writes the value to
		// /// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn bid(origin: OriginFor<T>, bid_price: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let is_auction_started = <IsStarted<T>>::get();

			if is_auction_started.is_none() {
				Err(Error::<T>::AuctionNotStartedOrEnded)?
			}

			if is_auction_started.unwrap() {
				match <HighestPrice<T>>::get() {
					None => {
						<HighestPrice<T>>::put(bid_price);
						Ok(())
					},
					Some(old) => {
						if bid_price > old {
							<HighestPrice<T>>::put(bid_price);

							// Update highest bidder
							<HighestBidder<T>>::put(who);

							let highest_bidder = <HighestBidder<T>>::get().unwrap();

							// Trả ra event HighestPriceUpdated
							Self::deposit_event(Event::HighestPriceUpdated(
								bid_price,
								highest_bidder,
							));
							Ok(())
						} else {
							Err(Error::<T>::PaidLower)?
						}
					},
				}
			} else {
				Err(Error::<T>::AuctionNotStartedOrEnded)?
			}
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn start(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Bắt đầu cho phép đấu giá
			<IsStarted<T>>::put(true);

			// Bắt đầu với giá cao nhất = 1000
			<HighestPrice<T>>::put(1000);

			// Trả ra 1 event tên là AuctionStarted
			Self::deposit_event(Event::AuctionStarted(0, who));
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn end(origin: OriginFor<T>) -> DispatchResult {
			// Dừng cuộc đấu giá
			<IsStarted<T>>::put(false);

			let highest_bidder = <HighestBidder<T>>::get().unwrap();
			let highest_price = <HighestPrice<T>>::get().unwrap();

			// Trả ra 1 event tên là AuctionEnded
			Self::deposit_event(Event::AuctionEnded(highest_price, highest_bidder));
			Ok(())
		}

		// /// An example dispatchable that may throw a custom error.
		// #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		// pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
		// 	let _who = ensure_signed(origin)?;

		// 	// Read a value from storage.
		// 	match <MyStorage<T>>::get() {
		// 		// Return an error if the value has not been set.
		// 		None => Err(Error::<T>::NoneValue)?,
		// 		Some(old) => {
		// 			// Increment the value read from storage; will error in the event of overflow.
		// 			let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
		// 			// Update the value in storage with the incremented result.
		// 			<MyStorage<T>>::put(new);
		// 			Ok(())
		// 		},
		// 	}
		// }
	}
}
