use super::*;

#[cw_serde]

pub struct CallRequest {
    from: String,
    to: String,
    rollback: Vec<u8>,
    enabled: bool,
}

impl CallRequest {
    pub fn new(from: String, to: String, rollback: Vec<u8>, enabled: bool) -> Self {
        Self {
            from,
            to,
            rollback,
            enabled,
        }
    }

    pub fn from(&self) -> &str {
        &self.from
    }

    pub fn to(&self) -> &str {
        &self.to
    }

    pub fn rollback(&self) -> &[u8] {
        &self.rollback
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn is_null(&self) -> bool {
        let r = to_binary(self).unwrap();
        r.is_empty()
    }
    pub fn set_enabled(&mut self) {
        self.enabled = true;
    }
}
