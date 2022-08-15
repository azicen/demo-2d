// 地皮
#[derive(Debug, Clone)]
pub struct Land {
    id: u32,
    name: &'static str,
    short_name: &'static str,
}

static mut LAND_LIST: Vec<Land> = Vec::new();
static mut LAND_NAME_LIST: Vec<String> = Vec::new();
static mut LAND_NAME_SHORT_LIST: Vec<String> = Vec::new();

impl Land {
    pub fn new(id: u32) -> Option<Land> {
        let land = unsafe { LAND_LIST.get(id as usize) }?;
        Some(land.clone())
    }

    // register 注册地皮数据
    // 这是不安全的，只被允许在程序初始化阶段使用
    // TODO 考虑之后使用一个数据类型来注册地皮数据，替代直接使用的基础数据类型，使意义更明确
    pub unsafe fn register(id: u32, name: String, short_name: String) {
        let i = id as usize;
        LAND_NAME_LIST.insert(i, name);
        LAND_NAME_SHORT_LIST.insert(i, short_name);
        let land = Land {
            id,
            name: LAND_NAME_LIST[i].as_str(),
            short_name: LAND_NAME_SHORT_LIST[i].as_str(),
        };
        LAND_LIST.insert(i, land);
    }
}

impl Land {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn short_name(&self) -> &str {
        self.short_name
    }
}
