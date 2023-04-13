#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
//use std::{process::Command, string};
use serde_derive::{Deserialize, Serialize};
//use std::fmt;
use std::fs;
use std::str;
extern crate serde_json;
use chrono::{Utc, DateTime};
use std::io::Error;
//use regex::Regex;
//use std::process::{Command, Stdio};
use std::process::Command;
//use std::sync::Exclusive;

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
    println!("record start btn execute");
    record_start_ffmepg_command().await;
}

#[tauri::command]
async fn record_stop_btn_pressed() -> () {
    println!("record stop execute");
    record_stop_ffmpeg_command().await;
}

#[tauri::command]
async fn screenshot_exe_btn_pressed() -> () {
    println!("screenshot exe btn execute");
    screenshot_exe_ffmpeg_command().await;
}

#[tauri::command]
async fn screen_caching_start_btn_pressed() -> () {
    println!("scren caching start execute");
    caching_start_ffmepg_command().await;
}

#[tauri::command]
async fn screen_caching_stop_btn_pressed() -> () {
    println!("screen caching stop execute");
    //caching_stop_ffmpeg_command().await;
}

#[tauri::command]
async fn action_replay_exe_btn_pressed() -> () {
    println!("action replay exe btn execute");
    action_replay_exe_ffmpeg_command().await;
}

#[tauri::command]
async fn action_replay_and_record_btn_start_pressed() -> () {
    println!("action replay and record btn start execute");
   // action_replay_and_record_start_ffmpeg_command().await;
}

#[tauri::command]
async fn action_replay_and_record_btn_stop_pressed() -> () {
    println!("action replay and record btn stop execute");
   // action_replay_and_record_stop_ffmpeg_command().await;
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
fn return_bool_int_string(x: bool) -> String {
    if x == true {
        return "1".to_string();
    } else {
        return "0".to_string();
    }
}
async fn screenshot_exe_ffmpeg_command() -> () {
    println!("screenshot exe ffmpeg command");
    let _variable_list = {
        let _variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();

        
        serde_json::from_str::<FfmpegVariables>(&_variable_list).unwrap()
    };
    let mut _ffmpegcommand = Command::new("ffmpeg")
    .arg("-hide_banner")
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
    )

    .status().expect("DID NTO WORK LOSER");
}
async fn action_replay_exe_ffmpeg_command() -> () {
    println!("action replay exe ffmpeg command");
    let _variable_list = {
        let _variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        serde_json::from_str::<FfmpegVariables>(&_variable_list).unwrap()
    };
    let mut files = fs::read_dir(_variable_list.stream_cache_dir.to_string()).unwrap()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().unwrap().is_file())
        .collect::<Vec<_>>();
    
    files.sort_by_key(|entry| entry.metadata().unwrap().modified().unwrap());
    files.reverse();
    let file_paths = files.iter().map(|entry| entry.path()).collect::<Vec<_>>();
    
    let mut ffmpegcommand = Command::new("ffmpeg");
    ffmpegcommand.arg("-hide_banner")
    .arg("-y")
    .arg("-i")
    .arg(file_paths[0].to_str().unwrap())
    .arg("-i")
    .arg(file_paths[0].to_str().unwrap())
    .arg("-filter_complex")
    .arg("[0:v][0:a][1:v][1:a]concat=n=2:v=1:a=1")
    .arg("-f")
    .arg("mp4")
    .arg("-strftime")
    .arg("1")
    .arg(_variable_list.video_output_dir.to_string() + 
    "\\ActionReplay." + 
    current_datetime_string(_variable_list.uniqe_file_name.to_string()).as_str() + (_variable_list.video_format.to_string()).as_str());
    ffmpegcommand.spawn().expect("YOU LOSE ERROR");
    println!("finished test 2");

    println!("Message from Rust: {}", _variable_list.stream_cache_dir);
    
}

async fn record_start_ffmepg_command() -> () {
    println!("record start ffmpeg command");
    let _variable_list = {
        let _variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        
        serde_json::from_str::<FfmpegVariables>(&_variable_list).unwrap()
    };
    //  -offset_x 10 -offset_y 20 -i desktop -strftime 1 "%Y-%m-%d_%H-%M-%S.mp4"
    let mut ffmpegcommand = Command::new("ffmpeg");
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
    .arg(return_bool_int_string(_variable_list.show_region))
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
    .arg( _variable_list.video_output_dir.to_string()
    + "\\recording"
    + (_variable_list.uniqe_file_name.to_string()).as_str()
    + (_variable_list.video_format.to_string()).as_str());
    ffmpegcommand.status().expect("DID NTO WORK LOSER");
    //println!("Message from Rust: {}", _variable_list.stream_cache_dir);
}

async fn record_stop_ffmpeg_command() -> () {
    println!("record stop ffmpeg command");
    let _variable_list = {
        let _variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();

        
        serde_json::from_str::<FfmpegVariables>(&_variable_list).unwrap()
    };

    println!("Message from Rust: {}", _variable_list.stream_cache_dir);
}

async fn caching_start_ffmepg_command() -> () {
    println!("caching start ffmpeg command");
    let _variable_list = {
        let _variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();

        
        serde_json::from_str::<FfmpegVariables>(&_variable_list).unwrap()
    };
    /*
    ffmpeg -hide_banner -f dshow -ac 2 -ar 48000 -i
     audio="@device_cm_{33D9A762-90C8-11D0-BD43-00A0C911CE86}\wave_{D9AE870B-D51C-4050-A7DE-20475EE70F0E}" 
     -f dshow -ac 1 -ar 48000 -i 
     audio="@device_cm_{33D9A762-90C8-11D0-BD43-00A0C911CE86}\wave_{DE99F572-E285-4EBC-A263-2D782B212FA7}" 
     -filter_complex amix=inputs=2 -f gdigrab -framerate 60 -show_region 1 -video_size 600x600 
     -offset_x 10 -offset_y 20 -i desktop -vcodec mpeg4 -q 12 -f mpegts udp://127.0.0.1:3000
 */
    let mut ffmpegcommand = Command::new("ffmpeg");
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
    .arg("-show_region")//comment out after testing
    .arg(return_bool_int_string(_variable_list.show_region))//comment out after testing
    .arg("-video_size")
    .arg(_variable_list.video_size_x.to_string()
    + "x"
    + (_variable_list.video_size_y.to_string()).as_str())
    .arg("-offset_x")
    .arg(_variable_list.x_offset.to_string().as_str())
    .arg("-offset_y")
    .arg(_variable_list.y_offset.to_string().as_str())
    .arg("-i")
    .arg("desktop")
    .arg("-vcodec")
    .arg("mpeg4")
    .arg("-q")
    .arg("12")
    .arg("-f")
    .arg("mpegts")
    .arg( _variable_list.stream_port.to_string().as_str());
    
    ffmpegcommand.status().expect("DID NTO WORK LOSER");
    //println!("Message from Rust: {}", _variable_list.stream_cache_dir);



async fn caching_stop_ffmpeg_command() -> () {
    println!("caching stop ffmpeg command");
    let _variable_list = {
        let _variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();

        
        serde_json::from_str::<FfmpegVariables>(&_variable_list).unwrap()
    };
    println!("Message from Rust: {}", _variable_list.stream_cache_dir);
}

async fn stream_segmentation_ffmpeg_command() -> (){
    println!("caching start ffmpeg command");
    let _variable_list = {
        let _variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        serde_json::from_str::<FfmpegVariables>(&_variable_list).unwrap()
    };
    let mut ffmpegcommand = Command::new("ffmpeg");
    ffmpegcommand.arg("-i")
    .arg( _variable_list.stream_port.to_string().as_str())
    .arg("-f")
    .arg("segment")
    .arg("-reset_timestamps")
    .arg("1")
    .arg("-segment_time")
    .arg(_variable_list.action_replay_dur.to_string())
    .arg("-min_seg_duration")
    .arg("00:00:".to_string() + _variable_list.action_replay_dur.to_string().as_str())
    .arg("-strftime")
    .arg("1")
    .arg( _variable_list.stream_cache_dir.to_string()
    + "\\cache"
    + (_variable_list.uniqe_file_name.to_string()).as_str()
    + (_variable_list.video_format.to_string()).as_str());
    ffmpegcommand.status().expect("DID NTO WORK LOSER");


    /*
    ffmpeg -i udp://127.0.0.1:3000 -f segment 
    -reset_timestamps 1 -segment_time 15 -min_seg_duration 00:00:15 
    -strftime 1 "F:\School\Capstone\FFMPEGCache\%Y-%m-%d_%H-%M-%S.mp4"
     */
}

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

}

//https://doc.rust-lang.org/std/process/struct.Stdio.html#method.piped
//format("{#}")