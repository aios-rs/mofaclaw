use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;

    // Chat message bubble style for user messages
    pub UserMessageBubble = <RoundedView> {
        width: Fill, height: Fit
        margin: {top: 8, bottom: 8, left: 40, right: 16}
        padding: 12
        show_bg: true
        draw_bg: {
            color: (COLOR_ACTIVE_PRIMARY)
            border_radius: 12.0
        }

        text = <Label> {
            width: Fill, height: Fit
            draw_text: {
                color: #xFFFFFF
                text_style: <THEME_FONT_REGULAR> {
                    font_size: 13.0
                    line_spacing: 1.6
                }
                wrap: Word
            }
        }
    }

    // Chat message bubble style for AI messages
    pub AiMessageBubble = <RoundedView> {
        width: Fill, height: Fit
        margin: {top: 8, bottom: 8, left: 16, right: 40}
        padding: 12
        show_bg: true
        draw_bg: {
            color: (COLOR_SECONDARY)
            border_radius: 12.0
        }

        text = <Label> {
            width: Fill, height: Fit
            draw_text: {
                color: (COLOR_TEXT_PRIMARY)
                text_style: <THEME_FONT_REGULAR> {
                    font_size: 13.0
                    line_spacing: 1.6
                }
                wrap: Word
            }
        }
    }

    pub ChatAssistantScreen = {{ChatAssistantScreen}} {
        width: Fill, height: Fill
        flow: Down
        show_bg: true
        draw_bg: {
            color: (COLOR_PRIMARY)
        }

        // Top toolbar with new chat and deep think buttons
        top_toolbar = <RoundedView> {
            width: Fill, height: Fit
            padding: 12
            margin: {bottom: 8}
            spacing: 8
            flow: Right
            align: {x: 0.0, y: 0.5}
            show_bg: true
            draw_bg: {
                color: (COLOR_SECONDARY)
                border_radius: 0.0
            }

            new_chat_button = <RobrixIconButton> {
                width: Fit, height: Fit
                padding: 8
                draw_bg: {
                    color: (COLOR_PRIMARY)
                    border_radius: 8.0
                }
                draw_icon: {
                    svg_file: (ICON_ADD)
                    color: (COLOR_TEXT_PRIMARY)
                }
                icon_walk: {width: 18, height: Fit}
                text: "New Chat"
            }

            deep_think_button = <RobrixIconButton> {
                width: Fit, height: Fit
                padding: 8
                draw_bg: {
                    color: (COLOR_PRIMARY)
                    border_radius: 8.0
                }
                draw_icon: {
                    svg_file: (ICON_SEARCH)
                    color: (COLOR_TEXT_PRIMARY)
                }
                icon_walk: {width: 18, height: Fit}
                text: "Deep Think"
            }
        }

        // Message list area
        messages_container = <View> {
            width: Fill, height: Fill
            flow: Down

            messages_scroll = <ScrollYView> {
                width: Fill, height: Fill
                flow: Down

                message_list = <View> {
                    width: Fill, height: Fit
                    flow: Down

                    // Welcome message
                    welcome_message = <AiMessageBubble> {
                        text = {
                            text: "Hello! I'm your AI assistant. How can I help you today?"
                        }
                    }
                }
            }
        }

        // Input area
        input_area = <RoundedView> {
            width: Fill, height: Fit
            padding: 12
            margin: {top: 8}
            flow: Right
            spacing: 8
            align: {x: 0.0, y: 0.5}
            show_bg: true
            draw_bg: {
                color: (COLOR_SECONDARY)
                border_radius: 0.0
            }

            message_input = <RobrixTextInput> {
                width: Fill, height: Fit
                empty_text: "Type your message..."
            }

            send_button = <RobrixIconButton> {
                width: Fit, height: Fit
                padding: 10
                draw_bg: {
                    color: (COLOR_ACTIVE_PRIMARY)
                    border_radius: 8.0
                }
                draw_icon: {
                    svg_file: (ICON_SEND)
                    color: #xFFFFFF
                }
                icon_walk: {width: 18, height: Fit}
            }
        }
    }
}

/// Actions for the chat assistant screen.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ChatAssistantAction {
    NewChat,
    DeepThink,
    SendMessage(String),
}

/// The chat assistant screen widget.
#[derive(Live, Widget)]
pub struct ChatAssistantScreen {
    #[deref] view: View,

    /// Stores the messages in the chat.
    #[rust] messages: Vec<ChatMessage>,
}

/// Represents a single chat message.
#[derive(Clone, Debug)]
#[allow(dead_code)]
struct ChatMessage {
    is_user: bool,
    content: String,
}

impl LiveHook for ChatAssistantScreen {
    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        // Initialize with welcome message
        self.messages.push(ChatMessage {
            is_user: false,
            content: "Hello! I'm your AI assistant. How can I help you today?".to_string(),
        });
    }
}

impl Widget for ChatAssistantScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            for action in actions {
                match action.downcast_ref() {
                    Some(ChatAssistantAction::NewChat) => {
                        self.messages.clear();
                        self.messages.push(ChatMessage {
                            is_user: false,
                            content: "Hello! I'm your AI assistant. How can I help you today?".to_string(),
                        });
                        self.redraw(cx);
                    }
                    Some(ChatAssistantAction::DeepThink) => {
                        // Handle deep think mode
                        cx.action(ChatAssistantAction::DeepThink);
                    }
                    Some(ChatAssistantAction::SendMessage(msg)) => {
                        // Add user message
                        self.messages.push(ChatMessage {
                            is_user: true,
                            content: msg.clone(),
                        });

                        // Add AI response (placeholder for now)
                        self.messages.push(ChatMessage {
                            is_user: false,
                            content: format!("I received your message: \"{}\"", msg),
                        });

                        self.redraw(cx);
                    }
                    None => {}
                }
            }

            // Handle button clicks
            if self.view.button(ids!(new_chat_button)).clicked(actions) {
                cx.action(ChatAssistantAction::NewChat);
            }

            if self.view.button(ids!(deep_think_button)).clicked(actions) {
                cx.action(ChatAssistantAction::DeepThink);
            }

            if self.view.button(ids!(send_button)).clicked(actions) {
                let input_text = self.view.text_input(ids!(message_input)).text();
                if !input_text.is_empty() {
                    cx.action(ChatAssistantAction::SendMessage(input_text));
                    // Clear the input
                    self.view.text_input(ids!(message_input)).set_text(cx, "");
                }
            }

            // Handle Enter key in text input
            for action in actions {
                if let Some(TextInputAction::Returned(_text, _)) = action.downcast_ref() {
                    let input_text = self.view.text_input(ids!(message_input)).text();
                    if !input_text.is_empty() {
                        cx.action(ChatAssistantAction::SendMessage(input_text));
                        // Clear the input
                        self.view.text_input(ids!(message_input)).set_text(cx, "");
                    }
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
