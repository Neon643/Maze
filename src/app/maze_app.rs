use crate::animation::generation_animation::GenerationAnimation;
use crate::animation::generation_playback::GenerationPlayback;
use crate::animation::search_animation::SearchAnimation;
use crate::animation::search_playback::SearchPlayback;
use crate::app::app_state::AppState;
use crate::app::maze_setup::MazeSetup;
use crate::domain::position::Position;
use crate::generator::wilson::wilson_generator::WilsonGenerator;
use crate::graphics::canvas::Canvas;
use crate::graphics::draw_color::DrawColor;
use crate::graphics::maze::maze_painter::MazePainter;
use crate::graphics::pointer::Pointer;
use crate::graphics::widgets::control_action::ControlAction;
use crate::graphics::widgets::control_panel::ControlPanel;
use crate::solver::depth_first_solver::DepthFirstSolver;

pub struct MazeApp {
    setup: MazeSetup,
    generator: WilsonGenerator,
    generation_playback: GenerationPlayback,
    state: AppState,
    search_playback: Option<SearchPlayback>,
    control_panel: ControlPanel,
    maze_painter: MazePainter,
}

impl MazeApp {
    pub fn new(
        setup: MazeSetup,
        generator: WilsonGenerator,
        control_panel: ControlPanel,
        maze_painter: MazePainter,
    ) -> MazeApp {
        let search_playback = None;
        let generation_playback =
            GenerationPlayback::new(GenerationAnimation::empty(), setup.blank_maze());
        let state = AppState::Idle;
        Self {
            setup,
            generator,
            generation_playback,
            state,
            search_playback,
            control_panel,
            maze_painter,
        }
    }

    pub fn tick(&mut self, canvas: &mut dyn Canvas, pointer: &dyn Pointer) {
        canvas.clear(DrawColor::DARK_GRAY);

        self.react_to_controls(pointer);

        self.advance_generation();

        self.advance_solving();

        self.draw(canvas)
    }

    fn current_path(&self) -> &[Position] {
        match &self.search_playback {
            Some(playback) => playback.path(),
            None => &[],
        }
    }

    fn react_to_controls(&mut self, pointer: &dyn Pointer) {
        match self.control_panel.action(pointer) {
            ControlAction::Generate => {
                let generation = self.generator.generate();
                let (_final_maze, steps) = generation.into_parts();
                self.generation_playback = GenerationPlayback::new(
                    GenerationAnimation::new(steps),
                    self.setup.blank_maze(),
                );
                self.search_playback = None;
                self.state = AppState::Generating;
            }
            ControlAction::Solve => {
                if matches!(self.state, AppState::Finished) {
                    let solve = DepthFirstSolver::new(
                        self.generation_playback.maze(),
                        self.setup.start(),
                        self.setup.finish(),
                    );
                    let (_path, steps) = solve.solve_with_steps().into_parts();
                    let animation = SearchAnimation::new(steps);
                    self.search_playback = Some(SearchPlayback::new(animation));
                    self.state = AppState::Solving;
                }
            }
            ControlAction::None => {}
        }
    }

    fn advance_generation(&mut self) {
        if matches!(self.state, AppState::Generating) {
            self.generation_playback.tick();

            if self.generation_playback.is_finished() {
                self.state = AppState::Finished;
            }
        }
    }

    fn advance_solving(&mut self) {
        if matches!(self.state, AppState::Solving)
            && let Some(playback) = &mut self.search_playback
        {
            playback.tick();

            if playback.is_finished() {
                self.state = AppState::Finished;
            }
        }
    }

    fn draw(&self, canvas: &mut dyn Canvas) {
        self.control_panel.draw(canvas);
        self.maze_painter.draw(
            canvas,
            self.generation_playback.maze(),
            self.current_path(),
            self.setup.start(),
            self.setup.finish(),
        );
    }
}
