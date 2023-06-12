# Sabil Shot
## A screen Recording and Action Replay desktop solution written in Rust and utilizing Tauri. 


This project was created for my Capstone. It is a desktop application to that is designed to assist you in taking screenshots, recording desktop video, desktop audio, and microphone audio. There is also a feature for action replay. 

This whole project is avaliable open source and utilizes the following:

* Written in [Rust](https://www.rust-lang.org/)
* [Tauri](https://tauri.app/) for the multi-platform application deployment
* [FFMPEG](https://ffmpeg.org/) for capturing devices audio and video inputs to make the desired output file.
 
## Utilization

* Take shotshots of your computers desktop
* Create recordings of your computers screen that include the speaker output and user microphone
* Action Replay to capture recording history when enabled and executed

## Before you install

1. You must setup a Stereo Mix audio device to replay your desktop audio as a microphone input so it can be in the output file. I recommend the Realtek Audio Drivers. This app will not work correctly without a stero mix audio device. 
2. You will also need a microphone input. The app will not work without it.
3. You will also need to add FFMPEG to your path.
4. Have three destination folders created:
    4a. Cache folder for the temp files of the action replay to be saved to, this folders context will be deleted when the action replay is turned off. Do not store essential files in this folder.
    4b. Screenshot folder for screenshots created by the application
    4c. Recordings folder for the output of  action replay and recording videos.

## Install for use

* To install for general end user use you must have finished the before you install steps first.
* Download and run the .msi file from the releases section of this github page, follow the instructions from there.

Before utilizing the app you will also have to figure out your unique device names for the settings page.
run the command: 
 `ffmpeg -hide_banner -list_devices true -f dshow -i dummy`
 and take note of the device names inbetween the quotation marks save the exact name of your micrphone and desktop stero mix device, for me it was 
 Microphone: [dshow @ 000001b17421e600] "**Microphone (Arctis Nova Pro Wireless)**" (audio)
[dshow @ 000001b17421e600]   Alternative name "@device_cm_{33D9A762-90C8-11D0-BD43-00A0C911CE86}\wave_{DE99F572-E285-4EBC-A263-2D782B212FA7}"

Desktop Audio: [dshow @ 000001b17421e600] "**What U Hear (Sound Blaster Audigy 5/Rx)**" (audio)
[dshow @ 000001b17421e600]   Alternative name "@device_cm_{33D9A762-90C8-11D0-BD43-00A0C911CE86}\wave_{D9AE870B-D51C-4050-A7DE-20475EE70F0E}"

You want to use what is in bold but if that does not work for you try the alternative names. These are the names for the Audio Sources. 

You will also need to know the region of your screen you want to capture or offsets if you would prefer. If the region you configure is not within your displays range the app will crash. 

Search your computers settings for display and copy the **Display Resolution** to know the maximum size you can record.

## Install for personal development

* to work on this app yourself or customize it you must install the following

1. Tauri API 
2. Rust
3. [Cargo](https://crates.io/) (Rusts package manager)

Clone the repo and go to the file in your command line

run the command:

`cargo tauri build`

you will start a live development environment for the application from there. More documentation about this is avaliable on the [Tauri website](https://tauri.app/)


## Features

## Known Bugs

* App will crash if there is not two valid audio input devices at the same time. 
* Recording and doing an action replay at the same time causes crashes
* Action replay on a region smaller than the whole resolution of the display causes crashes


## Features in development

* It would be really cool if someone made these possible!

1. Execute in Linux environments
 
 At the moment the app will compile and start in a linux environment but none of the buttons that run ffmeg commands will work because they are configured to run in windows. A function must be added that checks if the application is running in linux and execute a different ffmeg command from there

 2. Dropdown list for audio devices utilizing command:
 `ffmpeg -hide_banner -list_devices true -f dshow -i dummy`

    Put the results of this command into an array and display on a dropdown list.
    When user selects a different device from the list edit the variable for the appropriate field in the json file

3. Customize audio input devices quanitity

    at the moment you MUST have two usable audio output devices selected to execute the app. Adding boolean for using microphone and using the desktop audio to configure the ffmpeg commands so in the case that someone wants to record just the desktop audio or just the microphone audio in their usage of the app. 

4. Automatically choose default output destinations of created videos and screenshots
    Ideally the first output place would be the current users video or picture directory and they can change it in the settings tab. 

5. Automatically set min and max values for the region size, x, and y offsets based off the dsiplay being used and system variables. 


