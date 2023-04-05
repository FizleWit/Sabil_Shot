#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
//use std::{process::Command, string};
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::str;
extern crate serde_json;
use regex::Regex;
//use std::process::{Command, Stdio};
use std::process::Command;
//use std::sync::Exclusive;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
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
fn test2_btn_pressed() -> () {
    
    let output = return_audio_devices_ffmpeg_command();

    println!("{:#?}",output);
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
    caching_stop_ffmpeg_command().await;
}

#[tauri::command]
async fn action_replay_exe_btn_pressed() -> () {
    println!("action replay exe btn execute");
    action_replay_exe_ffmpeg_command().await;
}

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
   
    .arg("-ac")
    .arg("1")
    .arg("-ar")
    .arg("48000")
     .arg("-i")
     .arg("audio=".to_string() + _variable_list.microphone_device.as_str() )
    .arg("-f")
    .arg("dshow")
    .arg("-i")
    .arg("audio=\"What U Hear (Sound Blaster Audigy 5/Rx)\"")
    .arg("-filter_complex")
    .arg("amix=inputs=2")
    .arg("-f")
    .arg("gdigrab")
    .arg("-framerate")
    .arg("60")
    .arg("-show_region")
    .arg("1")
    .arg("-video_size")
    .arg("600x600")
    .arg("-offset_x")
    .arg("100")
    .arg("-offset_y")
    .arg("100")
    .arg("-i")
    .arg("desktop")
    .arg("-strftime")
    .arg("1")
    .arg("G:\\School\\Capstone\\screenshotFFMPEG\\%Y-%m-%d_%H-%M-%S.mp4");
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
    println!("Message from Rust: {}", _variable_list.stream_cache_dir);
}

async fn caching_stop_ffmpeg_command() -> () {
    println!("caching stop ffmpeg command");
    let _variable_list = {
        let _variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();

        
        serde_json::from_str::<FfmpegVariables>(&_variable_list).unwrap()
    };
    println!("Message from Rust: {}", _variable_list.stream_cache_dir);
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

fn return_audio_devices_ffmpeg_command() -> Result<String, std::io::Error> {
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

//https://doc.rust-lang.org/std/process/struct.Stdio.html#method.piped
//format("{#}")