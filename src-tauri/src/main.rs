#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
//use std::{process::Command, string};
use serde_derive::{Deserialize, Serialize};
use std::fs;
extern crate serde_json;
use std::process::Command;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    //add javascript script file to html reminder
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![return_framerate_data,return_show_region_data,return_video_size_x_data,return_video_size_y_data,return_unique_file_name_data,return_micropohne_device_data,return_desktop_device_data,return_stream_port_data,return_screenshot_output_dir_data,return_stream_cache_dir_data,return_video_output_dir_data,return_action_replay_dur_data,return_y_offset_data,return_x_offset_data,testfunction,test1_btn_pressed,test2_btn_pressed,greet,b_seetings_btn_pressed,record_start_btn_pressed,record_stop_btn_pressed,screenshot_exe_btn_pressed,screen_caching_start_btn_pressed,screen_caching_stop_btn_pressed,action_replay_exe_btn_pressed,action_replay_and_record_btn_start_pressed,action_replay_and_record_btn_stop_pressed,save_settings_btn_pressed])
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
    let mut variable_list = {
        let variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
      
        // Load the MissyFoodSchedule structure from the string.
        serde_json::from_str::<FfmpegVariables>(&variable_list).unwrap()
      
      };
      
      variable_list.stream_cache_dir = String::from("DEEON");
      println!("Message from Rust: {}", variable_list.stream_cache_dir);

    
        let text = serde_json::to_string(&variable_list).unwrap();
        fs::write("./Data/ffmpeg_variables.json", text).ok();
    
}

#[tauri::command]
fn test2_btn_pressed(test: &str) -> String {
    println!("test2 btn pressed execute");
    let variable_list= {
        let variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
      
        // Load the MissyFoodSchedule structure from the string.
        serde_json::from_str::<FfmpegVariables>(&variable_list).unwrap()
      
      };
      println!("Message from Rust: {}", variable_list.stream_cache_dir);
      println!("Message from Rust: {}", test);

      return variable_list.stream_cache_dir;
}

#[tauri::command(rename_all = "snake_case")]
fn save_settings_btn_pressed(framerate: i32, show_region: bool, video_size_x: i32, video_size_y: i32, x_offset:i32, y_offset:i32, uniqe_file_name: String, microphone_device: String, desktop_audio_device: String, microphone_device_audio_channels: i32, desktop_device_audio_channels: i32, microphone_device_audio_frequency: i32, desktop_device_audio_frequency: i32, stream_port: String, screenshot_output_dir: String, stream_cache_dir: String, video_output_dir: String, action_replay_dur: i32, audio_format: String, video_format: String, picture_format: String) -> () {
    println!("save settings btn execute");
    let mut variables_list= {
        let variables_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
      
        // Load the MissyFoodSchedule structure from the string.
        serde_json::from_str::<FfmpegVariables>(&variables_list).unwrap()
    };
    //println!("framerate 1: {} + {}", variables_list.framerate, framerate);
    variables_list.framerate = framerate;
    variables_list.show_region= show_region;
    variables_list.video_size_x= video_size_x;
    variables_list.video_size_y= video_size_y;
    variables_list.x_offset=x_offset;
    variables_list.y_offset=y_offset;
    variables_list.uniqe_file_name= uniqe_file_name;
    variables_list.microphone_device= microphone_device;
    variables_list.desktop_audio_device= desktop_audio_device;
    variables_list.microphone_device_audio_channels= microphone_device_audio_channels;
    variables_list.desktop_device_audio_channels= desktop_device_audio_channels;
    variables_list.microphone_device_audio_frequency= microphone_device_audio_frequency;
    variables_list.desktop_device_audio_frequency= desktop_device_audio_frequency;
    variables_list.stream_port= stream_port;
    variables_list.screenshot_output_dir= screenshot_output_dir;
    variables_list.stream_cache_dir= stream_cache_dir;
    variables_list.video_output_dir= video_output_dir;
    variables_list.action_replay_dur= action_replay_dur;
    variables_list.audio_format= audio_format;
    variables_list.video_format= video_format;
    variables_list.picture_format= picture_format;
    
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
fn record_start_btn_pressed() -> () {
    println!("record start btn execute");
}

#[tauri::command]
fn record_stop_btn_pressed() -> () {
    println!("record stop execute");

}

#[tauri::command]
async fn screenshot_exe_btn_pressed() -> () {
    println!("screenshot exe btn execute");
     screenshot_exe_ffmpeg_btn().await;
}

#[tauri::command]
fn screen_caching_start_btn_pressed() -> () {
    println!("scren caching start execute");
}

#[tauri::command]
fn screen_caching_stop_btn_pressed() -> () {
    println!("screen caching stop execute");
}

#[tauri::command]
fn action_replay_exe_btn_pressed() -> () {
    println!("action replay exe btn execute");
}

#[tauri::command]
fn action_replay_and_record_btn_start_pressed() -> () {
    println!("action replay and record btn start execute");
}

#[tauri::command]
async fn action_replay_and_record_btn_stop_pressed() -> () {
    println!("action replay and record btn stop execute");
}
#[tauri::command]
async fn return_framerate_data() ->i32{
    let mut variable_list = {
        let variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        serde_json::from_str::<FfmpegVariables>(&variable_list).unwrap()
      };
      return  variable_list.framerate;
}
#[tauri::command]
fn return_show_region_data() ->bool{
    let mut variable_list = {
        let variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        serde_json::from_str::<FfmpegVariables>(&variable_list).unwrap()
      };
      return  variable_list.show_region;
}
#[tauri::command]
fn return_video_size_x_data() ->i32{
    let mut variable_list = {
        let variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        serde_json::from_str::<FfmpegVariables>(&variable_list).unwrap()
      };
      return  variable_list.video_size_x;
}
#[tauri::command]
fn return_video_size_y_data() ->i32{
    let mut variable_list = {
        let variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        serde_json::from_str::<FfmpegVariables>(&variable_list).unwrap()
      };
      return  variable_list.video_size_y;
}
#[tauri::command]
fn return_unique_file_name_data() ->String{
    let mut variable_list = {
        let variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        serde_json::from_str::<FfmpegVariables>(&variable_list).unwrap()
      };
      return  variable_list.uniqe_file_name;
}
#[tauri::command]
fn return_micropohne_device_data() ->String{
    let mut variable_list = {
        let variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        serde_json::from_str::<FfmpegVariables>(&variable_list).unwrap()
      };
      return  variable_list.microphone_device;
}
#[tauri::command]
fn return_desktop_device_data() ->String{
    let mut variable_list = {
        let variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        serde_json::from_str::<FfmpegVariables>(&variable_list).unwrap()
      };
      return  variable_list.desktop_audio_device;
}
#[tauri::command]
fn return_stream_port_data() ->String{
    let mut variable_list = {
        let variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        serde_json::from_str::<FfmpegVariables>(&variable_list).unwrap()
      };
      return  variable_list.stream_port;
}
#[tauri::command]
fn return_screenshot_output_dir_data() ->String{
    let mut variable_list = {
        let variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        serde_json::from_str::<FfmpegVariables>(&variable_list).unwrap()
      };
      return  variable_list.screenshot_output_dir;
}
#[tauri::command]
fn return_stream_cache_dir_data() ->String{
    let mut variable_list = {
        let variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        serde_json::from_str::<FfmpegVariables>(&variable_list).unwrap()
      };
      return  variable_list.stream_cache_dir;
}
#[tauri::command]
fn return_video_output_dir_data() ->String{

    let mut variable_list = {
        let variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        serde_json::from_str::<FfmpegVariables>(&variable_list).unwrap()
      };
      return  variable_list.video_output_dir;
}
#[tauri::command]
fn return_action_replay_dur_data() ->i32{
    println!("return action replay data dur");
    let mut variable_list = {
        let variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        serde_json::from_str::<FfmpegVariables>(&variable_list).unwrap()
      };
      return  variable_list.action_replay_dur;
}
#[tauri::command]
fn return_x_offset_data() ->i32{
    println!("return offset data");
    let mut variable_list = {
        let variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        serde_json::from_str::<FfmpegVariables>(&variable_list).unwrap()
      };
      return  variable_list.x_offset;
}
#[tauri::command]
fn return_y_offset_data() ->i32{
    println!("return offset data");
    let mut variable_list = {
        let variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
        serde_json::from_str::<FfmpegVariables>(&variable_list).unwrap()
      };
      return  variable_list.y_offset;
}

//


#[derive(Deserialize, Serialize, Debug)]
struct FfmpegVariables {
    framerate: i32,
    show_region: bool,
    video_size_x: i32,
    video_size_y: i32,
    x_offset:i32,
    y_offset:i32,
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
    picture_format: String
}

//record start stop 
//screenshot exe
//screen caching start stop
//action replay exe
//action replay and record start stop

async fn screenshot_exe_ffmpeg_btn() ->(){
    println!("screenshot exe ffmpeg command");
    let variable_list = {
        let variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
      
        // Load the MissyFoodSchedule structure from the string.
        serde_json::from_str::<FfmpegVariables>(&variable_list).unwrap()
      
      };
    let mut screenshotcmd = Command::new("ffmpeg");
    screenshotcmd.arg("-hide_banner");
    screenshotcmd.arg("-f");
    screenshotcmd.arg("gdigrab");
    screenshotcmd.arg("-framerate");
    screenshotcmd.arg((variable_list.framerate.to_string()).as_str());
    screenshotcmd.arg("-show_region");
    if variable_list.show_region == false {
    screenshotcmd.arg("0");
    }
    else {
        screenshotcmd.arg("1");
    }
    screenshotcmd.arg("-video_size");
    screenshotcmd.arg(variable_list.video_size_x.to_string() +"x"+(variable_list.video_size_y.to_string()).as_str());
    screenshotcmd.arg("-offset_x");
    screenshotcmd.arg((variable_list.x_offset.to_string()).as_str());
    screenshotcmd.arg("-offset_y");
    screenshotcmd.arg((variable_list.y_offset.to_string()).as_str());
    screenshotcmd.arg("-i");
    screenshotcmd.arg("desktop");
    screenshotcmd.arg("-frames:v");
    screenshotcmd.arg("1");
    screenshotcmd.arg("-strftime");
    screenshotcmd.arg("1");
    screenshotcmd.arg( variable_list.screenshot_output_dir.to_string() + "\\"+(variable_list.uniqe_file_name.to_string()).as_str() +(variable_list.picture_format.to_string()).as_str());
    
    screenshotcmd
        .status()
        .expect("DID NTO WORK LOSER");
      

}
fn action_replay_ffmpeg_command() ->(){
    println!("action replay exe ffmpeg command");
    let mut variable_list = {
        let variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
      
        // Load the MissyFoodSchedule structure from the string.
        serde_json::from_str::<FfmpegVariables>(&variable_list).unwrap()
      
      };
      println!("Message from Rust: {}", variable_list.stream_cache_dir);

}
fn record_start_ffmepg_command() ->(){
    println!("record start ffmpeg command");
    let mut variable_list = {
        let variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
      
        // Load the MissyFoodSchedule structure from the string.
        serde_json::from_str::<FfmpegVariables>(&variable_list).unwrap()
      
      };
      println!("Message from Rust: {}", variable_list.stream_cache_dir);

}

fn record_stop_ffmpeg_command() -> ( ){
    println!("record stop ffmpeg command");
    let mut variable_list = {
        let variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
      
        // Load the MissyFoodSchedule structure from the string.
        serde_json::from_str::<FfmpegVariables>(&variable_list).unwrap()
      
      };
      println!("Message from Rust: {}", variable_list.stream_cache_dir);
}

fn caching_start_ffmepg_command() ->(){
    println!("caching start ffmpeg command");
    let mut variable_list = {
        let variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
      
        // Load the MissyFoodSchedule structure from the string.
        serde_json::from_str::<FfmpegVariables>(&variable_list).unwrap()
      
      };
      println!("Message from Rust: {}", variable_list.stream_cache_dir);

}

fn caching_stop_ffmpeg_command() -> ( ){
    println!("caching stop ffmpeg command");
    let mut variable_list = {
        let variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
      
        // Load the MissyFoodSchedule structure from the string.
        serde_json::from_str::<FfmpegVariables>(&variable_list).unwrap()
      
      };
      println!("Message from Rust: {}", variable_list.stream_cache_dir);
}

fn action_replay_and_record_start_ffmpeg_command() ->(){
    println!("action replay and record start ffmpeg command");
    let mut variable_list = {
        let variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
      
        // Load the MissyFoodSchedule structure from the string.
        serde_json::from_str::<FfmpegVariables>(&variable_list).unwrap()
      
      };
      println!("Message from Rust: {}", variable_list.stream_cache_dir);

}

fn action_replay_and_record_stop_ffmpeg_command() -> () {
    println!("action replay and record stop ffmpeg command");
    let mut variable_list = {
        let variable_list = std::fs::read_to_string("./Data/ffmpeg_variables.json").unwrap();
      
        // Load the MissyFoodSchedule structure from the string.
        serde_json::from_str::<FfmpegVariables>(&variable_list).unwrap()
      
      };
      println!("Message from Rust: {}", variable_list.stream_cache_dir);
}
























