use strum::IntoEnumIterator;
use strum_macros::AsRefStr;
use strum_macros::EnumIter;
use strum_macros::EnumString;
use strum_macros::IntoStaticStr;

/// Commands that can be invoked by starting a message with a leading slash.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, EnumIter, AsRefStr, IntoStaticStr,
)]
#[strum(serialize_all = "kebab-case")]
pub enum SlashCommand {
    // DO NOT ALPHA-SORT! Enum order is presentation order in the popup, so
    // more frequently used commands should be listed first.
    Model,
    Approvals,
    Permissions,
    #[strum(serialize = "setup-default-sandbox")]
    ElevateSandbox,
    #[strum(serialize = "sandbox-add-read-dir")]
    SandboxReadRoot,
    Experimental,
    Skills,
    Review,
    Rename,
    New,
    Resume,
    Fork,
    Init,
    Compact,
    Plan,
    Collab,
    Agent,
    // Undo,
    Diff,
    Mention,
    Status,
    DebugConfig,
    Statusline,
    Mcp,
    Apps,
    Logout,
    Quit,
    Exit,
    Feedback,
    Rollout,
    Ps,
    Clean,
    Personality,
    TestApproval,
    // Debugging commands.
    #[strum(serialize = "debug-m-drop")]
    MemoryDrop,
    #[strum(serialize = "debug-m-update")]
    MemoryUpdate,
}

impl SlashCommand {
    /// User-visible description shown in the popup.
    pub fn description(self) -> &'static str {
        if crate::i18n::is_chinese() {
            match self {
                SlashCommand::Feedback => "向维护者发送日志",
                SlashCommand::New => "在当前会话中开始新的聊天",
                SlashCommand::Init => "创建用于指导 Codex 的 AGENTS.md 文件",
                SlashCommand::Compact => "总结会话以避免触达上下文上限",
                SlashCommand::Review => "审查当前改动并查找问题",
                SlashCommand::Rename => "重命名当前线程",
                SlashCommand::Resume => "恢复一个已保存的聊天",
                SlashCommand::Fork => "从当前聊天分叉",
                // SlashCommand::Undo => "让 Codex 撤销一个回合",
                SlashCommand::Quit | SlashCommand::Exit => "退出 Codex",
                SlashCommand::Diff => "显示 git diff（含未跟踪文件）",
                SlashCommand::Mention => "提及一个文件",
                SlashCommand::Skills => "使用技能提升 Codex 在特定任务上的表现",
                SlashCommand::Status => "显示当前会话配置和 token 用量",
                SlashCommand::DebugConfig => "显示配置层与需求来源（用于调试）",
                SlashCommand::Statusline => "配置状态栏显示项",
                SlashCommand::Ps => "列出后台终端",
                SlashCommand::Clean => "停止所有后台终端",
                SlashCommand::MemoryDrop => "请勿使用",
                SlashCommand::MemoryUpdate => "请勿使用",
                SlashCommand::Model => "选择模型和推理强度",
                SlashCommand::Personality => "选择 Codex 的沟通风格",
                SlashCommand::Plan => "切换到规划模式",
                SlashCommand::Collab => "切换协作模式（实验）",
                SlashCommand::Agent => "切换当前活跃代理线程",
                SlashCommand::Approvals => "设置 Codex 可执行的操作范围",
                SlashCommand::Permissions => "设置 Codex 可执行的操作范围",
                SlashCommand::ElevateSandbox => "设置更高权限的代理沙箱",
                SlashCommand::SandboxReadRoot => {
                    "允许沙箱读取目录：/sandbox-add-read-dir <absolute_path>"
                }
                SlashCommand::Experimental => "切换实验功能",
                SlashCommand::Mcp => "列出已配置的 MCP 工具",
                SlashCommand::Apps => "管理应用",
                SlashCommand::Logout => "退出 Codex 账号",
                SlashCommand::Rollout => "打印 rollout 文件路径",
                SlashCommand::TestApproval => "测试审批请求",
            }
        } else {
            match self {
                SlashCommand::Feedback => "send logs to maintainers",
                SlashCommand::New => "start a new chat during a conversation",
                SlashCommand::Init => "create an AGENTS.md file with instructions for Codex",
                SlashCommand::Compact => {
                    "summarize conversation to prevent hitting the context limit"
                }
                SlashCommand::Review => "review my current changes and find issues",
                SlashCommand::Rename => "rename the current thread",
                SlashCommand::Resume => "resume a saved chat",
                SlashCommand::Fork => "fork the current chat",
                // SlashCommand::Undo => "ask Codex to undo a turn",
                SlashCommand::Quit | SlashCommand::Exit => "exit Codex",
                SlashCommand::Diff => "show git diff (including untracked files)",
                SlashCommand::Mention => "mention a file",
                SlashCommand::Skills => "use skills to improve how Codex performs specific tasks",
                SlashCommand::Status => "show current session configuration and token usage",
                SlashCommand::DebugConfig => {
                    "show config layers and requirement sources for debugging"
                }
                SlashCommand::Statusline => "configure which items appear in the status line",
                SlashCommand::Ps => "list background terminals",
                SlashCommand::Clean => "stop all background terminals",
                SlashCommand::MemoryDrop => "DO NOT USE",
                SlashCommand::MemoryUpdate => "DO NOT USE",
                SlashCommand::Model => "choose what model and reasoning effort to use",
                SlashCommand::Personality => "choose a communication style for Codex",
                SlashCommand::Plan => "switch to Plan mode",
                SlashCommand::Collab => "change collaboration mode (experimental)",
                SlashCommand::Agent => "switch the active agent thread",
                SlashCommand::Approvals => "choose what Codex is allowed to do",
                SlashCommand::Permissions => "choose what Codex is allowed to do",
                SlashCommand::ElevateSandbox => "set up elevated agent sandbox",
                SlashCommand::SandboxReadRoot => {
                    "let sandbox read a directory: /sandbox-add-read-dir <absolute_path>"
                }
                SlashCommand::Experimental => "toggle experimental features",
                SlashCommand::Mcp => "list configured MCP tools",
                SlashCommand::Apps => "manage apps",
                SlashCommand::Logout => "log out of Codex",
                SlashCommand::Rollout => "print the rollout file path",
                SlashCommand::TestApproval => "test approval request",
            }
        }
    }

    /// Command string without the leading '/'. Provided for compatibility with
    /// existing code that expects a method named `command()`.
    pub fn command(self) -> &'static str {
        self.into()
    }

    /// Whether this command supports inline args (for example `/review ...`).
    pub fn supports_inline_args(self) -> bool {
        matches!(
            self,
            SlashCommand::Review
                | SlashCommand::Rename
                | SlashCommand::Plan
                | SlashCommand::SandboxReadRoot
        )
    }

    /// Whether this command can be run while a task is in progress.
    pub fn available_during_task(self) -> bool {
        match self {
            SlashCommand::New
            | SlashCommand::Resume
            | SlashCommand::Fork
            | SlashCommand::Init
            | SlashCommand::Compact
            // | SlashCommand::Undo
            | SlashCommand::Model
            | SlashCommand::Personality
            | SlashCommand::Approvals
            | SlashCommand::Permissions
            | SlashCommand::ElevateSandbox
            | SlashCommand::SandboxReadRoot
            | SlashCommand::Experimental
            | SlashCommand::Review
            | SlashCommand::Plan
            | SlashCommand::Logout
            | SlashCommand::MemoryDrop
            | SlashCommand::MemoryUpdate => false,
            SlashCommand::Diff
            | SlashCommand::Rename
            | SlashCommand::Mention
            | SlashCommand::Skills
            | SlashCommand::Status
            | SlashCommand::DebugConfig
            | SlashCommand::Ps
            | SlashCommand::Clean
            | SlashCommand::Mcp
            | SlashCommand::Apps
            | SlashCommand::Feedback
            | SlashCommand::Quit
            | SlashCommand::Exit => true,
            SlashCommand::Rollout => true,
            SlashCommand::TestApproval => true,
            SlashCommand::Collab => true,
            SlashCommand::Agent => true,
            SlashCommand::Statusline => false,
        }
    }

    fn is_visible(self) -> bool {
        match self {
            SlashCommand::SandboxReadRoot => cfg!(target_os = "windows"),
            SlashCommand::Rollout | SlashCommand::TestApproval => cfg!(debug_assertions),
            _ => true,
        }
    }
}

/// Return all built-in commands in a Vec paired with their command string.
pub fn built_in_slash_commands() -> Vec<(&'static str, SlashCommand)> {
    SlashCommand::iter()
        .filter(|command| command.is_visible())
        .map(|c| (c.command(), c))
        .collect()
}
