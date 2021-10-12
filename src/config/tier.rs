use crate::config::routeConfig::ERoutes;
use std::collections::HashMap;

pub struct ITier {
    pub name: &'static str,
    tier: i8,
    key: &'static str,
}

pub fn getRoot() -> ITier {
    ITier {
        name: "Root",
        tier: 1,
        key: "root",
    }
}

pub fn getAdmin() -> ITier {
    ITier {
        name: "Admin",
        tier: 2,
        key: "admin",
    }
}
pub fn getWritter() -> ITier {
    ITier {
        name: "Writter",
        tier: 3,
        key: "writter",
    }
}
pub fn getBasic() -> ITier {
    ITier {
        name: "Basic",
        tier: 100,
        key: "basic",
    }
}

pub fn getTierNameByLevel(tierLevel: i8) -> Option<&'static str> {
    match tierLevel {
        1 => Some("Root"),
        2 => Some("Admin"),
        3 => Some("Writter"),
        100 => Some("Basic"),
        _ => None,
    }
}
pub fn getRouteTierMap() -> HashMap<ERoutes, ITier> {
    let mut routeTierMap = HashMap::new();
    routeTierMap.insert(ERoutes::LOGIN, getRoot());
    return routeTierMap;
}
