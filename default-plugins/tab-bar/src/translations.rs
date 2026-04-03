/// Translation support for tab-bar plugin
pub struct Translations {
    pub language: String,
}

impl Translations {
    pub fn new(language: &str) -> Self {
        Translations {
            language: language.to_string(),
        }
    }

    pub fn get(&self, key: &str) -> &'static str {
        match self.language.as_str() {
            "zh" | "zh-CN" | "zh_CN" | "zh-Hans" => Self::zh(key),
            _ => Self::en(key),
        }
    }

    fn zh(key: &str) -> &'static str {
        match key {
            "tab.enter_name" => "输入名称...",
            "tab.rename_tab" => "重命名标签",
            "tab.all" => "所有标签",
            "tab.sync" => "同步",
            "tab.fullscreen" => "全屏",
            _ => Self::en(key),
        }
    }

    fn en(key: &str) -> &'static str {
        match key {
            "tab.enter_name" => "Enter name...",
            "tab.rename_tab" => "RENAME TAB",
            "tab.all" => "All Tabs",
            "tab.sync" => "SYNC",
            "tab.fullscreen" => "FULLSCREEN",
            _ => "",
        }
    }
}
