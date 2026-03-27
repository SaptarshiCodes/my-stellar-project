#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Env, Symbol, symbol_short, String
};

#[contract]
pub struct SkillVerification;

#[contracttype]
#[derive(Clone)]
pub struct SkillKey {
    pub user: String,
    pub skill: String,
}

#[contractimpl]
impl SkillVerification {

    // Add a skill (default = not verified)
    pub fn add_skill(env: Env, user: String, skill: String) {
        let key = SkillKey { user, skill };
        env.storage().instance().set(&key, &false);
    }

    // Verify a skill
    pub fn verify_skill(env: Env, user: String, skill: String) {
        let key = SkillKey { user, skill };
        env.storage().instance().set(&key, &true);
    }

    // Get skill verification status
    pub fn get_skill(env: Env, user: String, skill: String) -> bool {
        let key = SkillKey { user, skill };
        env.storage().instance().get(&key).unwrap_or(false)
    }
}