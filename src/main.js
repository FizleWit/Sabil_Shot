const { invoke } = window.__TAURI__.tauri;
 


//



async function read_data_to_html() {
  document.querySelector("#screenshot_saving_dir_data_id").value = await invoke("return_screenshot_output_dir_data");
  document.querySelector("#video_output_dir_data_id").value =  await invoke("return_video_output_dir_data");
  document.querySelector("#video_caching_dir_data_id").value =  await invoke("return_stream_cache_dir_data");
  document.querySelector("#action_replay_data_id").value =await  invoke("return_action_replay_dur_data");
  document.querySelector("#streaming_url_data_id").value = await invoke("return_stream_port_data");
  document.querySelector("#unique_file_name_data_id").value = await invoke("return_unique_file_name_data");
  document.querySelector("#x_offset_data_id").value = await invoke("return_x_offset_data");
  document.querySelector("#y_offest_data_id").value = await invoke("return_y_offset_data");
  document.querySelector("#region_size_x_data_id").value = await invoke("return_video_size_x_data");
  document.querySelector("#region_size_y_data_id").value = await invoke("return_video_size_y_data");
  document.querySelector("#show_region_data_id").checked = await  invoke("return_show_region_data");
  document.querySelector("#framerate_data_id").value =await invoke("return_framerate_data");
  document.querySelector("#microphone_audio_input_id").value = await invoke("return_micropohne_device_data");
  document.querySelector("#desktop_audio_input_id").value =  await invoke("return_desktop_device_data");
//micropohone inputs still need here
}

async function write_data_to_json(){
  invoke("save_settings_btn_pressed", {
    framerate: parseInt(document.querySelector("#framerate_data_id").value,10),
    show_region: return_String_bool(document.querySelector("#show_region_data_id").value),
    video_size_x: parseInt(document.querySelector("#region_size_x_data_id").value,10),
    video_size_y: parseInt(document.querySelector("#region_size_y_data_id").value,10),
    x_offset:   parseInt(document.querySelector("#x_offset_data_id").value,10),
    y_offset: parseInt(document.querySelector("#y_offest_data_id").value,10),
    uniqe_file_name: document.querySelector("#unique_file_name_data_id").value,
    microphone_device: document.querySelector("#microphone_audio_input_id").value,
    desktop_audio_device: document.querySelector("#desktop_audio_input_id").value,
    microphone_device_audio_channels: parseInt("1",10),
    desktop_device_audio_channels: parseInt("2",10),
    microphone_device_audio_frequency: parseInt("1440",10),
    desktop_device_audio_frequency: parseInt("4800",10),
    stream_port: document.querySelector("#streaming_url_data_id").value,
    screenshot_output_dir: document.querySelector("#screenshot_saving_dir_data_id").value,
    stream_cache_dir: document.querySelector("#video_caching_dir_data_id").value,
    video_output_dir: document.querySelector("#video_output_dir_data_id").value,
    action_replay_dur: parseInt(document.querySelector("#action_replay_data_id").value,10),
    audio_format: ".wav",
    video_format: ".mp4",
    picture_format: ".jpeg"
  });
}
function return_String_bool(x){
  if(x == "false")
    return false;
  else
    return true;
}

function testDirectoryResult(){
  console.log(document.querySelector("#screenshot_saving_dir_data_id").value);
}

window.addEventListener("DOMContentLoaded", () => {
  document
    .querySelector("#save_settings_btn_id")
    //.addEventListener("click", () => write_data_to_json());
    .addEventListener("click",  () =>write_data_to_json());
});

window.addEventListener("DOMContentLoaded", () => {
  document
    .querySelector("#Load_settings_btn_id")
    .addEventListener("click", () => read_data_to_html());
});

//recording buttons
window.addEventListener("DOMContentLoaded", () => {
  document
    .querySelector("#recording_btn_start_id")
    .addEventListener("click", () => invoke("record_start_btn_pressed"));
});

window.addEventListener("DOMContentLoaded", () => {
  document
    .querySelector("#recording_btn_stop_id")
    .addEventListener("click", () => invoke("record_stop_btn_pressed"));
});


//screenshot buttons
window.addEventListener("DOMContentLoaded", () => {
  document
    .querySelector("#screenshot_btn_exe_id")
    .addEventListener("click", () => invoke("screenshot_exe_btn_pressed"));
});


//screen caching buttons
window.addEventListener("DOMContentLoaded", () => {
  document
    .querySelector("#screen_caching_btn_start_id")
    .addEventListener("click", () => invoke("screen_caching_start_btn_pressed"));
});
window.addEventListener("DOMContentLoaded", () => {
  document
    .querySelector("#screen_caching_btn_stop_id")
    .addEventListener("click", () => invoke("screen_caching_stop_btn_pressed"));
});


//action replay button
window.addEventListener("DOMContentLoaded", () => {
  document
    .querySelector("#action_replay_btn_exe_id")
    .addEventListener("click", () => invoke("action_replay_exe_btn_pressed"));
});


//Action Replay & Record
window.addEventListener("DOMContentLoaded", () => {
  document
    .querySelector("#action_replay_and_record_btn_start_id")
    .addEventListener("click", () => invoke("action_replay_and_record_btn_start_pressed"));
});
window.addEventListener("DOMContentLoaded", () => {
  document
    .querySelector("#action_replay_and_record_btn_stop_id")
    .addEventListener("click", () => invoke("action_replay_and_record_btn_stop_pressed"));
});

window.addEventListener("DOMContentLoaded", () => {
  document
    .querySelector("#test1_btn_id")
    .addEventListener("click", () => invoke("test2_btn_pressed"));
});

