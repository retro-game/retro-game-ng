const DEFAULT_COST: u32 = 10;

pub fn hash(password: &str) -> String {
    bcrypt::hash(password, DEFAULT_COST).unwrap()
}

pub fn verify(password: &str, hash: &str) -> bool {
    bcrypt::verify(password, hash).unwrap()
}
