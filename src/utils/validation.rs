use validator::Validate;
use crate::utils::errors::AppError;

pub fn validate_input<T: Validate>(input: &T) -> Result<(), AppError> {
    input.validate()
        .map_err(|e| AppError::Validation(format!("Validation failed: {}", e)))?;
    Ok(())
}
