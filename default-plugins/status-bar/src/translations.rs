/// Translation support for status-bar plugin
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
            // Mode names
            "mode.lock" => "锁定",
            "mode.unlock" => "解锁",
            "mode.pane" => "面板",
            "mode.tab" => "标签",
            "mode.resize" => "调整",
            "mode.search" => "搜索",
            "mode.quit" => "退出",
            "mode.session" => "会话",
            "mode.move" => "移动",
            "mode.tmux" => "TMUX",

            // Interface locked
            "interface.locked" => " -- 界面已锁定 -- ",

            // Clipboard messages
            "clipboard.copied_system" => "文本已复制到系统剪贴板",
            "clipboard.copied_primary" => "文本已复制到系统主选区",
            "clipboard.copied_command" => "文本已通过管道发送到外部命令",
            "clipboard.error" => " 使用系统剪贴板时出错。",

            // Tip prefix
            "tip.prefix" => " 提示: ",

            // Pane actions
            "action.new_pane" => "新建面板",
            "action.change_focus" => "切换焦点",
            "action.close" => "关闭",
            "action.fullscreen" => "全屏",
            "action.search" => "搜索",
            "action.rename_pane" => "重命名面板",
            "action.select_pane" => "选择面板",
            "action.move_pane_focus" => "移动面板焦点",
            "action.toggle_floating" => "切换浮动",
            "action.embed" => "嵌入",

            // Tab actions
            "action.new_tab" => "新建标签",
            "action.rename_tab" => "重命名标签",
            "action.close_tab" => "关闭标签",
            "action.sync" => "同步",

            // Resize actions
            "action.increase_size" => "增大尺寸",
            "action.decrease_size" => "减小尺寸",
            "action.increase_size_all" => "增大所有面板",
            "action.decrease_size_all" => "减小所有面板",
            "action.reset_size" => "重置尺寸",

            // Session/quit actions
            "action.detach" => "分离",
            "action.quit" => "退出",
            "action.disconnect_other_clients" => "断开其他客户端",

            // Other actions
            "action.undo" => "撤销重命名",

            // Tips
            "tip.quicknav.open_new_pane" => "=> 新建面板。",
            "tip.quicknav.navigate" => "=> 在面板间导航。",
            "tip.quicknav.resize" => "=> 调整面板大小。",
            "tip.quicknav.open_new_pane_short" => "=> 新建面板。",
            "tip.quicknav.navigate_short" => "=> 导航。",
            "tip.quicknav.resize_short" => "=> 调整大小。",

            "tip.floating_panes.toggle" => "切换浮动面板",
            "tip.floating_panes.move" => "移动浮动面板",
            "tip.mouse_click.info" => "按住 Shift 将鼠标点击发送到终端",
            "tip.move_focus.info" => "在面板间移动，超出边界时切换标签",
            "tip.setup_check.info" => "运行 'zellij setup --check' 进行故障排除",
            "tip.use_mouse.info" => "使用鼠标在面板间移动并调整大小",
            "tip.sync_tab.info" => "输入同步到标签中所有面板",
            "tip.edit_scrollbuffer.info" => "在编辑器中编辑终端回滚",
            "tip.compact_layout.info" => "紧凑布局隐藏状态栏",
            "tip.move_tabs.info" => "使用修饰键+方向键移动标签",

            // Fullscreen/floating pane indicators
            "indicator.fullscreen" => "全屏",
            "indicator.hidden_panes" => " 个隐藏的面板",
            "indicator.floating_panes_visible" => "浮动面板可见",
            "indicator.press" => "按下 ",
            "indicator.to_embed" => " 嵌入或 ",
            "indicator.to_toggle" => " 切换",

            _ => Self::en(key),
        }
    }

    fn en(key: &str) -> &'static str {
        match key {
            // Mode names
            "mode.lock" => "LOCK",
            "mode.unlock" => "UNLOCK",
            "mode.pane" => "PANE",
            "mode.tab" => "TAB",
            "mode.resize" => "RESIZE",
            "mode.search" => "SEARCH",
            "mode.quit" => "QUIT",
            "mode.session" => "SESSION",
            "mode.move" => "MOVE",
            "mode.tmux" => "TMUX",

            // Interface locked
            "interface.locked" => " -- INTERFACE LOCKED -- ",

            // Clipboard messages
            "clipboard.copied_system" => "Text copied to system clipboard",
            "clipboard.copied_primary" => "Text copied to system primary selection",
            "clipboard.copied_command" => "Text piped to external command",
            "clipboard.error" => " Error using the system clipboard.",

            // Tip prefix
            "tip.prefix" => " Tip: ",

            // Pane actions
            "action.new_pane" => "New pane",
            "action.change_focus" => "Change focus",
            "action.close" => "Close",
            "action.fullscreen" => "Fullscreen",
            "action.search" => "Search",
            "action.rename_pane" => "Rename pane",
            "action.select_pane" => "Select pane",
            "action.move_pane_focus" => "Move pane focus",
            "action.toggle_floating" => "Toggle floating",
            "action.embed" => "Embed",

            // Tab actions
            "action.new_tab" => "New tab",
            "action.rename_tab" => "Rename tab",
            "action.close_tab" => "Close tab",
            "action.sync" => "Sync",

            // Resize actions
            "action.increase_size" => "Increase size",
            "action.decrease_size" => "Decrease size",
            "action.increase_size_all" => "Increase all panes",
            "action.decrease_size_all" => "Decrease all panes",
            "action.reset_size" => "Reset size",

            // Session/quit actions
            "action.detach" => "Detach",
            "action.quit" => "Quit",
            "action.disconnect_other_clients" => "Disconnect other clients",

            // Other actions
            "action.undo" => "Undo rename",

            // Tips
            "tip.quicknav.open_new_pane" => "=> open new pane. ",
            "tip.quicknav.navigate" => "=> navigate between panes. ",
            "tip.quicknav.resize" => "=> increase/decrease pane size.",
            "tip.quicknav.open_new_pane_short" => "=> new pane. ",
            "tip.quicknav.navigate_short" => "=> navigate. ",
            "tip.quicknav.resize_short" => "=> resize pane.",

            "tip.floating_panes.toggle" => "Toggle floating panes",
            "tip.floating_panes.move" => "Move floating panes",
            "tip.mouse_click.info" => "Hold Shift to send mouse clicks to terminal",
            "tip.move_focus.info" => "Move between panes, switch tabs at boundaries",
            "tip.setup_check.info" => "Run 'zellij setup --check' for troubleshooting",
            "tip.use_mouse.info" => "Use the mouse to move between panes and resize",
            "tip.sync_tab.info" => "Input synced to all panes in tab",
            "tip.edit_scrollbuffer.info" => "Edit terminal scrollback in editor",
            "tip.compact_layout.info" => "Compact layout hides status bar",
            "tip.move_tabs.info" => "Move tabs with modifier+direction keys",

            // Fullscreen/floating pane indicators
            "indicator.fullscreen" => "FULLSCREEN",
            "indicator.hidden_panes" => " hidden panes",
            "indicator.floating_panes_visible" => "FLOATING PANES VISIBLE",
            "indicator.press" => "Press ",
            "indicator.to_embed" => " to embed or ",
            "indicator.to_toggle" => " to toggle",

            _ => "",
        }
    }
}
