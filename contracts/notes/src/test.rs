#![cfg(test)]
use super::{BreezeEscrow, BreezeEscrowClient, EscrowState};
use soroban_sdk::{testutils::Address as _, token, Address, Env};

fn setup_test_environment(env: &Env) -> (Address, Address, token::Client, token::StellarAssetClient) {
    let client = Address::generate(env);
    let freelancer = Address::generate(env);
    
    // Deploy a mock SAC (Stellar Asset Contract) token standard
    let token_admin = Address::generate(env);
    let contract_token_id = env.register_stellar_asset_contract(token_admin);
    let token_client = token::Client::new(env, &contract_token_id);
    let token_admin_client = token::StellarAssetClient::new(env, &contract_token_id);

    (client, freelancer, token_client, token_admin_client)
}

#[test]
fn test_happy_path_escrow_lifecycle() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, BreezeEscrow);
    let client = BreezeEscrowClient::new(&env, &contract_id);

    let (user_client, freelancer, token, token_admin) = setup_test_environment(&env);
    let deal_amount = 500_i128;

    // Mint token assets to client balance
    token_admin.mint(&user_client, &deal_amount);
    assert_eq!(token.balance(&user_client), deal_amount);

    // Run core flows
    client.initialize(&user_client, &freelancer, &token.address, &deal_amount);
    assert_eq!(client.get_state(), EscrowState::Initialized);

    client.fund();
    assert_eq!(client.get_state(), EscrowState::Funded);
    assert_eq!(token.balance(&contract_id), deal_amount);
    assert_eq!(token.balance(&user_client), 0);

    client.release();
    assert_eq!(client.get_state(), EscrowState::Released);
    assert_eq!(token.balance(&freelancer), deal_amount);
    assert_eq!(token.balance(&contract_id), 0);
}

#[test]
#[should_panic(expected = "Escrow is not funded")]
fn test_error_release_without_funding() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, BreezeEscrow);
    let client = BreezeEscrowClient::new(&env, &contract_id);

    let (user_client, freelancer, token, _) = setup_test_environment(&env);
    
    client.initialize(&user_client, &freelancer, &token.address, &100_i128);
    // Errant state invocation: executing release before funding
    client.release();
}

#[test]
fn test_state_verification_persistence() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, BreezeEscrow);
    let client = BreezeEscrowClient::new(&env, &contract_id);

    let (user_client, freelancer, token, token_admin) = setup_test_environment(&env);
    let deal_amount = 1000_i128;

    token_admin.mint(&user_client, &deal_amount);
    client.initialize(&user_client, &freelancer, &token.address, &deal_amount);
    
    client.fund();
    assert_eq!(client.get_state(), EscrowState::Funded);
}

#[test]
#[should_panic(expected = "Contract is already initialized")]
fn test_error_double_initialization() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, BreezeEscrow);
    let client = BreezeEscrowClient::new(&env, &contract_id);

    let (user_client, freelancer, token, _) = setup_test_environment(&env);
    
    client.initialize(&user_client, &freelancer, &token.address, &100_i128);
    // Malicious attempt to overwrite initialized parameters
    client.initialize(&user_client, &freelancer, &token.address, &200_i128);
}

#[test]
#[should_panic(expected = "Escrow amount must be positive")]
fn test_invalid_initialization_amount() {
    let env = Env::default();
    let contract_id = env.register_contract(None, BreezeEscrow);
    let client = BreezeEscrowClient::new(&env, &contract_id);

    let (user_client, freelancer, token, _) = setup_test_environment(&env);
    client.initialize(&user_client, &freelancer, &token.address, &-50_i128);
}