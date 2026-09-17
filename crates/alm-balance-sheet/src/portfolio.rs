use alm_cashflows::{CashflowAggregator, CashflowGenerator, CashflowSchedule};

pub struct Portfolio {
    pub name: String,
    instruments: Vec<Box<dyn CashflowGenerator>>,
}

impl Portfolio {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            instruments: Vec::new(),
        }
    }

    pub fn add_instrument(&mut self, instrument: Box<dyn CashflowGenerator>) {
        self.instruments.push(instrument);
    }
}

impl CashflowGenerator for Portfolio {
    fn generate_cashflows(&self) -> CashflowSchedule {
        let schedules: Vec<CashflowSchedule> = self
            .instruments
            .iter()
            .map(|instrument| instrument.generate_cashflows())
            .collect();
            
        let refs: Vec<&CashflowSchedule> = schedules.iter().collect();
        CashflowAggregator::aggregate(&refs)
    }
}
