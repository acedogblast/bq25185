/// Possible errors from the Bq25185.
#[derive(Debug, PartialEq, Eq)]
pub enum Bq25185Error<E> {
    /// Optional charge engable pin was not given.
    NoChargeEnablePin,
    /// Error from the GPIO pin (input/output).
    PinError(E),
}

impl<E> From<E> for Bq25185Error<E> {
    fn from(value: E) -> Self {
        Self::PinError(value)
    }
}
