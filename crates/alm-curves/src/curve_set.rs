use crate::{CurveId, YieldCurve, YieldTermStructure};
use std::collections::HashMap;
use std::sync::Arc;

/// A collection of yield curves mapped by their unique identifiers.
/// Used to maintain all market curves for a given pricing or risk run.
#[derive(Default, Clone)]
pub struct CurveSet {
    curves: HashMap<CurveId, Arc<YieldCurve>>,
}

impl CurveSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_curve(&mut self, curve: YieldCurve) {
        let id = curve.curve_id().clone();
        self.curves.insert(id, Arc::new(curve));
    }

    pub fn get_curve(&self, id: &CurveId) -> Option<Arc<YieldCurve>> {
        self.curves.get(id).cloned()
    }

    pub fn remove_curve(&mut self, id: &CurveId) -> Option<Arc<YieldCurve>> {
        self.curves.remove(id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&CurveId, &Arc<YieldCurve>)> {
        self.curves.iter()
    }
}
