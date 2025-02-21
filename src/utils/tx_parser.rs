use solana_transaction_status::{
    EncodedTransactionWithStatusMeta,
    UiMessage, 
    EncodedTransaction, 
};

#[derive(Debug)]
pub struct Instruction {
    pub action: String,
    pub params: String,
}

pub fn get_instrs(log: &Vec<String>, account_keys: &Vec<String>) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let mut create_found = false;
    let mut buy_found = false;
    let mut params = String::new();

    // Iterate through the log entries
    for entry in log.iter() {
        if entry.contains("Instruction: Create") {
            create_found = true;
            params = account_keys[1].clone(); // Store params from the create action
        } else if entry.contains("SellVestedTokens") {
            instructions.push(Instruction { action: "sell".to_string(), params: account_keys[2].clone() });
        } else if entry.contains("Buy") {
            buy_found = true;
            // We don't add the "buy" instruction directly
        }
    }

    // After the loop, handle the merging and individual cases
    if create_found && buy_found {
        instructions.push(Instruction { action: "create_buy".to_string(), params });
    } else {
        // If only create was found, add create instruction
        if create_found {
            instructions.push(Instruction { action: "create".to_string(), params });
        }

        // If only buy was found, add buy instruction
        if buy_found {
            instructions.push(Instruction { action: "buy".to_string(), params: account_keys[2].clone() });
        }
    }

    instructions
}

pub fn get_accounts(tx: &EncodedTransactionWithStatusMeta) -> Vec<String> {

    if let EncodedTransaction::Json(ui_transaction) = &tx.transaction {
        if let UiMessage::Raw(ui_raw_message) = &ui_transaction.message{
            return ui_raw_message.account_keys.clone();
        }

    }

    Vec::new()
}
