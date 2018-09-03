pub struct PassportConfig {
    manager_strategies: Bool,

}

impl Default for PassportConfig {
    fn default() -> Self {
        PassportConfig {
            manager_strategies: false
        }
    }
}