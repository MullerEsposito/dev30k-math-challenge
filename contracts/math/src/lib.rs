#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Error, Val, IntoVal, String, Symbol, TryFromVal, TryIntoVal, Vec};

#[derive(Clone)]
pub struct Operation {
    pub operation_type: String,
    pub operand1: i32,
    pub operand2: i32,
    pub result: i32,
}

impl TryFromVal<Env, Val> for Operation {
    type Error = Error;

    fn try_from_val(env: &Env, v: &Val) -> Result<Self, Self::Error> {
        let tuple: (String, i32, i32, i32) = v.try_into_val(env)?;

        Ok(Operation {
            operation_type: tuple.0,
            operand1: tuple.1,
            operand2: tuple.2,
            result: tuple.3,
        })
    }
    
}

pub struct OptionOperation(pub Option<Operation>);

impl TryFromVal<Env, Val> for OptionOperation {
    type Error = Error;

    fn try_from_val(env: &Env, v: &Val) -> Result<Self, Self::Error> {
        if v.is_void() {
            Ok(OptionOperation(None))
        } else {
            let operation: Operation = v.try_into_val(env)?;
            Ok(OptionOperation(Some(operation)))
        }
    }
}

impl IntoVal<Env, Val> for OptionOperation {
    fn into_val(&self, env: &Env) -> Val {
        match &self.0 {
            Some(operation) => operation.into_val(env),
            None => Val::from_void().into(),
        }
    }
}

impl TryFromVal<Env, Operation> for Val {
    type Error = Error;

    fn try_from_val(env: &Env, v: &Operation) -> Result<Self, Self::Error> {
        Ok((v.operation_type.clone(), v.operand1, v.operand2, v.result).into_val(env))
    }
}

// Removed custom IntoVal implementation for Operation to avoid conflict

const OPERATIONS_HISTORY: Symbol = symbol_short!("history");
const LAST_OPERATION: Symbol = symbol_short!("last_op");

#[contract]
pub struct CalculatorContract;

#[contractimpl]
impl CalculatorContract {
    pub fn sum(env: Env, operand1: i32, operand2: i32) -> i32 {
        let result = operand1 + operand2;
        let operation: Operation = Operation { operation_type: String::from_str(&env, "addition"), operand1, operand2, result };

        Self::save_operation(&env, operation);
        result
    }

    pub fn sub(env: Env, operand1: i32, operand2: i32) -> i32 {
        let result: i32 = (operand1 - operand2).abs();
        let operation: Operation = Operation { operation_type: String::from_str(&env, "subtraction"), operand1, operand2, result };

        Self::save_operation(&env, operation);
        result
    }

    pub fn mul(env: Env, operand1: i32, operand2: i32) -> i32 {
        let result: i32 = operand1 * operand2;
        let operation: Operation = Operation { operation_type: String::from_str(&env, "multiplication"), operand1, operand2, result };
        
        Self::save_operation(&env, operation);
        result
    }

    pub fn div(env: Env, operand1: i32, operand2: i32) -> Option<i32> {
        if operand2 == 0 {
            return None; // Retorna None se o divisor for zero
        }
        let result: i32 = operand1 / operand2;
        let operation: Operation = Operation { operation_type: String::from_str(&env, "division"), operand1, operand2, result };
        
        Self::save_operation(&env, operation);
        Some(result)
    }    
    fn save_operation(env: &Env, operation: Operation) {
        // Recupera ou cria o histórico
        let mut operations_history: Vec<Operation> = env
            .storage()
            .persistent()
            .get(&OPERATIONS_HISTORY)
            .unwrap_or(vec![env]);

        // Adiciona a nova operação ao histórico
        operations_history.push_back(operation);

        env.storage().persistent().set(&OPERATIONS_HISTORY, &operations_history);
    }

    pub fn all_op(env: Env) -> Vec<Operation> {
        env.storage().persistent().get(&OPERATIONS_HISTORY).unwrap_or_else(|| Vec::new(&env))
    }

    pub fn get_op(env: Env, index: u32) -> Option<Operation> {
        let history: Vec<Operation> = env
            .storage()
            .persistent()
            .get(&OPERATIONS_HISTORY)
            .unwrap_or_else(|| Vec::new(&env));
        history.get(index)
    }

    pub fn last_op(env: Env) -> Option<Operation> {
        env.storage().persistent().get(&LAST_OPERATION)
    }
}