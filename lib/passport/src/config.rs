pub struct PassportConfig {
   pub manager_strategies: bool,

}

impl Default for PassportConfig {
    fn default() -> Self {
        PassportConfig {
            manager_strategies: false
        }
    }
}