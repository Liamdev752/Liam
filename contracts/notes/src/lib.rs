#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Symbol};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StorageKey {
    Client,
    Freelancer,
    Amount,
    Token,
    IsCompleted,
}

#[contract]
pub struct BayanihanEscrowContract;

#[contractimpl]
impl BayanihanEscrowContract {
    /// Initializes the secure workspace, binding parties and depositing target funds into escrow.
    pub fn initialize(
        env: Env,
        client: Address,
        freelancer: Address,
        token_id: Address,
        amount: i128,
    ) {
        // Enforce that this transaction is explicitly signed by the funding client
        client.require_auth();

        if env.storage().instance().has(&StorageKey::Client) {
            panic!("Workspace has already been initialized");
        }
        if amount <= 0 {
            panic!("Escrow funding target must be greater than zero");
        }

        // Write configuration into persistent instance storage
        env.storage().instance().set(&StorageKey::Client, &client);
        env.storage().instance().set(&StorageKey::Freelancer, &freelancer);
        env.storage().instance().set(&StorageKey::Token, &token_id);
        env.storage().instance().set(&StorageKey::Amount, &amount);
        env.storage().instance().set(&StorageKey::IsCompleted, &false);

        // Pull down client asset funds into the contract's own balance footprint
        let client_token = token::Client::new(&env, &token_id);
        client_token.transfer(&client, &env.current_contract_address(), &amount);
    }

    /// Releases locked asset reserves directly to the freelancer upon milestone acceptance.
    pub fn release(env: Env) {
        // Fetch values from instance storage
        let client: Address = env.storage().instance().get(&StorageKey::Client).unwrap();
        let freelancer: Address = env.storage().instance().get(&StorageKey::Freelancer).unwrap();
        let token_id: Address = env.storage().instance().get(&StorageKey::Token).unwrap();
        let amount: i128 = env.storage().instance().get(&StorageKey::Amount).unwrap();
        let is_completed: bool = env.storage().instance().get(&StorageKey::IsCompleted).unwrap();

        if is_completed {
            panic!("Funds from this workspace have already been released");
        }

        // Require authorization from the client to sign off on deliverables
        client.require_auth();

        // Perform status update and transfer execution loop
        env.storage().instance().set(&StorageKey::IsCompleted, &true);
        let client_token = token::Client::new(&env, &token_id);
        client_token.transfer(&env.current_contract_address(), &freelancer, &amount);
    }

    /// Read function to easily audit contract details from external web interfaces.
    pub fn get_status(env: Env) -> bool {
        env.storage().instance().get(&StorageKey::IsCompleted).unwrap_or(false)
    }
}

mod test;