use std::sync::{Arc, OnceLock};
use std::collections::VecDeque;

use egui::{mutex::Mutex, ColorImage};

use pyo3::{prelude::*, types::PyIterator};
use pyo3::types::PyBytes;


// static GAME_INSTANCE: OnceLock<Py<PyAny>> = OnceLock::new();
pub static FRAME_IMAGE: OnceLock<Mutex<Option<ColorImage>>> = OnceLock::new();

lazy_static::lazy_static! {
    static ref INSTRUCTION_QUEUE: Arc<std::sync::Mutex<VecDeque<Instruction>>> = Arc::new(std::sync::Mutex::new(VecDeque::new()));
}
pub type Instruction = Box<dyn Fn(Python) + Send + 'static>;


pub fn run_code_threaded(code_string: &str) {
    let mut code = String::new();
    code_string.clone_into(&mut code);

    
    pyo3::prepare_freethreaded_python();

    std::thread::spawn(move || {
        Python::with_gil(|py| {
            let _ = Python::run_bound(py, &code, None, None);
          
            let main_module = PyModule::import(py, "__main__").expect("Import __main__ failed");
            let game = main_module.getattr("game").unwrap();
            println!("Running test method after instancing: {}", game);
            
            match game.call_method0("test_run") {
                Ok(generator) => {
                    let mut gen_iter = PyIterator::from_object(generator).unwrap();

                    loop { // or loop forever
                        
                        if let Some(task) = INSTRUCTION_QUEUE.lock().unwrap().pop_front() {
                            task(py); // run inside current Python interpreter
                        }

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
                            // Some(Err(e)) => {
                            //     e.print(py);
                            //     break;
                            // }
                            None => break,
                            
                        }
                    }
                }
                Err(e) => {
                    e.print(py);
                }
            }
            
        });

        println!("Gracefully closing thread.")
    });
    

}

pub fn queue_python_instruction<F>(func: F)
where
    F: Fn(Python) + Send + 'static,
{
    let mut queue = INSTRUCTION_QUEUE.lock().unwrap();
    queue.push_back(Box::new(func));
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

