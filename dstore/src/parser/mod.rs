use crate::pb::pb;
// use crate::errors::ParseCmdError;

impl From<Vec<&str>> for pb::Get {
    fn from(cmds: Vec<&str>) -> Self {
        Self {
            key: cmds[1].to_string(),
        }
    }
}

impl From<Vec<&str>> for pb::Set {
    fn from(cmds: Vec<&str>) -> Self {
        Self {
            kv: Some(pb::Kv {
                key: cmds[1].to_string(),
                value: cmds[2].to_string(),
            }),
        }
    }
}

impl From<Vec<&str>> for pb::HGet {
    fn from(cmds: Vec<&str>) -> Self {
        Self {
            key: cmds[1].to_string(),
            field: cmds[2].to_string(),
        }
    }
}

impl From<Vec<&str>> for pb::HSet {
    fn from(cmds: Vec<&str>) -> Self {
        Self {
            data: Some(pb::HMap {
                key: cmds[1].to_string(),
                field_values: group_kv(&cmds[2..]),
            }),
        }
    }
}

impl From<Vec<&str>> for pb::SAdd {
    fn from(cmds: Vec<&str>) -> Self {
        Self {
            key: cmds[1].to_string(),
            values: Vec::from_iter(cmds[2..].iter().map(|x| x.to_string())),
        }
    }
}

impl From<Vec<&str>> for pb::SMembers {
    fn from(cmds: Vec<&str>) -> Self {
        Self {
            key: cmds[1].to_string(),
        }
    }
}

impl From<Vec<&str>> for pb::LPush {
    fn from(cmds: Vec<&str>) -> Self {
        Self {
            key: cmds[1].to_string(),
            elements: Vec::from_iter(cmds[2..].iter().map(|x| x.to_string())),
        }
    }
}

impl From<Vec<&str>> for pb::LPop {
    fn from(cmds: Vec<&str>) -> Self {
        Self {
            key: cmds[1].to_string(),
            count: cmds[2].parse().unwrap(),
        }
    }
}

impl From<Vec<&str>> for pb::LRange {
    fn from(cmds: Vec<&str>) -> Self {
        Self {
            key: cmds[1].to_string(),
            start: cmds[2].parse().unwrap(),
            stop: cmds[3].parse().unwrap(),
        }
    }
}

fn group_kv(cmds: &[&str]) -> Vec<pb::Kv> {
    if cmds.len() % 2 != 0 {
        return vec![];
    }

    let mut idx = 0;
    let mut kv_pairs: Vec<pb::Kv> = vec![];
    loop {
        if idx >= cmds.len() {
            break;
        }

        kv_pairs.push(pb::Kv {
            key: cmds[idx].to_string(),
            value: cmds[idx + 1].to_string(),
        });
        idx += 2;
    }
    kv_pairs
}

#[cfg(test)]
mod tests {
    use crate::pb::pb::{self, Kv};

    #[test]
    fn test_build_get() {
        let cmds = vec!["GET", "test-key"];
        let cmd = pb::Get::from(cmds);
        assert_eq!(
            pb::Get {
                key: "test-key".to_string()
            },
            cmd
        );
    }

    #[test]
    fn test_build_set() {
        let cmds = vec!["SET", "test-key", "test-value"];
        let cmd = pb::Set::from(cmds);
        assert_eq!(
            pb::Set {
                kv: Some(pb::Kv {
                    key: "test-key".to_string(),
                    value: "test-value".to_string()
                })
            },
            cmd
        );
    }

    #[test]
    fn test_build_hget() {
        let cmds = vec!["HGet", "test-key", "test-field"];
        let cmd = pb::HGet::from(cmds);
        assert_eq!(
            pb::HGet {
                key: "test-key".to_string(),
                field: "test-field".to_string()
            },
            cmd
        );
    }

    #[test]
    fn test_build_hset() {
        let cmds = vec![
            "HSet",
            "test-key",
            "test-field1",
            "test-value1",
            "test-field2",
            "test-value2",
        ];
        let cmd = pb::HSet::from(cmds);
        assert_eq!(
            pb::HSet {
                data: Some(pb::HMap {
                    key: "test-key".to_string(),
                    field_values: vec![
                        Kv {
                            key: "test-field1".to_string(),
                            value: "test-value1".to_string()
                        },
                        Kv {
                            key: "test-field2".to_string(),
                            value: "test-value2".to_string()
                        }
                    ],
                })
            },
            cmd
        );
    }
}
