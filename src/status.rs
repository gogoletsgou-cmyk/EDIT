/// A user's displayable account status shown on the home page.
pub struct AccountStatus {
    pub display_name: &'static str,
    pub role: &'static str,
    pub supporters: u32,
}

/// Placeholder account data until authentication and persistent storage are added.
pub const ACCOUNT: AccountStatus = AccountStatus {
    display_name: "あなた",
    role: "編集者",
    supporters: 128,
};
