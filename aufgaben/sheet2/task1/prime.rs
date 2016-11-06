//! Aufgabe 2.1: Primzahltest

fn main() {

    println!("1");
    for prime in 2..21 {
        if is_prime(prime){
            println!("{}*", prime);
        } else {
            println!("{}", prime);
        }
    }

}

fn is_prime(prime: i32) -> bool {
    let mut iterator = 2;
    // testet ob ohne Rest teilbar, wenn nicht = Primzahl
    while iterator <= prime / 2 {
        if prime % iterator == 0 {
        return false;
        }
        iterator += 1;
    }
    true
}
    

#[test]
fn small_primes() {
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(is_prime(5));
    assert!(is_prime(7));
}

#[test]
fn small_composites() {
    assert!(!is_prime(1));
    assert!(!is_prime(4));
    assert!(!is_prime(6));
    assert!(!is_prime(8));
    assert!(!is_prime(9));
}

#[test]
fn large_primes() {
    assert!(is_prime(1_300_769));
    assert!(is_prime(1_300_297));
    assert!(is_prime(7_367_287));
}

#[test]
fn large_composites() {
    assert!(!is_prime(908_209));
    assert!(!is_prime(3_073_009));
    assert!(!is_prime(4_897_369));
}
