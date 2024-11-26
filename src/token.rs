// Pseudo code for sending tokens
pub fn transfer_tokens(from: &Address, to: &Address, amount: u64) -> Result<(), String> {
    let from_balance = get_balance(from)?;
    if from_balance < amount {
        return Err("Insufficient balance".to_string());
    }
    update_balance(from, from_balance - amount)?;
    let to_balance = get_balance(to)?;
    update_balance(to, to_balance + amount)?;
    Ok(())
}
