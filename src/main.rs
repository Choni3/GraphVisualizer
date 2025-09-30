use graph_visualizer::gui_rust::gui_model::GraphApp;

fn main() -> iced::Result {
    iced::run("Graph visualizer", GraphApp::update, GraphApp::view)
}
