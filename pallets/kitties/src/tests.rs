use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok, traits::Currency};

#[test]
fn create_kitty_should_work() {
	new_test_ext().execute_with(|| {
		let dna1 = vec![1];
		System::set_block_number(1);
		// create a kitty by 1 
		assert_ok!(KittiesModule::create_kitty(RuntimeOrigin::signed(1),dna1.clone() ));

		System::assert_last_event(Event::Created { kitty: dna1.clone(), owner: 1 }.into());
		// check onchain storage có như mình mong muốn hay ko 

		let current_id =KittiesModule::kitty_id();
		assert_eq!(current_id, 1);
		assert!(KittiesModule::get_kitty(dna1.clone()).is_some());
		let kitty = KittiesModule::get_kitty(dna1.clone()).unwrap();
		assert_eq!(kitty.owner, 1);
		let kitties_owned = KittiesModule::kitty_owned(1);
		assert_eq!(kitties_owned, vec![dna1.clone()]);

		//create a kitty by 2
		let dna2 = vec![2];
		assert_ok!(KittiesModule::create_kitty(RuntimeOrigin::signed(2),dna2.clone() ));
		let current_id =KittiesModule::kitty_id();
		assert_eq!(current_id, 2);

		assert!(KittiesModule::get_kitty(dna2.clone()).is_some());
		let kitty = KittiesModule::get_kitty(dna2.clone()).unwrap();
		assert_eq!(kitty.owner, 2);

		//create a kitty by 1
		let dna3 = vec![3];
		assert_ok!(KittiesModule::create_kitty(RuntimeOrigin::signed(1),dna3.clone() ));
		let kitties_owned = KittiesModule::kitty_owned(1);
		assert_eq!(kitties_owned, vec![dna1, dna3]);

	});
}

#[test]
fn set_price_should_work() {
	new_test_ext().execute_with(|| {
		let dna1 = vec![1];
		assert_ok!(KittiesModule::create_kitty(RuntimeOrigin::signed(1),dna1.clone() ));
		assert!(KittiesModule::get_kitty(dna1.clone()).is_some());
		let kitty = KittiesModule::get_kitty(dna1.clone()).unwrap();
		assert_eq!(kitty.price, None);

		assert_ok!(KittiesModule::set_price(RuntimeOrigin::signed(1),dna1.clone(), Some(100) ));
		let kitty = KittiesModule::get_kitty(dna1.clone()).unwrap();
		assert_eq!(kitty.price, Some(100));
	});
}

#[test]
fn buy_kitty_should_work() {
	new_test_ext().execute_with(|| {
		// first create kitty
		let dna1 = vec![1];
		System::set_block_number(1);

		assert_ok!(KittiesModule::create_kitty(RuntimeOrigin::signed(1),dna1.clone() ));
		assert!(KittiesModule::get_kitty(dna1.clone()).is_some());
		let kitty = KittiesModule::get_kitty(dna1.clone()).unwrap();
		assert_eq!(kitty.price, None);

		assert_ok!(KittiesModule::set_price(RuntimeOrigin::signed(1),dna1.clone(), Some(100) ));
		let kitty = KittiesModule::get_kitty(dna1.clone()).unwrap();
		
		assert_eq!(kitty.price, Some(100), "wrong price");
		assert_eq!(kitty.dna.clone(), dna1.clone(), "wrong dna");
		assert_ne!(kitty.owner, 2, "owner can't buy");

		assert_ok!(KittiesModule::buy_kitty(RuntimeOrigin::signed(5), dna1.clone(), 100));

		let balance_1 = Balances::free_balance(&1);
		let balance_5 = Balances::free_balance(&5);

		assert_eq!(balance_1, 1100, "wrong balance");
		assert_eq!(balance_5, 900, "wrong balance");
		
		System::assert_last_event(Event::Transferred { from: 1,to: 5, kitty: dna1.clone() }.into());

		let new_kitty = KittiesModule::get_kitty(dna1.clone()).unwrap();

		assert_eq!(new_kitty.price, None, "wrong price");
		assert_eq!(new_kitty.owner, 5, "wrong owner");

	});
}