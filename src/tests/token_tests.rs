#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_transfer_tokens() {
        let sender = Address::from("sender_address");
        let recipient = Address::from("recipient_address");
        assert_eq!(transfer_tokens(&sender, &recipient, 100), Ok(()));
    }

    #[test]
    fn test_insufficient_balance() {
        let sender = Address::from("sender_address");
        let recipient = Address::from("recipient_address");
        assert_eq!(transfer_tokens(&sender, &recipient, 1000), Err("Insufficient balance".to_string()));
    }
}
