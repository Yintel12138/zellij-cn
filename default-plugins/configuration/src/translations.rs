/// Translation support for configuration plugin
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
            // Navigation tabs
            "config.keybinds" => "键位绑定",
            "config.theme" => "主题",
            "config.ui" => "界面",
            "config.presets" => "预设",

            // Actions
            "config.save" => "保存",
            "config.cancel" => "取消",
            "config.back" => "返回",
            "config.apply" => "应用",
            "config.reset" => "重置",

            // Leader keys
            "config.leaders" => "引导键",
            "config.rebind" => "重新绑定",
            "config.unbound" => "未绑定",

            // Errors
            "config.error_write" => "写入配置文件失败",
            "config.error_parse" => "解析配置失败",

            // Sections
            "config.section.general" => "常规",
            "config.section.appearance" => "外观",
            "config.section.keybinds" => "键位绑定",
            "config.section.plugins" => "插件",

            // Options
            "config.option.theme" => "主题",
            "config.option.default_mode" => "默认模式",
            "config.option.default_shell" => "默认终端",
            "config.option.default_layout" => "默认布局",
            "config.option.simplified_ui" => "简化界面",
            "config.option.pane_frames" => "面板边框",
            "config.option.mouse_mode" => "鼠标模式",

            // Controls
            "config.controls.tab" => "切换标签",
            "config.controls.enter" => "选择",
            "config.controls.esc" => "取消",
            "config.controls.arrows" => "导航",
            "config.controls.save" => "保存",

            // Messages
            "config.message.saved" => "配置已保存",
            "config.message.unsaved" => "有未保存的更改",
            "config.message.no_changes" => "没有更改",

            // Presets
            "config.preset.default" => "默认",
            "config.preset.vim" => "Vim 风格",
            "config.preset.tmux" => "Tmux 风格",
            "config.preset.minimal" => "极简",
            "config.preset.unlock_first" => "先解锁",

            // Setup wizard
            "config.wizard.title" => "首次运行设置向导",
            "config.wizard.step" => "步骤",
            "config.wizard.of" => "/",
            "config.wizard.select_preset" => "选择一个预设配置:",
            "config.wizard.finish" => "完成",

            // Descriptions
            "config.desc.default" => "标准 Zellij 键位绑定",
            "config.desc.vim" => "类似 Vim 的导航和模式",
            "config.desc.tmux" => "Tmux 兼容的键位绑定",
            "config.desc.minimal" => "最少的键位绑定",
            "config.desc.unlock_first" => "需要先按键解锁才能使用其他功能",

            _ => Self::en(key),
        }
    }

    fn en(key: &str) -> &'static str {
        match key {
            // Navigation tabs
            "config.keybinds" => "Keybinds",
            "config.theme" => "Theme",
            "config.ui" => "UI",
            "config.presets" => "Presets",

            // Actions
            "config.save" => "Save",
            "config.cancel" => "Cancel",
            "config.back" => "Back",
            "config.apply" => "Apply",
            "config.reset" => "Reset",

            // Leader keys
            "config.leaders" => "Leaders",
            "config.rebind" => "Rebind",
            "config.unbound" => "UNBOUND",

            // Errors
            "config.error_write" => "Failed to write configuration file",
            "config.error_parse" => "Failed to parse configuration",

            // Sections
            "config.section.general" => "General",
            "config.section.appearance" => "Appearance",
            "config.section.keybinds" => "Keybinds",
            "config.section.plugins" => "Plugins",

            // Options
            "config.option.theme" => "Theme",
            "config.option.default_mode" => "Default mode",
            "config.option.default_shell" => "Default shell",
            "config.option.default_layout" => "Default layout",
            "config.option.simplified_ui" => "Simplified UI",
            "config.option.pane_frames" => "Pane frames",
            "config.option.mouse_mode" => "Mouse mode",

            // Controls
            "config.controls.tab" => "Switch tab",
            "config.controls.enter" => "Select",
            "config.controls.esc" => "Cancel",
            "config.controls.arrows" => "Navigate",
            "config.controls.save" => "Save",

            // Messages
            "config.message.saved" => "Configuration saved",
            "config.message.unsaved" => "Unsaved changes",
            "config.message.no_changes" => "No changes",

            // Presets
            "config.preset.default" => "Default",
            "config.preset.vim" => "Vim-style",
            "config.preset.tmux" => "Tmux-style",
            "config.preset.minimal" => "Minimal",
            "config.preset.unlock_first" => "Unlock First",

            // Setup wizard
            "config.wizard.title" => "First Run Setup Wizard",
            "config.wizard.step" => "Step",
            "config.wizard.of" => "/",
            "config.wizard.select_preset" => "Select a configuration preset:",
            "config.wizard.finish" => "Finish",

            // Descriptions
            "config.desc.default" => "Standard Zellij keybindings",
            "config.desc.vim" => "Vim-like navigation and modes",
            "config.desc.tmux" => "Tmux compatible keybindings",
            "config.desc.minimal" => "Minimal keybindings",
            "config.desc.unlock_first" => "Requires pressing a key to unlock before other actions",

            _ => "",
        }
    }
}
