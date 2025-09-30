use graphs::gui_rust::gui_model::GraphApp;

fn main() -> iced::Result {
    iced::run("Graph visualizer", GraphApp::update, GraphApp::view)
}
