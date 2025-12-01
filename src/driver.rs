use crate::Bq25185Error;
use embedded_hal::digital::{InputPin, OutputPin};

/// A basic and simple BQ25185 battey charging IC driver
pub struct Bq25185<IN, OUT> {
    /// The stat1 pin.
    stat1: IN,
    /// The stat2 pin.
    stat2: IN,
    /// An optional charge enable active low (n_CE). This is requried for the enable_charge() and disable_charge() functions.
    charge_enable: Option<OUT>,
}

/// The 4 possible status states of the charger.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    /// Charging is complete, the charger is in sleep mode, or charging is disabled.
    ChargeComplete,
    /// Normal charging in progress (including automatic recharge).
    NormalCharging,
    /// Recoverable fault (VIN_OVP, TS HOT, TS COLD, TSHUT, System short protection).
    RecoverableFault,
    /// Non-recoverable or latch-off fault (ILIM/ISET pin short, BATOCP, Safety TMR_EXP for charging).
    NonRecoverableFault,
}

impl<IN, OUT, E> Bq25185<IN, OUT>
where
    IN: InputPin<Error = E>,
    OUT: OutputPin<Error = E>,
    E: embedded_hal::digital::Error,
{
    /// Creates a new Bq25185 struct. Required pins are stat1 and stat2. Optionally accepts a charge_enable pin.
    pub fn new(stat1: IN, stat2: IN, charge_enable: Option<OUT>) -> Self {
        Bq25185 {
            stat1,
            stat2,
            charge_enable,
        }
    }

    /// Returns the current status of the charger IC.
    pub fn get_status(&mut self) -> Result<Status, Bq25185Error<E>> {
        let status1 = self.stat1.is_high()?;
        let status2 = self.stat2.is_high()?;

        match (status1, status2) {
            (false, false) => Ok(Status::NonRecoverableFault),
            (false, true) => Ok(Status::RecoverableFault),
            (true, false) => Ok(Status::NormalCharging),
            (true, true) => Ok(Status::ChargeComplete),
        }
    }

    /// Enables battery charging by setting the charge enable pin low.
    pub fn enable_charge(&mut self) -> Result<(), Bq25185Error<E>> {
        if let Some(ref mut output) = self.charge_enable {
            output.set_low()?;
            return Ok(());
        } else {
            Err(Bq25185Error::NoChargeEnablePin)
        }
    }

    /// Disables battery charging by setting the charge enable pin high.
    pub fn disable_charge(&mut self) -> Result<(), Bq25185Error<E>> {
        if let Some(ref mut output) = self.charge_enable {
            output.set_high()?;
            return Ok(());
        } else {
            Err(Bq25185Error::NoChargeEnablePin)
        }
    }
}
