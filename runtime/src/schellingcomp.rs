//! # Schellingcomp Module
//!
//! The `schellingcomp` Module allows offloading non-verifiable computations to untrusted nodes.
//!
//! - [`<schelingcomp>::Trait`](./trait.Trait.html)
//! - [`Call`](./enum.Call.html)
//! - [`Module`](./struct.Module.html)
//!
//! ## Overview
//!
//! This module allows nodes to register themselves as so called "clients" by paying
//! a security deposit. Then anyone can become an "owner" of a computation by offloading a computation to some of these clients. Because
//! the outcomes of these computations are not easily verifiable a mechansim based on
//! Schelling Points [as described by Vitalik Buterin](https://blog.ethereum.org/2014/03/28/schellingcoin-a-minimal-trust-universal-data-feed/)
//! is used to reward or punish the clients for their work.
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//! 
//! - `configure` - Only callable as the `Admin` origin. Configures the parameters of this Module. 
//! - `register` - Add the origin as a a client by depositing some currency.
//! - `unregister`- Remove the origin as a client by withdrawing the deposit.
//! - `offload` - Offload a computation to an arbitrary number of clients. Charges `Reward * client_clount` for the computation.
//! - `commit` - Commit to a an outcome as a client. The `commitment` is the hash of `origin_id | computation_id | outcome`.
//! - `reveal` - Reveal the previously commited to outcome. Can only be called after all clients commited or the time is up.
//! - `finish` - Callable by the cowner or any client to trigger reward calculation after each client revealed or the time is up.
//!
//! Please refer to the [`Call`](./enum.Call.html) enum and its associated variants for documentation on each function.
//!
//! ### Public Functions
//!
//! See the [module](./struct.Module.html) for details on publicly available functions.

use support::{
	decl_module, decl_storage, decl_event, StorageValue, StorageMap, dispatch::Result,
	ensure, traits::Currency, traits::ReservableCurrency, dispatch::Vec, dispatch::Parameter,
	traits::OnUnbalanced
};
use system::ensure_signed;
use parity_codec::{Encode, Decode};
use runtime_primitives::traits::{As, EnsureOrigin, Hash};
use core::convert::TryInto;
use core::mem::size_of;
use rand::{Rng, SeedableRng, rngs::SmallRng, distributions::Uniform};

type ClientIndex = u64;
type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;
type NegativeImbalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::NegativeImbalance;
type ClientOf<T> = Client<BalanceOf<T>>;
type ComputationOf<T> = Computation<
	<T as system::Trait>::Hash,
	<T as Trait>::Task,
	BalanceOf<T>,
	<T as system::Trait>::AccountId,
	<T as timestamp::Trait>::Moment,
	<T as Trait>::Outcome
>;

/// Static configuration of the schellingcomp module.
pub trait Trait: balances::Trait + timestamp::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	/// The currency in which computation power should be payed.
	type Currency: ReservableCurrency<Self::AccountId>;
	/// The computational task that should be offloaded. If its more complex than a number
	/// it should probably be an IPFS hash. In any case it should be immuteable so that
	/// all clients execute the same task.
	type Task: Parameter + Default;
	/// The result that is calculated by carrying out the task.
	type Outcome: Parameter + Default;
	/// The origin that is allowed configure rewards, deposits and time limits.
	type Admin: EnsureOrigin<Self::Origin>;
	/// Functor that is responsible for shelling out the reward.
	type Reward: OnReward<Self::AccountId, Self::Outcome, BalanceOf<Self>>;
	/// Called when a client deposit is slashed.
	type Slash: OnUnbalanced<NegativeImbalanceOf<Self>>;
}

/// Handler for when a computation is finished.
pub trait OnReward<AccountId, Outcome, Balance> {
	/// Handler that distributes rewards and determines the canonical result (if any).
	/// This function must do the actual transfer of reserved currency from the `owner` to
	/// eligible clients. Slashing deposits of `good_clients` that deliver bad results is not
	/// yet supported. However, all `good_clients` stayed true to their commitments.
	fn on_reward(good_clients: Vec<(AccountId, Outcome)>, reward: Balance, owner: AccountId) -> Option<Outcome>;
}

decl_event!(
	pub enum Event<T> where
		AccountId = <T as system::Trait>::AccountId,
		Hash = <T as system::Trait>::Hash,
		Outcome = <T as Trait>::Outcome,
	{
		ConfigurationChanged,
		ClientRegistered(AccountId),
		ClientUnregistered(AccountId),
		TaskOffloaded(Hash),
		ClientCommited(Hash, AccountId),
		ClientRevealed(Hash, AccountId),
		ComputationFinished(Hash, Option<Outcome>),
	}
);

decl_storage! {
	trait Store for Module<T: Trait> as Schellingcomp {
		Reward get(reward): BalanceOf<T>;
		Deposit get(deposit): BalanceOf<T>;
		TimelimitCommit get(timelimit_commit): T::Moment;
		TimelimitReveal get(timelimit_reveal): T::Moment;

		Computations: map T::Hash => ComputationOf<T>;
		Clients: map T::AccountId => ClientOf<T>;
		ClientCount get(client_count): ClientIndex;

		AvailableClientsArray: map ClientIndex => T::AccountId;
		AvailableClientsCount get(available_clients): ClientIndex;
		AvailableClientsIndex: map T::AccountId => ClientIndex;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event<T>() = default;

		// Set dynamic config as Admin origin.
		fn configure(origin, reward: BalanceOf<T>, deposit: BalanceOf<T>, timelimit_commit: T::Moment, timelimit_reveal: T::Moment) {
			T::Admin::ensure_origin(origin)?;

			<Reward<T>>::put(&reward);
			<Deposit<T>>::put(&deposit);
			<TimelimitCommit<T>>::put(&timelimit_reveal);
			<TimelimitReveal<T>>::put(&timelimit_commit);

			Self::deposit_event(RawEvent::ConfigurationChanged);
		}

		// Register origin as a client to offer computation power.
		fn register(origin) {
			let sender = ensure_signed(origin)?;

			ensure!(!<Clients<T>>::exists(&sender), "Client is already registered");

			let total_clients = Self::client_count().checked_add(1)
                .ok_or("Maximum number of clients reached")?;

			T::Currency::reserve(&sender, Self::deposit())
				.map_err(|_| "Client's balance too low")?;

			let client = Client {
				deposit: Self::deposit(),
			};

			<Clients<T>>::insert(&sender, &client);
			<ClientCount<T>>::put(total_clients);
			Self::add_available(&sender)
				.expect("`ClientCount` is already incremented to the count of all clients, \
				`AvailableClientsCount` has the same type as `ClientCount`, \
				`AvailableClientsCount` <= `ClientsCount - 1`, \
				therefore no overflow of `AvailableClientsCount` can happen; \
				`client` not in `Clients` -> `client` not in `AvailableClientsIndex`; \
				qed");

			Self::deposit_event(RawEvent::ClientRegistered(sender));
		}

		// Remove origin as client to withdraw computation power.
		fn unregister(origin) {
			let sender = ensure_signed(origin)?;
			ensure!(<Clients<T>>::exists(&sender), "Client is not registered");
			ensure!(<AvailableClientsIndex<T>>::exists(&sender), "Client must not be busy");

			let client = <Clients<T>>::get(&sender);

			Self::remove_available(&sender)?;
			T::Currency::unreserve(&sender, client.deposit);
			Self::remove(&sender)
				.expect("We checked that the `sender` is registered, \
				therefore the ::exists check will not fail; \
				In `register_client` the `ClientCount` is incremented, \
				it is nowhere decremented but in `Self::remove`, \
				therefore the substraction will not underflow; \
				qed");
		}

		// Offload a task to a random set of clients. Origin is charged based on `client_count`
		// and the static `Reward`.
		fn offload(origin, task: T::Task, client_count: ClientIndex) {
			let sender = ensure_signed(origin)?;
			let reward: BalanceOf<T> = Self::reward().as_().checked_mul(client_count)
				.map(As::sa)
				.ok_or("Reward calculation overflow")?;

			ensure!(T::Currency::can_reserve(&sender, reward), "Not enough balance");

			ensure!(client_count > 0, "Must offload to at least one client");
			ensure!(client_count <= Self::available_clients(), "Not enough clients available.");
			let client_count: usize = client_count.try_into().map_err(|_| "Too many clients")?;

			// Currently, one origin can only offload one computation per block. We should probably
			// include some nonce in the hash so this limitation is lifted.
			let random_hash = (<system::Module<T>>::random_seed(), &sender)
				.using_encoded(T::Hashing::hash);
			ensure!(!<Computations<T>>::exists(&random_hash), "This computation already exists.");

			type Seed = <SmallRng as SeedableRng>::Seed;
			let seed_slice = random_hash.as_ref().get(..size_of::<Seed>())
				.ok_or("Seed too small for chosen prng.")?;
			let seed: &<SmallRng as SeedableRng>::Seed = seed_slice.try_into()
				.map_err(|_| "Failed to convert to prng seed")?;

			// Alloc Vecs so that they cannot panic later on
			let mut clients = Vec::with_capacity(client_count);
			let mut indices = Vec::with_capacity(client_count);

			// Infallible from here here on

			// Draw clients randomly from available ones.
			// We know that this is bounded because we checked that there are enough
			// clients available. However, in a production environment this probably should
			// be handled differently.
			let mut prng = SmallRng::from_seed(*seed);
			let distribution = Uniform::new(0, Self::available_clients());
			for rng in prng.sample_iter::<ClientIndex, _>(&distribution) {
				if indices.contains(&rng) {
					continue;
				}
				let id = <AvailableClientsArray<T>>::get(&rng);
				Self::remove_available(&id)
					.expect("`id is pulled from the AvailableClientsArray, \
					therefore the availibilty check will not fail; \
					qed");
				indices.push(rng);
				clients.push(BusyClient {
					id,
					commit: None,
					reveal: None
				});
				if clients.len() == client_count {
					break;
				}
			}

			T::Currency::reserve(&sender, reward).map_err(|_| "Not enough balance.")
				.expect("Balance was checked early, \
				No changes to balance in between; \
				qed");

			let computation: ComputationOf<T> = Computation {
				owner: sender,
				task,
				reward,
				started_at: <timestamp::Module<T>>::get(),
				reveal_started_at: None,
				timelimit_commit: Self::timelimit_commit(),
				timelimit_reveal: Self::timelimit_reveal(),
				clients,
			};

			<Computations<T>>::insert(&random_hash, computation);

			Self::deposit_event(RawEvent::TaskOffloaded(random_hash));
		}

		// Commit to a value as a client.
		fn commit(origin, id: T::Hash, commitment: T::Hash) {
			let sender = ensure_signed(origin)?;

			ensure!(<Computations<T>>::exists(&id), "Computation does not exist.");
			let mut computation = <Computations<T>>::get(&id);
			let client = computation.clients.iter_mut().find(|x| x.id == sender)
				.ok_or("Client is not part of this computation")?;
			ensure!(computation.reveal_started_at.is_none(), "Reveal phase did start.");

			// Will not panic as clients and commits have the same size
			ensure!(client.commit.is_none(), "Client already committed.");
			client.commit = Some(commitment);
			<Computations<T>>::insert(&id, computation);

			Self::deposit_event(RawEvent::ClientCommited(id, sender));
		}

		// Reveal the outcome previously commited to.
		fn reveal(origin, id: T::Hash, revelation: T::Outcome) {
			let sender = ensure_signed(origin)?;

			ensure!(<Computations<T>>::exists(&id), "Computation does not exist.");
			let mut computation = <Computations<T>>::get(&id);

			ensure!(computation.clients.iter().find(|x| x.id == sender).is_some(),
				"Client is not part of this computation");

			// this reveal might finish the commit phase
			if computation.reveal_started_at.is_none() {
				// timestamp is no user input -> checked_sub not necessary
				let now = <timestamp::Module<T>>::get();
				let time_is_up = (now.clone() - computation.started_at.clone())
					>= Self::timelimit_commit();
				let all_commited = computation.clients.iter()
					.filter(|x| x.commit.is_some()).count() == computation.clients.len();
				ensure!(all_commited || time_is_up, "Reveal phase cannot be started, yet.");
				computation.reveal_started_at = Some(now);
			}

			let client = computation.clients.iter_mut().find(|x| x.id == sender)
				.expect("We checked that the client exists at the beginning, \
				we had no mutable borrow to the clients in between; \
				qed");

			let commit = client.commit.ok_or("Client did not commit.")?;
			ensure!(client.reveal.is_none(), "Client already revealed.");
			let is_valid = (&sender, &id, &revelation)
				.using_encoded(T::Hashing::hash) == commit;
			ensure!(is_valid, "Reveal does not match the commit.");

			client.reveal = Some(revelation);
			<Computations<T>>::insert(&id, computation);

			Self::deposit_event(RawEvent::ClientRevealed(id, sender));
		}

		// Wrap up a finished computation. This distributes the reward among the clients
		// and emits an event informing all nodes about the canonical outcome.
		fn finish(origin, id: T::Hash) {
			let sender = ensure_signed(origin)?;

			ensure!(<Computations<T>>::exists(&id), "Computation does not exist.");
			let computation = <Computations<T>>::get(&id);

			ensure!(
				computation.clients.iter().find(|x| x.id == sender).is_some() ||
				computation.owner == sender,
				"Only involved clients and the owner is allowed to finish."
			);

			// timestamps are no user input -> checked_sub not necessary
			let now = <timestamp::Module<T>>::get();
			let time_up_since_reveal = computation.reveal_started_at.and_then(|x| {
				Some((now.clone() - x) >= Self::timelimit_reveal())
			}).unwrap_or(false);
			let time_up_since_start = (now.clone() - computation.started_at)
					>= Self::timelimit_commit() + Self::timelimit_reveal();
			let all_revealed = computation.clients.iter()
				.filter(|x| x.reveal.is_some()).count() == computation.clients.len();
			ensure!(time_up_since_reveal || time_up_since_start || all_revealed,
				"Computation cannot be finished, yet.");

			let mut result = Vec::with_capacity(computation.clients.len());

			// Infallible from here on

			for client in computation.clients.into_iter() {
				if let Some(outcome) = client.reveal {
					result.push((client.id, outcome));
					Self::add_available(&sender)
						.expect("`AvailableClientsCount` <= `ClientsCount - 1`,
						because at least this client is not available, \
						therefore no overflow of `AvailableClientsCount` can happen; \
						`client` not in `AvailableClientsIndex` because it is part of this comp; \
						qed");
				} else {
					let deposit = <Clients<T>>::get(&client.id).deposit;
					let (imbalance, _) = T::Currency::slash_reserved(&client.id, deposit);
					T::Slash::on_unbalanced(imbalance);
					Self::remove(&client.id)
						.expect("`client` in `computations.clients` -> `client` is registered, \
						therefore ::exists does not fail; \
						because at least one client exists the substraction does not underflow; \
						qed");
				}
			}

			<Computations<T>>::remove(&id);
			let outcome = T::Reward::on_reward(result, computation.reward, computation.owner);

			Self::deposit_event(RawEvent::ComputationFinished(id, outcome));
		}
	}
}

impl<T: Trait> Module<T> {
	// Move an account from busy to available.
	fn add_available(client: &T::AccountId) -> Result {
		let available = Self::available_clients();
		let available_add = available.checked_add(1).ok_or("Available index overflow.")?;

		ensure!(!<AvailableClientsIndex<T>>::exists(client), "Client is already available.");

		<AvailableClientsArray<T>>::insert(available, client);
		<AvailableClientsCount<T>>::put(available_add);
		<AvailableClientsIndex<T>>::insert(client, available);

		Ok(())
	}

	// Move an account from available to busy.
	fn remove_available(client: &T::AccountId) -> Result {
		ensure!(<AvailableClientsIndex<T>>::exists(client), "Client is not available.");
		let index = <AvailableClientsIndex<T>>::get(client);
		let available = Self::available_clients();
		let index_tail = available.checked_sub(1).ok_or("Available index underflow.")
			.expect("`client`is contained in `AvailableClientsIndex`, \
			therefore `available` is at least 1; \
			qed");

		// swap tail with to be removed client
		if index != index_tail {
			let last_client = <AvailableClientsArray<T>>::get(index_tail);
			<AvailableClientsArray<T>>::insert(&index, &last_client);
			<AvailableClientsIndex<T>>::insert(last_client, &index);
		}

		<AvailableClientsArray<T>>::remove(index_tail);
		<AvailableClientsCount<T>>::put(index_tail);
		<AvailableClientsIndex<T>>::remove(client);

		Ok(())
	}

	// Remove an account so that it is no longer a client.
	fn remove(id: &T::AccountId) -> Result {
		ensure!(<Clients<T>>::exists(id), "Client is not registered");

		let total_clients = Self::client_count().checked_sub(1)
			.ok_or("Client number underflow")?;

		<ClientCount<T>>::put(total_clients);
		<Clients<T>>::remove(id);

		Self::deposit_event(RawEvent::ClientUnregistered(id.clone()));
		Ok(())
	}
}

/// An ongoing computation.
#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Default)]
struct Computation<Hash, Task, Balance, AccountId, Moment, Outcome>  {
	/// The account that offloaded this computation and pays for it.
	owner: AccountId,
	/// The assignment that is calculated by this computation.
	task: Task,
	/// The reward that is payed to the clients when the computation is finished.
	reward: Balance,
	/// Timestamp when the computation was started.
	started_at: Moment,
	/// Timestamp when reveal phase was started.
	reveal_started_at: Option<Moment>,
	/// Configurable time limit for the commit phase.
	timelimit_commit: Moment,
	/// Configurable timelimit for the reveal phase.
	timelimit_reveal: Moment,
	/// The clients that were selected to carry out the computation. Having this as a
	/// vector is sensible because the expected number of clients is low. Owner is paying for
	/// every entry.
	clients: Vec<BusyClient<AccountId, Hash, Outcome>>,
}

/// A client that is busy crunching numbers.
#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Clone, Default, PartialEq, Eq)]
struct BusyClient<AccountId, Hash, Outcome> {
	/// Account of this client.
	id: AccountId,
	/// The outcome the client commited to.
	commit: Option<Hash>,
	/// The reveal of the outcome previously commited to.
	reveal: Option<Outcome>,
}

/// A client that signed up to do some computation.
#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Clone, Default, PartialEq, Eq)]
struct Client<Balance>  {
	/// The deposit the client payed on registering.
	deposit: Balance,
}

/// tests for this module
#[cfg(test)]
mod tests {
	use super::*;

	use runtime_io::with_externalities;
	use primitives::{H256, Blake2Hasher};
	use support::{impl_outer_origin, assert_ok};
	use runtime_primitives::{
		BuildStorage,
		traits::{BlakeTwo256, IdentityLookup},
		testing::{Digest, DigestItem, Header}
	};

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	// For testing the module, we construct most of a mock runtime. This means
	// first constructing a configuration type (`Test`) which `impl`s each of the
	// configuration traits of modules we want to use.
	#[derive(Clone, Eq, PartialEq)]
	pub struct Test;
	impl system::Trait for Test {
		type Origin = Origin;
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type Digest = Digest;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = ();
		type Log = DigestItem;
	}
	impl Trait for Test {
		type Event = ();
	}
	type Schellingcomp = Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
		system::GenesisConfig::<Test>::default().build_storage().unwrap().0.into()
	}

	#[test]
	fn it_works_for_default_value() {
		with_externalities(&mut new_test_ext(), || {
			// Just a dummy test for the dummy funtion `do_something`
			// calling the `do_something` function with a value 42
			//assert_ok!(Schellingcomp::do_something(Origin::signed(1), 42));
			// asserting that the stored value is equal to what we stored
			//assert_eq!(Schellingcomp::something(), Some(42));
		});
	}
}
