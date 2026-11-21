// Units for Clients with Admin Permissions

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdminUnit {
    Kick,
    Remove,
    Add,
    MakeAdmin,
    RemoveAdmin,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Kick {
    id: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Remove {
    id: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Add {
    username: String,
    password: String,
}

#[derive(Debug, Clone, PartialEq, Eq)] 
pub struct MakeAdmin {
    id: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)] 
pub struct RemoveAdmin {
    id: u16,
}
