use frame_support::{
	traits::{Currency, OnRuntimeUpgrade},
	weights::Weight,
};
use pns_registrar::{migration, origin, price_oracle, registry};
use sp_std::marker::PhantomData;

pub struct PnsPalletsInitialize<T>(PhantomData<T>);

impl<T> OnRuntimeUpgrade for PnsPalletsInitialize<T>
where
	T: price_oracle::Config + registry::Config + origin::Config,
	T::AccountId: From<crate::AccountId>,
	<<T as pns_registrar::price_oracle::Config>::Currency as Currency<
		<T as frame_system::Config>::AccountId,
	>>::Balance: From<u32>,
	T::Hash: From<[u8; 32]>,
{
	fn on_runtime_upgrade() -> Weight {
		let mut weight = 50_000_000;

		let managers = sp_std::vec![
			T::AccountId::from(crate::AccountId::new([
				58, 134, 235, 121, 229, 191, 129, 169, 105, 74, 230, 244, 227, 158, 50, 233, 231,
				165, 27, 176, 43, 12, 142, 67, 84, 241, 64, 102, 161, 36, 125, 24
			])),
			T::AccountId::from(crate::AccountId::new([
				12, 48, 62, 83, 87, 189, 197, 0, 176, 151, 38, 193, 196, 72, 0, 43, 240, 136, 85,
				186, 194, 116, 38, 8, 164, 80, 232, 84, 190, 76, 9, 24
			]))
		];
		weight += migration::Initialize::<T>::initial_origin(managers);

		let official = T::AccountId::from(crate::AccountId::new([
			58, 134, 235, 121, 229, 191, 129, 169, 105, 74, 230, 244, 227, 158, 50, 233, 231, 165,
			27, 176, 43, 12, 142, 67, 84, 241, 64, 102, 161, 36, 125, 24,
		]));

		// ksm : [40, 176, 66, 80, 226, 106, 137, 121, 141, 170, 194, 128, 195, 181, 31, 184, 186,
		// 190, 216, 60, 185, 180,141, 134, 171, 252, 4, 74, 2, 250, 3, 144]
		let root_domain = T::Hash::from([
			40, 176, 66, 80, 226, 106, 137, 121, 141, 170, 194, 128, 195, 181, 31, 184, 186, 190,
			216, 60, 185, 180, 141, 134, 171, 252, 4, 74, 2, 250, 3, 144,
		]);

		weight += migration::Initialize::<T>::initial_registry(official, root_domain);

		// TODO: 价格设置过低无法正常交易
		let base_prices = [11.into(); 11];
		let rent_prices = [11.into(); 11];
		let deposit_prices = [11.into(); 11];
		let init_rate = 1_000.into();

		weight += migration::Initialize::<T>::initial_price_oracle(
			base_prices,
			rent_prices,
			deposit_prices,
			init_rate,
		);

		weight
	}
}
