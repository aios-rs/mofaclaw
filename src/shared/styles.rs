use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // ==================== FEISHU DESIGN SYSTEM ====================
    // 飞书设计风格配色方案

    // Icon Resources
    pub ICON_ADD             = dep("crate://self/resources/icons/add.svg")
    pub ICON_ADD_REACTION    = dep("crate://self/resources/icons/add_reaction.svg")
    pub ICON_ADD_USER        = dep("crate://self/resources/icons/add_user.svg") // TODO: FIX
    pub ICON_ADD_WALLET      = dep("crate://self/resources/icons/add_wallet.svg")
    pub ICON_FORBIDDEN       = dep("crate://self/resources/icons/forbidden.svg")
    pub ICON_CHECKMARK       = dep("crate://self/resources/icons/checkmark.svg")
    pub ICON_CLOSE           = dep("crate://self/resources/icons/close.svg")
    pub ICON_CLOUD_CHECKMARK = dep("crate://self/resources/icons/cloud_checkmark.svg")
    pub ICON_CLOUD_OFFLINE   = dep("crate://self/resources/icons/cloud_offline.svg")
    pub ICON_ROTATE_CW       = dep("crate://self/resources/icons/rotate_right_fa.svg")
    pub ICON_ROTATE_CCW      = dep("crate://self/resources/icons/rotate_left_fa.svg")
    pub ICON_COPY            = dep("crate://self/resources/icons/copy.svg")
    pub ICON_EDIT            = dep("crate://self/resources/icons/edit.svg")
    pub ICON_EXTERNAL_LINK   = dep("crate://self/resources/icons/external_link.svg")
    pub ICON_IMPORT          = dep("crate://self/resources/icons/import.svg") // TODO: FIX
    pub ICON_HIERARCHY       = dep("crate://self/resources/icons/hierarchy.svg")
    pub ICON_HOME            = dep("crate://self/resources/icons/home.svg")
    pub ICON_HTML_FILE       = dep("crate://self/resources/icons/html_file.svg")
    pub ICON_INFO            = dep("crate://self/resources/icons/info.svg")
    pub ICON_INVITE          = dep("crate://self/resources/icons/invite.svg")
    pub ICON_JOIN_ROOM       = dep("crate://self/resources/icons/join_room.svg")
    pub ICON_JUMP            = dep("crate://self/resources/icons/go_back.svg")
    pub ICON_LOGOUT          = dep("crate://self/resources/icons/logout.svg")
    pub ICON_LINK            = dep("crate://self/resources/icons/link.svg")
    pub ICON_PIN             = dep("crate://self/resources/icons/pin.svg")
    pub ICON_REPLY           = dep("crate://self/resources/icons/reply.svg")
    pub ICON_SEARCH          = dep("crate://self/resources/icons/search.svg")
    pub ICON_SEND            = dep("crate://self/resources/icon_send.svg")
    pub ICON_SETTINGS        = dep("crate://self/resources/icons/settings.svg")
    pub ICON_SQUARES         = dep("crate://self/resources/icons/squares_filled.svg")
    pub ICON_TOMBSTONE       = dep("crate://self/resources/icons/tombstone.svg")
    pub ICON_TRASH           = dep("crate://self/resources/icons/trash.svg")
    pub ICON_UPLOAD          = dep("crate://self/resources/icons/upload.svg")
    pub ICON_VIEW_SOURCE     = dep("crate://self/resources/icons/view_source.svg")
    pub ICON_WARNING         = dep("crate://self/resources/icons/warning.svg")
    pub ICON_ZOOM_IN         = dep("crate://self/resources/icons/zoom_in.svg")
    pub ICON_ZOOM_OUT        = dep("crate://self/resources/icons/zoom_out.svg")

    // ==================== COLOR PALETTE - 飞书风格 ====================

    // Primary Colors - 主要色彩
    pub COLOR_PRIMARY = #FFFFFF              // 白色 - 主背景
    pub COLOR_PRIMARY_DARKER = #FAFAFA       // 极浅灰 - 次级背景
    pub COLOR_SECONDARY = #F5F5F5            // 浅灰 - 侧边栏/次要区域
    pub COLOR_SECONDARY_DARKER = #E8E8E8     // 中灰 - 边框/分隔线

    // Accent Colors - 强调色
    pub COLOR_ACTIVE_PRIMARY = #1877F2       // 深蓝 - 主强调色（按钮、激活状态）
    pub COLOR_ACTIVE_PRIMARY_DARKER = #1465CC // 深蓝暗色 - 按下状态

    // Brand Purple - 飞书特色紫色
    pub COLOR_FEISHU_PURPLE = #6366F1        // 鲜艳紫 - 特殊元素高亮
    pub COLOR_FEISHU_PURPLE_DARKER = #4F46E5 // 紫色暗色

    // Brand Orange - 飞书特色橙色（非关键通知）
    pub COLOR_FEISHU_ORANGE = #FFA500        // 橙色 - 通知提醒
    pub COLOR_FEISHU_ORANGE_LIGHT = #FFD700  // 浅橙 - 轻提示

    // Legacy Colors (保留兼容)
    pub COLOR_ROBRIX_PURPLE = #6366F1        // 飞书风格紫色
    pub COLOR_ROBRIX_CYAN = #05CDC7          // 青色保留

    // Text Colors - 文字色彩
    pub COLOR_TEXT_PRIMARY = #333333         // 深灰 - 主要文字
    pub COLOR_TEXT_SECONDARY = #666666       // 中灰 - 次要文字
    pub COLOR_TEXT_TERTIARY = #999999        // 浅灰 - 辅助文字
    pub COLOR_TEXT_IDLE = #CCCCCC            // 极浅灰 - 禁用/闲置文字

    pub COLOR_TEXT = #333333                 // 默认文字颜色
    pub COLOR_TEXT_INPUT_IDLE = #CCCCCC      // 输入框占位符

    // Status Colors - 状态色彩
    pub COLOR_FG_ACCEPT_GREEN = #10B981      // 绿色 - 成功文字
    pub COLOR_BG_ACCEPT_GREEN = #ECFDF5      // 浅绿 - 成功背景
    pub COLOR_FG_DANGER_RED = #EF4444        // 红色 - 危险文字
    pub COLOR_BG_DANGER_RED = #FEF2F2        // 浅红 - 危险背景
    pub COLOR_FG_DISABLED = #B3B3B3          // 禁用文字
    pub COLOR_BG_DISABLED = #F3F4F6          // 禁用背景
    pub COLOR_WARNING_NOT_FOUND = #F59E0B    // 警告橙色

    // Special Colors - 特殊用途
    pub COLOR_SELECT_TEXT = #BFDBFE          // 文字选择背景（浅蓝）
    pub COLOR_LINK_HOVER = #1877F2           // 链接悬停
    pub COLOR_WARNING = #FBBF24              // 警告黄
    pub COLOR_TRANSPARENT = #00000000        // 透明

    // UI Component Colors - 组件色彩
    pub COLOR_DIVIDER = #E5E7EB              // 分隔线
    pub COLOR_DIVIDER_DARK = #D1D5DB         // 深色分隔线
    pub COLOR_META = #9CA3AF                 // 元信息颜色

    // Location Preview
    pub COLOR_LOCATION_PREVIEW_BG = #EFF6FF  // 位置预览背景（淡蓝）

    // Avatar Colors - 头像色彩
    pub COLOR_AVATAR_BG = #6366F1            // 默认头像背景（紫色）
    pub COLOR_AVATAR_BG_IDLE = #E5E7EB       // 闲置头像背景

    // Unread Badge Colors - 未读标记
    pub COLOR_UNREAD_BADGE_MENTIONS = #EF4444 // 提及未读（红）
    pub COLOR_UNREAD_BADGE_MARKED   = #6366F1 // 标记未读（紫）
    pub COLOR_UNREAD_BADGE_MESSAGES = #D1D5DB // 普通未读（灰）

    // Image Viewer
    pub COLOR_IMAGE_VIEWER_BACKGROUND = #1F2937CC // 图片查看器背景（80%深灰）
    pub COLOR_IMAGE_VIEWER_META_BACKGROUND = #F3F4F6 // 图片元信息背景

    // ==================== TYPOGRAPHY - 字体排版 ====================

    // Title Styles
    pub TITLE_TEXT = <THEME_FONT_REGULAR>{
        font_size: (15),
        font_size: 14.0,
    }

    pub REGULAR_TEXT = <THEME_FONT_REGULAR>{
        font_size: (11),
    }

    pub TEXT_SUB = <THEME_FONT_REGULAR>{
        font_size: (11),
    }

    // Username Styles
    pub USERNAME_FONT_SIZE = 13
    pub USERNAME_TEXT_COLOR = #333333
    pub USERNAME_TEXT_STYLE = <THEME_FONT_BOLD>{
        font_size: (USERNAME_FONT_SIZE),
    }

    // Message Styles
    pub MESSAGE_FONT_SIZE = 13
    pub MESSAGE_TEXT_COLOR = #333333
    pub COLOR_MESSAGE_NOTICE_TEXT = #999999
    pub MESSAGE_TEXT_LINE_SPACING = 1.6
    pub MESSAGE_TEXT_STYLE = <THEME_FONT_REGULAR>{
        font_size: (MESSAGE_FONT_SIZE),
        line_spacing: (MESSAGE_TEXT_LINE_SPACING),
    }

    pub MESSAGE_REPLY_PREVIEW_FONT_SIZE = 11

    // Small State Text
    pub SMALL_STATE_FONT_SIZE = 11
    pub SMALL_STATE_TEXT_COLOR = #999999
    pub SMALL_STATE_TEXT_STYLE = <THEME_FONT_REGULAR>{
        font_size: (SMALL_STATE_FONT_SIZE),
    }

    // Timestamp
    pub TIMESTAMP_FONT_SIZE = 10
    pub TIMESTAMP_TEXT_COLOR = #999999
    pub TIMESTAMP_TEXT_STYLE = <THEME_FONT_REGULAR>{
        font_size: (TIMESTAMP_FONT_SIZE),
    }

    // Typing Notice
    pub TYPING_NOTICE_TEXT_COLOR = #6366F1

    // Room Name
    pub ROOM_NAME_TEXT_COLOR = #111827

    // ==================== NAVIGATION - 导航样式 ====================

    pub NAVIGATION_TAB_BAR_SIZE = 68
    pub COLOR_NAVIGATION_TAB_FG        = #333333
    pub COLOR_NAVIGATION_TAB_FG_HOVER  = #333333
    pub COLOR_NAVIGATION_TAB_FG_ACTIVE = #1877F2
    pub COLOR_NAVIGATION_TAB_BG        = #FFFFFF
    pub COLOR_NAVIGATION_TAB_BG_HOVER  = #F5F5F5
    pub COLOR_NAVIGATION_TAB_BG_ACTIVE = #EFF6FF

    // ==================== UI COMPONENTS - 飞书风格组件 ====================

    // An icon that can be rotated at a custom angle.
    pub IconRotated = <Icon> {
        draw_icon: {
            instance rotation_angle: 0.0,

            // Support rotation of the icon
            fn clip_and_transform_vertex(self, rect_pos: vec2, rect_size: vec2) -> vec4 {
                let clipped: vec2 = clamp(
                    self.geom_pos * rect_size + rect_pos,
                    self.draw_clip.xy,
                    self.draw_clip.zw
                )
                self.pos = (clipped - rect_pos) / rect_size

                // Calculate the texture coordinates based on the rotation angle
                let angle_rad = self.rotation_angle * 3.14159265359 / 180.0;
                let cos_angle = cos(angle_rad);
                let sin_angle = sin(angle_rad);
                let rot_matrix = mat2(
                    cos_angle, -sin_angle,
                    sin_angle, cos_angle
                );
                self.tex_coord1 = mix(
                    self.icon_t1.xy,
                    self.icon_t2.xy,
                    (rot_matrix * (self.pos.xy - vec2(0.5))) + vec2(0.5)
                );

                return self.camera_projection * (self.camera_view * (self.view_transform * vec4(
                    clipped.x,
                    clipped.y,
                    self.draw_depth + self.draw_zbias,
                    1.
                )))
            }
        }
    }

    // ==================== TEXT INPUT - 飞书风格输入框 ====================
    // 飞书风格：圆角8px，淡灰色边框，聚焦时蓝色边框高亮

    // A text input widget styled for Feishu/Robrix.
    pub RobrixTextInput = <TextInput> {
        width: Fill, height: Fit,
        margin: 0,
        align: {y: 0.5}

        draw_bg: {
            color: (COLOR_PRIMARY)
            border_radius: 8.0              // 飞书风格圆角
            border_size: 1.0                 // 飞书风格边框

            color_hover: (COLOR_PRIMARY)
            color_focus: (COLOR_PRIMARY)
            color_down: (COLOR_PRIMARY)
            color_empty: (COLOR_PRIMARY)
            color_disabled: (COLOR_BG_DISABLED)

            border_color: #E5E7EB            // 默认边框（浅灰）
            border_color_focus: #1877F2      // 聚焦边框（飞书蓝）
            border_color_hover: #D1D5DB      // 悬停边框
        }

        draw_selection: {
            color: (COLOR_SELECT_TEXT)
            color_hover:  (COLOR_SELECT_TEXT)
            color_focus:  (COLOR_SELECT_TEXT)
            color_down:  (COLOR_SELECT_TEXT)
            color_empty:  (COLOR_SELECT_TEXT)
            color_disabled: (COLOR_SELECT_TEXT)
        }

        draw_cursor: {
            color: (COLOR_ACTIVE_PRIMARY)    // 飞书蓝色光标
        }

        draw_text: {
            text_style: <MESSAGE_TEXT_STYLE>{},
            color: (MESSAGE_TEXT_COLOR),
            uniform color_hover: (MESSAGE_TEXT_COLOR),
            uniform color_focus: (MESSAGE_TEXT_COLOR),
            uniform color_down: (MESSAGE_TEXT_COLOR),
            uniform color_disabled: (COLOR_FG_DISABLED),
            uniform color_empty: #CCCCCC,     // 占位符颜色
            uniform color_empty_hover: #CCCCCC,
            uniform color_empty_focus: #CCCCCC,

            fn get_color(self) -> vec4 {
                return mix(
                    mix(
                        mix(
                            mix(
                                self.color,
                                mix(
                                    self.color_hover,
                                    self.color_down,
                                    self.down
                                ),
                                self.hover
                            ),
                            self.color_focus,
                            self.focus
                        ),
                        self.color_empty,
                        self.empty
                    ),
                    self.color_disabled,
                    self.disabled
                )
            }
        }
    }

    // Simple text input with Feishu styling
    pub SimpleTextInput = <RobrixTextInput> {
        padding: 12,                          // 飞书风格内边距
        width: Fill, height: Fit
        flow: RightWrap,
        draw_bg: {
            color: (COLOR_PRIMARY)
            border_radius: 8.0                // 飞书风格圆角
            border_size: 1.0                   // 飞书风格边框

            color: (COLOR_PRIMARY)
            color_hover: (COLOR_PRIMARY)
            color_focus: (COLOR_PRIMARY)
            color_down: (COLOR_PRIMARY)
            color_empty: (COLOR_PRIMARY)
            color_disabled: (COLOR_BG_DISABLED)

            border_color: #E5E7EB              // 默认边框（浅灰）
            border_color_hover: #1877F2        // 悬停边框（飞书蓝）
            border_color_focus: #1877F2        // 聚焦边框（飞书蓝）
            border_color_down: #1465CC         // 按下边框
            border_color_empty: #E5E7EB        // 空状态边框
            border_color_disabled: #D1D5DB     // 禁用边框

            border_color_2: #E5E7EB
            border_color_2_hover: #1877F2
            border_color_2_focus: #1877F2
            border_color_2_down: #1465CC
            border_color_2_empty: #E5E7EB
            border_color_2_disabled: #D1D5DB
        }
        draw_text: {
            wrap: Word,
        }
        empty_text: "Add a display name..."
    }
}

// ==================== RUST CONSTANTS - 飞书风格颜色常量 ====================

pub const NAVIGATION_TAB_BAR_SIZE: f64 = 68.0;
pub const REDACTED_MESSAGE_FONT_SIZE: f32 = 11.0;

// Primary Colors - 主要色彩
/// #FFFFFF
pub const COLOR_PRIMARY:               Vec4 = vec4(1.0, 1.0, 1.0, 1.0);
/// #1877F2 - 飞书蓝色
pub const COLOR_ACTIVE_PRIMARY:        Vec4 = vec4(0.094, 0.467, 0.949, 1.0);
/// #1465CC - 飞书蓝色暗色
pub const COLOR_ACTIVE_PRIMARY_DARKER: Vec4 = vec4(0.078, 0.396, 0.8, 1.0);
/// #6366F1 - 飞书紫色
pub const COLOR_ROBRIX_PURPLE:         Vec4 = vec4(0.388, 0.4, 0.945, 1.0);
/// #05CDC7
pub const COLOR_ROBRIX_CYAN:           Vec4 = vec4(0.031, 0.804, 0.78, 1.0);

// Status Colors - 状态色彩
/// #10B981 - 绿色文字
pub const COLOR_FG_ACCEPT_GREEN:       Vec4 = vec4(0.063, 0.725, 0.506, 1.0);
/// #ECFDF5 - 绿色背景
pub const COLOR_BG_ACCEPT_GREEN:       Vec4 = vec4(0.925, 0.992, 0.961, 1.0);
/// #B3B3B3 - 禁用文字
pub const COLOR_FG_DISABLED:           Vec4 = vec4(0.7, 0.7, 0.7, 1.0);
/// #F3F4F6 - 禁用背景
pub const COLOR_BG_DISABLED:           Vec4 = vec4(0.953, 0.957, 0.965, 1.0);
/// #EF4444 - 红色文字
pub const COLOR_FG_DANGER_RED:         Vec4 = vec4(0.937, 0.267, 0.267, 1.0);
/// #FEF2F2 - 红色背景
pub const COLOR_BG_DANGER_RED:         Vec4 = vec4(0.996, 0.949, 0.949, 1.0);
/// #F59E0B - 警告橙
pub const COLOR_WARNING_NOT_FOUND:     Vec4 = vec4(0.961, 0.620, 0.043, 1.0);

// Badge Colors - 标记颜色
/// #EF4444 - 红色标记
pub const COLOR_UNREAD_BADGE_MENTIONS: Vec4 = vec4(0.937, 0.267, 0.267, 1.0);
/// #6366F1 - 紫色标记
pub const COLOR_UNREAD_BADGE_MARKED:   Vec4 = COLOR_ROBRIX_PURPLE;
/// #D1D5DB - 灰色标记
pub const COLOR_UNREAD_BADGE_MESSAGES: Vec4 = vec4(0.82, 0.835, 0.859, 1.0);
/// #FF6E00 - 未知房间头像
pub const COLOR_UNKNOWN_ROOM_AVATAR:   Vec4 = vec4(1.0, 0.431, 0.0, 1.0);

// Utility Colors
/// #FBBF24 - 警告黄
pub const COLOR_WARNING_YELLOW:        Vec4 = vec4(0.984, 0.749, 0.141, 1.0);
/// #1877F2 - 信息蓝
pub const COLOR_INFO_BLUE:             Vec4 = vec4(0.094, 0.467, 0.949, 1.0);
/// #FFFFFF
pub const COLOR_WHITE:                 Vec4 = vec4(1.0, 1.0, 1.0, 1.0);
/// #999999 - 消息通知文字
pub const COLOR_MESSAGE_NOTICE_TEXT:   Vec4 = vec4(0.6, 0.6, 0.6, 1.0);
