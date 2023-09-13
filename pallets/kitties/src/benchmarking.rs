//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Kitties;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn create_kitty() {
		let dna1 = vec![3];
		let caller: T::AccountId = whitelisted_caller();
		#[extrinsic_call] // macro
		create_kitty(RawOrigin::Signed(caller), dna1);
	}

	#[benchmark]
	fn set_price() {
	let dna1 = vec![3];

		let caller: T::AccountId = whitelisted_caller();
		#[extrinsic_call] // macro
		set_price(RawOrigin::Signed(caller), dna1, Some(100));
	}

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
