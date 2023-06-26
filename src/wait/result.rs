use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::thread;

fn sum(a: i32, b: i32) -> Receiver<i32> {
    let (tx, rx) = channel();

    let tx = tx.clone();
    thread::spawn(move || {
        tx.send(a + b).expect("disconnect from receiver");
    });

    rx
}

#[test]
fn test_sum() {
    // Given
    let a: i32 = 5;
    let b: i32 = 3;
    let got: i32;
    let expected_result = 8;

    // When
    let rx = sum(a, b);
    match rx.recv() {
        Ok(sum) => got = sum,
        Err(e) => panic!("unexpected error: {}", e),
    }

    // Then
    assert_eq!(got, expected_result);
}
