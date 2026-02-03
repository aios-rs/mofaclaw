use makepad_widgets::*;

use crate::llm_client::{
    LlmAction, LlmClient, ChatMessage as LlmChatMessage, DEFAULT_API_BASE, DEFAULT_MODEL,
};

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;
    use crate::shared::icon_button::*;

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
                color: (COLOR_TEXT)
                text_style: <THEME_FONT_REGULAR> {
                    font_size: 13.0
                    line_spacing: 1.6
                }
                wrap: Word
            }
        }
    }

    // Loading indicator for AI typing
    pub AiTypingIndicator = <RoundedView> {
        width: Fit, height: Fit
        margin: {top: 8, bottom: 8, left: 16, right: 40}
        padding: 12
        show_bg: true
        draw_bg: {
            color: (COLOR_SECONDARY)
            border_radius: 12.0
        }

        dots = <View> {
            width: Fit, height: Fit
            flow: Right
            spacing: 4

            dot1 = <RoundedView> {
                width: 8, height: 8
                show_bg: true
                draw_bg: {
                    color: (COLOR_TEXT_IDLE)
                    border_radius: 4.0
                }
            }
            dot2 = <RoundedView> {
                width: 8, height: 8
                show_bg: true
                draw_bg: {
                    color: (COLOR_TEXT_IDLE)
                    border_radius: 4.0
                }
            }
            dot3 = <RoundedView> {
                width: 8, height: 8
                show_bg: true
                draw_bg: {
                    color: (COLOR_TEXT_IDLE)
                    border_radius: 4.0
                }
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
                    color: (COLOR_TEXT)
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
                    color: (COLOR_TEXT)
                }
                icon_walk: {width: 18, height: Fit}
                text: "Deep Think"
            }

            <View> { width: Fill }

            // API Key configuration button
            api_key_button = <RobrixIconButton> {
                width: Fit, height: Fit
                padding: 8
                draw_bg: {
                    color: (COLOR_PRIMARY)
                    border_radius: 8.0
                }
                draw_icon: {
                    svg_file: (ICON_SETTINGS)
                    color: (COLOR_TEXT)
                }
                icon_walk: {width: 18, height: Fit}
                text: "API Key"
            }
        }

        // Message list area using PortalList for efficient rendering
        messages_container = <View> {
            width: Fill, height: Fill
            flow: Down

            messages_scroll = <ScrollYView> {
                width: Fill, height: Fill
                flow: Down

                message_list = <PortalList> {
                    width: Fill, height: Fit
                    flow: Down

                    UserMessageBubble = <UserMessageBubble> {}
                    AiMessageBubble = <AiMessageBubble> {}
                }
            }
        }

        // Loading indicator
        loading_indicator = <AiTypingIndicator> {
            visible: false
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

        // API Key input modal
        api_key_modal = <RoundedView> {
            width: Fill, height: Fill
            visible: false
            show_bg: true
            draw_bg: {
                color: #x00000080
            }

            modal_content = <RoundedView> {
                width: 400, height: Fit
                align: {x: 0.5, y: 0.5}
                padding: 24
                show_bg: true
                draw_bg: {
                    color: (COLOR_SECONDARY)
                    border_radius: 12.0
                }

                <Label> {
                    text: "Enter API Key"
                    draw_text: {
                color: (COLOR_TEXT)
                        text_style: <THEME_FONT_BOLD> {
                            font_size: 16.0
                        }
                    }
                }

                api_key_input = <RobrixTextInput> {
                    width: Fill, height: Fit
                    margin: {top: 16, bottom: 16}
                    empty_text: "sk-..."
                }

                <View> {
                    width: Fill, height: Fit
                    flow: Right
                    spacing: 8

                    cancel_api_key_button = <Button> {
                        width: Fit, height: Fit
                        padding: 12
                        text: "Cancel"
                        draw_bg: {
                            color: (COLOR_PRIMARY)
                            border_radius: 8.0
                        }
                    }

                    save_api_key_button = <Button> {
                        width: Fit, height: Fit
                        padding: 12
                        text: "Save"
                        draw_bg: {
                            color: (COLOR_ACTIVE_PRIMARY)
                            border_radius: 8.0
                        }
                        draw_text: {
                            color: #xFFFFFF
                        }
                    }
                }
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
    SetApiKey(String),
}

/// Internal message type for chat
#[derive(Clone, Debug)]
pub struct ChatMessage {
    is_user: bool,
    content: String,
    is_streaming: bool,
}

/// The chat assistant screen widget.
#[derive(Live, Widget)]
pub struct ChatAssistantScreen {
    #[deref]
    view: View,

    /// Stores the messages in the chat.
    #[rust]
    messages: Vec<ChatMessage>,

    /// LLM client for API calls
    #[rust]
    llm_client: Option<LlmClient>,

    /// API key for LLM service
    #[rust]
    api_key: String,

    /// Whether the AI is currently responding
    #[rust]
    is_loading: bool,

    /// Whether deep think mode is enabled
    #[rust]
    deep_think_mode: bool,
}

impl LiveHook for ChatAssistantScreen {
    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        // Initialize with welcome message
        self.messages.push(ChatMessage {
            is_user: false,
            content: "Hello! I'm your AI assistant. How can I help you today?".to_string(),
            is_streaming: false,
        });

        // Try to load API key from environment or storage
        if let Ok(key) = std::env::var("OPENAI_API_KEY") {
            self.api_key = key;
            self.init_llm_client();
        }
    }
}

impl ChatAssistantScreen {
    fn init_llm_client(&mut self) {
        if !self.api_key.is_empty() {
            let base =
                std::env::var("OPENAI_API_BASE").unwrap_or_else(|_| DEFAULT_API_BASE.to_string());
            let model = std::env::var("OPENAI_MODEL").unwrap_or_else(|_| DEFAULT_MODEL.to_string());

            self.llm_client = Some(LlmClient::new(base, self.api_key.clone(), model));
        }
    }

    fn show_api_key_modal(&mut self, cx: &mut Cx) {
        self.view.view(ids!(api_key_modal)).set_visible(cx, true);
    }

    fn hide_api_key_modal(&mut self, cx: &mut Cx) {
        self.view.view(ids!(api_key_modal)).set_visible(cx, false);
    }

    fn send_message_to_llm(&mut self, cx: &mut Cx, user_message: String) {
        if self.llm_client.is_none() {
            // Add error message
            self.messages.push(ChatMessage {
                is_user: false,
                content: "Please configure your API key first by clicking the 'API Key' button."
                    .to_string(),
                is_streaming: false,
            });
            self.redraw(cx);
            return;
        }

        // Set loading state
        self.is_loading = true;
        self.view
            .view(ids!(loading_indicator))
            .set_visible(cx, true);
        self.redraw(cx);

        // Build conversation history
        let mut llm_messages: Vec<LlmChatMessage> = vec![LlmChatMessage {
            role: "system".to_string(),
            content: if self.deep_think_mode {
                "You are a helpful AI assistant. Think deeply and provide detailed responses."
                    .to_string()
            } else {
                "You are a helpful AI assistant.".to_string()
            },
        }];

        // Add previous messages
        for msg in &self.messages {
            llm_messages.push(LlmChatMessage {
                role: if msg.is_user {
                    "user".to_string()
                } else {
                    "assistant".to_string()
                },
                content: msg.content.clone(),
            });
        }

        // Add current user message
        llm_messages.push(LlmChatMessage {
            role: "user".to_string(),
            content: user_message,
        });

        // Add empty AI message placeholder
        let ai_message_index = self.messages.len();
        self.messages.push(ChatMessage {
            is_user: false,
            content: String::new(),
            is_streaming: true,
        });

        // Clone necessary data for async task
        let client = self.llm_client.as_ref().unwrap().clone();

        // Spawn async task to call LLM using tokio
        tokio::runtime::Handle::current().spawn(async move {
            let result = client.send_message(&llm_messages).await;

            match result {
                Ok(response) => {
                    // Send response back to UI thread
                    Cx::post_action(LlmAction::ResponseChunk(response));
                    Cx::post_action(LlmAction::ResponseComplete);
                    SignalToUI::set_ui_signal();
                }
                Err(e) => {
                    Cx::post_action(LlmAction::Error(e.to_string()));
                    SignalToUI::set_ui_signal();
                }
            }
        });
    }

    fn handle_llm_response_chunk(&mut self, cx: &mut Cx, index: usize, chunk: String) {
        if index < self.messages.len() {
            self.messages[index].content.push_str(&chunk);
            self.messages[index].is_streaming = true;
            self.redraw(cx);
        }
    }

    fn handle_llm_response_complete(&mut self, cx: &mut Cx, index: usize) {
        if index < self.messages.len() {
            self.messages[index].is_streaming = false;
        }
        self.is_loading = false;
        self.view
            .view(ids!(loading_indicator))
            .set_visible(cx, false);
        self.redraw(cx);
    }

    fn handle_llm_error(&mut self, cx: &mut Cx, error: String) {
        self.messages.push(ChatMessage {
            is_user: false,
            content: format!("Error: {}", error),
            is_streaming: false,
        });
        self.is_loading = false;
        self.view
            .view(ids!(loading_indicator))
            .set_visible(cx, false);
        self.redraw(cx);
    }

    fn clear_chat(&mut self, cx: &mut Cx) {
        self.messages.clear();
        self.messages.push(ChatMessage {
            is_user: false,
            content: "Hello! I'm your AI assistant. How can I help you today?".to_string(),
            is_streaming: false,
        });
        self.is_loading = false;
        self.deep_think_mode = false;
        self.view
            .view(ids!(loading_indicator))
            .set_visible(cx, false);
        self.redraw(cx);
    }

    fn toggle_deep_think(&mut self, cx: &mut Cx) {
        self.deep_think_mode = !self.deep_think_mode;
        // Update button appearance
        let button = self.view.button(ids!(deep_think_button));
        if self.deep_think_mode {
            // Button is in active state
        } else {
            // Button is in inactive state
        }
        self.redraw(cx);
    }
}

impl Widget for ChatAssistantScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if let Event::Actions(actions) = event {
            // Handle LLM actions
            for action in actions {
                if let Some(llm_action) = action.downcast_ref::<LlmAction>() {
                    match llm_action {
                        LlmAction::ResponseChunk(chunk) => {
                            // Find the streaming message and update it
                            for (i, msg) in self.messages.iter().enumerate().rev() {
                                if msg.is_streaming {
                                    self.handle_llm_response_chunk(cx, i, chunk.clone());
                                    break;
                                }
                            }
                        }
                        LlmAction::ResponseComplete => {
                            for (i, msg) in self.messages.iter().enumerate().rev() {
                                if msg.is_streaming {
                                    self.handle_llm_response_complete(cx, i);
                                    break;
                                }
                            }
                        }
                        LlmAction::Error(err) => {
                            self.handle_llm_error(cx, err.clone());
                        }
                    }
                }

                match action.downcast_ref() {
                    Some(ChatAssistantAction::NewChat) => {
                        self.clear_chat(cx);
                    }
                    Some(ChatAssistantAction::DeepThink) => {
                        self.toggle_deep_think(cx);
                    }
                    Some(ChatAssistantAction::SendMessage(msg)) => {
                        // Add user message to chat
                        self.messages.push(ChatMessage {
                            is_user: true,
                            content: msg.clone(),
                            is_streaming: false,
                        });

                        // Send to LLM
                        self.send_message_to_llm(cx, msg.clone());
                        self.redraw(cx);
                    }
                    Some(ChatAssistantAction::SetApiKey(key)) => {
                        self.api_key = key.clone();
                        self.init_llm_client();
                        self.hide_api_key_modal(cx);
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

            if self.view.button(ids!(api_key_button)).clicked(actions) {
                self.show_api_key_modal(cx);
            }

            if self.view.button(ids!(send_button)).clicked(actions) {
                let input_text = self.view.text_input(ids!(message_input)).text();
                if !input_text.is_empty() {
                    cx.action(ChatAssistantAction::SendMessage(input_text));
                    // Clear the input
                    self.view.text_input(ids!(message_input)).set_text(cx, "");
                }
            }

            // Handle modal buttons
            if self
                .view
                .button(ids!(cancel_api_key_button))
                .clicked(actions)
            {
                self.hide_api_key_modal(cx);
            }

            if self.view.button(ids!(save_api_key_button)).clicked(actions) {
                let api_key = self.view.text_input(ids!(api_key_input)).text();
                if !api_key.is_empty() {
                    cx.action(ChatAssistantAction::SetApiKey(api_key));
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
        // Draw the view first
        let step = self.view.draw_walk(cx, scope, walk);

        // Draw messages using PortalList
        if let Some(mut list) = self.view.portal_list(ids!(message_list)).borrow_mut() {
            list.set_item_range(cx, 0, self.messages.len());

            while let Some(item_id) = list.next_visible_item(cx) {
                let index = item_id as usize;
                if index < self.messages.len() {
                    let msg = &self.messages[index];

                    // Create appropriate template based on message type
                    let template = if msg.is_user {
                        live_id!(UserMessageBubble)
                    } else {
                        live_id!(AiMessageBubble)
                    };

                    let item = list.item(cx, item_id, template);
                    item.label(ids!(text)).set_text(cx, &msg.content);
                    item.draw_all(cx, scope);
                }
            }
        }

        step
    }
}
