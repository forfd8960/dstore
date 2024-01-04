pub mod pb;

pub const GET_CMD: &str = "Get";
pub const SET_CMD: &str = "Set";
pub const HGET_CMD: &str = "HGet";
pub const HSET_CMD: &str = "HSet";
pub const SADD_CMD: &str = "SAdd";
pub const SMEMBERS_CMD: &str = "SMembers";
pub const LPUSH: &str = "LPush";
pub const LPOP: &str = "LPop";

impl std::fmt::Display for pb::CommandResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.pairs.len() > 0 {
            for kv in &self.pairs {
                let _ = writeln!(f, "{}: {}", kv.key, kv.value);
            }
            return write!(f, "{}", "");
        }

        if self.values.len() > 0 {
            for value in &self.values {
                let _ = writeln!(f, "{}", value.val);
            }
            return write!(f, "{}", "");
        }

        write!(f, "{}", self.message)
    }
}
