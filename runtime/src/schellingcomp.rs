//! # Example Module
//!
//! <!-- Original author of paragraph: @gavofyork --> 
//! The Example: A simple example of a runtime module demonstrating
//! concepts, APIs and structures common to most runtime modules.
//!
//! Run `cargo doc --package srml-example --open` to view this module's documentation.
//!
//! ### Documentation Guidelines:
//!
//! <!-- Original author of paragraph: Various. Based on collation of review comments to PRs addressing issues with -->
//! <!-- label 'S3-SRML' in https://github.com/paritytech/substrate-developer-hub/issues -->
//! <ul>
//!		<li>Documentation comments (i.e. <code>/// comment</code>) - should accompany module functions and be
//!         restricted to the module interface, not the internals of the module implementation. Only state inputs,
//!         outputs, and a brief description that mentions whether calling it requires root, but without repeating
//!         the source code details. Capitalise the first word of each documentation comment and end it with a full
//!         stop. See <a href="https://github.com/paritytech/substrate#72-contributing-to-documentation-for-substrate-packages"
//!         target="_blank">Generic example of annotating source code with documentation comments</a></li>
//! 	<li>Self-documenting code - Try to refactor code to be self-documenting.</li>
//!		<li>Code comments - Supplement complex code with a brief explanation, not every line of code.</li>
//!		<li>Identifiers - surround by backticks (i.e. <code>INHERENT_IDENTIFIER</code>, <code>InherentType</code>,
//!         <code>u64</code>)</li>
//!		<li>Usage scenarios - should be simple doctests. The compiler should ensure they stay valid.</li>
//!		<li>Extended tutorials - should be moved to external files and refer to.</li>
//!		<!-- Original author of paragraph: @AmarRSingh -->
//!		<li>Mandatory - include all of the sections/subsections where <b>MUST</b> is specified.</li>
//!		<li>Optional - optionally include sections/subsections where <b>CAN</b> is specified.</li>
//! </ul>
//!
//! ### Documentation Template:<br>
//!
//! Copy and paste this template from srml/example/src/lib.rs into file srml/<INSERT_CUSTOM_MODULE_NAME>/src/lib.rs of
//! your own custom module and complete it.
//! <details><p><pre>
//! // Add heading with custom module name
//!
//! \# <INSERT_CUSTOM_MODULE_NAME> Module
//!
//! // Add simple description
//!
//! // Include the following links that shows what trait needs to be implemented to use the module
//! // and the supported dispatchables that are documented in the Call enum.
//!
//! - \[`<INSERT_CUSTOM_MODULE_NAME>::Trait`](./trait.Trait.html)
//! - \[`Call`](./enum.Call.html)
//! - \[`Module`](./struct.Module.html)
//!
//! \## Overview
//!
//! <!-- Original author of paragraph: Various. See https://github.com/paritytech/substrate-developer-hub/issues/44 --> 
//! // Short description of module purpose.
//! // Links to Traits that should be implemented.
//! // What this module is for.
//! // What functionality the module provides.
//! // When to use the module (use case examples).
//! // How it is used.
//! // Inputs it uses and the source of each input.
//! // Outputs it produces.
//!
//! <!-- Original author of paragraph: @Kianenigma in PR https://github.com/paritytech/substrate/pull/1951 -->
//! <!-- and comment https://github.com/paritytech/substrate-developer-hub/issues/44#issuecomment-471982710 -->
//!
//! \## Terminology
//!
//! // Add terminology used in the custom module. Include concepts, storage items, or actions that you think
//! // deserve to be noted to give context to the rest of the documentation or module usage. The author needs to
//! // use some judgment about what is included. We don't want a list of every storage item nor types - the user
//! // can go to the code for that. For example, "transfer fee" is obvious and should not be included, but
//! // "free balance" and "reserved balance" should be noted to give context to the module.
//! // Please do not link to outside resources. The reference docs should be the ultimate source of truth.
//!
//! <!-- Original author of heading: @Kianenigma in PR https://github.com/paritytech/substrate/pull/1951 -->
//!
//! \## Goals
//!
//! // Add goals that the custom module is designed to achieve.
//!
//! <!-- Original author of heading: @Kianenigma in PR https://github.com/paritytech/substrate/pull/1951 -->
//!
//! \### Scenarios
//!
//! <!-- Original author of paragraph: @Kianenigma. Based on PR https://github.com/paritytech/substrate/pull/1951 -->
//!
//! \#### <INSERT_SCENARIO_NAME>
//!
//! // Describe requirements prior to interacting with the custom module.
//! // Describe the process of interacting with the custom module for this scenario and public API functions used.
//!
//! \## Interface
//!
//! \### Supported Origins
//!
//! // What origins are used and supported in this module (root, signed, none)
//! // i.e. root when <code>\`ensure_root\`</code> used
//! // i.e. none when <code>\`ensure_none\`</code> used
//! // i.e. signed when <code>\`ensure_signed\`</code> used
//!
//! <code>\`inherent\`</code> <INSERT_DESCRIPTION>
//!
//! <!-- Original author of paragraph: @Kianenigma in comment -->
//! <!-- https://github.com/paritytech/substrate-developer-hub/issues/44#issuecomment-471982710 -->
//!
//! \### Types
//!
//! // Type aliases. Include any associated types and where the user would typically define them.
//!
//! <code>\`ExampleType\`</code> <INSERT_DESCRIPTION>
//!
//! <!-- Original author of paragraph: ??? -->
//!
//! // Reference documentation of aspects such as `storageItems` and `dispatchable` functions should only be
//! // included in the https://docs.rs Rustdocs for Substrate and not repeated in the README file.
//!
//! \### Dispatchable Functions
//!
//! <!-- Original author of paragraph: @AmarRSingh & @joepetrowski -->
//!
//! // A brief description of dispatchable functions and a link to the rustdoc with their actual documentation.
//!
//! // <b>MUST</b> have link to Call enum
//! // <b>MUST</b> have origin information included in function doc
//! // <b>CAN</b> have more info up to the user
//!
//! \### Public Functions
//!
//! <!-- Original author of paragraph: @joepetrowski -->
//!
//! // A link to the rustdoc and any notes about usage in the module, not for specific functions.
//! // For example, in the balances module: "Note that when using the publicly exposed functions,
//! // you (the runtime developer) are responsible for implementing any necessary checks
//! // (e.g. that the sender is the signer) before calling a function that will affect storage."
//!
//! <!-- Original author of paragraph: @AmarRSingh -->
//!
//! // It is up to the writer of the respective module (with respect to how much information to provide).
//!
//! \#### Public Inspection functions - Immutable (getters)
//!
//! // Insert a subheading for each getter function signature
//!
//! \##### <code>\`example_getter_name()\`</code>
//!
//! // What it returns
//! // Why, when, and how often to call it
//! // When it could panic or error
//! // When safety issues to consider
//!
//! \#### Public Mutable functions (changing state)
//!
//! // Insert a subheading for each setter function signature
//!
//! \##### <code>\`example_setter_name(origin, parameter_name: T::ExampleType)\`</code>
//!
//! // What state it changes
//! // Why, when, and how often to call it
//! // When it could panic or error
//! // When safety issues to consider
//! // What parameter values are valid and why
//!
//! \### Storage Items
//!
//! // Explain any storage items included in this module
//!
//! \### Digest Items
//!
//! // Explain any digest items included in this module
//!
//! \### Inherent Data
//!
//! // Explain what inherent data (if any) is defined in the module and any other related types
//!
//! \### Events:
//!
//! // Insert events for this module if any
//!
//! \### Errors:
//!
//! // Explain what generates errors
//!
//! \## Usage
//!
//! // Insert 2-3 examples of usage and code snippets that show how to use <INSERT_CUSTOM_MODULE_NAME> module in a custom module.
//!
//! \### Prerequisites
//!
//! // Show how to include necessary imports for <INSERT_CUSTOM_MODULE_NAME> and derive
//! // your module configuration trait with the `INSERT_CUSTOM_MODULE_NAME` trait.
//!
//! \```rust
//! use <INSERT_CUSTOM_MODULE_NAME>;
//! 
//! pub trait Trait: <INSERT_CUSTOM_MODULE_NAME>::Trait { }
//! \```
//!
//! \### Simple Code Snippet
//!
//! // Show a simple example (e.g. how to query a public getter function of <INSERT_CUSTOM_MODULE_NAME>)
//!
//! \### Example from SRML
//!
//! // Show a usage example in an actual runtime
//!
//! // See:
//! // - Substrate TCR https://github.com/parity-samples/substrate-tcr
//! // - Substrate Kitties https://shawntabrizi.github.io/substrate-collectables-workshop/#/
//!
//! \## Genesis Config
//!
//! <!-- Original author of paragraph: @joepetrowski -->
//!
//! \## Dependencies
//!
//! // Dependencies on other SRML modules and the genesis config should be mentioned,
//! // but not the Rust Standard Library.
//! // Genesis configuration modifications that may be made to incorporate this module
//! // Interaction with other modules
//!
//! <!-- Original author of heading: @AmarRSingh -->
//!
//! \## Related Modules
//!
//! // Interaction with other modules in the form of a bullet point list
//!
//! \## References
//!
//! <!-- Original author of paragraph: @joepetrowski -->
//!
//! // Links to reference material, if applicable. For example, Phragmen, W3F research, etc.
//! // that the implementation is based on.
//! </pre></p></details>

use support::{decl_module, decl_storage, decl_event, StorageValue, dispatch::Result};
use system::ensure_signed;

/// The module's configuration trait.
pub trait Trait: system::Trait {
	// TODO: Add other types and constants required configure this module.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

/// This module's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as Schellingcomp {
		// Just a dummy storage item. 
		// Here we are declaring a StorageValue, `Something` as a Option<u32>
		// `get(something)` is the default getter which returns either the stored `u32` or `None` if nothing stored
		Something get(something): Option<u32>;
	}
}

decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing events
		// this is needed only if you are using events in your module
		fn deposit_event<T>() = default;

		// Just a dummy entry point.
		// function that can be called by the external world as an extrinsics call
		// takes a parameter of the type `AccountId`, stores it and emits an event
		pub fn do_something(origin, something: u32) -> Result {
			// TODO: You only need this if you want to check it was signed.
			let who = ensure_signed(origin)?;

			// TODO: Code to execute when something calls this.
			// For example: the following line stores the passed in u32 in the storage
			<Something<T>>::put(something);

			// here we are raising the Something event
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			Ok(())
		}
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		// Just a dummy event.
		// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		// To emit this event, we call the deposit funtion, from our runtime funtions
		SomethingStored(u32, AccountId),
	}
);

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
			assert_ok!(Schellingcomp::do_something(Origin::signed(1), 42));
			// asserting that the stored value is equal to what we stored
			assert_eq!(Schellingcomp::something(), Some(42));
		});
	}
}
