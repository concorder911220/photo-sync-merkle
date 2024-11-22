use photo_sync_merkle::MerkleTree; 

fn main() {
    let elems = vec![
        "A".to_string(),
        "B".to_string(),
        "C".to_string(),
        "D".to_string(),
        "E".to_string(),
        "For".to_string(),
        "Go".to_string(),
    ];

    let elems2 = vec![
        "A".to_string(),
        "XX".to_string(),
        "C".to_string(),
        "D".to_string(),
        "E".to_string(),
        "For2".to_string(),
        "Rust".to_string(),
    ];

    let elems3 = vec![
        "A".to_string(),
        "B".to_string(),
        "C".to_string(),
        "D".to_string(),
        "E".to_string(),
        "For3".to_string(),
        "Solana".to_string(),
    ];

    let mut mtree = MerkleTree::new(elems);
    let mtree2 = MerkleTree::new(elems2);
    let mtree3 = MerkleTree::new(elems3);

    mtree.sync(&mtree2);
    mtree.sync(&mtree3);

    println!("Final Array: {:?}", mtree.arr);
}
