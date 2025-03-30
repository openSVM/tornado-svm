//! Verifier implementation for the Tornado Cash Privacy Solution

use ark_bn254::{Bn254, Fr, G1Affine, G2Affine};
use ark_ec::pairing::Pairing;
use ark_ff::{BigInteger, PrimeField};
use ark_groth16::{prepare_verifying_key, verify_proof, Proof, VerifyingKey};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use solana_program::{
    msg,
    program_error::ProgramError,
};

use crate::error::TornadoError;

/// Verifies a zkSNARK proof
pub fn verify_tornado_proof(
    proof_data: &[u8],
    public_inputs: &[u8; 192], // 6 public inputs * 32 bytes
) -> Result<bool, ProgramError> {
    // Deserialize the proof
    let proof = deserialize_proof(proof_data)?;
    
    // Deserialize the public inputs
    let inputs = deserialize_public_inputs(public_inputs)?;
    
    // Get the hardcoded verifying key
    let vk = get_verifying_key()?;
    
    // Prepare the verifying key
    let pvk = prepare_verifying_key(&vk);
    
    // Verify the proof
    let result = verify_proof(&pvk, &proof, &inputs);
    
    match result {
        Ok(valid) => {
            if valid {
                msg!("Proof verification successful");
                Ok(true)
            } else {
                msg!("Proof verification failed");
                Err(TornadoError::InvalidProof.into())
            }
        }
        Err(e) => {
            msg!("Error verifying proof: {:?}", e);
            Err(TornadoError::InvalidProof.into())
        }
    }
}

/// Deserialize a proof from bytes
fn deserialize_proof(proof_data: &[u8]) -> Result<Proof<Bn254>, ProgramError> {
    // Ensure the proof data is the correct length
    if proof_data.len() != 256 {
        msg!("Invalid proof data length: {}", proof_data.len());
        return Err(TornadoError::InvalidProof.into());
    }
    
    // Extract the proof components
    let a_x = extract_field_element(&proof_data[0..32])?;
    let a_y = extract_field_element(&proof_data[32..64])?;
    let b_x_1 = extract_field_element(&proof_data[64..96])?;
    let b_x_2 = extract_field_element(&proof_data[96..128])?;
    let b_y_1 = extract_field_element(&proof_data[128..160])?;
    let b_y_2 = extract_field_element(&proof_data[160..192])?;
    let c_x = extract_field_element(&proof_data[192..224])?;
    let c_y = extract_field_element(&proof_data[224..256])?;
    
    // Create the G1 and G2 points
    let a = G1Affine::new(a_x, a_y);
    let b = G2Affine::new([b_x_1, b_x_2], [b_y_1, b_y_2]);
    let c = G1Affine::new(c_x, c_y);
    
    // Create the proof
    Ok(Proof { a, b, c })
}

/// Extract a field element from bytes
fn extract_field_element(data: &[u8]) -> Result<Fr, ProgramError> {
    if data.len() != 32 {
        return Err(TornadoError::InvalidProof.into());
    }
    
    // Convert bytes to field element
    let mut repr = <Fr as PrimeField>::BigInt::default();
    repr.read_le(data)
        .map_err(|_| TornadoError::InvalidProof)?;
    
    // Create the field element
    Fr::from_le_bytes_mod_order(data)
}

/// Deserialize public inputs from bytes
fn deserialize_public_inputs(data: &[u8; 192]) -> Result<Vec<Fr>, ProgramError> {
    let mut inputs = Vec::with_capacity(6);
    
    for i in 0..6 {
        let start = i * 32;
        let end = start + 32;
        let input = extract_field_element(&data[start..end])?;
        inputs.push(input);
    }
    
    Ok(inputs)
}

/// Get the hardcoded verifying key
fn get_verifying_key() -> Result<VerifyingKey<Bn254>, ProgramError> {
    // This would be the hardcoded verifying key from the trusted setup
    // For simplicity, we're creating a dummy key here
    // In a real implementation, this would be the actual verifying key
    
    // Alpha in G1
    let alpha_g1 = G1Affine::new(
        Fr::from(1),
        Fr::from(2),
    );
    
    // Beta in G2
    let beta_g2 = G2Affine::new(
        [Fr::from(3), Fr::from(4)],
        [Fr::from(5), Fr::from(6)],
    );
    
    // Gamma in G2
    let gamma_g2 = G2Affine::new(
        [Fr::from(7), Fr::from(8)],
        [Fr::from(9), Fr::from(10)],
    );
    
    // Delta in G2
    let delta_g2 = G2Affine::new(
        [Fr::from(11), Fr::from(12)],
        [Fr::from(13), Fr::from(14)],
    );
    
    // IC (7 elements for 6 public inputs + 1)
    let mut ic = Vec::with_capacity(7);
    for i in 0..7 {
        ic.push(G1Affine::new(
            Fr::from((i * 2 + 15) as u64),
            Fr::from((i * 2 + 16) as u64),
        ));
    }
    
    Ok(VerifyingKey {
        alpha_g1,
        beta_g2,
        gamma_g2,
        delta_g2,
        gamma_abc_g1: ic,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::{Bn254, Fr, G1Affine, G2Affine};
    use ark_ec::pairing::Pairing;
    use ark_ff::{Field, One, Zero};
    use ark_groth16::{Proof, VerifyingKey};
    use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
    
    // Helper function to create a dummy proof
    fn create_dummy_proof() -> Vec<u8> {
        // Create dummy field elements
        let a_x = Fr::one();
        let a_y = Fr::one();
        let b_x_1 = Fr::one();
        let b_x_2 = Fr::one();
        let b_y_1 = Fr::one();
        let b_y_2 = Fr::one();
        let c_x = Fr::one();
        let c_y = Fr::one();
        
        // Create G1 and G2 points
        let a = G1Affine::new(a_x, a_y);
        let b = G2Affine::new([b_x_1, b_x_2], [b_y_1, b_y_2]);
        let c = G1Affine::new(c_x, c_y);
        
        // Create the proof
        let proof = Proof { a, b, c };
        
        // Serialize the proof components to bytes
        let mut proof_data = Vec::new();
        
        // Add a_x, a_y
        let mut a_x_bytes = [0u8; 32];
        let mut a_y_bytes = [0u8; 32];
        a_x_bytes[0] = 1;
        a_y_bytes[0] = 1;
        proof_data.extend_from_slice(&a_x_bytes);
        proof_data.extend_from_slice(&a_y_bytes);
        
        // Add b_x_1, b_x_2, b_y_1, b_y_2
        let mut b_x_1_bytes = [0u8; 32];
        let mut b_x_2_bytes = [0u8; 32];
        let mut b_y_1_bytes = [0u8; 32];
        let mut b_y_2_bytes = [0u8; 32];
        b_x_1_bytes[0] = 1;
        b_x_2_bytes[0] = 1;
        b_y_1_bytes[0] = 1;
        b_y_2_bytes[0] = 1;
        proof_data.extend_from_slice(&b_x_1_bytes);
        proof_data.extend_from_slice(&b_x_2_bytes);
        proof_data.extend_from_slice(&b_y_1_bytes);
        proof_data.extend_from_slice(&b_y_2_bytes);
        
        // Add c_x, c_y
        let mut c_x_bytes = [0u8; 32];
        let mut c_y_bytes = [0u8; 32];
        c_x_bytes[0] = 1;
        c_y_bytes[0] = 1;
        proof_data.extend_from_slice(&c_x_bytes);
        proof_data.extend_from_slice(&c_y_bytes);
        
        proof_data
    }
    
    // Helper function to create dummy public inputs
    fn create_dummy_public_inputs() -> [u8; 192] {
        let mut inputs = [0u8; 192];
        // Set some non-zero values
        for i in 0..6 {
            inputs[i * 32] = (i + 1) as u8;
        }
        inputs
    }
    
    #[test]
    fn test_deserialize_proof() {
        let proof_data = create_dummy_proof();
        let result = deserialize_proof(&proof_data);
        assert!(result.is_ok());
        
        // Test with invalid length
        let invalid_proof = vec![0u8; 128]; // Too short
        let result = deserialize_proof(&invalid_proof);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_extract_field_element() {
        // Test with valid data
        let mut data = [0u8; 32];
        data[0] = 1;
        let result = extract_field_element(&data);
        assert!(result.is_ok());
        
        // Test with invalid length
        let invalid_data = [0u8; 16]; // Too short
        let result = extract_field_element(&invalid_data);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_deserialize_public_inputs() {
        let inputs = create_dummy_public_inputs();
        let result = deserialize_public_inputs(&inputs);
        assert!(result.is_ok());
        
        let deserialized = result.unwrap();
        assert_eq!(deserialized.len(), 6);
        
        // Check that the values were correctly deserialized
        for i in 0..6 {
            assert!(!deserialized[i].is_zero());
        }
    }
    
    #[test]
    fn test_get_verifying_key() {
        let result = get_verifying_key();
        assert!(result.is_ok());
        
        let vk = result.unwrap();
        assert_eq!(vk.gamma_abc_g1.len(), 7); // 6 public inputs + 1
    }
    
    #[test]
    fn test_verify_tornado_proof() {
        let proof_data = create_dummy_proof();
        let public_inputs = create_dummy_public_inputs();
        
        // This should fail because we're using dummy values
        // In a real scenario, we would use a valid proof and inputs
        let result = verify_tornado_proof(&proof_data, &public_inputs);
        assert!(result.is_err());
        
        // Test with invalid proof data
        let invalid_proof = vec![0u8; 128]; // Too short
        let result = verify_tornado_proof(&invalid_proof, &public_inputs);
        assert!(result.is_err());
    }
}