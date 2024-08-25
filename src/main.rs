use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Entry, FileChooserDialog, Grid, Label, CheckButton, Orientation, Align};
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Enforce the dark theme
    std::env::set_var("GTK_THEME", "Adwaita:dark");

    let application = Application::new(
        Some("com.example.youtube_alarm"),
        Default::default(),
    );

    application.connect_activate(|app| {
        // Set up the main window
        let window = ApplicationWindow::new(app);
        window.set_title("YouTube Alarm");
        window.set_default_size(450, 250);
        window.set_resizable(false);

        // Set the application icon
        let icon_path = PathBuf::from("/home/shawaz/Code/youtubealarm/assets/youtube_alarm_icon.png");
        if icon_path.exists() {
            window.set_icon_from_file(icon_path.to_str().unwrap()).expect("Failed to set window icon");
        } else {
            eprintln!("Icon file not found: {:?}", icon_path);
        }

        // Create the main container with padding and spacing
        let vbox = gtk::Box::new(Orientation::Vertical, 10);
        vbox.set_margin_top(15);
        vbox.set_margin_bottom(15);
        vbox.set_margin_start(15);
        vbox.set_margin_end(15);

        // Create a grid for better layout control
        let grid = Grid::new();
        grid.set_column_spacing(10);
        grid.set_row_spacing(10);

        // Directory selector button
        let label_dir = Label::new(Some("Download Directory:"));
        label_dir.set_halign(Align::Start);
        grid.attach(&label_dir, 0, 0, 1, 1);

        let select_dir_button = Button::with_label("Select Download Directory");
        select_dir_button.set_halign(Align::Start);
        grid.attach(&select_dir_button, 1, 0, 2, 1);

        let current_dir_label = Label::new(Some("No directory selected"));
        current_dir_label.set_halign(Align::Start);
        grid.attach(&current_dir_label, 0, 1, 3, 1);

        // URL input field
        let label_url = Label::new(Some("YouTube URL:"));
        label_url.set_halign(Align::Start);
        grid.attach(&label_url, 0, 2, 1, 1);

        let url_entry = Entry::new();
        url_entry.set_placeholder_text(Some("Enter YouTube video URL"));
        grid.attach(&url_entry, 1, 2, 2, 1);

        // Add a checkbox to toggle video download
        let video_checkbox = CheckButton::with_label("Download Video (uncheck for audio only)");
        video_checkbox.set_active(false); // Default to audio only
        grid.attach(&video_checkbox, 1, 3, 2, 1);

        // Download button
        let download_button = Button::with_label("Download");
        download_button.set_halign(Align::Center);
        grid.attach(&download_button, 1, 4, 1, 1);

        // Add the grid to the main container
        vbox.pack_start(&grid, true, true, 0);

        // Add the container to the window
        window.add(&vbox);

        // Show all widgets
        window.show_all();

        // Variables to store directory and URL
        let download_dir = std::rc::Rc::new(std::cell::RefCell::new(None::<PathBuf>));

        // Directory selection button action
        let window_clone = window.clone();
        let download_dir_clone = download_dir.clone();
        let current_dir_label_clone = current_dir_label.clone();
        select_dir_button.connect_clicked(move |_| {
            let file_chooser = FileChooserDialog::new(
                Some("Select Download Directory"),
                Some(&window_clone), // Use the window as the parent
                gtk::FileChooserAction::SelectFolder,
            );

            file_chooser.add_buttons(&[
                ("Cancel", gtk::ResponseType::Cancel),
                ("Select", gtk::ResponseType::Accept),
            ]);

            let response = file_chooser.run();
            if response == gtk::ResponseType::Accept {
                if let Some(folder) = file_chooser.filename() {
                    *download_dir_clone.borrow_mut() = Some(folder.clone());
                    current_dir_label_clone.set_text(&folder.display().to_string());
                }
            }

            file_chooser.close();
        });

        // Download button action
        download_button.connect_clicked(move |_| {
            if let Some(download_dir) = download_dir.borrow().clone() {
                let youtube_url = url_entry.text();

                // Determine whether to download video or audio only
                let (format_flag, ext) = if video_checkbox.is_active() {
                    ("bestvideo+bestaudio", "mkv") // Use mkv to encapsulate both video and audio
                } else {
                    ("bestaudio", "mp3") // mp3 or another audio format
                };

                // Download the YouTube video or audio with the title as filename
                if !youtube_url.is_empty() {
                    let output = Command::new("yt-dlp")
                        .arg("-f")
                        .arg(format_flag)
                        .arg("-o")
                        .arg(format!("{}/%(title)s.{}", download_dir.display(), ext))
                        .arg(youtube_url.as_str())
                        .output();

                    match output {
                        Ok(output) => {
                            if output.status.success() {
                                println!("Download complete.");
                            } else {
                                eprintln!("Download failed: {}", String::from_utf8_lossy(&output.stderr));
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to execute yt-dlp command: {}", e);
                        }
                    }
                } else {
                    eprintln!("Please enter a YouTube URL.");
                }
            } else {
                eprintln!("No download directory selected.");
            }
        });
    });

    application.run();
}
