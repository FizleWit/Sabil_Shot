#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
// -hwaccel cuda flag for gpu passthrough reduces cpu bottlnecks tremendously

use winapi::um::processthreadsapi::{ OpenProcess, TerminateProcess};
use winapi::um::winnt::{PROCESS_TERMINATE,HANDLE};

use std::process::Command;

use std::fs;
use std::path::Path;
use std::io;

use std::io::Error;
use serde_derive::{Deserialize, Serialize};

use std::process::Stdio;



use std::str;
extern crate serde_json;
use chrono::{Utc, DateTime};
//use async_process::Command;
//use std::io::{Error, Write};
static mut GLOBAL_IS_SEGMENTING: u32 = 0 as u32;
static mut GLOBAL_IS_STREAMING: u32 = 0 as u32;
static mut GLOBAL_IS_RECORDING: u32 = 0 as u32;
static mut GLOBAL_IS_ARR: u32 = 0 as u32;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
fn current_datetime_string(datetimeformat: String) -> String {
    let current_datetime: DateTime<Utc> = Utc::now();
    current_datetime
    .format(&datetimeformat)
    .to_string()
}
fn main() {
    //add javascript script file to html reminder
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            return_framerate_data,
            return_show_region_data,
            return_video_size_x_data,
            return_video_size_y_data,
            return_unique_file_name_data,
            return_micropohne_device_data,
            return_desktop_device_data,
            return_stream_port_data,
            return_screenshot_output_dir_data,
            return_stream_cache_dir_data,
            return_video_output_dir_data,
            return_action_replay_dur_data,
            return_y_offset_data,
            return_x_offset_data,
            testfunction,
            test1_btn_pressed,
            test2_btn_pressed,
            greet,
            b_seetings_btn_pressed,
            record_start_btn_pressed,
            record_stop_btn_pressed,
            screenshot_exe_btn_pressed,
            screen_caching_start_btn_pressed,
            screen_caching_stop_btn_pressed,
            action_replay_exe_btn_pressed,
            action_replay_and_record_btn_start_pressed,
            action_replay_and_record_btn_stop_pressed,
            save_settings_btn_pressed
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    println!("Message from Rust: {}", "Deon Game time");
}

#[tauri::command]
fn testfunction() -> () {
    println!("Message from Rust: ");
}

#[tauri::command]
fn test1_btn_pressed() -> () {
    println!("Test1_btn_pressed execute");
    let mut _variable_list = {
        let _variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        serde_json::from_str::<FfmpegVariables>(&_variable_list).unwrap()
    };

    _variable_list.stream_cache_dir = String::from("DEEON");
    println!("Message from Rust: {}", _variable_list.stream_cache_dir);

    let text = serde_json::to_string(&_variable_list).unwrap();
    fs::write("./Data/ffmpeg_variables.json", text).ok();
}

#[tauri::command]
async fn test2_btn_pressed() -> () {
   //stream_segmentation_ffmpeg_command().await;
   println!("WOOHOOMADEIT");
}


#[tauri::command(rename_all = "snake_case")]
fn save_settings_btn_pressed(
    framerate: i32,
    show_region: bool,
    video_size_x: i32,
    video_size_y: i32,
    x_offset: i32,
    y_offset: i32,
    uniqe_file_name: String,
    microphone_device: String,
    desktop_audio_device: String,
    microphone_device_audio_channels: i32,
    desktop_device_audio_channels: i32,
    microphone_device_audio_frequency: i32,
    desktop_device_audio_frequency: i32,
    stream_port: String,
    screenshot_output_dir: String,
    stream_cache_dir: String,
    video_output_dir: String,
    action_replay_dur: i32,
    audio_format: String,
    video_format: String,
    picture_format: String,
) -> () {
    println!("save settings btn execute");
    let mut variables_list = {
        let variables_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();

        
        serde_json::from_str::<FfmpegVariables>(&variables_list).unwrap()
    };
    //println!("framerate 1: {} + {}", variables_list.framerate, framerate);
    variables_list.framerate = framerate;
    variables_list.show_region = show_region;
    variables_list.video_size_x = video_size_x;
    variables_list.video_size_y = video_size_y;
    variables_list.x_offset = x_offset;
    variables_list.y_offset = y_offset;
    variables_list.uniqe_file_name = uniqe_file_name;
    variables_list.microphone_device = microphone_device;
    variables_list.desktop_audio_device = desktop_audio_device;
    variables_list.microphone_device_audio_channels = microphone_device_audio_channels;
    variables_list.desktop_device_audio_channels = desktop_device_audio_channels;
    variables_list.microphone_device_audio_frequency = microphone_device_audio_frequency;
    variables_list.desktop_device_audio_frequency = desktop_device_audio_frequency;
    variables_list.stream_port = stream_port;
    variables_list.screenshot_output_dir = screenshot_output_dir;
    variables_list.stream_cache_dir = stream_cache_dir;
    variables_list.video_output_dir = video_output_dir;
    variables_list.action_replay_dur = action_replay_dur;
    variables_list.audio_format = audio_format;
    variables_list.video_format = video_format;
    variables_list.picture_format = picture_format;

    let text = serde_json::to_string(&variables_list).unwrap();
    fs::write("./Data/ffmpeg_variables.json", text).ok();
    //println!("framerate 1: {} + {}", variables_list.framerate, framerate);
    //println!("save settings btn execute 2 ");
}

#[tauri::command]
fn b_seetings_btn_pressed() -> () {
    println!("settings listview page button execute");
}

#[tauri::command]
 async fn record_start_btn_pressed() -> () {
unsafe{    
    println!("record start btn execute");
        //let result2 =  ;
        if GLOBAL_IS_RECORDING == 0 {
        match record_start_ffmepg_command() {
            Ok(child) => {
                println!("Started ffmpeg process with PID: {}", child.id());
                // Do something with the child process here
                
                    GLOBAL_IS_RECORDING = child.id() as u32;  
                      println!("PID2a {}",GLOBAL_IS_RECORDING);
                
            }
            Err(e) => {
                println!("Error starting ffmpeg process: {}", e);     
        }
    }
}}
}
#[tauri::command]
async fn record_stop_btn_pressed() -> () {
    println!("record stop execute");
    unsafe{
        println!("record stop execute2");
            //let _result = send_input_to_pid(GLOBAL_IS_RECORDING, "q" as &str);
            if GLOBAL_IS_RECORDING != 0 {
         
            match send_input_to_pid(GLOBAL_IS_RECORDING, "q" as &str).await {
                Ok(_ok) => {  
                    GLOBAL_IS_RECORDING = 0 as u32;
                },
                Err(e) => {
                    println!("Bad happen {}",e);
                }
            }

        }
        

    }

}

#[tauri::command]
async fn screenshot_exe_btn_pressed() -> () {
    println!("screenshot exe btn execute");
    screenshot_exe_ffmpeg_command();
}

#[tauri::command]
async fn screen_caching_start_btn_pressed() -> () {
    let _variable_list = {
        let _variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        serde_json::from_str::<FfmpegVariables>(&_variable_list).unwrap()
    };
    //start stream
    unsafe{    
        println!("record start btn execute");
            //let result2 =  ;
            if GLOBAL_IS_STREAMING == 0  {
            match start_stream_ffmpeg_command() {
                Ok(child) => {
                    println!("Started ffmpeg process with PID: {}", child.id());
                    // Do something with the child process here
                    
                        GLOBAL_IS_STREAMING = child.id() as u32;  
                          println!("PID2a {}",GLOBAL_IS_STREAMING);
                    
                }
                Err(e) => {
                    println!("Error starting ffmpeg process: {}", e);     
            }
        }
    }
        //start segmentation
        if GLOBAL_IS_SEGMENTING == 0 {
        match stream_segmentation_ffmpeg_command(){
            Ok(child) => {
                println!("Started ffmpeg process with PID: {}", child.id());
                // Do something with the child process here
                
                    GLOBAL_IS_SEGMENTING = child.id() as u32;  
                      println!("PID2a {}",GLOBAL_IS_SEGMENTING);
                
            }
            Err(e) => {
                println!("Error starting ffmpeg process: {}", e);     
        }
        } 
    } 
}
     delete_oldest(Path::new(_variable_list.stream_cache_dir.to_string().as_str()), (_variable_list.action_replay_dur * 2) as u64).await;
   
}
    



#[tauri::command]
async fn screen_caching_stop_btn_pressed() -> () {
    println!("screen cache stop button pressed" );
    let _variable_list = {
        let _variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        serde_json::from_str::<FfmpegVariables>(&_variable_list).unwrap()
    };
    unsafe{
       // println!("record stop execute2");
            //let _result = send_input_to_pid(GLOBAL_IS_RECORDING, "q" as &str);
            if GLOBAL_IS_SEGMENTING != 0 as u32{
                //stop segmentation send signint not q
                match send_input_to_pid(GLOBAL_IS_SEGMENTING, "q" as &str).await {
                    Ok(_ok) => {  
                        GLOBAL_IS_SEGMENTING = 0 as u32;
                    },
                    Err(e) => {
                        println!("Bad happen {}",e);
                    }
                }
            }
            if GLOBAL_IS_STREAMING != 0 as u32{
                match send_input_to_pid(GLOBAL_IS_STREAMING, "q" as &str).await {
                    Ok(_ok) => {  
                        GLOBAL_IS_STREAMING = 0 as u32;
                    },
                    Err(e) => {
                        println!("Bad happen {}",e);
                    }
                }
            GLOBAL_IS_STREAMING = 0 as u32;
            }
           
        }
           //  
    std::thread::sleep(std::time::Duration::from_secs(2)); //ideally a wait for stream segmentation to be done but
    delete_files_in_dir(_variable_list.stream_cache_dir.to_string().as_str()).unwrap();    
}

#[tauri::command]
async fn action_replay_exe_btn_pressed() -> () {
    let  _variable_list = {
        let _variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        serde_json::from_str::<FfmpegVariables>(&_variable_list).unwrap()
    };
    unsafe{
        if GLOBAL_IS_SEGMENTING != 0 {
            //is streaming 
            match send_input_to_pid(GLOBAL_IS_SEGMENTING, "q" as &str).await {
                Ok(_ok) => {  
                    GLOBAL_IS_SEGMENTING = 0 as u32;
                    if action_replay_exe_ffmpeg_command().await== false {
                      
                        fix_action_replay().await;
                       if let Err(e) = fs::remove_file(Path::new(&(_variable_list.stream_cache_dir.to_string() + "\\ActionReplay.tempAR.mp4".to_string().as_str()))) {
                                   eprintln!("Error deleting file: {}", e);
                               }
                    }
                },
                Err(e) => {
                    println!("{} did not success kill process", e);
                }
            }

            
        }
            println!("PID2a {}",GLOBAL_IS_SEGMENTING);
            //turn off segmentations

            //run action replay stuff
          // let if_long_AR =  action_replay_exe_ffmpeg_command();

        //  if action_replay_exe_ffmpeg_command().await== false {
         //   fix_Action_Replay();
         //   if let Err(e) = fs::remove_file(Path::new(&(_variable_list.stream_cache_dir.to_string() + "\\ActionReplay.tempAR.mp4".to_string().as_str()))) {
         //       eprintln!("Error deleting file: {}", e);
         //   }
            //delete temp file
        //  }
         

            
           


           

            
            //delete all files in directory

            //turn on stream segmentation
             
            if GLOBAL_IS_SEGMENTING == 0 as u32 {
             match stream_segmentation_ffmpeg_command() {
                Ok(child) => {
                    println!("Started ffmpeg process with PID: {}", child.id());
                    // Do something with the child process here
                    
                        GLOBAL_IS_SEGMENTING = child.id() as u32;  
                          println!("PID2a {}",GLOBAL_IS_SEGMENTING);
                          delete_oldest(Path::new(_variable_list.stream_cache_dir.to_string().as_str()), (_variable_list.action_replay_dur * 2) as u64);
                    
                }
                Err(e) => {
                    println!("Error starting ffmpeg process: {}", e);     
            }
}  
            }}}




#[tauri::command]
async fn action_replay_and_record_btn_start_pressed() -> () {
    println!("action replay and record btn start execute");
    action_replay_and_record_start_ffmpeg_command().await;
}

#[tauri::command]
async fn action_replay_and_record_btn_stop_pressed() -> () {
    println!("action replay and record btn stop execute");
    action_replay_and_record_stop_ffmpeg_command().await;
}
#[tauri::command]
fn return_framerate_data() -> i32 {
    let _variable_list = {
        let _variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        serde_json::from_str::<FfmpegVariables>(&_variable_list).unwrap()
    };
    return _variable_list.framerate;
}
#[tauri::command]
fn return_show_region_data() -> bool {
    let _variable_list = {
        let _variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        serde_json::from_str::<FfmpegVariables>(&_variable_list).unwrap()
    };
    return _variable_list.show_region;
}
#[tauri::command]
fn return_video_size_x_data() -> i32 {
    let _variable_list = {
        let _variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        serde_json::from_str::<FfmpegVariables>(&_variable_list).unwrap()
    };
    return _variable_list.video_size_x;
}
#[tauri::command]
fn return_video_size_y_data() -> i32 {
    let _variable_list = {
        let _variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        serde_json::from_str::<FfmpegVariables>(&_variable_list).unwrap()
    };
    return _variable_list.video_size_y;
}
#[tauri::command]
fn return_unique_file_name_data() -> String {
    let _variable_list = {
        let _variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        serde_json::from_str::<FfmpegVariables>(&_variable_list).unwrap()
    };
    return _variable_list.uniqe_file_name;
}
#[tauri::command]
fn return_micropohne_device_data() -> String {
    let _variable_list = {
        let _variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        serde_json::from_str::<FfmpegVariables>(&_variable_list).unwrap()
    };
    return _variable_list.microphone_device;
}
#[tauri::command]
fn return_desktop_device_data() -> String {
    let _variable_list = {
        let _variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        serde_json::from_str::<FfmpegVariables>(&_variable_list).unwrap()
    };
    return _variable_list.desktop_audio_device;
}
#[tauri::command]
fn return_stream_port_data() -> String {
    let _variable_list = {
        let _variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        serde_json::from_str::<FfmpegVariables>(&_variable_list).unwrap()
    };
    return _variable_list.stream_port;
}
#[tauri::command]
 fn return_screenshot_output_dir_data() -> String {
    let _variable_list = {
        let _variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        serde_json::from_str::<FfmpegVariables>(&_variable_list).unwrap()
    };
    return _variable_list.screenshot_output_dir;
}
#[tauri::command]
fn return_stream_cache_dir_data() -> String {
    let _variable_list = {
        let _variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        serde_json::from_str::<FfmpegVariables>(&_variable_list).unwrap()
    };
    return _variable_list.stream_cache_dir;
}
#[tauri::command]
fn return_video_output_dir_data() -> String {
    let _variable_list = {
        let _variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        serde_json::from_str::<FfmpegVariables>(&_variable_list).unwrap()
    };
    return _variable_list.video_output_dir;
}
#[tauri::command]
fn return_action_replay_dur_data() -> i32 {
    println!("return action replay data dur");
    let _variable_list = {
        let _variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        serde_json::from_str::<FfmpegVariables>(&_variable_list).unwrap()
    };
    return _variable_list.action_replay_dur;
}
#[tauri::command]
fn return_x_offset_data() -> i32 {
    println!("return offset data");
    let _variable_list = {
        let _variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        serde_json::from_str::<FfmpegVariables>(&_variable_list).unwrap()
    };
    return _variable_list.x_offset;
}
#[tauri::command]
fn return_y_offset_data() -> i32 {
    println!("return offset data");
    let _variable_list = {
        let _variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        serde_json::from_str::<FfmpegVariables>(&_variable_list).unwrap()
    };
    return _variable_list.y_offset;
}

//

#[derive(Deserialize, Serialize, Debug)]
struct  FfmpegVariables {
    framerate: i32,
    show_region: bool,
    video_size_x: i32,
    video_size_y: i32,
    x_offset: i32,
    y_offset: i32,
    uniqe_file_name: String,
    microphone_device: String,
    desktop_audio_device: String,
    microphone_device_audio_channels: i32,
    desktop_device_audio_channels: i32,
    microphone_device_audio_frequency: i32,
    desktop_device_audio_frequency: i32,
    stream_port: String,
    screenshot_output_dir: String,
    stream_cache_dir: String,
    video_output_dir: String,
    action_replay_dur: i32,
    audio_format: String,
    video_format: String,
    picture_format: String,
}

//record start stop
//screenshot exe
//screen caching start stop
//action replay exe
//action replay and record start stop
/*
 -vcodec mpeg4 -q 12 -f mpegts udp://127.0.0.1:3000
 */
fn start_stream_ffmpeg_command() -> Result<std::process::Child, std::io::Error> {
    let _variable_list = {
        let _variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        
        serde_json::from_str::<FfmpegVariables>(&_variable_list).unwrap()
    };
    let mut ffmpegcommand = Command::new("./Data/sabilshotffmpeg");
    ffmpegcommand.arg("-hide_banner")
    .arg("-thread_queue_size")
    .arg("5096")
    .arg("-f")
    .arg("dshow")
   // .arg("-ac")
   // .arg("1")
   // .arg("-ar")
   // .arg("48000")
     .arg("-i")
     .arg("audio=".to_string() + _variable_list.microphone_device.to_string().as_str())
    .arg("-f")
    .arg("dshow")
    .arg("-i")
    .arg("audio=".to_string() + _variable_list.desktop_audio_device.to_string().as_str())
    .arg("-filter_complex")
    .arg("amix=inputs=2")
    .arg("-f")
    .arg("gdigrab")
    .arg("-framerate")
    .arg(_variable_list.framerate.to_string().as_str())
    .arg("-show_region")
    //.arg(return_bool_int_string(_variable_list.show_region))
    .arg("0")
    .arg("-video_size")
    .arg(_variable_list.video_size_x.to_string()
    + "x"
    + (_variable_list.video_size_y.to_string()).as_str(),)
    .arg("-offset_x")
    .arg(_variable_list.x_offset.to_string().as_str())
    .arg("-offset_y")
    .arg(_variable_list.y_offset.to_string().as_str())
    .arg("-i")
    .arg("desktop")
    .arg("-vcodec")
    .arg("mpeg4")
    .arg("-f")
    .arg("mpegts")
    .arg(_variable_list.stream_port.to_string())
    .stdin(Stdio::null())
    .stdout(Stdio::null())
    .stderr(Stdio::null());
    let child = ffmpegcommand.spawn()?;
    Ok(child)


}

fn return_bool_int_string(x: bool) -> String {
    if x == true {
        return "1".to_string();
    } else {
        return "0".to_string();
    }
}
async fn fix_action_replay()  {
    println!("action replay put file in right spot");
    //ffmpeg -sseof -6 -i output.mp4 -codec copy output2.mp4
   let _variable_list = {
    let _variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
    serde_json::from_str::<FfmpegVariables>(&_variable_list).unwrap()
};
let filename =current_datetime_string(_variable_list.uniqe_file_name.to_string());
    //trim output to correct directory
    //ffmpeg -sseof -15 -i ActionReplay.tempAR.mp4 output.mp4
    let mut ffmpegcommand2 = Command::new("./Data/sabilshotffmpeg");
    ffmpegcommand2
    .arg("-sseof")
    .arg("-".to_string() + _variable_list.action_replay_dur.to_string().as_str())
    .arg("-i")
    .arg(_variable_list.stream_cache_dir.to_string() + 
    "\\ActionReplay.tempAR.mp4".to_string().as_str())
    .arg("-codec")
    .arg("copy")
    .arg(_variable_list.video_output_dir.to_string() + 
    "\\ActionReplay." + 
    filename.as_str()+ (_variable_list.video_format.to_string()).as_str())
    .stdin(Stdio::null())
    .stdout(Stdio::null())
    .stderr(Stdio::null());
    let mut child = ffmpegcommand2.spawn().expect("YOU LOSE ERROR");
    let _waiter = child.wait();
   
}
 fn screenshot_exe_ffmpeg_command() -> () {
    println!("screenshot exe ffmpeg command");
    let _variable_list = {
        let _variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        serde_json::from_str::<FfmpegVariables>(&_variable_list).unwrap()
    };
    let  _ffmpegcommand = Command::new("ffmpeg")
    .arg("-hide_banner")
    .arg("-y")
    .arg("-f")
    .arg("gdigrab")
    .arg("-framerate")
    .arg((_variable_list.framerate.to_string()).as_str())
    .arg("-show_region")
    .arg(return_bool_int_string(_variable_list.show_region))
    .arg("-video_size")
    .arg(
        _variable_list.video_size_x.to_string()
            + "x"
            + (_variable_list.video_size_y.to_string()).as_str(),
    )
    .arg("-offset_x")
    .arg((_variable_list.x_offset.to_string()).as_str())
    .arg("-offset_y")
    .arg((_variable_list.y_offset.to_string()).as_str())
    .arg("-i")
    .arg("desktop")
    .arg("-frames:v")
    .arg("1")
    .arg("-strftime")
    .arg("1")
    .arg(
        _variable_list.screenshot_output_dir.to_string()
            + "\\"
            + (_variable_list.uniqe_file_name.to_string()).as_str()
            + (_variable_list.picture_format.to_string()).as_str(),
    )    .stdin(Stdio::null())
    .stdout(Stdio::null())
    .stderr(Stdio::null()).spawn().expect("DID NTO WORK LOSER");


  

//delete 

}

 async fn action_replay_exe_ffmpeg_command() -> bool{
    //if less then 2 files in directory (1) file in the directory grab the one file and return it
    //else merdge the two files name temp trim to video output and delete temp

    println!("action replay exe ffmpeg command");
    let _variable_list = {
        let _variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        serde_json::from_str::<FfmpegVariables>(&_variable_list).unwrap()
    };
    let filename =current_datetime_string(_variable_list.uniqe_file_name.to_string());
    //if files less than 2 files in directory

    let mut files = fs::read_dir(_variable_list.stream_cache_dir.to_string()).unwrap()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().unwrap().is_file())
        .collect::<Vec<_>>();
    
    files.sort_by_key(|entry| entry.metadata().unwrap().modified().unwrap());
    files.reverse();
    let file_paths = files.iter().map(|entry| entry.path()).collect::<Vec<_>>();
    let mut ffmpegcommand = Command::new("./Data/sabilshotffmpeg");
    if file_paths.len() < 2  {
        //return actionreplay to video directory only one file

        
        ffmpegcommand.arg("-hide_banner")
        .arg("-y")
        .arg("-i")
        .arg(file_paths[0].to_str().unwrap())
        .arg(_variable_list.video_output_dir.to_string() + 
    "\\ActionReplay." + 
    filename.as_str()+ (_variable_list.video_format.to_string()).as_str());
    let mut child = ffmpegcommand.spawn().expect("workingweird");
    let _waiter = child.wait();
    
    return true;
    }
    else{
        //ffmpeg -i "concat:a.ts|b.ts" -codec copy output.mp4
     
    ffmpegcommand.arg("-hide_banner")
    .arg("-y")
    .arg("-i")
    .arg("concat:".to_string() + file_paths[0].to_str().unwrap() + "|".to_string().as_str() + file_paths[1].to_str().unwrap())
    .arg("-codec")
    .arg("copy")
    .arg(_variable_list.stream_cache_dir.to_string() + 
    "\\ActionReplay." + 
    "tempAR.mp4".to_string().as_str());
    let mut child = ffmpegcommand.spawn().expect("workingweird");
    let _waiter = child.wait();
    
    return false;

    //segment temp file and write to video directory
    //delete temp

}

//let child = ffmpegcommand.spawn()?;

//Ok(child)
 }

 fn record_start_ffmepg_command() -> Result<std::process::Child, std::io::Error> {
    println!("record start ffmpeg command");
    let _variable_list = {
        let _variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        
        serde_json::from_str::<FfmpegVariables>(&_variable_list).unwrap()
    };
    //  -offset_x 10 -offset_y 20 -i desktop -strftime 1 "%Y-%m-%d_%H-%M-%S.mp4"
    let mut ffmpegcommand = Command::new("./Data/sabilshotffmpeg");
    ffmpegcommand.arg("-hide_banner")
    .arg("-y")
    .arg("-f")
    .arg("dshow")
   // .arg("-ac")
   // .arg("1")
   // .arg("-ar")
   // .arg("48000")
     .arg("-i")
     .arg("audio=".to_string() + _variable_list.microphone_device.to_string().as_str())
    .arg("-f")
    .arg("dshow")
    .arg("-i")
    .arg("audio=".to_string() + _variable_list.desktop_audio_device.to_string().as_str())
    .arg("-filter_complex")
    .arg("amix=inputs=2")
    .arg("-f")
    .arg("gdigrab")
    .arg("-framerate")
    .arg(_variable_list.framerate.to_string().as_str())
    .arg("-show_region")
    //.arg(return_bool_int_string(_variable_list.show_region))
    .arg("0")
    .arg("-video_size")
    .arg(_variable_list.video_size_x.to_string()
    + "x"
    + (_variable_list.video_size_y.to_string()).as_str(),)
    .arg("-offset_x")
    .arg(_variable_list.x_offset.to_string().as_str())
    .arg("-offset_y")
    .arg(_variable_list.y_offset.to_string().as_str())
    .arg("-i")
    .arg("desktop")
    .arg("-strftime")
    .arg("1")
    .arg("-f")
    .arg("mpegts")
    .arg( _variable_list.video_output_dir.to_string()
    + "\\recording"
    + current_datetime_string(_variable_list.uniqe_file_name.to_string()).as_str()
    + (_variable_list.video_format.to_string()).as_str())
    .stdin(Stdio::null())
    .stdout(Stdio::null())
    .stderr(Stdio::null());
    let child = ffmpegcommand.spawn()?;
    Ok(child)
    //println!("Message from Rust: {}", _variable_list.stream_cache_dir);
}

#[cfg(windows)]
fn kill_process(pid: u32) -> Result<(), Error> {
    Command::new("taskkill")
        .arg("/PID")
        .arg(pid.to_string())
        .spawn()?
        .wait()?;
    Ok(())
}

#[cfg(not(windows))]
fn kill_process(pid: u32) -> Result<(), Error> {
    Command::new("kill")
        .arg("-2")
        .arg(pid.to_string())
        .spawn()?
        .wait()?;
    Ok(())
}


fn stream_segmentation_ffmpeg_command() -> Result<std::process::Child, std::io::Error>{
    println!("caching start ffmpeg command");
    let _variable_list = {
        let _variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        serde_json::from_str::<FfmpegVariables>(&_variable_list).unwrap()
    };
    //delete_files_in_dir(_variable_list.stream_cache_dir.to_string().as_str()).unwrap();   
    let mut ffmpegcommand = Command::new("./Data/sabilshotffmpeg");
    ffmpegcommand.arg("-hide_banner")
    .arg("-i")
    .arg( _variable_list.stream_port.to_string().as_str())
    .arg("-f")
    .arg("segment")
    .arg("-reset_timestamps")
    .arg("1")
    .arg("-segment_time")
    .arg(_variable_list.action_replay_dur.to_string())
    .arg("-min_seg_duration")
    //.arg("00:00:".to_string() + _variable_list.action_replay_dur.to_string().as_str())
    .arg(_variable_list.action_replay_dur.to_string().as_str())
    .arg("-strftime")
    .arg("1")
    .arg( _variable_list.stream_cache_dir.to_string()
    + "\\cache"
    + (_variable_list.uniqe_file_name.to_string()).as_str()
   // + (_variable_list.video_format.to_string()).as_str());
   + ".ts".to_string().as_str())
   .stdin(Stdio::null())
   .stdout(Stdio::null())
   .stderr(Stdio::null());
    let child = ffmpegcommand.spawn()?;
    
    Ok(child)
}


    /*
    ffmpeg -i udp://127.0.0.1:3000 -f segment 
    -reset_timestamps 1 -segment_time 15 -min_seg_duration 00:00:15 
    -strftime 1 "F:\School\Capstone\FFMPEGCache\%Y-%m-%d_%H-%M-%S.mp4"
     */


async fn action_replay_and_record_start_ffmpeg_command() -> () {
    println!("action replay and record start ffmpeg command");
    let _variable_list = {
        let _variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();

        
        serde_json::from_str::<FfmpegVariables>(&_variable_list).unwrap()
    };
    println!("Message from Rust: {}", _variable_list.stream_cache_dir);
}

async fn action_replay_and_record_stop_ffmpeg_command() -> () {
    println!("action replay and record stop ffmpeg command");
    let _variable_list = {
        let _variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();

        
        serde_json::from_str::<FfmpegVariables>(&_variable_list).unwrap()
    };
    println!("Message from Rust: {}", _variable_list.stream_cache_dir);
}
/*
fn return_audio_devices_ffmpeg_command() -> Result<String,Error> {
    let output = Command::new("ffmpeg")
    .arg("-hide_banner")
    .arg("-f")
    .arg("dshow")
    .arg("-list_devices")
    .arg("1")
    .arg("-i")
    .arg("dummy")
    .output()?;
    
let stdout = String::from_utf8_lossy(&output.stdout).to_string();
println!("{:#?} test", stdout);
Ok(stdout)
}
*/
//https://doc.rust-lang.org/std/process/struct.Stdio.html#method.piped
//format("{#}")
async fn send_input_to_pid(pid: u32, _input: &str) -> Result<(), std::io::Error> {
    let handle: HANDLE = unsafe { OpenProcess(PROCESS_TERMINATE, 0, pid) };
    if handle.is_null() {
        return Err(std::io::Error::last_os_error());
    }
    unsafe { TerminateProcess(handle, 0) };
    Ok(())
}


async fn delete_oldest(path: &Path,segment_time: u64 ){
unsafe{
while GLOBAL_IS_SEGMENTING != 0 {
    std::thread::sleep(std::time::Duration::from_secs(segment_time));
    let entries = fs::read_dir(path).unwrap();
    let mut files = vec![];

    for entry in entries {
        let entry = entry.unwrap();
        let file_type = entry.file_type().unwrap();
        if file_type.is_file() {
            files.push((entry.path(), entry.metadata().unwrap().modified().unwrap()));
        }
    }

    if files.len() >= 4 {
        files.sort_by_key(|f| f.1);
        let oldest_files = files.drain(0..files.len()-3);
        for oldest_file in oldest_files {
            fs::remove_file(oldest_file.0).unwrap();
        }
    }
}
}
}



fn delete_files_in_dir(dir_path: &str) -> io::Result<()> {
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_file() {
            fs::remove_file(entry.path())?;
        }
    }
    Ok(())
}

