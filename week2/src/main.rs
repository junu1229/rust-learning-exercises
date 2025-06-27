#[derive(Debug, Clone)]
struct Wallet {
    id: String,
    balance: u64,
}

use std::sync::atomic::{AtomicU64, Ordering};

static ID_COUNT: AtomicU64 = AtomicU64::new(0);

fn main() {
    // Task 1: Create a wallet with some ETH
    println!("1. Creating wallets...");
    let mut alice_wallet = new_wallet(1000);
    let mut bob_wallet = new_wallet(500);
    let charlie_wallet = new_wallet(750);
    println!("   Alice: {}", get_wallet_info(&alice_wallet));
    println!("   Bob: {}", get_wallet_info(&bob_wallet));
    println!("   Charlie: {}", get_wallet_info(&charlie_wallet));

    // Task 2: Check balance without losing ownership
    println!("\n2. Checking balances (immutable borrow)...");
    println!("   Alice balance: {} ETH", check_balance(&alice_wallet));
    println!("   Bob balance: {} ETH", check_balance(&bob_wallet));
    println!(
        "   Alice can still use wallet: {}",
        get_wallet_info(&alice_wallet)
    );

    // Task 3: Send some ETH using mutable borrow
    println!("\n3. Alice sending ETH (mutable borrow)...");
    println!("   Before: Alice has {} ETH", check_balance(&alice_wallet));
    send_money(&mut alice_wallet, 200);
    println!("   After: Alice has {} ETH", check_balance(&alice_wallet));

    // Task 4: Transfer money between two wallets
    println!("\n4. Transfer between wallets...");
    println!("   Before transfer:");
    println!("     Alice: {} ETH", check_balance(&alice_wallet));
    println!("     Bob: {} ETH", check_balance(&bob_wallet));

    transfer_between(&mut alice_wallet, &mut bob_wallet, 150);

    println!("   After transfer:");
    println!("     Alice: {} ETH", check_balance(&alice_wallet));
    println!("     Bob: {} ETH", check_balance(&bob_wallet));

    // Task 5: Calculate total balance of 3+ wallets
    println!("\n5. Calculating total balance of multiple wallets...");
    let wallets = vec![
        alice_wallet.clone(),
        bob_wallet.clone(),
        charlie_wallet.clone(),
    ];
    let total_balance = batch_check(&wallets);
    println!("   Total balance across all wallets: {} ETH", total_balance);

    // Task 6: Transfer wallet ownership to another variable
    println!("\n6. Transferring wallet ownership...");
    println!(
        "   Before ownership transfer: {}",
        get_wallet_info(&alice_wallet)
    );
    let new_alice_wallet = transfer_ownership(alice_wallet);
    println!(
        "   After ownership transfer: {}",
        get_wallet_info(&new_alice_wallet)
    );

    // Task 7: Try to use the original wallet after transfer (should error!)
    println!("\n7. Attempting to use original wallet after ownership transfer...");
    // Uncomment the line below to see the compile error:
    // println!("   This would cause a compile error: {}", get_wallet_info(&alice_wallet));
    println!("   ✓ Compile-time safety: Cannot use alice_wallet after ownership transfer!");

    // Task 8: Handle insufficient balance
    println!("\n8. Testing insufficient balance handling...");
    println!("   Bob's balance: {} ETH", check_balance(&bob_wallet));
    println!("   Attempting to send 1000 ETH (more than balance)...");
    send_money(&mut bob_wallet, 1000); // This will show an error message
    println!(
        "   Bob's balance after failed transaction: {} ETH",
        check_balance(&bob_wallet)
    );

    // Demonstrate cloning
    println!("\n9. Creating wallet backup...");
    let backup_wallet = clone_wallet(&new_alice_wallet);
    println!("   Original: {}", get_wallet_info(&new_alice_wallet));
    println!("   Backup: {}", get_wallet_info(&backup_wallet));
}

// 1. Create wallet
fn new_wallet(balance: u64) -> Wallet {
    let id_num = ID_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
    let id = format!("wallet_{}", id_num);
    Wallet { id, balance }
}

// 2. Read balance (immutable borrow)
fn check_balance(wallet: &Wallet) -> u64 {
    wallet.balance
}

// 3. Send ETH (mutable borrow)
fn send_money(wallet: &mut Wallet, amount: u64) {
    if wallet.balance >= amount {
        wallet.balance -= amount;
        println!("   ✓ Successfully sent {} ETH", amount);
    } else {
        println!(
            "   ✗ Insufficient balance! Cannot send {} ETH (balance: {} ETH)",
            amount, wallet.balance
        );
    }
}

// 4. Move ownership
fn transfer_ownership(wallet: Wallet) -> Wallet {
    println!("   ✓ Ownership transferred successfully");
    wallet
}

// 5. Send between wallets
fn transfer_between(from: &mut Wallet, to: &mut Wallet, amount: u64) {
    if from.balance >= amount {
        from.balance -= amount;
        to.balance += amount;
        println!(
            "   ✓ Successfully transferred {} ETH from {} to {}",
            amount, from.id, to.id
        );
    } else {
        println!("   ✗ Transfer failed! Insufficient balance in source wallet");
        println!(
            "     Attempted: {} ETH, Available: {} ETH",
            amount, from.balance
        );
    }
}

// 6. Return formatted wallet details
fn get_wallet_info(wallet: &Wallet) -> String {
    format!("Wallet[{}] - Balance: {} ETH", wallet.id, wallet.balance)
}

// 7. Total balance of multiple wallets
fn batch_check(wallets: &[Wallet]) -> u64 {
    wallets.iter().map(|wallet| wallet.balance).sum()
}

// 8. Create backup wallet
fn clone_wallet(wallet: &Wallet) -> Wallet {
    let mut cloned = wallet.clone();
    cloned.id = format!("{}_backup", wallet.id);
    cloned
}
