#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol, symbol_short};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,           
    StudentGpa(Address),
    ScholarshipAmt, 
}

#[contract]
pub struct ScholarTrackContract;

#[contractimpl]
impl ScholarTrackContract {
    

    pub fn initialize(env: Env, admin: Address, amount: i128) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Contract already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::ScholarshipAmt, &amount);
    }

    pub fn update_gpa(env: Env, student: Address, gpa: u32) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        env.storage().instance().set(&DataKey::StudentGpa(student), &gpa);
    }

    
    pub fn claim_scholarship(env: Env, student: Address, token_address: Address) {
        student.require_auth();

        let gpa: u32 = env.storage().instance().get(&DataKey::StudentGpa(student.clone()))
            .expect("No GPA record found");
        
        let amount: i128 = env.storage().instance().get(&DataKey::ScholarshipAmt).unwrap();

      
        if gpa >= 350 {
            
            let client = soroban_sdk::token::Client::new(&env, &token_address);
            let contract_address = env.current_contract_address();
            
            client.transfer(&contract_address, &student, &amount);
            
            env.storage().instance().remove(&DataKey::StudentGpa(student));
        } else {
            panic!("GPA not high enough for scholarship");
        }
    }
}

