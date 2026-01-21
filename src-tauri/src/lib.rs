use nonogram_solver_core::solver::NonogramSolver;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize)]
struct SolveResponse {
    rows: usize,
    cols: usize,
    // 0 empty, 1 filled, 2 unknown
    grid: Vec<Vec<u8>>,
    valid: bool,
    solved: bool,
    unsolvable: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SolveRequest {
    rows: usize,
    cols: usize,
    row_clues: Vec<Vec<usize>>,
    col_clues: Vec<Vec<usize>>,
    // 0 empty, 1 filled, 2 unknown
    known_grid: Option<Vec<Vec<u8>>>,
}

#[tauri::command]
fn solve(request: SolveRequest) -> Result<SolveResponse, String> {
    if request.row_clues.len() != request.rows {
        return Err("rowClues length must equal rows".to_string());
    }
    if request.col_clues.len() != request.cols {
        return Err("colClues length must equal cols".to_string());
    }

    let mut solver = NonogramSolver::new(request.rows, request.cols);
    solver.set_row_clues(&request.row_clues);
    solver.set_col_clues(&request.col_clues);
    if let Some(grid) = request.known_grid.as_ref() {
        solver.set_known_grid(grid)?;
    }
    solver.solve();

    let grid: Vec<Vec<u8>> = solver
        .grid()
        .iter()
        .map(|r| r.iter().map(|&v| v as u8).collect())
        .collect();

    Ok(SolveResponse {
        rows: request.rows,
        cols: request.cols,
        grid,
        valid: solver.is_valid(),
        solved: solver.is_solved(),
        unsolvable: solver.is_unsolvable(),
    })
}

#[tauri::command]
fn solve_sample() -> SolveResponse {
    let row_clues: Vec<Vec<usize>> = vec![
        vec![6, 1, 2],
        vec![5, 1, 3],
        vec![8, 5],
        vec![1, 3, 1, 5],
        vec![9],
        //
        vec![1, 1, 6],
        vec![1, 1, 2, 1],
        vec![1, 3, 1, 1],
        vec![2, 1],
        vec![6, 2],
        //
        vec![2, 7],
        vec![2, 5],
        vec![6, 1],
        vec![3, 1, 1],
        vec![3, 1, 1],
    ];

    let col_clues: Vec<Vec<usize>> = vec![
        vec![4, 3],
        vec![3, 3],
        vec![4, 1],
        vec![4, 3, 1],
        vec![5, 1, 2],
        //
        vec![1, 1, 4, 3],
        vec![1, 1, 1, 1, 2],
        vec![4, 1, 1, 1],
        vec![1, 3],
        vec![6, 3],
        //
        vec![6, 3],
        vec![5, 6],
        vec![5, 2],
        vec![4, 1, 2, 2],
        vec![2, 6],
    ];

    let rows = 15;
    let cols = 15;
    let mut solver = NonogramSolver::new(rows, cols);
    solver.set_row_clues(&row_clues);
    solver.set_col_clues(&col_clues);
    solver.solve();

    let grid: Vec<Vec<u8>> = solver
        .grid()
        .iter()
        .map(|r| r.iter().map(|&v| v as u8).collect())
        .collect();

    SolveResponse {
        rows,
        cols,
        grid,
        valid: solver.is_valid(),
        solved: solver.is_solved(),
        unsolvable: solver.is_unsolvable(),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init()) // ← 注册 dialog 插件
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![solve_sample, solve])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
