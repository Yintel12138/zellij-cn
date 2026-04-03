/// Translation support for session-manager plugin
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
            // Session states
            "session.connected" => "已连接",
            "session.exited" => "已退出",
            "session.active" => "活跃",

            // Actions
            "session.create" => "创建",
            "session.attach" => "附加",
            "session.switch" => "切换到",
            "session.kill" => "终止",
            "session.resurrect" => "恢复",
            "session.delete" => "删除",
            "session.rename" => "重命名",

            // UI labels
            "session.sessions" => "会话列表",
            "session.no_sessions" => "未找到会话",
            "session.resurrect_title" => "恢复会话:",
            "session.new_name_prompt" => "新名称:",
            "session.new_session" => "新建会话",
            "session.new_session_prefix" => "新建会话: ",

            // Search
            "session.search" => "搜索:",
            "session.search_placeholder" => "搜索...",

            // Sort options
            "session.sort_by" => "排序方式:",
            "session.sort_alphabetically" => "按字母",
            "session.sort_creation_time" => "创建时间",
            "session.sort_activity" => "活跃度",

            // Controls
            "session.controls.up_down" => "上/下",
            "session.controls.select" => "选择",
            "session.controls.navigate" => "导航",
            "session.controls.enter" => "确认",
            "session.controls.esc" => "取消",
            "session.controls.tab" => "切换",
            "session.controls.scroll" => "滚动",
            "session.controls.new" => "新建",
            "session.controls.rename" => "重命名",
            "session.controls.disconnect" => "断开",
            "session.controls.kill" => "终止",
            "session.controls.kill_all" => "终止全部",
            "session.controls.delete" => "删除",
            "session.controls.delete_all" => "删除全部",
            "session.controls.back" => "返回",

            // Headers
            "session.header.name" => "名称",
            "session.header.tabs" => "标签",
            "session.header.panes" => "面板",
            "session.header.status" => "状态",

            // Prompts
            "session.prompt.enter_name" => "请输入会话名称...",
            "session.prompt.new_session" => "新建会话:",
            "session.prompt.folder" => "文件夹:",
            "session.prompt.layout" => "布局:",
            "session.prompt.browse" => "浏览",

            // Warning messages
            "session.warning.kill_all" => "确定要终止所有会话吗?",
            "session.warning.delete_all" => "确定要删除所有会话吗?",
            "session.warning.yes" => "是",
            "session.warning.no" => "否",

            // Status indicators
            "session.status.current" => "(当前)",
            "session.status.this" => "当前会话",
            "session.status.other_attached" => "已附加",

            // Welcome screen
            "session.welcome.title" => "欢迎使用 Zellij!",
            "session.welcome.new_session" => "新建会话",
            "session.welcome.recent_sessions" => "最近的会话",

            // Misc
            "session.ago" => "前",
            "session.just_now" => "刚刚",
            "session.unsaved_changes" => "未保存的更改",
            "session.last_saved" => "上次保存:",

            _ => Self::en(key),
        }
    }

    fn en(key: &str) -> &'static str {
        match key {
            // Session states
            "session.connected" => "CONNECTED",
            "session.exited" => "EXITED",
            "session.active" => "Active",

            // Actions
            "session.create" => "Create",
            "session.attach" => "Attach",
            "session.switch" => "Switch to",
            "session.kill" => "Kill",
            "session.resurrect" => "Resurrect",
            "session.delete" => "Delete",
            "session.rename" => "Rename",

            // UI labels
            "session.sessions" => "SESSIONS",
            "session.no_sessions" => "No sessions found",
            "session.resurrect_title" => "Resurrect a session:",
            "session.new_name_prompt" => "New name:",
            "session.new_session" => "New session",
            "session.new_session_prefix" => "New session: ",

            // Search
            "session.search" => "Search:",
            "session.search_placeholder" => "Search...",

            // Sort options
            "session.sort_by" => "Sort by:",
            "session.sort_alphabetically" => "Alphabetically",
            "session.sort_creation_time" => "Creation Time",
            "session.sort_activity" => "Activity",

            // Controls
            "session.controls.up_down" => "Up/Down",
            "session.controls.select" => "Select",
            "session.controls.navigate" => "Navigate",
            "session.controls.enter" => "Enter",
            "session.controls.esc" => "Cancel",
            "session.controls.tab" => "Switch",
            "session.controls.scroll" => "Scroll",
            "session.controls.new" => "New",
            "session.controls.rename" => "Rename",
            "session.controls.disconnect" => "Disconnect",
            "session.controls.kill" => "Kill",
            "session.controls.kill_all" => "Kill All",
            "session.controls.delete" => "Delete",
            "session.controls.delete_all" => "Delete All",
            "session.controls.back" => "Back",

            // Headers
            "session.header.name" => "NAME",
            "session.header.tabs" => "TABS",
            "session.header.panes" => "PANES",
            "session.header.status" => "STATUS",

            // Prompts
            "session.prompt.enter_name" => "Enter session name...",
            "session.prompt.new_session" => "New session:",
            "session.prompt.folder" => "Folder:",
            "session.prompt.layout" => "Layout:",
            "session.prompt.browse" => "Browse",

            // Warning messages
            "session.warning.kill_all" => "Are you sure you want to kill all sessions?",
            "session.warning.delete_all" => "Are you sure you want to delete all sessions?",
            "session.warning.yes" => "Yes",
            "session.warning.no" => "No",

            // Status indicators
            "session.status.current" => "(current)",
            "session.status.this" => "This session",
            "session.status.other_attached" => "Attached",

            // Welcome screen
            "session.welcome.title" => "Welcome to Zellij!",
            "session.welcome.new_session" => "New Session",
            "session.welcome.recent_sessions" => "Recent Sessions",

            // Misc
            "session.ago" => "ago",
            "session.just_now" => "just now",
            "session.unsaved_changes" => "Unsaved changes",
            "session.last_saved" => "Last saved:",

            _ => "",
        }
    }
}
