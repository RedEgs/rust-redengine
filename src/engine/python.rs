use std::sync::OnceLock;

use egui::{mutex::Mutex, ColorImage};
use pyo3::{prelude::*, types::{PyDict, PyIterator, PyTuple}};
use pyo3::{types::PyBytes, PyAny};


// static GAME_INSTANCE: OnceLock<Py<PyAny>> = OnceLock::new();
pub static FRAME_IMAGE: OnceLock<Mutex<Option<ColorImage>>> = OnceLock::new();

pub fn run_code_threaded(code_string: &str) {
    let mut code = String::new();
    code_string.clone_into(&mut code);

    
    pyo3::prepare_freethreaded_python();

    std::thread::spawn(move || {
        Python::with_gil(|py| {
            Python::run_bound(py, &code, None, None);
          
            let main_module = PyModule::import(py, "__main__").expect("Import __main__ failed");
            let game = main_module.getattr("game").unwrap();
            println!("Running test method after instancing: {}", game);
            
            match game.call_method0("test_run") {
                Ok(generator) => {
                    let mut gen_iter = PyIterator::from_object(generator).unwrap();

                    loop { // or loop forever
                        let pool = unsafe { py.new_pool() };

                        match gen_iter.next() {
                            Some(_) => {
                                let py_frame_buffer = game.getattr("_frame_buffer").unwrap();

                                let pybuf: Result<&PyBytes, pyo3::PyDowncastError<'_>> = py_frame_buffer.downcast::<PyBytes>();
                                let frame_buffer_bytes = pybuf.unwrap().as_bytes();

                                let image = egui::ColorImage::from_rgba_unmultiplied([1280, 720], frame_buffer_bytes);
                                
                                if let Some(lock) = FRAME_IMAGE.get() {
                                    let mut slot = lock.lock();
                                    *slot = Some(image); // 🔁 Overwrite each frame
                                }

                                drop(pool);
                            }
                            Some(Err(e)) => {
                                e.print(py);
                                break;
                            }
                            None => {
                                 // println!("Generator finished");
                                break;
                            }
                        }
                    }
                }
                Err(e) => {
                    e.print(py);
                }
            }
            
        });
    });
}

// pub fn call_game_method(method_name: &str) {
//     let method = method_name.to_string();

   
//     std::thread::spawn(move || {
//         Python::with_gil(|py| {
//             if let Some(game) = GAME_INSTANCE.get() {
//                 let game = game.as_ref(py);

//                 println!("Game instance exists, calling: {}", game);
//                 if let Err(e) = game.call_method0(&*method) {
//                     e.print(py);
//                 }
//             } else {
//                 println!("Game instance not set yet.");
//             }
//         });
//     });
// }

