use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: usize,
    #[serde(default = "default_per_page")]
    pub per_page: usize,
    pub search: Option<String>,
    pub category: Option<String>,
    pub protocol: Option<String>,
}

fn default_page() -> usize {
    1
}

fn default_per_page() -> usize {
    20
}

impl PaginationParams {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.page == 0 {
            return Err("page must be >= 1");
        }
        if self.per_page > 100 {
            return Err("per_page must be <= 100");
        }
        if self.per_page == 0 {
            return Err("per_page must be >= 1");
        }
        Ok(())
    }

    pub fn offset(&self) -> usize {
        (self.page - 1) * self.per_page
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_params() {
        let params = PaginationParams { page: 1, per_page: 20, search: None, category: None, protocol: None };
        params.validate().unwrap();
        assert_eq!(params.offset(), 0);
    }

    #[test]
    fn page_zero_invalid() {
        let params = PaginationParams { page: 0, per_page: 20, search: None, category: None, protocol: None };
        assert!(params.validate().is_err());
    }

    #[test]
    fn per_page_over_100_invalid() {
        let params = PaginationParams { page: 1, per_page: 101, search: None, category: None, protocol: None };
        assert!(params.validate().is_err());
    }

    #[test]
    fn per_page_zero_invalid() {
        let params = PaginationParams { page: 1, per_page: 0, search: None, category: None, protocol: None };
        assert!(params.validate().is_err());
    }

    #[test]
    fn offset_calculation() {
        let params = PaginationParams { page: 3, per_page: 10, search: None, category: None, protocol: None };
        assert_eq!(params.offset(), 20);
    }
}
