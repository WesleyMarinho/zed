fn send(&mut self, window: &mut Window, cx: &mut Context<Self>) {
    let Some(thread) = self.thread() else { return };

    if self.is_loading_contents {
        return;
    }

    // Check if auto-condensation should be triggered
    if self.check_auto_condensation_needed(window, cx) {
        // For now, just show a warning - full implementation would create a new thread with summary
        log::warn!("Context limit approaching - auto-condensation needed but not yet fully implemented");
        // TODO: Implement full auto-condensation flow
    }

    let agent_type = crate::ExternalAgent::parse_built_in(self.agent.as_ref())
        .map(|agent| agent.to_string());

    self.history_store.update(cx, |history, cx| {
        history.push_recently_opened_entry(
            HistoryEntryId::AcpThread(thread.read(cx).session_id().clone(), agent_type),
            cx,
        );
    });

    if thread.read(cx).status() != ThreadStatus::Idle {
        self.stop_current_and_send_new_message(window, cx);
        return;
    }

    let text = self.message_editor.read(cx).text(cx);
    let text = text.trim();
