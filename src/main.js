const { invoke } = window.__TAURI__.tauri;
 


//



async function write_data() {
  document.querySelector("#screenshot_saving_dir_data_id").value = await invoke("return_screenshot_output_dir_data");
  document.querySelector("#video_output_dir_data_id").value = await invoke("return_video_output_dir_data");
  document.querySelector("#video_caching_dir_data_id").value =await invoke("return_stream_cache_dir_data");
  document.querySelector("#action_replay_data_id").value =await invoke("return_action_replay_dur_data");
  document.querySelector("#streaming_url_data_id").value =await invoke("return_stream_port_data");
  document.querySelector("#unique_file_name_data_id").value =await invoke("return_unique_file_name_data");
  document.querySelector("#x_offset_data_id").value =await invoke("return_x_offset_data");
  document.querySelector("#y_offest_data_id").value =await invoke("return_y_offset_data");
  document.querySelector("#region_size_x_data_id").value =await invoke("return_video_size_x_data");
  document.querySelector("#region_size_y_data_id").value =await invoke("return_video_size_y_data");
  document.querySelector("#show_region_data_id").value = await invoke("return_show_region_data");
  document.querySelector("#framerate_data_id").value =await invoke("return_framerate_data");
//micropohone inputs still need here
}

window.addEventListener("DOMContentLoaded", () => {
  document
    .querySelector("#Load_settings_btn_id")
    .addEventListener("click", () => write_data());
});

window.addEventListener("DOMContentLoaded", () => {
  document
    .querySelector("#a_settings_page_btn_id")
    .addEventListener("click", () => write_data());
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

//save settings button write json file
window.addEventListener("DOMContentLoaded", () => {
  document
  .querySelector("#save_settings_btn_id")
  //.addEventListener("click", () => write_data());
  .addEventListener("click", () => invoke("save_settings_btn_pressed"));
});

window.addEventListener("DOMContentLoaded", () => {
  document
    .querySelector("#test1_btn_id")
    .addEventListener("click", () => invoke("test1_btn_pressed", {
      framerate: document.querySelector("#screenshot_saving_dir_data_id").value,
    }));
});

//save settings button write json file
window.addEventListener("DOMContentLoaded", () => {
  document
  .querySelector("#test2_btn_id")
  .addEventListener("click", () => invoke("testfunction"));
});
