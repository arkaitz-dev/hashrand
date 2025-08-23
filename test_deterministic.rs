// Test para verificar determinismo con semillas
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

fn main() {
    println!("=== Test de Determinismo ===\n");

    // Test 1: Misma semilla, mismo resultado
    println("1. Probando misma semilla múltiples veces:");
    
    let seed = 42u64;
    for i in 1..=5 {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let result: String = (0..10)
            .map(|_| (rng.gen_range(b'a'..=b'z')) as char)
            .collect();
        println!("  Iteración {}: {}", i, result);
    }
    
    println!("\n2. Probando diferentes semillas:");
    
    // Test 2: Diferentes semillas, diferentes resultados
    for seed in [42, 123, 999] {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let result: String = (0..10)
            .map(|_| (rng.gen_range(b'a'..=b'z')) as char)
            .collect();
        println!("  Semilla {}: {}", seed, result);
    }
    
    println!("\n3. Test con nanoid-style generation:");
    
    // Test 3: Simulando el comportamiento de nanoid
    fn seeded_bytes(seed: u64, size: usize) -> Vec<u8> {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        (0..size).map(|_| rng.gen()).collect()
    }
    
    let alphabet = ['a', 'b', 'c', 'd', 'e', 'f', '0', '1', '2', '3'];
    
    for seed in [42, 42, 42] { // Misma semilla 3 veces
        let bytes = seeded_bytes(seed, 15);
        let result: String = bytes.iter()
            .map(|&b| alphabet[(b as usize) % alphabet.len()])
            .take(10)
            .collect();
        println!("  Semilla {} -> {}", seed, result);
    }
}