#[cfg(not(feature = "visualization_wasm"))]
mod tests {
    use krabmaga::utils::monitoring::ui::{TabsState, UI};
    use tui::widgets::ListState;

    #[test]
    fn tabs_state_cycles() {
        let mut tabs = TabsState::new(vec!["A".to_string(), "B".to_string()]);

        assert_eq!(tabs.index, 0);
        tabs.next();
        assert_eq!(tabs.index, 1);
        tabs.next();
        assert_eq!(tabs.index, 0);
        tabs.previous();
        assert_eq!(tabs.index, 1);
    }

    #[test]
    fn ui_key_toggles() {
        let mut ui = UI::new(10, 2);

        assert!(!ui.should_quit);
        assert!(ui.show_chart);
        assert!(!ui.show_description);

        ui.on_key('c');
        assert!(!ui.show_chart);
        ui.on_key('s');
        assert!(ui.show_description);
        ui.on_key('q');
        assert!(ui.should_quit);
    }

    #[test]
    fn ui_log_navigation_wraps() {
        let mut ui = UI::new(10, 2);
        ui.tot_logs = 3;
        ui.logs_state = ListState::default();

        ui.on_down();
        assert_eq!(ui.logs_state.selected(), Some(0));
        ui.on_down();
        assert_eq!(ui.logs_state.selected(), Some(1));
        ui.on_down();
        assert_eq!(ui.logs_state.selected(), Some(2));
        ui.on_down();
        assert_eq!(ui.logs_state.selected(), Some(0));

        ui.on_up();
        assert_eq!(ui.logs_state.selected(), Some(2));
    }
}
