use crate::config::route_config::ERoutes;
use std::collections::HashMap;

pub struct ITier {
    pub name: &'static str,
    pub tier: i8,
    pub key: &'static str,
}

pub fn get_root() -> ITier {
    ITier {
        name: "Root",
        tier: 1,
        key: "root",
    }
}

pub fn get_admin() -> ITier {
    ITier {
        name: "Admin",
        tier: 2,
        key: "admin",
    }
}
pub fn get_writter() -> ITier {
    ITier {
        name: "Writter",
        tier: 3,
        key: "writter",
    }
}
pub fn get_basic() -> ITier {
    ITier {
        name: "Basic",
        tier: 100,
        key: "basic",
    }
}

pub fn get_tier_name_by_level(tier_level: i8) -> Option<&'static str> {
    match tier_level {
        1 => Some("Root"),
        2 => Some("Admin"),
        3 => Some("Writter"),
        100 => Some("Basic"),
        _ => None,
    }
}
pub fn get_route_tier_map() -> HashMap<ERoutes, ITier> {
    let mut route_tier_map = HashMap::new();
    route_tier_map.insert(ERoutes::LOGIN, get_root());
    route_tier_map.insert(ERoutes::CREATE_ACCESS_KEY, get_admin());
    return route_tier_map;
}
